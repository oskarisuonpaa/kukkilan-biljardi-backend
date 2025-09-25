use super::{
    data_transfer_objects::{CreateMediaRequest, MediaResponse, UpdateMediaRequest},
    model::MediaRow,
};
use crate::{
    error::AppError,
    response::{Created, NoContent},
    state::AppState,
};
use axum::{
    Json, Router,
    body::Bytes,
    extract::{Multipart, Path, Query, State},
    routing::{get, post},
};
use chrono::Datelike;
use image::{GenericImageView, ImageReader};
use serde::Deserialize;
use tokio::{fs, io::AsyncWriteExt};
use uuid::Uuid;

pub fn routes() -> Router<AppState> {
    Router::new()
        .route("/api/media", get(list).post(create))
        .route("/api/media/upload", post(upload))
        .route(
            "/api/media/{id}",
            get(get_by_id).patch(update).delete(delete_one),
        )
}

#[derive(Debug, Deserialize)]
struct ListQuery {
    is_background: Option<bool>,
}

async fn list(
    State(state): State<AppState>,
    Query(q): Query<ListQuery>,
) -> Result<Json<Vec<MediaResponse>>, AppError> {
    let rows = state.media.list(q.is_background).await?;
    Ok(Json(rows.into_iter().map(row_to_response).collect()))
}

async fn get_by_id(
    State(state): State<AppState>,
    Path(id): Path<u32>,
) -> Result<Json<MediaResponse>, AppError> {
    let row = state.media.get(id).await?;
    Ok(Json(row_to_response(row)))
}

async fn create(
    State(state): State<AppState>,
    Json(body): Json<CreateMediaRequest>,
) -> Result<Created<MediaResponse>, AppError> {
    let new_id = state.media.create(body).await?;
    let row = state.media.get(new_id).await?;
    Ok(Created {
        location: format!("/api/media/{new_id}"),
        body: row_to_response(row),
    })
}

async fn update(
    State(state): State<AppState>,
    Path(id): Path<u32>,
    Json(body): Json<UpdateMediaRequest>,
) -> Result<Json<MediaResponse>, AppError> {
    state.media.update(id, body).await?;
    let row = state.media.get(id).await?;
    Ok(Json(row_to_response(row)))
}

async fn delete_one(
    State(state): State<AppState>,
    Path(id): Path<u32>,
) -> Result<NoContent, AppError> {
    // Fetch row so we know the stored file URL
    let row = state.media.get(id).await?;

    // Try to map public URL back to a filesystem path inside media_root.
    // Only attempt deletion if the URL is under our configured media_base_url.
    let file_delete_result: Result<(), AppError> = {
        let file_url = row.file_url.as_str();
        let base = state.config.media_base_url.trim_end_matches('/');
        if !file_url.starts_with(base) {
            // Not a local/media-root-managed file; skip deletion.
            Ok(())
        } else {
            let rel = file_url[base.len()..].trim_start_matches('/');
            let path = state.config.media_root.join(std::path::Path::new(rel));
            // Attempt to remove file; ignore NotFound, return error on other IO errors.
            match fs::remove_file(&path).await {
                Ok(_) => Ok(()),
                Err(e) if e.kind() == std::io::ErrorKind::NotFound => Ok(()),
                Err(_) => Err(AppError::Internal("Could not delete media file")),
            }
        }
    };

    // If file deletion failed with a real IO error, return it.
    file_delete_result?;

    // Now delete DB record (still runs even if file was missing)
    state.media.delete(id).await?;
    Ok(NoContent)
}

async fn upload(
    State(state): State<AppState>,
    mut multipart: Multipart,
) -> Result<Created<MediaResponse>, AppError> {
    // Default metadata
    let mut is_background = false;
    let mut kind = String::from("image");
    let mut alt_text = String::new();

    // The actual file buffer and original filename
    let mut file_bytes: Option<Bytes> = None;
    let mut original_filename: Option<String> = None;

    while let Some(field) = multipart
        .next_field()
        .await
        .map_err(|_| AppError::BadRequest("Invalid multipart payload"))?
    {
        let name = field.name().unwrap_or("").to_string();

        match name.as_str() {
            "file" => {
                original_filename = field.file_name().map(|s| s.to_string());
                file_bytes = Some(
                    field
                        .bytes()
                        .await
                        .map_err(|_| AppError::BadRequest("Could not read uploaded file"))?,
                );
            }
            "is_background" => {
                let v = field.text().await.unwrap_or_default();
                is_background = matches!(v.as_str(), "true" | "1" | "yes" | "on");
            }
            "kind" => {
                let v = field.text().await.unwrap_or_default();
                if v == "image" || v == "video" {
                    kind = v;
                }
            }
            "alt_text" => {
                alt_text = field.text().await.unwrap_or_default();
            }
            _ => {}
        }
    }

    let file_bytes = file_bytes.ok_or(AppError::BadRequest("Missing file field"))?;

    // Build destination path: {MEDIA_ROOT}/{YYYY}/{MM}/{unique-filename}
    let today = chrono::Utc::now().date_naive();
    let year = today.year();
    let month = today.month();

    let root = state.config.media_root.clone();
    let dir = root.join(format!("{year:04}")).join(format!("{month:02}"));
    tokio::fs::create_dir_all(&dir)
        .await
        .map_err(|_| AppError::Internal("Could not create upload directory"))?;

    // Safe file name
    let base = original_filename.unwrap_or_else(|| "upload.bin".to_string());
    let sanitized = base.replace(
        |c: char| !c.is_ascii_alphanumeric() && c != '.' && c != '_' && c != '-',
        "_",
    );

    // Ensure uniqueness
    let unique = format!("{}-{}", Uuid::new_v4(), sanitized);
    let full_path = dir.join(&unique);

    // Write file
    let mut file = tokio::fs::File::create(&full_path)
        .await
        .map_err(|_| AppError::Internal("Could not create file"))?;
    file.write_all(&file_bytes)
        .await
        .map_err(|_| AppError::Internal("Could not create file"))?;
    file.flush()
        .await
        .map_err(|_| AppError::Internal("Could not create file"))?;

    // Build the public URL that will be stored in the database
    let rel = full_path
        .strip_prefix(&state.config.media_root)
        .unwrap()
        .to_owned();
    let public_url = format!(
        "{}/{}",
        state.config.media_base_url.trim_end_matches('/'),
        rel.to_string_lossy().replace('\\', "/")
    );

    // Read dimensions from the actual saved file (use full_path, not the relative path)
    let (width_i32, height_i32) = read_image_dimensions_if_any(&full_path)
        .await
        .unwrap_or((None, None));
    // image crate yields non-negative dimensions; map directly to u32
    let width = width_i32.map(|w| w as u32);
    let height = height_i32.map(|h| h as u32);

    // Create the media row
    let new_id = state
        .media
        .create(super::data_transfer_objects::CreateMediaRequest {
            is_background: Some(is_background),
            kind,
            file_url: public_url.clone(),
            alt_text: Some(alt_text),
            width,
            height,
        })
        .await?;

    let row = state.media.get(new_id).await?;
    Ok(Created {
        location: format!("/api/media/{new_id}"),
        body: row_to_response(row),
    })
}

fn row_to_response(row: MediaRow) -> MediaResponse {
    MediaResponse {
        id: row.id,
        is_background: row.is_background,
        kind: row.kind,
        file_url: row.file_url,
        alt_text: row.alt_text,
        width: row.width,
        height: row.height,
    }
}

async fn read_image_dimensions_if_any(
    saved_path: &std::path::Path,
) -> anyhow::Result<(Option<i32>, Option<i32>)> {
    // Quick magic-bytes sniff
    let bytes = fs::read(saved_path).await?;
    let kind = infer::get(&bytes);

    // Only images have meaningful pixel dimensions
    let is_image = kind
        .map(|k| k.mime_type().starts_with("image/"))
        .unwrap_or(false);

    if !is_image {
        return Ok((None, None));
    }

    // Use the image crate to get dimensions without fully decoding if possible
    // If format cannot be guessed, fall back to full decode.
    let dims = match ImageReader::new(std::io::Cursor::new(bytes)).with_guessed_format() {
        Ok(reader) => {
            // Try lightweight dimensions extraction first (supported on newer versions)
            if let Ok(d) = reader.into_dimensions() {
                Some(d)
            } else {
                None
            }
        }
        Err(_) => None,
    };

    let (w, h) = if let Some((w, h)) = dims {
        (Some(w as i32), Some(h as i32))
    } else {
        // Fallback: reopen from path and fully decode
        match ImageReader::open(saved_path)
            .and_then(|r| r.with_guessed_format())
            .map_err(|e| e.into())
            .and_then(|r| r.decode())
        {
            Ok(img) => {
                let (w, h) = img.dimensions();
                (Some(w as i32), Some(h as i32))
            }
            Err(_) => (None, None),
        }
    };

    Ok((w, h))
}

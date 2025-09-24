use crate::error::AppError;

use super::{
    data_transfer_objects::{CreateMediaRequest, UpdateMediaRequest},
    model::MediaRow,
    repository::DynamicMediaRepository,
};

#[derive(Clone)]
pub struct MediaService {
    repository: DynamicMediaRepository,
}

impl MediaService {
    pub fn new(repository: DynamicMediaRepository) -> Self {
        Self { repository }
    }

    pub async fn list(&self, only_backgrounds: Option<bool>) -> Result<Vec<MediaRow>, AppError> {
        Ok(self.repository.list(only_backgrounds).await?)
    }

    pub async fn get(&self, id: u32) -> Result<MediaRow, AppError> {
        self.repository
            .get_by_id(id)
            .await?
            .ok_or(AppError::NotFound("Media not found"))
    }

    pub async fn create(&self, req: CreateMediaRequest) -> Result<u32, AppError> {
        if req.kind != "image" && req.kind != "video" {
            return Err(AppError::BadRequest("kind must be 'image' or 'video'"));
        }
        let new_id = self
            .repository
            .insert(
                req.is_background.unwrap_or(false),
                &req.kind,
                &req.file_url,
                req.alt_text.as_deref().unwrap_or(""),
                req.width,
                req.height,
            )
            .await?;
        Ok(new_id)
    }

    pub async fn update(&self, id: u32, req: UpdateMediaRequest) -> Result<(), AppError> {
        if let Some(kind) = &req.kind {
            if kind != "image" && kind != "video" {
                return Err(AppError::BadRequest("kind must be 'image' or 'video'"));
            }
        }
        let n = self
            .repository
            .update(
                id,
                req.is_background,
                req.kind.as_deref(),
                req.file_url.as_deref(),
                req.alt_text.as_deref(),
                req.width,
                req.height,
            )
            .await?;
        if n == 0 {
            Err(AppError::NotFound("Media not found"))
        } else {
            Ok(())
        }
    }

    pub async fn delete(&self, id: u32) -> Result<(), AppError> {
        let ok = self.repository.delete(id).await?;
        if ok {
            Ok(())
        } else {
            Err(AppError::NotFound("Media not found"))
        }
    }
}

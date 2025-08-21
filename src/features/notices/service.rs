use crate::{
    error::AppError,
    features::notices::{
        data_transfer_objects::{CreateNoticeRequest, UpdateNoticeRequest},
        model::NoticeRow,
    },
};

use super::repository::DynamicNoticesRepository;

#[derive(Clone)]
pub struct NoticesService {
    repository: DynamicNoticesRepository,
}

impl NoticesService {
    pub fn new(repository: DynamicNoticesRepository) -> Self {
        Self { repository }
    }

    pub async fn list(&self) -> Result<Vec<NoticeRow>, AppError> {
        Ok(self.repository.list().await?)
    }

    pub async fn create(&self, request: CreateNoticeRequest) -> Result<NoticeRow, AppError> {
        let is_active = request.active;
        let new_notice_id = self
            .repository
            .insert(&request.title, &request.content, is_active)
            .await?;

        let notice_row = self
            .repository
            .get_by_id(new_notice_id)
            .await?
            .ok_or(AppError::NotFound("Failed to fetch newly created notice"))?;

        Ok(notice_row)
    }

    pub async fn update(
        &self,
        notice_id: u32,
        request: UpdateNoticeRequest,
    ) -> Result<NoticeRow, AppError> {
        let no_fields_provided =
            request.title.is_none() && request.content.is_none() && request.active.is_none();

        if no_fields_provided {
            return Err(AppError::BadRequest("No fields provided"));
        }

        let update_result = self
            .repository
            .update(
                notice_id,
                request.title.as_deref(),
                request.content.as_deref(),
                request.active,
            )
            .await;

        match update_result {
            Ok(_rows_affected) => {
                let notice_row = self.repository.get_by_id(notice_id).await?;
                notice_row.ok_or(AppError::NotFound("Notice not found"))
            }
            Err(sqlx::Error::Database(database_error)) => {
                Err(AppError::Database(sqlx::Error::Database(database_error)))
            }
            Err(error) => Err(AppError::Database(error)),
        }
    }

    pub async fn delete(&self, notice_id: u32) -> Result<(), AppError> {
        let was_deleted = self
            .repository
            .delete(notice_id)
            .await
            .map_err(AppError::Database)?;

        if !was_deleted {
            Err(AppError::NotFound("Notice not found"))
        } else {
            Ok(())
        }
    }
}

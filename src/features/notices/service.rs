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

    pub async fn get_by_id(&self, id: u32) -> Result<NoticeRow, AppError> {
        let row = self
            .repository
            .get_by_id(id)
            .await?
            .ok_or(AppError::NotFound("Notice not found".into()))?;

        Ok(row)
    }

    pub async fn create(&self, request: CreateNoticeRequest) -> Result<NoticeRow, AppError> {
        let active = request.active.unwrap_or(true);
        let id = self
            .repository
            .insert(&request.name, &request.content, active)
            .await?;

        let row = self
            .repository
            .get_by_id(id)
            .await?
            .ok_or(AppError::NotFound("Failed to fetch newly created notice"))?;

        Ok(row)
    }

    pub async fn update(
        &self,
        id: u32,
        request: UpdateNoticeRequest,
    ) -> Result<NoticeRow, AppError> {
        if request.title.is_none() && request.content.is_none() && request.active.is_none() {
            return Err(AppError::BadRequest("No fields provided"));
        }

        let update_result = self
            .repository
            .update(
                id,
                request.title.as_deref(),
                request.content.as_deref(),
                request.active,
            )
            .await;

        match update_result {
            Ok(_rows_affected) => {
                let row = self.repository.get_by_id(id).await?;
                row.ok_or(AppError::NotFound("Notice not found"))
            }
            Err(sqlx::Error::Database(database_error)) => {
                Err(AppError::Database(sqlx::Error::Database(database_error)))
            }
            Err(error) => Err(AppError::Database(error)),
        }
    }

    pub async fn delete(&self, id: u32) -> Result<(), AppError> {
        let n = self
            .repository
            .delete(id)
            .await
            .map_err(AppError::Database)?;

        if n == false {
            Err(AppError::NotFound("Notice not found"))
        } else {
            Ok(())
        }
    }
}

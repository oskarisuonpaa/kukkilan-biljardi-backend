use crate::{
    error::AppError,
    features::calendars::{
        data_transfer_objects::{CreateCalendarRequest, UpdateCalendarRequest},
        model::CalendarRow,
    },
};

use super::repository::DynamicCalendarsRepository;

#[derive(Clone)]
pub struct CalendarsService {
    repository: DynamicCalendarsRepository,
}
impl CalendarsService {
    pub fn new(repository: DynamicCalendarsRepository) -> Self {
        Self { repository }
    }

    pub async fn list(&self) -> Result<Vec<CalendarRow>, AppError> {
        Ok(self.repository.list().await?)
    }

    pub async fn get(&self, id: u32) -> Result<CalendarRow, AppError> {
        self.repository
            .get_by_id(id)
            .await?
            .ok_or(AppError::NotFound("Calendar not found"))
    }

    pub async fn create(&self, req: CreateCalendarRequest) -> Result<u32, AppError> {
        let id = self
            .repository
            .insert(&req.name, req.active.unwrap_or(true), req.thumbnail_id)
            .await?;
        Ok(id)
    }

    pub async fn update(&self, id: u32, req: UpdateCalendarRequest) -> Result<(), AppError> {
        let n = self
            .repository
            .update(
                id,
                req.name.as_deref(),
                req.active,
                Some(req.thumbnail_id), // wrap Option in Option to signal presence
            )
            .await?;

        if n == 0 {
            Err(AppError::NotFound("Calendar not found"))
        } else {
            Ok(())
        }
    }

    pub async fn delete(&self, id: u32) -> Result<(), AppError> {
        let n = self.repository.delete(id).await?;
        if n {
            Ok(())
        } else {
            Err(AppError::NotFound("Calendar not found"))
        }
    }
}

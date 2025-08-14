use crate::{
    error::AppError,
    features::calendars::{
        data_transfer_objects::{CalendarResponse, CreateCalendarRequest},
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

    pub async fn get(&self, id: u64) -> sqlx::Result<Option<CalendarRow>> {
        Ok(self.repository.get(id).await?)
    }

    pub async fn create(
        &self,
        request: CreateCalendarRequest,
    ) -> sqlx::Result<CalendarRow, AppError> {
        let id = self
            .repository
            .insert(&request.name, request.active.unwrap_or(true))
            .await?;

        Ok(self.get(id).await?.unwrap())
    }
}

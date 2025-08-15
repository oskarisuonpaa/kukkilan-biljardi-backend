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

    pub async fn get_by_id(&self, id: u64) -> Result<CalendarRow, AppError> {
        match self.repository.get_by_id(id).await? {
            Some(row) => Ok(row),
            None => Err(AppError::NotFound("Calendar not found".into())),
        }
    }

    pub async fn create(&self, request: CreateCalendarRequest) -> Result<CalendarRow, AppError> {
        if self.repository.get_by_name(&request.name).await?.is_some() {
            return Err(AppError::Conflict("Calendar name is already in use"));
        }

        let active = request.active.unwrap_or(true);
        let id = self.repository.insert(&request.name, active).await?;

        let row = self
            .repository
            .get_by_id(id)
            .await?
            .ok_or(AppError::NotFound("Failed to fetch newly created calendar"))?;

        Ok(row)
    }

    pub async fn update(
        &self,
        id: u64,
        request: UpdateCalendarRequest,
    ) -> Result<CalendarRow, AppError> {
        if self.repository.get_by_id(id).await?.is_none() {
            return Err(AppError::NotFound("Calendar not found".into()));
        }

        let updated = self
            .repository
            .update(id, request.name.as_deref(), request.active)
            .await?;

        if updated {
            Ok(self.get_by_id(id).await?)
        } else {
            Err(AppError::Database(sqlx::Error::InvalidArgument(
                "No arguments given".into(),
            )))
        }
    }
}

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
        let row = self
            .repository
            .get_by_id(id)
            .await?
            .ok_or(AppError::NotFound("Calendar not found".into()))?;

        Ok(row)
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
        if request.name.is_none() && request.active.is_none() {
            return Err(AppError::BadRequest("No fields provided"));
        }

        if let Some(ref new_name) = request.name {
            if let Some(existing) = self.repository.get_by_name(new_name).await? {
                if existing.id != id {
                    return Err(AppError::Conflict("Calendar name is already in use"));
                }
            }
        }

        match self
            .repository
            .update(id, request.name.as_deref(), request.active)
            .await
        {
            Ok(_rows_affected) => {
                if let Some(row) = self.repository.get_by_id(id).await? {
                    Ok(row)
                } else {
                    Err(AppError::NotFound("Calendar not found"))
                }
            }
            Err(sqlx::Error::Database(db_err)) => {
                if db_err.code().as_deref() == Some("1062") {
                    Err(AppError::Conflict("Calendar name is already in use"))
                } else {
                    Err(AppError::Database(sqlx::Error::Database(db_err)))
                }
            }
            Err(e) => Err(AppError::Database(e)),
        }
    }
}

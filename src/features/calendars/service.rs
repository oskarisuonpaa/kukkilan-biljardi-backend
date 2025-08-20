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

    pub async fn get_by_id(&self, id: u32) -> Result<CalendarRow, AppError> {
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

        let active = request.active;
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
        id: u32,
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

        let update_result = self
            .repository
            .update(id, request.name.as_deref(), request.active)
            .await;

        match update_result {
            Ok(_rows_affected) => {
                let row = self.repository.get_by_id(id).await?;
                row.ok_or(AppError::NotFound("Calendar not found"))
            }
            Err(sqlx::Error::Database(database_error))
                if database_error.code().as_deref() == Some("1062") =>
            {
                Err(AppError::Conflict("Calendar name is already in use"))
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
            Err(AppError::NotFound("Calendar not found"))
        } else {
            Ok(())
        }
    }
}

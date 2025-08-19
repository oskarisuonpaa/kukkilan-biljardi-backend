use super::{model::BookingRow, repository::DynamicBookingsRepository};
use crate::{error::AppError, features::bookings::data_transfer_objects::CreateBookingRequest};

#[derive(Clone)]
pub struct BookingsService {
    repository: DynamicBookingsRepository,
}

impl BookingsService {
    pub fn new(repository: DynamicBookingsRepository) -> Self {
        Self { repository }
    }

    pub async fn list(&self, calendar_id: u32) -> Result<Vec<BookingRow>, AppError> {
        Ok(self.repository.list(calendar_id).await?)
    }

    pub async fn create(&self, request: CreateBookingRequest) -> Result<BookingRow, AppError> {
        /* TODO: Check that there is no overlap */

        let id = self.repository.insert(request).await?;

        let row = self
            .repository
            .get(id)
            .await?
            .ok_or(AppError::NotFound("Failed to fetch newly created booking"))?;

        Ok(row)
    }

    pub async fn delete(&self, id: u32) -> Result<(), AppError> {
        let n = self
            .repository
            .delete(id)
            .await
            .map_err(AppError::Database)?;

        if n == false {
            Err(AppError::NotFound("Booking not found"))
        } else {
            Ok(())
        }
    }
}

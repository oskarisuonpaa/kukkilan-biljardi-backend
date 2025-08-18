use super::{model::BookingRow, repository::DynamicBookingsRepository};
use crate::error::AppError;

#[derive(Clone)]
pub struct BookingsService {
    repository: DynamicBookingsRepository,
}

impl BookingsService {
    pub fn new(repository: DynamicBookingsRepository) -> Self {
        Self { repository }
    }

    pub async fn list(&self, calendar_id: u64) -> Result<Vec<BookingRow>, AppError> {
        Ok(self.repository.list(calendar_id).await?)
    }
}

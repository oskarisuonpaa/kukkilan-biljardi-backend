use super::{model::BookingRow, repository::DynamicBookingsRepository};
use crate::{
    error::AppError,
    features::{
        bookings::data_transfer_objects::{
            BookingOverview, CreateBookingRequest, DailyOverviewResponse,
        },
        calendars::repository::DynamicCalendarsRepository,
    },
};

#[derive(Clone)]
pub struct BookingsService {
    repository: DynamicBookingsRepository,
    calendars_repository: DynamicCalendarsRepository,
}

impl BookingsService {
    pub fn new(
        repository: DynamicBookingsRepository,
        calendars_repository: DynamicCalendarsRepository,
    ) -> Self {
        Self {
            repository,
            calendars_repository,
        }
    }

    pub async fn list(&self, calendar_id: u32) -> Result<Vec<BookingRow>, AppError> {
        Ok(self.repository.list(calendar_id).await?)
    }

    pub async fn create(&self, request: CreateBookingRequest) -> Result<BookingRow, AppError> {
        // Validate booking is not in the past
        let now = chrono::Utc::now();
        if request.start <= now {
            return Err(AppError::BadRequest("Booking cannot be in the past"));
        }

        // Validate end time is after start time
        if request.end <= request.start {
            return Err(AppError::BadRequest("End time must be after start time"));
        }

        // Validate minimum booking duration (1 hour)
        let duration = request.end - request.start;
        if duration.num_minutes() < 60 {
            return Err(AppError::BadRequest("Booking must be at least 1 hour long"));
        }

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
            .map_err(AppError::DatabaseError)?;

        if n == false {
            Err(AppError::NotFound("Booking not found"))
        } else {
            Ok(())
        }
    }

    pub async fn daily_overview(&self, date: &str) -> Result<DailyOverviewResponse, AppError> {
        // Get all bookings for the date
        let bookings = self.repository.list_by_date(date).await?;
        let total_bookings = bookings.len();

        if bookings.is_empty() {
            return Ok(DailyOverviewResponse {
                date: date.to_string(),
                bookings: vec![],
                total_bookings: 0,
                total_revenue: 0,
                tables_used: 0,
                utilization_percentage: 0.0,
            });
        }

        // Get all calendars for names and pricing
        let all_calendars = self.calendars_repository.list().await?;
        let calendar_map: std::collections::HashMap<u32, _> =
            all_calendars.into_iter().map(|cal| (cal.id, cal)).collect();

        // Create detailed booking overview
        let mut overview_bookings = Vec::new();
        let mut total_revenue = 0u32;
        let mut tables_used = std::collections::HashSet::new();

        for booking in bookings {
            tables_used.insert(booking.calendar_id);

            let calendar = calendar_map.get(&booking.calendar_id);
            let calendar_name = calendar
                .map(|c| c.name.clone())
                .unwrap_or("Unknown".to_string());

            // Derive table type from name (simple heuristic)
            let calendar_type = calendar_name
                .to_lowercase()
                .split_whitespace()
                .next()
                .map(|word| match word {
                    "snooker" => "Snooker".to_string(),
                    "pool" | "biljardi" => "Pool".to_string(),
                    _ => "Table".to_string(),
                })
                .unwrap_or("Table".to_string());

            // Calculate duration and price
            let duration = booking.ends_at_utc - booking.starts_at_utc;
            let duration_hours = duration.num_minutes() as f64 / 60.0;

            let total_price = if let Some(calendar) = calendar {
                calendar
                    .hourly_price_cents
                    .map(|price| (price as f64 * duration_hours) as u32)
            } else {
                None
            };

            if let Some(price) = total_price {
                total_revenue += price;
            }

            // Format times (convert to local time - assuming Europe/Helsinki)
            let local_start = booking.starts_at_utc + chrono::Duration::hours(2); // UTC+2
            let local_end = booking.ends_at_utc + chrono::Duration::hours(2);

            overview_bookings.push(BookingOverview {
                id: booking.id,
                calendar_name,
                calendar_type,
                date: date.to_string(),
                start_time: local_start.format("%H:%M").to_string(),
                end_time: local_end.format("%H:%M").to_string(),
                duration_hours,
                customer_name: booking.customer_name,
                customer_phone: booking.customer_phone,
                customer_email: if booking.customer_email.is_empty() {
                    None
                } else {
                    Some(booking.customer_email)
                },
                created_at: chrono::DateTime::<chrono::Utc>::from_naive_utc_and_offset(
                    booking.created_at,
                    chrono::Utc,
                ),
                total_price,
            });
        }

        // Calculate utilization (simplified - based on active tables vs total tables)
        let total_tables = calendar_map.len();
        let utilization_percentage = if total_tables > 0 {
            (tables_used.len() as f64 / total_tables as f64) * 100.0
        } else {
            0.0
        };

        Ok(DailyOverviewResponse {
            date: date.to_string(),
            bookings: overview_bookings,
            total_bookings,
            total_revenue,
            tables_used: tables_used.len(),
            utilization_percentage,
        })
    }
}

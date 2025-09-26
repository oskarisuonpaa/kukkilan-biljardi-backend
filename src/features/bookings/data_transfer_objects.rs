use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize)]
pub struct BookingResponse {
    pub id: u32,
    pub calendar_id: u32,
    pub name: String,
    pub email: String,
    pub phone: String,
    pub notes: Option<String>,
    pub start: DateTime<Utc>,
    pub end: DateTime<Utc>,
}

#[derive(Debug, Deserialize)]
pub struct CreateBookingRequest {
    pub calendar_id: u32,
    pub name: String,
    pub email: String,
    pub phone: String,
    pub notes: Option<String>,
    pub start: DateTime<Utc>,
    pub end: DateTime<Utc>,
}

#[derive(Debug, Serialize)]
pub struct BookingOverview {
    pub id: u32,
    pub calendar_name: String,
    pub calendar_type: String,
    pub date: String,
    pub start_time: String,
    pub end_time: String,
    pub duration_hours: f64,
    pub customer_name: String,
    pub customer_phone: String,
    pub customer_email: Option<String>,
    pub created_at: DateTime<Utc>,
    pub total_price: Option<u32>, // in cents
}

#[derive(Debug, Serialize)]
pub struct DailyOverviewResponse {
    pub date: String,
    pub bookings: Vec<BookingOverview>,
    pub total_bookings: usize,
    pub total_revenue: u32, // in cents
    pub tables_used: usize,
    pub utilization_percentage: f64,
}

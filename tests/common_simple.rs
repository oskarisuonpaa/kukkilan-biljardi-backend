#![allow(dead_code)]

use chrono::{DateTime, Duration, Utc};

pub struct TestContext {
    pub pool: sqlx::MySqlPool,
}

impl TestContext {
    pub async fn new() -> Self {
        // Load environment variables
        dotenvy::dotenv().ok();

        let database_url =
            std::env::var("DATABASE_URL").expect("DATABASE_URL must be set for tests");

        let pool = sqlx::MySqlPool::connect(&database_url)
            .await
            .expect("Failed to connect to database");

        // Run migrations
        sqlx::migrate!("./migrations")
            .run(&pool)
            .await
            .expect("Failed to run migrations");

        Self { pool }
    }

    pub async fn cleanup(&self) {
        // Clean up test data in correct order (due to foreign key constraints)
        // Delete bookings first
        let _ = sqlx::query("DELETE FROM bookings WHERE name LIKE '%Test%' OR email LIKE '%test%' OR email LIKE '%example.com%'")
            .execute(&self.pool).await;

        // Then calendars
        let _ = sqlx::query("DELETE FROM calendars WHERE name LIKE '%Test Calendar%'")
            .execute(&self.pool)
            .await;
    }
}

pub mod builders {
    use chrono::{DateTime, Duration, Utc};

    #[derive(Debug)]
    pub struct CreateCalendarRequest {
        pub name: String,
        pub active: Option<bool>,
        pub thumbnail_id: Option<i32>,
    }

    #[derive(Debug)]
    pub struct CreateBookingRequest {
        pub calendar_id: i32,
        pub customer_name: String,
        pub customer_email: String,
        pub customer_phone: String,
        pub starts_at_utc: DateTime<Utc>,
        pub ends_at_utc: DateTime<Utc>,
        pub customer_notes: Option<String>,
    }

    pub struct CalendarBuilder {
        name: String,
        active: Option<bool>,
        thumbnail_id: Option<i32>,
    }

    impl CalendarBuilder {
        pub fn new(name: &str) -> Self {
            Self {
                name: name.to_string(),
                active: None,
                thumbnail_id: None,
            }
        }

        pub fn active(mut self, active: bool) -> Self {
            self.active = Some(active);
            self
        }

        pub fn thumbnail_id(mut self, thumbnail_id: i32) -> Self {
            self.thumbnail_id = Some(thumbnail_id);
            self
        }

        pub fn build(self) -> CreateCalendarRequest {
            CreateCalendarRequest {
                name: self.name,
                active: self.active,
                thumbnail_id: self.thumbnail_id,
            }
        }
    }

    pub struct BookingBuilder {
        calendar_id: i32,
        name: String,
        email: String,
        phone: Option<String>,
        start_time: DateTime<Utc>,
        end_time: DateTime<Utc>,
        notes: Option<String>,
    }

    impl BookingBuilder {
        pub fn new(calendar_id: i32, name: &str, email: &str) -> Self {
            let start_time = Utc::now() + Duration::hours(24); // Default to tomorrow
            let end_time = start_time + Duration::hours(1); // Default 1 hour duration

            Self {
                calendar_id,
                name: name.to_string(),
                email: email.to_string(),
                phone: None,
                start_time,
                end_time,
                notes: None,
            }
        }

        pub fn phone(mut self, phone: &str) -> Self {
            self.phone = Some(phone.to_string());
            self
        }

        pub fn start_time(mut self, start_time: DateTime<Utc>) -> Self {
            self.start_time = start_time;
            self
        }

        pub fn end_time(mut self, end_time: DateTime<Utc>) -> Self {
            self.end_time = end_time;
            self
        }

        pub fn duration(mut self, hours: i64) -> Self {
            self.end_time = self.start_time + Duration::hours(hours);
            self
        }

        pub fn notes(mut self, notes: &str) -> Self {
            self.notes = Some(notes.to_string());
            self
        }

        pub fn build(self) -> CreateBookingRequest {
            CreateBookingRequest {
                calendar_id: self.calendar_id,
                customer_name: self.name,
                customer_email: self.email,
                customer_phone: self.phone.unwrap_or_else(|| "000-000-0000".to_string()),
                starts_at_utc: self.start_time,
                ends_at_utc: self.end_time,
                customer_notes: self.notes,
            }
        }
    }
}

pub mod fixtures {
    use super::builders::*;
    use chrono::{Duration, Utc};

    pub fn sample_calendar() -> CreateCalendarRequest {
        CalendarBuilder::new("Test Snooker Table")
            .active(true)
            .build()
    }

    pub fn sample_booking(calendar_id: i32) -> CreateBookingRequest {
        BookingBuilder::new(calendar_id, "Test User", "test@example.com")
            .phone("+358401234567")
            .duration(2)
            .notes("Test booking note")
            .build()
    }

    pub fn booking_tomorrow(calendar_id: i32) -> CreateBookingRequest {
        let start = Utc::now() + Duration::hours(24);
        BookingBuilder::new(calendar_id, "Tomorrow User", "tomorrow@example.com")
            .start_time(start)
            .duration(2)
            .build()
    }
}

#![allow(dead_code)]

use sqlx::MySqlPool;
use std::sync::Once;

static INIT: Once = Once::new();

pub struct TestContext {
    pub pool: MySqlPool,
}

impl TestContext {
    pub async fn new() -> Self {
        INIT.call_once(|| {
            // Initialize test environment
            dotenvy::from_filename(".env.test").ok();
        });

        let database_url = std::env::var("TEST_DATABASE_URL")
            .unwrap_or_else(|_| "mysql://root:password@localhost:3306/kukkila_test".to_string());

        let pool = MySqlPool::connect(&database_url)
            .await
            .expect("Failed to connect to test database");

        // Run migrations
        sqlx::migrate!("./migrations")
            .run(&pool)
            .await
            .expect("Failed to run test migrations");

        TestContext { pool }
    }

    pub async fn cleanup(&self) {
        // Clean all test data
        let _ = sqlx::query("DELETE FROM bookings")
            .execute(&self.pool)
            .await;
        let _ = sqlx::query("DELETE FROM calendars")
            .execute(&self.pool)
            .await;
        let _ = sqlx::query("DELETE FROM media").execute(&self.pool).await;
        let _ = sqlx::query("DELETE FROM notices").execute(&self.pool).await;
        let _ = sqlx::query("DELETE FROM opening_hours")
            .execute(&self.pool)
            .await;
        let _ = sqlx::query("DELETE FROM contact_info")
            .execute(&self.pool)
            .await;
    }
}

// Test data builders
pub mod builders {
    use chrono::{DateTime, Utc};

    // Simple request structures for testing
    #[derive(Debug)]
    pub struct CreateCalendarRequest {
        pub name: String,
        pub price_per_hour: i32,
        pub description: Option<String>,
        pub thumbnail_id: Option<i32>,
    }

    #[derive(Debug)]
    pub struct CreateBookingRequest {
        pub calendar_id: i32,
        pub name: String,
        pub email: String,
        pub phone: Option<String>,
        pub start_time: DateTime<Utc>,
        pub end_time: DateTime<Utc>,
        pub notes: Option<String>,
    }

    pub struct CalendarBuilder {
        name: String,
        price_per_hour: i32,
        description: Option<String>,
        thumbnail_id: Option<i32>,
    }

    impl CalendarBuilder {
        pub fn new(name: &str) -> Self {
            Self {
                name: name.to_string(),
                price_per_hour: 2500, // 25.00 EUR
                description: None,
                thumbnail_id: None,
            }
        }

        pub fn price(mut self, price_cents: i32) -> Self {
            self.price_per_hour = price_cents;
            self
        }

        pub fn description(mut self, desc: &str) -> Self {
            self.description = Some(desc.to_string());
            self
        }

        pub fn thumbnail(mut self, id: i32) -> Self {
            self.thumbnail_id = Some(id);
            self
        }

        pub fn build(self) -> CreateCalendarRequest {
            CreateCalendarRequest {
                name: self.name,
                price_per_hour: self.price_per_hour,
                description: self.description,
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
            let now = Utc::now();
            Self {
                calendar_id,
                name: name.to_string(),
                email: email.to_string(),
                phone: None,
                start_time: now + chrono::Duration::hours(24), // Future booking
                end_time: now + chrono::Duration::hours(25),
                notes: None,
            }
        }

        pub fn phone(mut self, phone: &str) -> Self {
            self.phone = Some(phone.to_string());
            self
        }

        pub fn times(mut self, start: DateTime<Utc>, end: DateTime<Utc>) -> Self {
            self.start_time = start;
            self.end_time = end;
            self
        }

        pub fn notes(mut self, notes: &str) -> Self {
            self.notes = Some(notes.to_string());
            self
        }

        pub fn build(self) -> CreateBookingRequest {
            CreateBookingRequest {
                calendar_id: self.calendar_id,
                name: self.name,
                email: self.email,
                phone: self.phone,
                start_time: self.start_time,
                end_time: self.end_time,
                notes: self.notes,
            }
        }
    }
}

// Mock data generators
pub mod fixtures {
    use chrono::{DateTime, Utc};
    use rand::Rng;

    pub fn random_email() -> String {
        let mut rng = rand::thread_rng();
        let id: u32 = rng.gen_range(1000..9999);
        format!("test{}@example.com", id)
    }

    pub fn random_phone() -> String {
        let mut rng = rand::thread_rng();
        let number: u32 = rng.gen_range(100000000..999999999);
        format!("+358{}", number)
    }

    pub fn future_datetime() -> DateTime<Utc> {
        Utc::now() + chrono::Duration::hours(24)
    }

    pub fn valid_booking_time_range() -> (DateTime<Utc>, DateTime<Utc>) {
        let start = future_datetime();
        let end = start + chrono::Duration::hours(2);
        (start, end)
    }
}

use std::sync::Arc;

use crate::{
    config::AppConfig,
    features::{
        bookings::{repository::MySqlBookingsRepository, service::BookingsService},
        calendars::{repository::MySqlCalendarsRepository, service::CalendarsService},
    },
};
use sqlx::{MySql, Pool};

#[allow(dead_code)]
#[derive(Clone)]
pub struct AppState {
    pub config: AppConfig,
    pub pool: Pool<MySql>,
    pub calendars: CalendarsService,
    pub bookings: BookingsService,
}

impl AppState {
    pub fn new(config: AppConfig, pool: Pool<MySql>) -> Self {
        let calendars_repository = Arc::new(MySqlCalendarsRepository::new(pool.clone()));
        let bookings_repository = Arc::new(MySqlBookingsRepository::new(pool.clone()));

        Self {
            config,
            pool,
            calendars: CalendarsService::new(calendars_repository),
            bookings: BookingsService::new(bookings_repository),
        }
    }
}

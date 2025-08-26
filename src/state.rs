use std::sync::Arc;

use crate::{
    config::AppConfig,
    features::{
        bookings::{repository::MySqlBookingsRepository, service::BookingsService},
        calendars::{repository::MySqlCalendarsRepository, service::CalendarsService},
        contact_info::{repository::MySqlContactInfoRepository, service::ContactInfoService},
        notices::{repository::MySqlNoticesRepository, service::NoticesService},
        opening_hours::{
            repository::{MySqlOpeningExceptionsRepository, MySqlOpeningHoursRepository},
            service::{OpeningExceptionsService, OpeningHoursService},
        },
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
    pub notices: NoticesService,
    pub opening_hours: OpeningHoursService,
    pub opening_exceptions: OpeningExceptionsService,
    pub contact_info: ContactInfoService,
}

impl AppState {
    pub fn new(config: AppConfig, pool: Pool<MySql>) -> Self {
        let calendars_repository = Arc::new(MySqlCalendarsRepository::new(pool.clone()));
        let bookings_repository = Arc::new(MySqlBookingsRepository::new(pool.clone()));
        let notices_repository = Arc::new(MySqlNoticesRepository::new(pool.clone()));
        let opening_hours_repository = Arc::new(MySqlOpeningHoursRepository::new(pool.clone()));
        let opening_exceptions_repository =
            Arc::new(MySqlOpeningExceptionsRepository::new(pool.clone()));
        let contact_info_repository = Arc::new(MySqlContactInfoRepository::new(pool.clone()));

        Self {
            config,
            pool,
            calendars: CalendarsService::new(calendars_repository),
            bookings: BookingsService::new(bookings_repository),
            notices: NoticesService::new(notices_repository),
            opening_hours: OpeningHoursService::new(opening_hours_repository),
            opening_exceptions: OpeningExceptionsService::new(opening_exceptions_repository),
            contact_info: ContactInfoService::new(contact_info_repository),
        }
    }
}

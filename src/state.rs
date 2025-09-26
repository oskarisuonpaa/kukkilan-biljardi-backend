use std::sync::Arc;

use crate::{
    config::AppConfig,
    features::{
        auth::{repository::MySqlAuthRepository, service::AuthService},
        bookings::{repository::MySqlBookingsRepository, service::BookingsService},
        calendars::{repository::MySqlCalendarsRepository, service::CalendarsService},
        contact_info::{repository::MySqlContactInfoRepository, service::ContactInfoService},
        email::{model::EmailConfig, service::EmailService},
        encryption::EncryptionService,
        media::{repository::MySqlMediaRepository, service::MediaService},
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
    pub auth: AuthService,
    pub calendars: CalendarsService,
    pub bookings: BookingsService,
    pub notices: NoticesService,
    pub opening_hours: OpeningHoursService,
    pub opening_exceptions: OpeningExceptionsService,
    pub contact_info: ContactInfoService,
    pub media: MediaService,
    pub email: EmailService,
    pub encryption: EncryptionService,
}

impl AppState {
    pub fn new(config: AppConfig, pool: Pool<MySql>) -> Self {
        let auth_repository = Arc::new(MySqlAuthRepository::new(pool.clone()));
        let calendars_repository = Arc::new(MySqlCalendarsRepository::new(pool.clone()));
        let bookings_repository = Arc::new(MySqlBookingsRepository::new(pool.clone()));
        let notices_repository = Arc::new(MySqlNoticesRepository::new(pool.clone()));
        let opening_hours_repository = Arc::new(MySqlOpeningHoursRepository::new(pool.clone()));
        let opening_exceptions_repository =
            Arc::new(MySqlOpeningExceptionsRepository::new(pool.clone()));
        let contact_info_repository = Arc::new(MySqlContactInfoRepository::new(pool.clone()));
        let media_repository = Arc::new(MySqlMediaRepository::new(pool.clone()));

        // Email configuration from environment
        let email_config = EmailConfig {
            smtp_host: std::env::var("SMTP_HOST").unwrap_or_else(|_| "localhost".to_string()),
            smtp_port: std::env::var("SMTP_PORT")
                .unwrap_or_else(|_| "587".to_string())
                .parse()
                .unwrap_or(587),
            smtp_username: std::env::var("SMTP_USERNAME").unwrap_or_default(),
            smtp_password: std::env::var("SMTP_PASSWORD").unwrap_or_default(),
            from_email: std::env::var("FROM_EMAIL")
                .unwrap_or_else(|_| "noreply@kukkilan-biljardi.fi".to_string()),
            from_name: std::env::var("FROM_NAME")
                .unwrap_or_else(|_| "Kukkilan Biljardi".to_string()),
            enabled: std::env::var("EMAIL_ENABLED")
                .unwrap_or_else(|_| "false".to_string())
                .parse()
                .unwrap_or(false),
        };

        let encryption_service = EncryptionService::from_config(config.encryption_key.as_ref());

        Self {
            config,
            pool,
            auth: AuthService::new(auth_repository),
            calendars: CalendarsService::new(calendars_repository.clone()),
            bookings: BookingsService::new(
                bookings_repository,
                calendars_repository.clone(),
                encryption_service.clone(),
            ),
            notices: NoticesService::new(notices_repository),
            opening_hours: OpeningHoursService::new(opening_hours_repository),
            opening_exceptions: OpeningExceptionsService::new(opening_exceptions_repository),
            contact_info: ContactInfoService::new(contact_info_repository),
            media: MediaService::new(media_repository),
            email: EmailService::new(email_config),
            encryption: encryption_service.clone(),
        }
    }
}

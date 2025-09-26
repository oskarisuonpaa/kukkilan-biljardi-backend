ALTER TABLE calendars
ADD COLUMN hourly_price_cents INT UNSIGNED DEFAULT NULL COMMENT 'Hourly price in cents (EUR), NULL means use default pricing';
use serial_test::serial;

mod common_simple;
use common_simple::TestContext;

#[tokio::test]
#[serial]
async fn test_basic_database_connection() {
    let ctx = TestContext::new().await;

    // Test that we can connect to the database
    let result = sqlx::query("SELECT 1 as test").fetch_one(&ctx.pool).await;

    assert!(result.is_ok(), "Should be able to connect to database");

    ctx.cleanup().await;
}

#[tokio::test]
#[serial]
async fn test_database_migrations() {
    let ctx = TestContext::new().await;

    // Test that tables exist (migrations ran successfully)
    let calendars_table = sqlx::query("SELECT COUNT(*) FROM calendars")
        .fetch_one(&ctx.pool)
        .await;

    let bookings_table = sqlx::query("SELECT COUNT(*) FROM bookings")
        .fetch_one(&ctx.pool)
        .await;

    let notices_table = sqlx::query("SELECT COUNT(*) FROM notices")
        .fetch_one(&ctx.pool)
        .await;

    assert!(calendars_table.is_ok(), "Calendars table should exist");
    assert!(bookings_table.is_ok(), "Bookings table should exist");
    assert!(notices_table.is_ok(), "Notices table should exist");

    ctx.cleanup().await;
}

#[tokio::test]
#[serial]
async fn test_table_structure() {
    let ctx = TestContext::new().await;

    // Test calendar table structure
    let calendar_columns = sqlx::query("DESCRIBE calendars").fetch_all(&ctx.pool).await;

    assert!(
        calendar_columns.is_ok(),
        "Should be able to describe calendars table"
    );
    assert!(
        calendar_columns.unwrap().len() > 0,
        "Calendars table should have columns"
    );

    // Test booking table structure
    let booking_columns = sqlx::query("DESCRIBE bookings").fetch_all(&ctx.pool).await;

    assert!(
        booking_columns.is_ok(),
        "Should be able to describe bookings table"
    );
    assert!(
        booking_columns.unwrap().len() > 0,
        "Bookings table should have columns"
    );

    ctx.cleanup().await;
}

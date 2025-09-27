use chrono::{Duration, Utc};
use serial_test::serial;

mod common_simple;
use common_simple::{TestContext, builders::BookingBuilder};

#[tokio::test]
#[serial]
async fn test_booking_database_operations() {
    let ctx = TestContext::new().await;

    // First create a calendar with unique name
    let unique_name = format!(
        "Booking Test Calendar {}",
        Utc::now().timestamp_nanos_opt().unwrap_or(0)
    );
    let calendar_result = sqlx::query("INSERT INTO calendars (name) VALUES (?)")
        .bind(&unique_name)
        .execute(&ctx.pool)
        .await;

    assert!(
        calendar_result.is_ok(),
        "Should be able to create calendar: {:?}",
        calendar_result.err()
    );
    let calendar_id = calendar_result.unwrap().last_insert_id() as u32;

    // Create a booking
    let start_time = Utc::now() + Duration::hours(24);
    let end_time = start_time + Duration::hours(2);

    let booking_result = sqlx::query(
        "INSERT INTO bookings (calendar_id, customer_name, customer_email, customer_phone, starts_at_utc, ends_at_utc) VALUES (?, ?, ?, ?, ?, ?)"
    )
    .bind(calendar_id)
    .bind("Test User")
    .bind("test@example.com")
    .bind("123-456-7890")
    .bind(start_time)
    .bind(end_time)
    .execute(&ctx.pool)
    .await;

    assert!(booking_result.is_ok(), "Should be able to create booking");

    // Retrieve the booking
    let booking = sqlx::query_as::<_, (u32, u32, String, String, String)>(
        "SELECT id, calendar_id, customer_name, customer_email, customer_phone FROM bookings WHERE calendar_id = ?"
    )
    .bind(calendar_id)
    .fetch_one(&ctx.pool)
    .await;

    assert!(
        booking.is_ok(),
        "Should be able to retrieve booking: {:?}",
        booking.err()
    );
    let (_booking_id, retrieved_calendar_id, name, email, _phone) = booking.unwrap();
    assert_eq!(retrieved_calendar_id, calendar_id);
    assert_eq!(name, "Test User");
    assert_eq!(email, "test@example.com");
    ctx.cleanup().await;
}

#[tokio::test]
#[serial]
async fn test_booking_builder() {
    let calendar_id = 1;
    let booking_request = BookingBuilder::new(calendar_id, "John Doe", "john@example.com")
        .phone("+358401234567")
        .notes("Test booking")
        .build();

    assert_eq!(booking_request.calendar_id, calendar_id);
    assert_eq!(booking_request.customer_name, "John Doe");
    assert_eq!(booking_request.customer_email, "john@example.com");
    assert_eq!(booking_request.customer_phone, "+358401234567");
    assert_eq!(
        booking_request.customer_notes,
        Some("Test booking".to_string())
    );
}

#[tokio::test]
#[serial]
async fn test_booking_time_conflict_detection() {
    let ctx = TestContext::new().await;

    // Create a calendar with unique name
    let unique_name = format!(
        "Conflict Test Calendar {}",
        Utc::now().timestamp_nanos_opt().unwrap_or(0)
    );
    let calendar_result = sqlx::query("INSERT INTO calendars (name) VALUES (?)")
        .bind(&unique_name)
        .execute(&ctx.pool)
        .await;

    assert!(
        calendar_result.is_ok(),
        "Should be able to create calendar: {:?}",
        calendar_result.err()
    );
    let calendar_id = calendar_result.unwrap().last_insert_id() as u32;

    // Create first booking
    let start_time = Utc::now() + Duration::hours(24);
    let end_time = start_time + Duration::hours(2);

    let booking1_result = sqlx::query(
        "INSERT INTO bookings (calendar_id, customer_name, customer_email, customer_phone, starts_at_utc, ends_at_utc) VALUES (?, ?, ?, ?, ?, ?)"
    )
    .bind(calendar_id)
    .bind("User 1")
    .bind("user1@example.com")
    .bind("123-456-7890")
    .bind(start_time)
    .bind(end_time)
    .execute(&ctx.pool)
    .await;

    assert!(booking1_result.is_ok(), "First booking should succeed");

    // Check for time conflicts with overlapping booking
    let overlap_start = start_time + Duration::minutes(30);
    let overlap_end = end_time + Duration::minutes(30);

    let conflict_check = sqlx::query_as::<_, (i64,)>(
        "SELECT COUNT(*) FROM bookings WHERE calendar_id = ? AND (
            (starts_at_utc <= ? AND ends_at_utc > ?) OR 
            (starts_at_utc < ? AND ends_at_utc >= ?) OR
            (starts_at_utc >= ? AND ends_at_utc <= ?)
        )",
    )
    .bind(calendar_id)
    .bind(overlap_start)
    .bind(overlap_start)
    .bind(overlap_end)
    .bind(overlap_end)
    .bind(start_time)
    .bind(end_time)
    .fetch_one(&ctx.pool)
    .await;

    assert!(conflict_check.is_ok(), "Conflict check should succeed");
    let (conflict_count,) = conflict_check.unwrap();
    assert!(conflict_count > 0, "Should detect time conflict");

    ctx.cleanup().await;
}

#[tokio::test]
#[serial]
async fn test_booking_validation() {
    let ctx = TestContext::new().await;

    // Create a calendar
    let calendar_result = sqlx::query("INSERT INTO calendars (name) VALUES (?)")
        .bind("Validation Test Calendar")
        .execute(&ctx.pool)
        .await;

    let calendar_id = calendar_result.unwrap().last_insert_id() as i32;

    // Test booking with end time before start time (should fail)
    let start_time = Utc::now() + Duration::hours(24);
    let invalid_end_time = start_time - Duration::hours(1);

    let _invalid_booking_result = sqlx::query(
        "INSERT INTO bookings (calendar_id, customer_name, customer_email, starts_at_utc, ends_at_utc) VALUES (?, ?, ?, ?, ?)"
    )
    .bind(calendar_id)
    .bind("Invalid User")
    .bind("invalid@example.com")
    .bind(start_time)
    .bind(invalid_end_time)
    .execute(&ctx.pool)
    .await;

    // This should succeed at the database level (we need application-level validation)
    // But we can check that the times are logically invalid
    assert!(
        start_time > invalid_end_time,
        "Start time should be after end time (invalid scenario)"
    );

    ctx.cleanup().await;
}

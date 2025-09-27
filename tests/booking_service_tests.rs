use crate::common::{
    TestContext,
    builders::{BookingBuilder, CalendarBuilder},
    fixtures,
};
use crate::features::{
    bookings::{repository::BookingsRepository, service::BookingsService},
    calendars::{repository::CalendarsRepository, service::CalendarsService},
};
use chrono::{Duration, Utc};
use serial_test::serial;

mod common;

#[tokio::test]
#[serial]
async fn test_create_booking() {
    let ctx = TestContext::new().await;

    // Setup calendar
    let calendar_repo = CalendarsRepository::new(ctx.pool.clone());
    let calendar_service = CalendarsService::new(calendar_repo);
    let calendar_data = CalendarBuilder::new("Test Table").build();
    let calendar = calendar_service.create(calendar_data).await.unwrap();

    // Setup booking service
    let booking_repo = BookingsRepository::new(ctx.pool.clone());
    let booking_service = BookingsService::new(booking_repo);

    let (start_time, end_time) = fixtures::valid_booking_time_range();
    let booking_data = BookingBuilder::new(calendar.id, "John Doe", &fixtures::random_email())
        .phone(&fixtures::random_phone())
        .times(start_time, end_time)
        .notes("Test booking")
        .build();

    let result = booking_service.create(booking_data).await;

    assert!(
        result.is_ok(),
        "Failed to create booking: {:?}",
        result.err()
    );

    let booking = result.unwrap();
    assert_eq!(booking.name, "John Doe");
    assert_eq!(booking.calendar_id, calendar.id);
    assert_eq!(booking.start_time, start_time);
    assert_eq!(booking.end_time, end_time);

    ctx.cleanup().await;
}

#[tokio::test]
#[serial]
async fn test_booking_time_conflict() {
    let ctx = TestContext::new().await;

    // Setup calendar
    let calendar_repo = CalendarsRepository::new(ctx.pool.clone());
    let calendar_service = CalendarsService::new(calendar_repo);
    let calendar_data = CalendarBuilder::new("Test Table").build();
    let calendar = calendar_service.create(calendar_data).await.unwrap();

    // Setup booking service
    let booking_repo = BookingsRepository::new(ctx.pool.clone());
    let booking_service = BookingsService::new(booking_repo);

    let (start_time, end_time) = fixtures::valid_booking_time_range();

    // Create first booking
    let booking1 = BookingBuilder::new(calendar.id, "User 1", &fixtures::random_email())
        .times(start_time, end_time)
        .build();
    let result1 = booking_service.create(booking1).await;
    assert!(result1.is_ok(), "First booking should succeed");

    // Try to create overlapping booking
    let overlap_start = start_time + Duration::minutes(30);
    let overlap_end = end_time + Duration::minutes(30);
    let booking2 = BookingBuilder::new(calendar.id, "User 2", &fixtures::random_email())
        .times(overlap_start, overlap_end)
        .build();

    let result2 = booking_service.create(booking2).await;
    assert!(result2.is_err(), "Overlapping booking should fail");

    ctx.cleanup().await;
}

#[tokio::test]
#[serial]
async fn test_get_bookings_for_calendar() {
    let ctx = TestContext::new().await;

    // Setup calendar
    let calendar_repo = CalendarsRepository::new(ctx.pool.clone());
    let calendar_service = CalendarsService::new(calendar_repo);
    let calendar_data = CalendarBuilder::new("Test Table").build();
    let calendar = calendar_service.create(calendar_data).await.unwrap();

    // Setup booking service
    let booking_repo = BookingsRepository::new(ctx.pool.clone());
    let booking_service = BookingsService::new(booking_repo);

    // Create multiple bookings
    let now = Utc::now();
    let booking1 = BookingBuilder::new(calendar.id, "User 1", &fixtures::random_email())
        .times(now + Duration::hours(1), now + Duration::hours(2))
        .build();
    let booking2 = BookingBuilder::new(calendar.id, "User 2", &fixtures::random_email())
        .times(now + Duration::hours(3), now + Duration::hours(4))
        .build();

    booking_service.create(booking1).await.unwrap();
    booking_service.create(booking2).await.unwrap();

    // Get bookings for calendar
    let result = booking_service.get_bookings_for_calendar(calendar.id).await;

    assert!(result.is_ok(), "Failed to get bookings: {:?}", result.err());

    let bookings = result.unwrap();
    assert_eq!(
        bookings.len(),
        2,
        "Expected 2 bookings, got {}",
        bookings.len()
    );

    let names: Vec<&str> = bookings.iter().map(|b| b.name.as_str()).collect();
    assert!(names.contains(&"User 1"));
    assert!(names.contains(&"User 2"));

    ctx.cleanup().await;
}

#[tokio::test]
#[serial]
async fn test_update_booking() {
    let ctx = TestContext::new().await;

    // Setup calendar
    let calendar_repo = CalendarsRepository::new(ctx.pool.clone());
    let calendar_service = CalendarsService::new(calendar_repo);
    let calendar_data = CalendarBuilder::new("Test Table").build();
    let calendar = calendar_service.create(calendar_data).await.unwrap();

    // Setup booking service
    let booking_repo = BookingsRepository::new(ctx.pool.clone());
    let booking_service = BookingsService::new(booking_repo);

    // Create booking
    let (start_time, end_time) = fixtures::valid_booking_time_range();
    let booking_data = BookingBuilder::new(calendar.id, "Original Name", &fixtures::random_email())
        .times(start_time, end_time)
        .build();
    let created = booking_service.create(booking_data).await.unwrap();

    // Update booking
    let new_start = start_time + Duration::hours(1);
    let new_end = end_time + Duration::hours(1);
    let update_data = BookingBuilder::new(calendar.id, "Updated Name", &fixtures::random_email())
        .times(new_start, new_end)
        .notes("Updated notes")
        .build();

    let result = booking_service.update(created.id, update_data).await;
    assert!(
        result.is_ok(),
        "Failed to update booking: {:?}",
        result.err()
    );

    // Verify update
    let updated = booking_service.get_by_id(created.id).await.unwrap();
    assert_eq!(updated.name, "Updated Name");
    assert_eq!(updated.start_time, new_start);
    assert_eq!(updated.end_time, new_end);
    assert_eq!(updated.notes.as_deref(), Some("Updated notes"));

    ctx.cleanup().await;
}

#[tokio::test]
#[serial]
async fn test_delete_booking() {
    let ctx = TestContext::new().await;

    // Setup calendar
    let calendar_repo = CalendarsRepository::new(ctx.pool.clone());
    let calendar_service = CalendarsService::new(calendar_repo);
    let calendar_data = CalendarBuilder::new("Test Table").build();
    let calendar = calendar_service.create(calendar_data).await.unwrap();

    // Setup booking service
    let booking_repo = BookingsRepository::new(ctx.pool.clone());
    let booking_service = BookingsService::new(booking_repo);

    // Create booking
    let (start_time, end_time) = fixtures::valid_booking_time_range();
    let booking_data = BookingBuilder::new(calendar.id, "To Delete", &fixtures::random_email())
        .times(start_time, end_time)
        .build();
    let created = booking_service.create(booking_data).await.unwrap();

    // Delete booking
    let result = booking_service.delete(created.id).await;
    assert!(
        result.is_ok(),
        "Failed to delete booking: {:?}",
        result.err()
    );

    // Verify deletion
    let get_result = booking_service.get_by_id(created.id).await;
    assert!(get_result.is_err(), "Booking should have been deleted");

    ctx.cleanup().await;
}

#[tokio::test]
#[serial]
async fn test_booking_validation() {
    let ctx = TestContext::new().await;

    // Setup calendar
    let calendar_repo = CalendarsRepository::new(ctx.pool.clone());
    let calendar_service = CalendarsService::new(calendar_repo);
    let calendar_data = CalendarBuilder::new("Test Table").build();
    let calendar = calendar_service.create(calendar_data).await.unwrap();

    // Setup booking service
    let booking_repo = BookingsRepository::new(ctx.pool.clone());
    let booking_service = BookingsService::new(booking_repo);

    // Test past time validation
    let past_time = Utc::now() - Duration::hours(1);
    let invalid_booking = BookingBuilder::new(calendar.id, "Test User", &fixtures::random_email())
        .times(past_time, past_time + Duration::hours(1))
        .build();

    let result = booking_service.create(invalid_booking).await;
    assert!(result.is_err(), "Should fail with past time");

    // Test end time before start time
    let (start_time, _) = fixtures::valid_booking_time_range();
    let invalid_times = BookingBuilder::new(calendar.id, "Test User", &fixtures::random_email())
        .times(start_time, start_time - Duration::hours(1))
        .build();

    let result = booking_service.create(invalid_times).await;
    assert!(
        result.is_err(),
        "Should fail when end time is before start time"
    );

    // Test invalid email
    let invalid_email = BookingBuilder::new(calendar.id, "Test User", "invalid-email")
        .times(
            fixtures::valid_booking_time_range().0,
            fixtures::valid_booking_time_range().1,
        )
        .build();

    let result = booking_service.create(invalid_email).await;
    assert!(result.is_err(), "Should fail with invalid email");

    ctx.cleanup().await;
}

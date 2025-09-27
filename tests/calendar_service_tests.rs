use serial_test::serial;

mod common;
use common::{TestContext, builders::CalendarBuilder};

#[tokio::test]
#[serial]
async fn test_create_calendar() {
    let ctx = TestContext::new().await;
    let repo = CalendarsRepository::new(ctx.pool.clone());
    let service = CalendarsService::new(repo);

    let calendar_data = CalendarBuilder::new("Test Snooker Table")
        .price(3000) // 30.00 EUR
        .description("Professional snooker table")
        .build();

    let result = service.create(calendar_data).await;

    assert!(
        result.is_ok(),
        "Failed to create calendar: {:?}",
        result.err()
    );

    let calendar = result.unwrap();
    assert_eq!(calendar.name, "Test Snooker Table");
    assert_eq!(calendar.price_per_hour, 3000);
    assert_eq!(
        calendar.description.as_deref(),
        Some("Professional snooker table")
    );

    ctx.cleanup().await;
}

#[tokio::test]
#[serial]
async fn test_get_calendar_by_id() {
    let ctx = TestContext::new().await;
    let repo = CalendarsRepository::new(ctx.pool.clone());
    let service = CalendarsService::new(repo);

    // Create a calendar first
    let calendar_data = CalendarBuilder::new("Pool Table 1").build();
    let created = service.create(calendar_data).await.unwrap();

    // Get it by ID
    let result = service.get_by_id(created.id).await;

    assert!(result.is_ok(), "Failed to get calendar: {:?}", result.err());

    let calendar = result.unwrap();
    assert_eq!(calendar.id, created.id);
    assert_eq!(calendar.name, "Pool Table 1");

    ctx.cleanup().await;
}

#[tokio::test]
#[serial]
async fn test_list_calendars() {
    let ctx = TestContext::new().await;
    let repo = CalendarsRepository::new(ctx.pool.clone());
    let service = CalendarsService::new(repo);

    // Create multiple calendars
    let calendar1 = CalendarBuilder::new("Snooker 1").build();
    let calendar2 = CalendarBuilder::new("Pool 1").price(2000).build();

    service.create(calendar1).await.unwrap();
    service.create(calendar2).await.unwrap();

    // List all calendars
    let result = service.list().await;

    assert!(
        result.is_ok(),
        "Failed to list calendars: {:?}",
        result.err()
    );

    let calendars = result.unwrap();
    assert!(
        calendars.len() >= 2,
        "Expected at least 2 calendars, got {}",
        calendars.len()
    );

    let names: Vec<&str> = calendars.iter().map(|c| c.name.as_str()).collect();
    assert!(names.contains(&"Snooker 1"));
    assert!(names.contains(&"Pool 1"));

    ctx.cleanup().await;
}

#[tokio::test]
#[serial]
async fn test_update_calendar() {
    let ctx = TestContext::new().await;
    let repo = CalendarsRepository::new(ctx.pool.clone());
    let service = CalendarsService::new(repo);

    // Create a calendar
    let calendar_data = CalendarBuilder::new("Original Name").build();
    let created = service.create(calendar_data).await.unwrap();

    // Update it
    let update_data = CalendarBuilder::new("Updated Name")
        .price(4000)
        .description("Updated description")
        .build();

    let result = service.update(created.id, update_data).await;

    assert!(
        result.is_ok(),
        "Failed to update calendar: {:?}",
        result.err()
    );

    // Verify the update
    let updated = service.get_by_id(created.id).await.unwrap();
    assert_eq!(updated.name, "Updated Name");
    assert_eq!(updated.price_per_hour, 4000);
    assert_eq!(updated.description.as_deref(), Some("Updated description"));

    ctx.cleanup().await;
}

#[tokio::test]
#[serial]
async fn test_delete_calendar() {
    let ctx = TestContext::new().await;
    let repo = CalendarsRepository::new(ctx.pool.clone());
    let service = CalendarsService::new(repo);

    // Create a calendar
    let calendar_data = CalendarBuilder::new("To Be Deleted").build();
    let created = service.create(calendar_data).await.unwrap();

    // Delete it
    let result = service.delete(created.id).await;

    assert!(
        result.is_ok(),
        "Failed to delete calendar: {:?}",
        result.err()
    );

    // Verify it's gone
    let get_result = service.get_by_id(created.id).await;
    assert!(get_result.is_err(), "Calendar should have been deleted");

    ctx.cleanup().await;
}

#[tokio::test]
#[serial]
async fn test_calendar_validation() {
    let ctx = TestContext::new().await;
    let repo = CalendarsRepository::new(ctx.pool.clone());
    let service = CalendarsService::new(repo);

    // Test empty name validation
    let invalid_calendar = CalendarBuilder::new("").build();
    let result = service.create(invalid_calendar).await;
    assert!(result.is_err(), "Should fail with empty name");

    // Test negative price validation
    let invalid_price = CalendarBuilder::new("Valid Name").price(-100).build();
    let result = service.create(invalid_price).await;
    assert!(result.is_err(), "Should fail with negative price");

    ctx.cleanup().await;
}

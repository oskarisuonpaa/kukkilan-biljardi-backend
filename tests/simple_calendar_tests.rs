use serial_test::serial;

mod common_simple;
use common_simple::{TestContext, builders::CalendarBuilder};

#[tokio::test]
#[serial]
async fn test_calendar_database_operations() {
    let ctx = TestContext::new().await;
    
    // Create a calendar
    let test_name = format!("Test Calendar {}", chrono::Utc::now().timestamp_nanos_opt().unwrap_or_default());
    let calendar_result = sqlx::query("INSERT INTO calendars (name) VALUES (?)")
        .bind(&test_name)
        .execute(&ctx.pool)
        .await;
    
    if let Err(e) = &calendar_result {
        eprintln!("Database error: {:?}", e);
    }
    assert!(calendar_result.is_ok(), "Should be able to create calendar: {:?}", calendar_result.err());
    let calendar_id = calendar_result.unwrap().last_insert_id() as i32;
    
    // Retrieve the calendar
    let calendar = sqlx::query_as::<_, (u32, String)>(
        "SELECT id, name FROM calendars WHERE id = ?"
    )
    .bind(calendar_id)
    .fetch_one(&ctx.pool)
    .await;
    
    if let Err(e) = &calendar {
        eprintln!("Database retrieve error: {:?}", e);
    }
    assert!(calendar.is_ok(), "Should be able to retrieve calendar: {:?}", calendar.err());
    let (retrieved_id, name) = calendar.unwrap();
    assert_eq!(retrieved_id as i32, calendar_id);
    assert_eq!(name, test_name);
    
    ctx.cleanup().await;
}

#[tokio::test]
#[serial]
async fn test_calendar_builder() {
    let calendar_request = CalendarBuilder::new("Test Calendar")
        .active(true)
        .build();
    
    assert_eq!(calendar_request.name, "Test Calendar");
    assert_eq!(calendar_request.active, Some(true));
    assert_eq!(calendar_request.thumbnail_id, None);
    
    // Test without optional fields
    let simple_calendar = CalendarBuilder::new("Simple Calendar").build();
    assert_eq!(simple_calendar.active, None);
}

#[tokio::test]
#[serial]
async fn test_calendar_list() {
    let ctx = TestContext::new().await;
    
    // Create multiple calendars
    let test_name_1 = format!("Test Calendar 1 {}", chrono::Utc::now().timestamp_nanos_opt().unwrap_or_default());
    let test_name_2 = format!("Test Calendar 2 {}", chrono::Utc::now().timestamp_nanos_opt().unwrap_or_default() + 1);
    
    let _result1 = sqlx::query("INSERT INTO calendars (name) VALUES (?)")
        .bind(&test_name_1)
        .execute(&ctx.pool)
        .await;
    
    let _result2 = sqlx::query("INSERT INTO calendars (name) VALUES (?)")
        .bind(&test_name_2)
        .execute(&ctx.pool)
        .await;
    
    // List all calendars
    let calendars = sqlx::query_as::<_, (u32, String)>(
        "SELECT id, name FROM calendars WHERE name LIKE '%Test Calendar%'"
    )
    .fetch_all(&ctx.pool)
    .await;
    
    if let Err(e) = &calendars {
        eprintln!("Database list error: {:?}", e);
    }
    assert!(calendars.is_ok(), "Should be able to list calendars: {:?}", calendars.err());
    let calendar_list = calendars.unwrap();
    assert!(calendar_list.len() >= 2, "Should have at least 2 test calendars");
    
    ctx.cleanup().await;
}
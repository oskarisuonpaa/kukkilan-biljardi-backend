use serial_test::serial;

mod common;
use common::{TestContext, builders::CalendarBuilder};

#[tokio::test]
#[serial]
async fn test_basic_database_connection() {
    let ctx = TestContext::new().await;
    
    // Simple query to test database connection
    let result = sqlx::query("SELECT 1 as test_value")
        .fetch_one(&ctx.pool)
        .await;
    
    assert!(result.is_ok(), "Database connection should work");
    ctx.cleanup().await;
}

#[tokio::test] 
#[serial]
async fn test_calendar_builder() {
    let calendar_request = CalendarBuilder::new("Test Calendar")
        .price(3000)
        .description("Test description")
        .build();
    
    assert_eq!(calendar_request.name, "Test Calendar");
    assert_eq!(calendar_request.price_per_hour, 3000);
    assert_eq!(calendar_request.description, Some("Test description".to_string()));
}

#[tokio::test]
#[serial] 
async fn test_cleanup_works() {
    let ctx = TestContext::new().await;
    
    // Insert some test data
    let _ = sqlx::query("INSERT INTO calendars (name, price_per_hour) VALUES ('Test', 1000)")
        .execute(&ctx.pool)
        .await;
    
    // Cleanup should work
    ctx.cleanup().await;
    
    // Verify data is gone
    let count: (i64,) = sqlx::query_as("SELECT COUNT(*) FROM calendars")
        .fetch_one(&ctx.pool)
        .await
        .unwrap_or((0,));
    
    assert_eq!(count.0, 0, "Cleanup should remove all test data");
}
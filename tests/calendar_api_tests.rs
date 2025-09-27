use axum::http::StatusCode;
use axum_test::TestServer;
use serde_json::json;
use serial_test::serial;

use crate::common::{
    TestContext,
    builders::CalendarBuilder,
    http::{assert_json_contains, assert_status, create_test_server},
};
use crate::{features::calendars::repository::CalendarsRepository, state::AppState};

mod common;

#[tokio::test]
#[serial]
async fn test_get_calendars_api() {
    let ctx = TestContext::new().await;

    // Setup state
    let calendar_repo = CalendarsRepository::new(ctx.pool.clone());
    let state = AppState::new(ctx.pool.clone());

    // Create test data
    let calendar_data = CalendarBuilder::new("API Test Table")
        .price(2500)
        .description("Test description")
        .build();
    calendar_repo.create(calendar_data).await.unwrap();

    let server = create_test_server(state).await;

    // Test GET /api/calendars
    let response = server.get("/api/calendars").await;

    assert_status(&response, StatusCode::OK);

    let calendars: serde_json::Value = response.json();
    assert!(calendars.is_array(), "Response should be an array");
    assert!(
        calendars.as_array().unwrap().len() > 0,
        "Should have at least one calendar"
    );

    ctx.cleanup().await;
}

#[tokio::test]
#[serial]
async fn test_create_calendar_api() {
    let ctx = TestContext::new().await;
    let state = AppState::new(ctx.pool.clone());
    let server = create_test_server(state).await;

    let calendar_data = json!({
        "name": "New API Table",
        "price_per_hour": 3000,
        "description": "Created via API"
    });

    // Test POST /api/calendars
    let response = server.post("/api/calendars").json(&calendar_data).await;

    assert_status(&response, StatusCode::CREATED);

    let created_calendar: serde_json::Value = response.json();
    assert_json_contains(&response, "name", &json!("New API Table"));
    assert_json_contains(&response, "price_per_hour", &json!(3000));

    ctx.cleanup().await;
}

#[tokio::test]
#[serial]
async fn test_get_calendar_by_id_api() {
    let ctx = TestContext::new().await;

    // Setup test data
    let calendar_repo = CalendarsRepository::new(ctx.pool.clone());
    let calendar_data = CalendarBuilder::new("Specific Table").build();
    let created = calendar_repo.create(calendar_data).await.unwrap();

    let state = AppState::new(ctx.pool.clone());
    let server = create_test_server(state).await;

    // Test GET /api/calendars/{id}
    let response = server.get(&format!("/api/calendars/{}", created.id)).await;

    assert_status(&response, StatusCode::OK);
    assert_json_contains(&response, "id", &json!(created.id));
    assert_json_contains(&response, "name", &json!("Specific Table"));

    ctx.cleanup().await;
}

#[tokio::test]
#[serial]
async fn test_update_calendar_api() {
    let ctx = TestContext::new().await;

    // Setup test data
    let calendar_repo = CalendarsRepository::new(ctx.pool.clone());
    let calendar_data = CalendarBuilder::new("Original API Name").build();
    let created = calendar_repo.create(calendar_data).await.unwrap();

    let state = AppState::new(ctx.pool.clone());
    let server = create_test_server(state).await;

    let update_data = json!({
        "name": "Updated API Name",
        "price_per_hour": 4000,
        "description": "Updated via API"
    });

    // Test PUT /api/calendars/{id}
    let response = server
        .put(&format!("/api/calendars/{}", created.id))
        .json(&update_data)
        .await;

    assert_status(&response, StatusCode::OK);
    assert_json_contains(&response, "name", &json!("Updated API Name"));
    assert_json_contains(&response, "price_per_hour", &json!(4000));

    ctx.cleanup().await;
}

#[tokio::test]
#[serial]
async fn test_delete_calendar_api() {
    let ctx = TestContext::new().await;

    // Setup test data
    let calendar_repo = CalendarsRepository::new(ctx.pool.clone());
    let calendar_data = CalendarBuilder::new("To Delete API").build();
    let created = calendar_repo.create(calendar_data).await.unwrap();

    let state = AppState::new(ctx.pool.clone());
    let server = create_test_server(state).await;

    // Test DELETE /api/calendars/{id}
    let response = server
        .delete(&format!("/api/calendars/{}", created.id))
        .await;

    assert_status(&response, StatusCode::NO_CONTENT);

    // Verify it's gone
    let get_response = server.get(&format!("/api/calendars/{}", created.id)).await;

    assert_status(&get_response, StatusCode::NOT_FOUND);

    ctx.cleanup().await;
}

#[tokio::test]
#[serial]
async fn test_api_validation_errors() {
    let ctx = TestContext::new().await;
    let state = AppState::new(ctx.pool.clone());
    let server = create_test_server(state).await;

    // Test empty name validation
    let invalid_data = json!({
        "name": "",
        "price_per_hour": 2500
    });

    let response = server.post("/api/calendars").json(&invalid_data).await;

    assert_status(&response, StatusCode::BAD_REQUEST);

    // Test negative price validation
    let invalid_price = json!({
        "name": "Valid Name",
        "price_per_hour": -100
    });

    let response = server.post("/api/calendars").json(&invalid_price).await;

    assert_status(&response, StatusCode::BAD_REQUEST);

    // Test malformed JSON
    let response = server
        .post("/api/calendars")
        .header("content-type", "application/json")
        .text("invalid json")
        .await;

    assert_status(&response, StatusCode::BAD_REQUEST);

    ctx.cleanup().await;
}

#[tokio::test]
#[serial]
async fn test_api_not_found() {
    let ctx = TestContext::new().await;
    let state = AppState::new(ctx.pool.clone());
    let server = create_test_server(state).await;

    // Test non-existent calendar
    let response = server.get("/api/calendars/99999").await;
    assert_status(&response, StatusCode::NOT_FOUND);

    // Test invalid calendar ID format
    let response = server.get("/api/calendars/invalid").await;
    assert_status(&response, StatusCode::BAD_REQUEST);

    ctx.cleanup().await;
}

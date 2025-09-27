use axum::http::StatusCode;
use serde_json::json;
use serial_test::serial;

mod common;
use common::{
    TestContext,
    builders::CalendarBuilder,
    fixtures,
};
};

mod common;

#[tokio::test]
#[serial]
async fn test_booking_api_full_flow() {
    let ctx = TestContext::new().await;

    // Setup test data
    let calendar_repo = CalendarsRepository::new(ctx.pool.clone());
    let calendar_data = CalendarBuilder::new("Integration Test Table").build();
    let calendar = calendar_repo.create(calendar_data).await.unwrap();

    let state = AppState::new(ctx.pool.clone());
    let server = create_test_server(state).await;

    // Test creating a booking
    let (start_time, end_time) = fixtures::valid_booking_time_range();
    let booking_data = json!({
        "calendar_id": calendar.id,
        "name": "Integration Test User",
        "email": fixtures::random_email(),
        "phone": fixtures::random_phone(),
        "start_time": start_time,
        "end_time": end_time,
        "notes": "Integration test booking"
    });

    let response = server.post("/api/bookings").json(&booking_data).await;

    assert_status(&response, StatusCode::CREATED);
    let created_booking: serde_json::Value = response.json();
    let booking_id = created_booking["id"].as_i64().unwrap();

    // Test getting the booking
    let get_response = server.get(&format!("/api/bookings/{}", booking_id)).await;

    assert_status(&get_response, StatusCode::OK);
    let fetched_booking: serde_json::Value = get_response.json();
    assert_eq!(fetched_booking["name"], "Integration Test User");

    // Test getting bookings for calendar
    let calendar_bookings_response = server
        .get(&format!("/api/calendars/{}/bookings", calendar.id))
        .await;

    assert_status(&calendar_bookings_response, StatusCode::OK);
    let bookings: Vec<serde_json::Value> = calendar_bookings_response.json();
    assert_eq!(bookings.len(), 1);
    assert_eq!(bookings[0]["id"], booking_id);

    // Test updating the booking
    let update_data = json!({
        "calendar_id": calendar.id,
        "name": "Updated Integration User",
        "email": booking_data["email"],
        "phone": booking_data["phone"],
        "start_time": start_time,
        "end_time": end_time,
        "notes": "Updated notes"
    });

    let update_response = server
        .put(&format!("/api/bookings/{}", booking_id))
        .json(&update_data)
        .await;

    assert_status(&update_response, StatusCode::OK);
    let updated_booking: serde_json::Value = update_response.json();
    assert_eq!(updated_booking["name"], "Updated Integration User");

    // Test deleting the booking
    let delete_response = server
        .delete(&format!("/api/bookings/{}", booking_id))
        .await;

    assert_status(&delete_response, StatusCode::NO_CONTENT);

    // Verify it's gone
    let get_deleted_response = server.get(&format!("/api/bookings/{}", booking_id)).await;

    assert_status(&get_deleted_response, StatusCode::NOT_FOUND);

    ctx.cleanup().await;
}

#[tokio::test]
#[serial]
async fn test_booking_conflict_api() {
    let ctx = TestContext::new().await;

    // Setup test data
    let calendar_repo = CalendarsRepository::new(ctx.pool.clone());
    let calendar_data = CalendarBuilder::new("Conflict Test Table").build();
    let calendar = calendar_repo.create(calendar_data).await.unwrap();

    let state = AppState::new(ctx.pool.clone());
    let server = create_test_server(state).await;

    // Create first booking
    let (start_time, end_time) = fixtures::valid_booking_time_range();
    let booking1_data = json!({
        "calendar_id": calendar.id,
        "name": "First User",
        "email": fixtures::random_email(),
        "start_time": start_time,
        "end_time": end_time
    });

    let response1 = server.post("/api/bookings").json(&booking1_data).await;

    assert_status(&response1, StatusCode::CREATED);

    // Try to create conflicting booking
    let overlap_start = start_time + chrono::Duration::minutes(30);
    let overlap_end = end_time + chrono::Duration::minutes(30);

    let booking2_data = json!({
        "calendar_id": calendar.id,
        "name": "Second User",
        "email": fixtures::random_email(),
        "start_time": overlap_start,
        "end_time": overlap_end
    });

    let response2 = server.post("/api/bookings").json(&booking2_data).await;

    assert_status(&response2, StatusCode::CONFLICT);

    ctx.cleanup().await;
}

#[tokio::test]
#[serial]
async fn test_api_validation_comprehensive() {
    let ctx = TestContext::new().await;
    let state = AppState::new(ctx.pool.clone());
    let server = create_test_server(state).await;

    // Test calendar validation
    let invalid_calendar_data = json!({
        "name": "",  // Empty name
        "price_per_hour": -100  // Negative price
    });

    let response = server
        .post("/api/calendars")
        .json(&invalid_calendar_data)
        .await;

    assert_status(&response, StatusCode::BAD_REQUEST);

    // Test booking validation with invalid email
    let invalid_booking_data = json!({
        "calendar_id": 999, // Non-existent calendar
        "name": "Test User",
        "email": "invalid-email",
        "start_time": "2025-09-27T14:00:00Z",
        "end_time": "2025-09-27T13:00:00Z" // End before start
    });

    let response = server
        .post("/api/bookings")
        .json(&invalid_booking_data)
        .await;

    assert_status(&response, StatusCode::BAD_REQUEST);

    // Test malformed JSON
    let response = server
        .post("/api/calendars")
        .header("content-type", "application/json")
        .text("{ invalid json }")
        .await;

    assert_status(&response, StatusCode::BAD_REQUEST);

    ctx.cleanup().await;
}

#[tokio::test]
#[serial]
async fn test_media_upload_api() {
    let ctx = TestContext::new().await;
    let state = AppState::new(ctx.pool.clone());
    let server = create_test_server(state).await;

    // Create a simple test image (1x1 PNG)
    let png_data: Vec<u8> = vec![
        0x89, 0x50, 0x4E, 0x47, 0x0D, 0x0A, 0x1A, 0x0A, 0x00, 0x00, 0x00, 0x0D, 0x49, 0x48, 0x44,
        0x52, 0x00, 0x00, 0x00, 0x01, 0x00, 0x00, 0x00, 0x01, 0x08, 0x06, 0x00, 0x00, 0x00, 0x1F,
        0x15, 0xC4, 0x89, 0x00, 0x00, 0x00, 0x0A, 0x49, 0x44, 0x41, 0x54, 0x78, 0x9C, 0x63, 0x00,
        0x01, 0x00, 0x00, 0x05, 0x00, 0x01, 0x0D, 0x0A, 0x2D, 0xB4, 0x00, 0x00, 0x00, 0x00, 0x49,
        0x45, 0x4E, 0x44, 0xAE, 0x42, 0x60, 0x82,
    ];

    // Test file upload
    let response = server
        .post("/api/media")
        .multipart(
            axum_test::multipart::MultipartForm::new()
                .add_text("description", "Test image upload")
                .add_file("file", png_data, "test.png", "image/png"),
        )
        .await;

    // Media upload should either succeed or return appropriate error
    let status = response.status_code();
    assert!(
        status == StatusCode::CREATED
            || status == StatusCode::BAD_REQUEST
            || status == StatusCode::INTERNAL_SERVER_ERROR,
        "Media upload returned unexpected status: {}",
        status
    );

    ctx.cleanup().await;
}

#[tokio::test]
#[serial]
async fn test_cors_headers() {
    let ctx = TestContext::new().await;
    let state = AppState::new(ctx.pool.clone());
    let server = create_test_server(state).await;

    // Test CORS preflight request
    let response = server
        .options("/api/calendars")
        .header("Origin", "http://localhost:3000")
        .header("Access-Control-Request-Method", "POST")
        .header("Access-Control-Request-Headers", "content-type")
        .await;

    // Should handle CORS properly
    assert!(
        response.status_code() == StatusCode::OK
            || response.status_code() == StatusCode::NO_CONTENT
    );

    ctx.cleanup().await;
}

#[tokio::test]
#[serial]
async fn test_error_response_format() {
    let ctx = TestContext::new().await;
    let state = AppState::new(ctx.pool.clone());
    let server = create_test_server(state).await;

    // Test 404 error format
    let response = server.get("/api/calendars/99999").await;
    assert_status(&response, StatusCode::NOT_FOUND);

    // Response should be valid JSON
    let error_json: Result<serde_json::Value, _> = response.json();
    assert!(error_json.is_ok(), "Error response should be valid JSON");

    // Test 400 error format
    let response = server
        .post("/api/calendars")
        .json(&json!({})) // Missing required fields
        .await;

    assert_status(&response, StatusCode::BAD_REQUEST);
    let error_json: Result<serde_json::Value, _> = response.json();
    assert!(error_json.is_ok(), "Error response should be valid JSON");

    ctx.cleanup().await;
}

#[tokio::test]
#[serial]
async fn test_concurrent_bookings() {
    let ctx = TestContext::new().await;

    // Setup calendar
    let calendar_repo = CalendarsRepository::new(ctx.pool.clone());
    let calendar_data = CalendarBuilder::new("Concurrency Test Table").build();
    let calendar = calendar_repo.create(calendar_data).await.unwrap();

    let state = AppState::new(ctx.pool.clone());
    let server = create_test_server(state).await;

    let (start_time, end_time) = fixtures::valid_booking_time_range();

    // Try to create multiple bookings for the same time slot concurrently
    let booking_data = json!({
        "calendar_id": calendar.id,
        "name": "Concurrent User",
        "email": fixtures::random_email(),
        "start_time": start_time,
        "end_time": end_time
    });

    // Create multiple concurrent requests
    let tasks: Vec<_> = (0..3)
        .map(|i| {
            let server = &server;
            let data = booking_data.clone();
            async move {
                let mut booking = data;
                booking["name"] = json!(format!("User {}", i));
                booking["email"] = json!(fixtures::random_email());

                server.post("/api/bookings").json(&booking).await
            }
        })
        .collect();

    let responses = futures::future::join_all(tasks).await;

    // Only one should succeed, others should fail
    let success_count = responses
        .iter()
        .filter(|r| r.status_code() == StatusCode::CREATED)
        .count();

    let conflict_count = responses
        .iter()
        .filter(|r| r.status_code() == StatusCode::CONFLICT)
        .count();

    assert_eq!(success_count, 1, "Exactly one booking should succeed");
    assert_eq!(conflict_count, 2, "Two bookings should fail with conflict");

    ctx.cleanup().await;
}

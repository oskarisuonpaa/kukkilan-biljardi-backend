use axum::{
    Json,
    http::{StatusCode, header},
    response::IntoResponse,
};
use serde::Serialize;

pub struct Created<T> {
    pub location: String,
    pub body: T,
}

impl<T: Serialize> IntoResponse for Created<T> {
    fn into_response(self) -> axum::response::Response {
        let mut res = (StatusCode::CREATED, Json(self.body)).into_response();
        res.headers_mut().insert(
            header::LOCATION,
            header::HeaderValue::from_str(&self.location).unwrap(),
        );
        res
    }
}

pub struct NoContent;

impl IntoResponse for NoContent {
    fn into_response(self) -> axum::response::Response {
        StatusCode::NO_CONTENT.into_response()
    }
}

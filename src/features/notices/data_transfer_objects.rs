use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize)]
pub struct NoticeResponse {
    pub id: u32,
    pub title: String,
    pub content: String,
    pub active: bool,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CreateNoticeRequest {
    pub title: String,
    pub content: String,
    pub active: bool,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct UpdateNoticeRequest {
    pub title: Option<String>,
    pub content: Option<String>,
    pub is_active: Option<bool>,
}

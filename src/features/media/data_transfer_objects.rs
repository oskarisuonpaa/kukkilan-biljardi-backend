use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize)]
pub struct MediaResponse {
    pub id: u32,
    pub is_background: bool,
    pub kind: String,
    pub file_url: String,
    pub alt_text: String,
    pub width: Option<u32>,
    pub height: Option<u32>,
}

#[derive(Debug, Deserialize)]
pub struct CreateMediaRequest {
    pub is_background: Option<bool>,
    pub kind: String, // must be 'image' or 'video'
    pub file_url: String,
    pub alt_text: Option<String>,
    pub width: Option<u32>,
    pub height: Option<u32>,
}

#[derive(Debug, Deserialize)]
pub struct UpdateMediaRequest {
    pub is_background: Option<bool>,
    pub kind: Option<String>,
    pub file_url: Option<String>,
    pub alt_text: Option<String>,
    pub width: Option<u32>,
    pub height: Option<u32>,
}

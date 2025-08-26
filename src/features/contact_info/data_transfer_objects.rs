use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize)]
pub struct ContactInfoResponse {
    pub address: String,
    pub phone: String,
    pub email: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct UpdateContactInfoRequest {
    pub address: Option<String>,
    pub phone: Option<String>,
    pub email: Option<String>,
}

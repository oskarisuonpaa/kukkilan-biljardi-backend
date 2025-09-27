use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EmailMessage {
    pub to: String,
    pub to_name: Option<String>,
    pub subject: String,
    pub html_body: String,
    pub text_body: String,
}

#[derive(Debug, Clone)]
#[allow(dead_code)]
pub struct EmailConfig {
    pub smtp_host: String,
    pub smtp_port: u16,
    pub smtp_username: String,
    pub smtp_password: String,
    pub from_email: String,
    pub from_name: String,
    pub enabled: bool,
}

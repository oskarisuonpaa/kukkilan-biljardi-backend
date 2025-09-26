use super::{model::{EmailMessage, EmailConfig}, templates::{generate_booking_confirmation_html, generate_booking_confirmation_text, BookingConfirmationData}};
use crate::error::AppError;

#[derive(Clone)]
pub struct EmailService {
    config: EmailConfig,
}

impl EmailService {
    pub fn new(config: EmailConfig) -> Self {
        Self { config }
    }

    pub async fn send_booking_confirmation(
        &self,
        customer_email: &str,
        customer_name: &str,
        booking_data: BookingConfirmationData,
    ) -> Result<(), AppError> {
        if !self.config.enabled {
            tracing::info!("Email sending disabled, would send confirmation to: {}", customer_email);
            return Ok(());
        }

        let html_body = generate_booking_confirmation_html(&booking_data);
        let text_body = generate_booking_confirmation_text(&booking_data);

        let message = EmailMessage {
            to: customer_email.to_string(),
            to_name: Some(customer_name.to_string()),
            subject: format!("Varausvahvistus #{} - Kukkilan Biljardi", booking_data.booking_id),
            html_body,
            text_body,
        };

        self.send_email(message).await
    }

    async fn send_email(&self, message: EmailMessage) -> Result<(), AppError> {
        // For now, just log the email content instead of actually sending
        // This allows development and testing without SMTP setup
        tracing::info!("📧 Email would be sent:");
        tracing::info!("To: {} <{}>", message.to_name.unwrap_or_else(|| "Unknown".to_string()), message.to);
        tracing::info!("Subject: {}", message.subject);
        tracing::info!("Content preview: {}", &message.text_body[..message.text_body.len().min(200)]);
        
        // In production, this would integrate with lettre or similar SMTP library
        /*
        use lettre::prelude::*;
        use lettre::{SmtpTransport, Transport};
        
        let email = Message::builder()
            .from(format!("{} <{}>", self.config.from_name, self.config.from_email).parse().unwrap())
            .to(format!("{} <{}>", message.to_name.unwrap_or_default(), message.to).parse().unwrap())
            .subject(message.subject)
            .multipart(
                MultiPart::alternative()
                    .singlepart(SinglePart::builder()
                        .header(header::ContentType::TEXT_PLAIN)
                        .body(message.text_body))
                    .singlepart(SinglePart::builder()
                        .header(header::ContentType::TEXT_HTML)
                        .body(message.html_body))
            )
            .unwrap();

        let creds = Credentials::new(self.config.smtp_username.clone(), self.config.smtp_password.clone());
        
        let mailer = SmtpTransport::relay(&self.config.smtp_host)?
            .port(self.config.smtp_port)
            .credentials(creds)
            .build();

        match mailer.send(&email) {
            Ok(_) => {
                log::info!("Email sent successfully to: {}", message.to);
                Ok(())
            },
            Err(e) => {
                log::error!("Failed to send email to {}: {}", message.to, e);
                Err(AppError::Internal("Failed to send email"))
            }
        }
        */

        Ok(())
    }
}
use std::sync::Arc;
use chrono::Utc;
use jsonwebtoken::{encode, decode, Header, Validation, EncodingKey, DecodingKey};
use crate::features::auth::{
    repository::AuthRepository,
    model::{AdminUser, AdminSession},
    data_transfer_objects::{LoginRequest, LoginResponse, AdminUserResponse, TokenClaims},
};
use crate::error::AppError;

#[derive(Clone)]
pub struct AuthService {
    repository: Arc<dyn AuthRepository>,
    jwt_secret: String,
}

impl AuthService {
    pub fn new(repository: Arc<dyn AuthRepository>) -> Self {
        let jwt_secret = std::env::var("JWT_SECRET")
            .unwrap_or_else(|_| "your-secret-key-change-this-in-production".to_string());
        
        Self { 
            repository,
            jwt_secret,
        }
    }

    pub async fn authenticate(&self, request: LoginRequest) -> Result<LoginResponse, AppError> {
        // First, try to find user in database
        if let Some(user) = self.repository.find_by_username(&request.username).await? {
            // Verify password against database hash
            let is_valid = match bcrypt::verify(&request.password, &user.password_hash) {
                Ok(valid) => valid,
                Err(e) => {
                    tracing::error!("Password verification error: {}", e);
                    false
                }
            };

            if is_valid {
                // Generate JWT token
                let now = Utc::now().timestamp() as usize;
                let claims = TokenClaims {
                    sub: user.id.clone(),
                    username: user.username.clone(),
                    exp: now + 24 * 60 * 60, // 24 hours
                    iat: now,
                };

                let token = encode(
                    &Header::default(),
                    &claims,
                    &EncodingKey::from_secret(self.jwt_secret.as_ref()),
                )?;

                // Update last login
                if let Err(e) = self.repository.update_last_login(&user.id).await {
                    tracing::warn!("Failed to update last login: {}", e);
                }

                return Ok(LoginResponse {
                    success: true,
                    message: "Kirjautuminen onnistui".to_string(),
                    token: Some(token),
                    user: Some(AdminUserResponse {
                        id: user.id,
                        username: user.username,
                        email: user.email,
                    }),
                });
            }
        }

        // Fallback: Check for hardcoded default credentials for development
        if request.username == "admin" && request.password == "admin123" {
            tracing::warn!("Using hardcoded admin credentials - NOT SECURE FOR PRODUCTION!");
            
            let now = Utc::now().timestamp() as usize;
            let claims = TokenClaims {
                sub: "default-admin".to_string(),
                username: "admin".to_string(),
                exp: now + 24 * 60 * 60, // 24 hours
                iat: now,
            };

            let token = encode(
                &Header::default(),
                &claims,
                &EncodingKey::from_secret(self.jwt_secret.as_ref()),
            )?;

            return Ok(LoginResponse {
                success: true,
                message: "Kirjautuminen onnistui (dev mode)".to_string(),
                token: Some(token),
                user: Some(AdminUserResponse {
                    id: "default-admin".to_string(),
                    username: "admin".to_string(),
                    email: Some("admin@kukkilan-biljardi.fi".to_string()),
                }),
            });
        }

        // Authentication failed
        Ok(LoginResponse {
            success: false,
            message: "Virheellinen käyttäjätunnus tai salasana".to_string(),
            token: None,
            user: None,
        })
    }

    pub fn verify_token(&self, token: &str) -> Result<TokenClaims, AppError> {
        match decode::<TokenClaims>(
            token,
            &DecodingKey::from_secret(self.jwt_secret.as_ref()),
            &Validation::default(),
        ) {
            Ok(token_data) => Ok(token_data.claims),
            Err(e) => {
                tracing::debug!("Token verification failed: {}", e);
                Err(AppError::Unauthorized("Invalid token".to_string()))
            }
        }
    }

    pub fn create_session(&self, user: AdminUser) -> AdminSession {
        AdminSession {
            user_id: user.id,
            username: user.username,
            login_time: Utc::now(),
        }
    }
}
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
        // Find user by username
        let user = match self.repository.find_by_username(&request.username).await? {
            Some(user) => user,
            None => {
                return Ok(LoginResponse {
                    success: false,
                    message: "Virheellinen käyttäjätunnus tai salasana".to_string(),
                    token: None,
                    user: None,
                });
            }
        };

        // Verify password
        let is_valid = match bcrypt::verify(&request.password, &user.password_hash) {
            Ok(valid) => valid,
            Err(e) => {
                tracing::error!("Password verification error: {}", e);
                return Ok(LoginResponse {
                    success: false,
                    message: "Autentikointivirhe".to_string(),
                    token: None,
                    user: None,
                });
            }
        };

        if !is_valid {
            return Ok(LoginResponse {
                success: false,
                message: "Virheellinen käyttäjätunnus tai salasana".to_string(),
                token: None,
                user: None,
            });
        }

        // Update last login
        if let Err(e) = self.repository.update_last_login(&user.id).await {
            tracing::warn!("Failed to update last login for user {}: {}", user.username, e);
        }

        // Generate JWT token
        let now = Utc::now().timestamp() as usize;
        let claims = TokenClaims {
            sub: user.id.clone(),
            username: user.username.clone(),
            exp: now + 24 * 60 * 60, // 24 hours
            iat: now,
        };

        let token = match encode(
            &Header::default(),
            &claims,
            &EncodingKey::from_secret(self.jwt_secret.as_ref()),
        ) {
            Ok(token) => token,
            Err(e) => {
                tracing::error!("JWT token generation error: {}", e);
                return Err(AppError::InternalServerError("Failed to generate token".to_string()));
            }
        };

        Ok(LoginResponse {
            success: true,
            message: "Kirjautuminen onnistui".to_string(),
            token: Some(token),
            user: Some(AdminUserResponse {
                id: user.id,
                username: user.username,
                email: user.email,
            }),
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
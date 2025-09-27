use crate::error::AppError;
use base64::{Engine as _, engine::general_purpose};

/// Service for encrypting and decrypting sensitive customer data
#[derive(Clone)]
pub struct EncryptionService {
    key: [u8; 32], // 256-bit key for AES-256
}

impl EncryptionService {
    /// Create a new encryption service with a key from environment variable
    #[allow(dead_code)]
    pub fn new() -> Result<Self, AppError> {
        let key_str = std::env::var("ENCRYPTION_KEY")
            .map_err(|_| AppError::Internal("ENCRYPTION_KEY environment variable not set"))?;

        if key_str.len() != 64 {
            return Err(AppError::Internal(
                "ENCRYPTION_KEY must be 64 hex characters (32 bytes)",
            ));
        }

        let mut key = [0u8; 32];
        for i in 0..32 {
            let hex_byte = &key_str[i * 2..i * 2 + 2];
            key[i] = u8::from_str_radix(hex_byte, 16)
                .map_err(|_| AppError::Internal("Invalid hex in ENCRYPTION_KEY"))?;
        }

        Ok(Self { key })
    }

    /// Create a new encryption service with a default development key
    /// WARNING: Only for development use! Do not use in production!
    pub fn new_with_default_key() -> Self {
        tracing::warn!("Using default encryption key - NOT SECURE FOR PRODUCTION!");
        let key = [
            0x01, 0x02, 0x03, 0x04, 0x05, 0x06, 0x07, 0x08, 0x09, 0x0A, 0x0B, 0x0C, 0x0D, 0x0E,
            0x0F, 0x10, 0x11, 0x12, 0x13, 0x14, 0x15, 0x16, 0x17, 0x18, 0x19, 0x1A, 0x1B, 0x1C,
            0x1D, 0x1E, 0x1F, 0x20,
        ];
        Self { key }
    }

    /// Create encryption service from config, with fallback to default key
    pub fn from_config(config_key: Option<&String>) -> Self {
        match config_key {
            Some(key_str) => {
                if key_str.len() != 64 {
                    tracing::warn!(
                        "ENCRYPTION_KEY must be 64 hex characters. Using fallback encryption."
                    );
                    return Self::new_with_default_key();
                }

                let mut key = [0u8; 32];
                for i in 0..32 {
                    let hex_byte = &key_str[i * 2..i * 2 + 2];
                    match u8::from_str_radix(hex_byte, 16) {
                        Ok(byte) => key[i] = byte,
                        Err(_) => {
                            tracing::warn!(
                                "Invalid hex in ENCRYPTION_KEY. Using fallback encryption."
                            );
                            return Self::new_with_default_key();
                        }
                    }
                }

                tracing::info!("Using encryption key from configuration");
                Self { key }
            }
            None => {
                tracing::warn!("ENCRYPTION_KEY not provided in config. Using fallback encryption.");
                Self::new_with_default_key()
            }
        }
    }

    /// Encrypt a string and return base64-encoded result
    pub fn encrypt(&self, plaintext: &str) -> Result<String, AppError> {
        if plaintext.is_empty() {
            return Ok(String::new());
        }

        // For now, use a simple XOR cipher for demonstration
        // In production, use proper AES encryption with libraries like `aes-gcm`
        let mut encrypted = Vec::new();
        let plaintext_bytes = plaintext.as_bytes();

        for (i, &byte) in plaintext_bytes.iter().enumerate() {
            let key_byte = self.key[i % self.key.len()];
            encrypted.push(byte ^ key_byte);
        }

        Ok(general_purpose::STANDARD.encode(encrypted))
    }

    /// Decrypt a base64-encoded string
    pub fn decrypt(&self, ciphertext: &str) -> Result<String, AppError> {
        if ciphertext.is_empty() {
            return Ok(String::new());
        }

        // Try to decode as base64 first
        match general_purpose::STANDARD.decode(ciphertext) {
            Ok(encrypted_bytes) => {
                // Successfully decoded base64, try to decrypt
                let mut decrypted = Vec::new();
                for (i, &byte) in encrypted_bytes.iter().enumerate() {
                    let key_byte = self.key[i % self.key.len()];
                    decrypted.push(byte ^ key_byte);
                }

                match String::from_utf8(decrypted) {
                    Ok(decrypted_string) => Ok(decrypted_string),
                    Err(_) => {
                        // If decryption fails, assume it's plain text and return as-is
                        tracing::warn!(
                            "Failed to decrypt data, assuming plain text: {}",
                            ciphertext
                        );
                        Ok(ciphertext.to_string())
                    }
                }
            }
            Err(_) => {
                // Not valid base64, assume it's plain text
                tracing::debug!(
                    "Data not base64 encoded, assuming plain text: {}",
                    ciphertext
                );
                Ok(ciphertext.to_string())
            }
        }
    }

    /// Encrypt customer email
    pub fn encrypt_email(&self, email: &str) -> Result<String, AppError> {
        self.encrypt(email)
    }

    /// Decrypt customer email
    pub fn decrypt_email(&self, encrypted_email: &str) -> Result<String, AppError> {
        self.decrypt(encrypted_email)
    }

    /// Encrypt customer phone number
    pub fn encrypt_phone(&self, phone: &str) -> Result<String, AppError> {
        self.encrypt(phone)
    }

    /// Decrypt customer phone number
    pub fn decrypt_phone(&self, encrypted_phone: &str) -> Result<String, AppError> {
        self.decrypt(encrypted_phone)
    }

    /// Encrypt customer name
    pub fn encrypt_name(&self, name: &str) -> Result<String, AppError> {
        self.encrypt(name)
    }

    /// Decrypt customer name
    pub fn decrypt_name(&self, encrypted_name: &str) -> Result<String, AppError> {
        self.decrypt(encrypted_name)
    }
}

/// Generate a random 256-bit encryption key in hex format
#[allow(dead_code)]
pub fn generate_encryption_key() -> String {
    use rand::RngCore;
    let mut key = [0u8; 32];
    rand::thread_rng().fill_bytes(&mut key);
    hex::encode(key)
}

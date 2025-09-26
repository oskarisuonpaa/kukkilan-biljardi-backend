use crate::error::AppError;
use base64::{Engine as _, engine::general_purpose};

/// Service for encrypting and decrypting sensitive customer data
#[derive(Clone)]
pub struct EncryptionService {
    key: [u8; 32], // 256-bit key for AES-256
}

impl EncryptionService {
    /// Create a new encryption service with a key from environment variable
    pub fn new() -> Result<Self, AppError> {
        let key_str = std::env::var("ENCRYPTION_KEY")
            .map_err(|_| AppError::Internal("ENCRYPTION_KEY environment variable not set"))?;
        
        if key_str.len() != 64 {
            return Err(AppError::Internal("ENCRYPTION_KEY must be 64 hex characters (32 bytes)"));
        }

        let mut key = [0u8; 32];
        for i in 0..32 {
            let hex_byte = &key_str[i * 2..i * 2 + 2];
            key[i] = u8::from_str_radix(hex_byte, 16)
                .map_err(|_| AppError::Internal("Invalid hex in ENCRYPTION_KEY"))?;
        }

        Ok(Self { key })
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

        let encrypted_bytes = general_purpose::STANDARD
            .decode(ciphertext)
            .map_err(|_| AppError::Internal("Invalid base64 in encrypted data"))?;

        let mut decrypted = Vec::new();
        for (i, &byte) in encrypted_bytes.iter().enumerate() {
            let key_byte = self.key[i % self.key.len()];
            decrypted.push(byte ^ key_byte);
        }

        String::from_utf8(decrypted)
            .map_err(|_| AppError::Internal("Invalid UTF-8 in decrypted data"))
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
pub fn generate_encryption_key() -> String {
    use rand::RngCore;
    let mut key = [0u8; 32];
    rand::thread_rng().fill_bytes(&mut key);
    hex::encode(key)
}
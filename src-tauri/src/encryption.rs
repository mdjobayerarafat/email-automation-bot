use aes_gcm::{Aes256Gcm, Nonce, KeyInit};
use aes_gcm::aead::Aead;
use base64::{Engine as _, engine::general_purpose};
use rand::RngCore;
use crate::models::AppError;
use std::env;

pub struct EncryptionService {
    cipher: Aes256Gcm,
}

impl EncryptionService {
    pub fn new() -> Result<Self, AppError> {
        let key_string = env::var("ENCRYPTION_KEY")
            .map_err(|_| AppError::Config("ENCRYPTION_KEY not found".to_string()))?;
        
        if key_string.len() != 64 {
            return Err(AppError::Config("ENCRYPTION_KEY must be 64 characters (32 bytes hex)".to_string()));
        }
        
        let key_bytes = hex::decode(&key_string)
            .map_err(|_| AppError::Config("Invalid ENCRYPTION_KEY format".to_string()))?;
        
        let cipher = Aes256Gcm::new_from_slice(&key_bytes)
            .map_err(|_| AppError::Config("Invalid key length".to_string()))?;
        
        Ok(EncryptionService { cipher })
    }

    pub fn encrypt(&self, plaintext: &str) -> Result<String, AppError> {
        // Generate a random 12-byte nonce
        let mut nonce_bytes = [0u8; 12];
        rand::thread_rng().fill_bytes(&mut nonce_bytes);
        let nonce = Nonce::from_slice(&nonce_bytes);
        
        // Encrypt the plaintext
        let ciphertext = self.cipher.encrypt(nonce, plaintext.as_bytes())
            .map_err(|e| AppError::Internal(format!("Encryption failed: {}", e)))?;
        
        // Combine nonce and ciphertext
        let mut result = Vec::new();
        result.extend_from_slice(&nonce_bytes);
        result.extend_from_slice(&ciphertext);
        
        // Encode as base64
        Ok(general_purpose::STANDARD.encode(&result))
    }

    pub fn decrypt(&self, encrypted_data: &str) -> Result<String, AppError> {
        // Decode from base64
        let data = general_purpose::STANDARD.decode(encrypted_data)
            .map_err(|e| AppError::Internal(format!("Base64 decode failed: {}", e)))?;
        
        if data.len() < 12 {
            return Err(AppError::Internal("Invalid encrypted data length".to_string()));
        }
        
        // Extract nonce and ciphertext
        let (nonce_bytes, ciphertext) = data.split_at(12);
        let nonce = Nonce::from_slice(nonce_bytes);
        
        // Decrypt
        let plaintext = self.cipher.decrypt(nonce, ciphertext)
            .map_err(|e| AppError::Internal(format!("Decryption failed: {}", e)))?;
        
        String::from_utf8(plaintext)
            .map_err(|e| AppError::Internal(format!("UTF-8 conversion failed: {}", e)))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_encryption_decryption() {
        let service = EncryptionService::new().unwrap();
        let original = "test_password_123";
        
        let encrypted = service.encrypt(original).unwrap();
        let decrypted = service.decrypt(&encrypted).unwrap();
        
        assert_eq!(original, decrypted);
    }

    #[test]
    fn test_different_encryptions() {
        let service = EncryptionService::new().unwrap();
        let original = "same_password";
        
        let encrypted1 = service.encrypt(original).unwrap();
        let encrypted2 = service.encrypt(original).unwrap();
        
        // Should be different due to random nonce
        assert_ne!(encrypted1, encrypted2);
        
        // But both should decrypt to the same value
        assert_eq!(service.decrypt(&encrypted1).unwrap(), original);
        assert_eq!(service.decrypt(&encrypted2).unwrap(), original);
    }
}
use jsonwebtoken::{decode, encode, DecodingKey, EncodingKey, Header, Validation};
use serde::{Deserialize, Serialize};
use chrono::{Duration, Utc};
use crate::models::{AppError, User, UserInfo};
use std::env;

#[derive(Debug, Serialize, Deserialize)]
pub struct Claims {
    pub sub: String, // Subject (user id)
    pub email: String,
    pub username: String,
    pub exp: usize, // Expiration time
    pub iat: usize, // Issued at
}

pub struct AuthService {
    encoding_key: EncodingKey,
    decoding_key: DecodingKey,
}

impl AuthService {
    pub fn new() -> Result<Self, AppError> {
        let secret = env::var("JWT_SECRET")
            .unwrap_or_else(|_| "your-secret-key-change-this-in-production".to_string());
        
        let encoding_key = EncodingKey::from_secret(secret.as_ref());
        let decoding_key = DecodingKey::from_secret(secret.as_ref());
        
        Ok(AuthService {
            encoding_key,
            decoding_key,
        })
    }

    pub fn generate_token(&self, user: &User) -> Result<String, AppError> {
        let now = Utc::now();
        let exp = now + Duration::hours(24); // Token expires in 24 hours
        
        let claims = Claims {
            sub: user.id.to_string(),
            email: user.email.clone(),
            username: user.username.clone(),
            exp: exp.timestamp() as usize,
            iat: now.timestamp() as usize,
        };

        encode(&Header::default(), &claims, &self.encoding_key)
            .map_err(|e| AppError::Auth(format!("Failed to generate token: {}", e)))
    }

    pub fn verify_token(&self, token: &str) -> Result<Claims, AppError> {
        let token_data = decode::<Claims>(
            token,
            &self.decoding_key,
            &Validation::default(),
        )
        .map_err(|e| AppError::Auth(format!("Invalid token: {}", e)))?;

        Ok(token_data.claims)
    }

    pub fn verify_password(&self, password: &str, hash: &str) -> Result<bool, AppError> {
        bcrypt::verify(password, hash)
            .map_err(|e| AppError::Auth(format!("Password verification failed: {}", e)))
    }

    pub fn hash_password(&self, password: &str) -> Result<String, AppError> {
        bcrypt::hash(password, bcrypt::DEFAULT_COST)
            .map_err(|e| AppError::Auth(format!("Password hashing failed: {}", e)))
    }

    pub fn extract_user_from_token(&self, token: &str) -> Result<UserInfo, AppError> {
        let claims = self.verify_token(token)?;
        
        let user_id = claims.sub.parse::<i32>()
            .map_err(|_| AppError::Auth("Invalid user ID in token".to_string()))?;
        
        Ok(UserInfo {
            id: user_id,
            username: claims.username,
            email: claims.email,
        })
    }
}

// Middleware function to extract user from Authorization header
pub fn extract_user_from_header(auth_header: Option<&str>, auth_service: &AuthService) -> Result<UserInfo, AppError> {
    let auth_header = auth_header.ok_or_else(|| AppError::Auth("Missing Authorization header".to_string()))?;
    
    if !auth_header.starts_with("Bearer ") {
        return Err(AppError::Auth("Invalid Authorization header format".to_string()));
    }
    
    let token = &auth_header[7..]; // Remove "Bearer " prefix
    auth_service.extract_user_from_token(token)
}

#[cfg(test)]
mod tests {
    use super::*;
    use chrono::Utc;

    #[test]
    fn test_token_generation_and_verification() {
        let auth_service = AuthService::new().unwrap();
        
        let user = User {
            id: 1,
            username: "testuser".to_string(),
            email: "test@example.com".to_string(),
            password_hash: "hash".to_string(),
            created_at: Utc::now(),
            updated_at: Utc::now(),
        };

        let token = auth_service.generate_token(&user).unwrap();
        let claims = auth_service.verify_token(&token).unwrap();
        
        assert_eq!(claims.sub, "1");
        assert_eq!(claims.email, "test@example.com");
        assert_eq!(claims.username, "testuser");
    }

    #[test]
    fn test_password_hashing_and_verification() {
        let auth_service = AuthService::new().unwrap();
        let password = "test_password";
        
        let hash = auth_service.hash_password(password).unwrap();
        let is_valid = auth_service.verify_password(password, &hash).unwrap();
        
        assert!(is_valid);
        
        let is_invalid = auth_service.verify_password("wrong_password", &hash).unwrap();
        assert!(!is_invalid);
    }
}
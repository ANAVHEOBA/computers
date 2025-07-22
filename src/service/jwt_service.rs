use jsonwebtoken::{encode, decode, Header, EncodingKey, DecodingKey, Validation, errors::Error as JwtError};
use serde::{Deserialize, Serialize};
use std::env;
use chrono::{Utc, Duration};
use uuid::Uuid;

#[derive(Debug, Serialize, Deserialize)]
pub enum Role {
    User,
    Admin,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Claims {
    pub sub: String,  // Subject (user ID)
    pub exp: i64,     // Expiration time
    pub iat: i64,     // Issued at
    pub jti: String,  // JWT ID
    pub email: String,
    pub first_name: String,
    pub last_name: String,
    pub role: Role,   // User role
}

impl Claims {
    pub fn new(user_id: String, email: String, first_name: String, last_name: String, role: Role) -> Self {
        let now = Utc::now();
        let expires_at = now + Duration::hours(24); // Token expires in 24 hours

        Self {
            sub: user_id,
            exp: expires_at.timestamp(),
            iat: now.timestamp(),
            jti: Uuid::new_v4().to_string(),
            email,
            first_name,
            last_name,
            role,
        }
    }
}

pub struct JwtService {
    encoding_key: EncodingKey,
    decoding_key: DecodingKey,
}

impl JwtService {
    pub fn new() -> Self {
        let secret = env::var("JWT_SECRET").expect("JWT_SECRET must be set");
        Self {
            encoding_key: EncodingKey::from_secret(secret.as_bytes()),
            decoding_key: DecodingKey::from_secret(secret.as_bytes()),
        }
    }

    pub fn generate_token(
        &self,
        user_id: String,
        email: String,
        first_name: String,
        last_name: String,
        role: Role,
    ) -> Result<String, JwtError> {
        let claims = Claims::new(user_id, email, first_name, last_name, role);
        encode(&Header::default(), &claims, &self.encoding_key)
    }

    pub fn verify_token(&self, token: &str) -> Result<Claims, JwtError> {
        let validation = Validation::default();
        let token_data = decode::<Claims>(token, &self.decoding_key, &validation)?;
        Ok(token_data.claims)
    }
} 
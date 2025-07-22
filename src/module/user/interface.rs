use async_trait::async_trait;
use mongodb::error::Error as MongoError;
use chrono::{DateTime, Utc};

use crate::module::user::{
    model::User,
    schema::{UserRegistrationSchema, LoginSchema},
};

#[async_trait]
pub trait UserRepository {
    async fn create_user(&self, user: User) -> Result<User, MongoError>;
    async fn find_by_email(&self, email: &str) -> Result<Option<User>, MongoError>;
    async fn find_by_phone(&self, phone: &str) -> Result<Option<User>, MongoError>;
    async fn update_user_verification_status(&self, email: &str, status: bool) -> Result<(), MongoError>;
    async fn delete_user_by_email(&self, email: &str) -> Result<(), MongoError>;
    async fn update_verification_code(&self, email: &str, code: &str, expires_at: DateTime<Utc>) -> Result<(), MongoError>;
}

pub trait UserValidation {
    fn validate_registration(data: &UserRegistrationSchema) -> Result<(), String> {
        // Validate password match
        if data.password != data.confirm_password {
            return Err("Passwords do not match".to_string());
        }

        // Validate password strength (you might want to add more rules)
        if data.password.len() < 8 {
            return Err("Password must be at least 8 characters long".to_string());
        }

        Ok(())
    }

    fn validate_login(data: &LoginSchema) -> Result<(), String> {
        // Add any additional login validation rules here
        if data.password.is_empty() {
            return Err("Password cannot be empty".to_string());
        }

        Ok(())
    }
}

#[async_trait]
pub trait UserService: UserValidation {
    async fn register_user(&self, data: UserRegistrationSchema) -> Result<User, String>;
    async fn verify_email(&self, email: &str, code: &str) -> Result<(), String>;
    async fn login(&self, data: LoginSchema) -> Result<User, String>;
    async fn resend_verification_code(&self, email: &str) -> Result<(), String>;
    // async fn verify_phone(&self, user_id: String, code: String) -> Result<(), String>;
    // async fn request_password_reset(&self, email: String) -> Result<(), String>;
    // async fn reset_password(&self, token: String, new_password: String) -> Result<(), String>;
}

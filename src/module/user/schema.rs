use serde::{Deserialize, Serialize};
use validator::Validate;

#[derive(Debug, Serialize, Deserialize, Validate)]
pub struct UserRegistrationSchema {
    #[validate(length(min = 2, max = 50, message = "First name must be between 2 and 50 characters"))]
    pub first_name: String,

    #[validate(length(min = 2, max = 50, message = "Last name must be between 2 and 50 characters"))]
    pub last_name: String,

    #[validate(email(message = "Invalid email format"))]
    pub email: String,

    #[validate(length(min = 10, max = 15, message = "Phone number must be between 10 and 15 characters"))]
    pub phone_number: String,

    #[validate(length(min = 8, message = "Password must be at least 8 characters long"))]
    pub password: String,

    pub confirm_password: String,

    #[serde(default)]
    pub profile_picture: Option<String>,
    
    #[serde(default = "default_is_active")]
    pub is_active: bool,
    
    #[serde(default)]
    pub bio: Option<String>,
}

#[derive(Debug, Serialize, Deserialize, Validate)]
pub struct LoginSchema {
    #[validate(email(message = "Invalid email format"))]
    pub email: String,

    #[validate(length(min = 1, message = "Password is required"))]
    pub password: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct VerifyEmailSchema {
    pub email: String,
    pub verification_code: String,
}

fn default_is_active() -> bool {
    true
}

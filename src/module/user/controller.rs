use actix_web::web;
use chrono::{Duration, Utc};
use serde_json::{json, Value};
use async_trait::async_trait;
use tokio;
use bcrypt::verify;
use uuid::Uuid;

use crate::module::user::{
    crud::UserCrud,
    interface::{UserRepository, UserService, UserValidation},
    model::User,
    schema::{UserRegistrationSchema, VerifyEmailSchema, LoginSchema},
};
use crate::service::{
    email_service, email_templates, jwt_service::JwtService,
    google_oauth_service::GoogleOauthService,
};

pub struct UserController {
    crud: UserCrud,
    jwt_service: JwtService,
    google_oauth_service: GoogleOauthService,
}

impl UserController {
    pub fn new(crud: UserCrud) -> Self {
        Self { 
            crud,
            jwt_service: JwtService::new(),
            google_oauth_service: GoogleOauthService::new(),
        }
    }

    pub async fn handle_google_oauth(&self, data: Value) -> web::Json<Value> {
        let code = match data.get("code").and_then(|v| v.as_str()) {
            Some(c) => c,
            None => {
                return web::Json(json!({
                    "status": "error",
                    "message": "Authorization code not provided."
                }));
            }
        };

        // Exchange code for user info
        let user_info = match self.google_oauth_service.exchange_code_for_user_info(code).await {
            Ok(info) => info,
            Err(e) => {
                return web::Json(json!({
                    "status": "error",
                    "message": e
                }));
            }
        };

        // Check if user exists, otherwise create a new one
        let user = match self.crud.find_by_email(&user_info.email).await {
            Ok(Some(user)) => user, // User exists
            Ok(None) => { // User does not exist, create them
                let new_user = User {
                    id: None,
                    first_name: user_info.given_name,
                    last_name: user_info.family_name,
                    email: user_info.email.clone(),
                    phone_number: "".to_string(), // No phone number from Google
                    password_hash: Uuid::new_v4().to_string(), // No password, use a placeholder
                    profile_picture: user_info.picture,
                    is_active: true,
                    bio: None,
                    email_verified: true, // Verified by Google
                    phone_verified: false,
                    verification_code: None,
                    verification_code_expires_at: None,
                    created_at: Utc::now(),
                    updated_at: Utc::now(),
                };

                match self.crud.create_user(new_user).await {
                    Ok(user) => user,
                    Err(e) => {
                        return web::Json(json!({
                            "status": "error",
                            "message": format!("Failed to create user: {}", e)
                        }));
                    }
                }
            },
            Err(e) => {
                return web::Json(json!({
                    "status": "error",
                    "message": format!("Database error: {}", e)
                }));
            }
        };
        
        // At this point, `user` is either the existing or newly created user.
        // Generate JWT token
        let user_id = user.id.map(|id| id.to_string()).unwrap_or_else(|| "unknown".to_string());
        
        let token = match self.jwt_service.generate_token(
            user_id.clone(),
            user.email.clone(),
            user.first_name.clone(),
            user.last_name.clone(),
        ) {
            Ok(token) => token,
            Err(e) => {
                return web::Json(json!({
                    "status": "error",
                    "message": format!("Failed to generate token: {}", e)
                }));
            }
        };

        web::Json(json!({
            "status": "success",
            "message": "Google sign-in successful",
            "data": {
                "token": token,
                "user": {
                    "user_id": user_id,
                    "email": user.email,
                    "first_name": user.first_name,
                    "last_name": user.last_name,
                    "is_verified": user.email_verified
                }
            }
        }))
    }

    pub async fn handle_registration(&self, data: Value) -> web::Json<Value> {
        // Parse and validate input
        let registration_data = match serde_json::from_value::<UserRegistrationSchema>(data) {
            Ok(data) => data,
            Err(e) => {
                return web::Json(json!({
                    "status": "error",
                    "message": format!("Invalid input data: {}", e)
                }));
            }
        };

        // Process registration
        match self.register_user(registration_data).await {
            Ok(user) => web::Json(json!({
                "status": "success",
                "message": "Registration successful. Please check your email for a verification code.",
                "data": { "user_id": user.id }
            })),
            Err(e) => web::Json(json!({
                "status": "error",
                "message": e
            })),
        }
    }

    pub async fn handle_email_verification(&self, data: Value) -> web::Json<Value> {
        let verification_data = match serde_json::from_value::<VerifyEmailSchema>(data) {
            Ok(data) => data,
            Err(e) => {
                return web::Json(json!({
                    "status": "error",
                    "message": format!("Invalid input data: {}", e)
                }));
            }
        };

        match self.verify_email(&verification_data.email, &verification_data.verification_code).await {
            Ok(()) => web::Json(json!({
                "status": "success",
                "message": "Email verified successfully."
            })),
            Err(e) => web::Json(json!({
                "status": "error",
                "message": e
            })),
        }
    }

    // Temporary handler to delete a user for testing purposes
    pub async fn handle_delete_user(&self, data: web::Json<Value>) -> web::Json<Value> {
        let email = match data.get("email").and_then(|v| v.as_str()) {
            Some(e) => e.to_string(),
            None => {
                return web::Json(json!({
                    "status": "error",
                    "message": "Email is required."
                }));
            }
        };

        match self.crud.delete_user_by_email(&email).await {
            Ok(_) => web::Json(json!({
                "status": "success",
                "message": format!("User with email {} deleted.", email)
            })),
            Err(_) => web::Json(json!({
                "status": "error",
                "message": "Failed to delete user."
            })),
        }
    }

    pub async fn handle_resend_verification(&self, data: Value) -> web::Json<Value> {
        let email = match data.get("email").and_then(|v| v.as_str()) {
            Some(e) => e.to_string(),
            None => {
                return web::Json(json!({
                    "status": "error",
                    "message": "Email is required."
                }));
            }
        };

        match self.resend_verification_code(&email).await {
            Ok(()) => web::Json(json!({
                "status": "success",
                "message": "Verification code has been resent. Please check your email (including spam folder)."
            })),
            Err(e) => web::Json(json!({
                "status": "error",
                "message": e
            })),
        }
    }

    pub async fn handle_login(&self, data: Value) -> web::Json<Value> {
        // Parse and validate input
        let login_data = match serde_json::from_value::<LoginSchema>(data) {
            Ok(data) => data,
            Err(e) => {
                return web::Json(json!({
                    "status": "error",
                    "message": format!("Invalid input data: {}", e)
                }));
            }
        };

        // Process login
        match self.login(login_data).await {
            Ok(user) => {
                // Generate JWT token
                let user_id = user.id.map(|id| id.to_string())
                    .unwrap_or_else(|| "unknown".to_string());
                
                let token = match self.jwt_service.generate_token(
                    user_id.clone(),
                    user.email.clone(),
                    user.first_name.clone(),
                    user.last_name.clone(),
                ) {
                    Ok(token) => token,
                    Err(e) => {
                        return web::Json(json!({
                            "status": "error",
                            "message": format!("Failed to generate token: {}", e)
                        }));
                    }
                };

                web::Json(json!({
                    "status": "success",
                    "message": "Login successful",
                    "data": {
                        "token": token,
                        "user": {
                            "user_id": user_id,
                            "email": user.email,
                            "first_name": user.first_name,
                            "last_name": user.last_name,
                            "is_verified": user.email_verified
                        }
                    }
                }))
            },
            Err(e) => web::Json(json!({
                "status": "error",
                "message": e
            })),
        }
    }
}

impl UserValidation for UserController {}

#[async_trait]
impl UserService for UserController {
    async fn register_user(&self, data: UserRegistrationSchema) -> Result<User, String> {
        // Validate registration data
        Self::validate_registration(&data)?;

        // Check if email already exists
        if self.crud.find_by_email(&data.email).await.unwrap_or(None).is_some() {
            return Err("Email already registered".to_string());
        }

        // Check if phone number already exists
        if self.crud.find_by_phone(&data.phone_number).await.unwrap_or(None).is_some() {
            return Err("Phone number already registered".to_string());
        }

        // Hash password
        let password_hash = UserCrud::hash_password(&data.password).await?;

        // Generate verification code
        let verification_code = email_templates::generate_verification_code();
        let expires_at = Utc::now() + Duration::minutes(10);

        // Create new user
        let mut user = User::new(
            data.first_name.clone(),
            data.last_name,
            data.email.clone(),
            data.phone_number,
            password_hash,
        );
        user.verification_code = Some(verification_code.clone());
        user.verification_code_expires_at = Some(expires_at);

        // Save user to database
        let saved_user = self.crud.create_user(user).await.map_err(|e| format!("Failed to create user: {}", e))?;

        // Send verification email
        let (subject, body) = email_templates::get_verification_email_template(&data.first_name, &verification_code);
        
        // We run email sending in a separate thread so it doesn't block the API response.
        tokio::task::spawn_blocking(move || {
            if let Err(e) = email_service::send_email(&data.email, &data.first_name, &subject, &body) {
                // In a real application, you'd want more robust error handling here,
                // like logging the error or adding the email to a retry queue.
                eprintln!("Failed to send verification email via Gmail: {}", e);
            }
        });

        Ok(saved_user)
    }

    // We'll implement these methods later as needed
    async fn verify_email(&self, email: &str, code: &str) -> Result<(), String> {
        // Find user by email
        let user = self.crud.find_by_email(email).await
            .map_err(|_| "An error occurred.".to_string())?
            .ok_or_else(|| "User not found.".to_string())?;

        // Check if user is already verified
        if user.email_verified {
            return Err("Email is already verified.".to_string());
        }

        // Check if the code is correct and not expired
        if let (Some(saved_code), Some(expires_at)) = (user.verification_code, user.verification_code_expires_at) {
            if saved_code == code && Utc::now() < expires_at {
                // Code is valid, update user status
                self.crud.update_user_verification_status(email, true).await
                    .map_err(|_| "Failed to update verification status.".to_string())?;
                Ok(())
            } else {
                Err("Invalid or expired verification code.".to_string())
            }
        } else {
            Err("No verification process started for this user.".to_string())
        }
    }

    async fn verify_phone(&self, _user_id: String, _code: String) -> Result<(), String> {
        unimplemented!()
    }

    async fn request_password_reset(&self, _email: String) -> Result<(), String> {
        unimplemented!()
    }

    async fn reset_password(&self, _token: String, _new_password: String) -> Result<(), String> {
        unimplemented!()
    }

    async fn resend_verification_code(&self, email: &str) -> Result<(), String> {
        // Find user by email
        let user = self.crud.find_by_email(email).await
            .map_err(|e| format!("Database error: {}", e))?  // Show the actual error
            .ok_or_else(|| "User not found.".to_string())?;

        // Check if user is already verified
        if user.email_verified {
            return Err("Email is already verified.".to_string());
        }

        // Generate new verification code
        let verification_code = email_templates::generate_verification_code();
        let expires_at = Utc::now() + Duration::minutes(10);

        // Update user with new verification code
        self.crud.update_verification_code(email, &verification_code, expires_at).await
            .map_err(|e| format!("Failed to update verification code: {}", e))?;

        // Send new verification email
        let (subject, body) = email_templates::get_verification_email_template(&user.first_name, &verification_code);
        
        // Clone values for the async closure
        let email_owned = email.to_string();
        let first_name = user.first_name.clone();
        
        // Send email asynchronously
        tokio::task::spawn_blocking(move || {
            if let Err(e) = email_service::send_email(&email_owned, &first_name, &subject, &body) {
                eprintln!("Failed to send verification email: {}", e);
            }
        });

        Ok(())
    }

    async fn login(&self, data: LoginSchema) -> Result<User, String> {
        // Validate login data
        Self::validate_login(&data)?;

        // Find user by email
        let user = self.crud.find_by_email(&data.email).await
            .map_err(|e| format!("Database error: {}", e))?
            .ok_or_else(|| "Invalid email or password".to_string())?;

        // Verify password
        if !verify(&data.password, &user.password_hash)
            .map_err(|_| "Password verification failed".to_string())? {
            return Err("Invalid email or password".to_string());
        }

        // Check if email is verified
        if !user.email_verified {
            return Err("Please verify your email before logging in".to_string());
        }

        Ok(user)
    }
}

use actix_web::web;
use bcrypt::verify;
use serde_json::{json, Value};

use crate::module::admin::{
    crud::AdminCrud,
    interface::{AdminService, AdminValidation},
    schema::{AdminLoginSchema, ADMIN_EMAIL, ADMIN_PASSWORD},
};
use crate::service::jwt_service::{JwtService, Role};

pub struct AdminController {
    crud: AdminCrud,
    jwt_service: JwtService,
}

impl AdminController {
    pub fn new(crud: AdminCrud) -> Self {
        Self {
            crud,
            jwt_service: JwtService::new(),
        }
    }

    pub async fn handle_login(&self, data: Value) -> web::Json<Value> {
        // Parse and validate input
        let login_data = match serde_json::from_value::<AdminLoginSchema>(data) {
            Ok(data) => data,
            Err(e) => {
                return web::Json(json!({
                    "status": "error",
                    "message": format!("Invalid input data: {}", e)
                }));
            }
        };

        // Process login
        match self.login(&login_data).await {
            Ok(token) => web::Json(json!({
                "status": "success",
                "message": "Login successful",
                "data": {
                    "token": token
                }
            })),
            Err(e) => web::Json(json!({
                "status": "error",
                "message": e
            })),
        }
    }

    // For development purposes only - to create initial admin
    pub async fn initialize_admin(&self) -> Result<(), String> {
        // Check if admin already exists
        if let Some(_) = self.crud.find_by_email(ADMIN_EMAIL).await? {
            return Ok(());
        }

        // Create admin with default credentials
        let password_hash = AdminCrud::hash_password(ADMIN_PASSWORD).await?;
        let admin = crate::module::admin::model::Admin::new(
            ADMIN_EMAIL.to_string(),
            password_hash,
        );

        self.crud.create_admin(admin).await?;
        Ok(())
    }
}

impl AdminValidation for AdminController {}

#[async_trait::async_trait]
impl AdminService for AdminController {
    async fn login(&self, credentials: &AdminLoginSchema) -> Result<String, String> {
        // Validate login data
        Self::validate_login(credentials)?;

        // Find admin by email
        let admin = self.crud.find_by_email(&credentials.email).await?
            .ok_or_else(|| "Invalid email or password".to_string())?;

        // Verify password
        if !verify(&credentials.password, &admin.password_hash)
            .map_err(|_| "Password verification failed".to_string())? {
            return Err("Invalid email or password".to_string());
        }

        // Generate JWT token with admin role
        let admin_id = admin.id.map(|id| id.to_string()).unwrap_or_else(|| "unknown".to_string());
        self.jwt_service.generate_token(
            admin_id,
            admin.email,
            "Admin".to_string(),  // Using "Admin" as first_name
            "User".to_string(),   // Using "User" as last_name
            Role::Admin,
        ).map_err(|e| format!("Failed to generate token: {}", e))
    }
}

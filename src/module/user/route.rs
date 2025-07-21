use actix_web::{web, post};
use crate::module::user::controller::UserController;

pub fn config(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::scope("/users")
            .service(register)
            .service(verify_email)
            .service(login)
            .service(resend_verification)
            .service(delete_user) // Temporary route
    );
}

#[post("/register")]
pub async fn register(
    data: web::Json<serde_json::Value>,
    user_controller: web::Data<UserController>,
) -> web::Json<serde_json::Value> {
    user_controller.handle_registration(data.into_inner()).await
}

#[post("/verify-email")]
pub async fn verify_email(
    data: web::Json<serde_json::Value>,
    user_controller: web::Data<UserController>,
) -> web::Json<serde_json::Value> {
    user_controller.handle_email_verification(data.into_inner()).await
}

#[post("/login")]
pub async fn login(
    data: web::Json<serde_json::Value>,
    user_controller: web::Data<UserController>,
) -> web::Json<serde_json::Value> {
    user_controller.handle_login(data.into_inner()).await
}

#[post("/resend-verification")]
pub async fn resend_verification(
    data: web::Json<serde_json::Value>,
    user_controller: web::Data<UserController>,
) -> web::Json<serde_json::Value> {
    user_controller.handle_resend_verification(data.into_inner()).await
}

// Temporary route for testing
#[post("/delete-user")]
pub async fn delete_user(
    data: web::Json<serde_json::Value>,
    user_controller: web::Data<UserController>,
) -> web::Json<serde_json::Value> {
    user_controller.handle_delete_user(data).await
}

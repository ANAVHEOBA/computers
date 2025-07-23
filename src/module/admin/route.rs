use actix_web::{web, post, Scope};
use crate::module::admin::controller::AdminController;

// Public routes (no authentication required)
pub fn public_routes() -> Scope {
    web::scope("/admin-public")
        .service(login)
}

// Protected routes (requires admin authentication)
pub fn protected_routes(cfg: &mut web::ServiceConfig) {
    // Add protected admin routes here
    cfg.service(web::scope("")); // Empty for now, will add protected routes later
}

#[post("/login")]
pub async fn login(
    data: web::Json<serde_json::Value>,
    admin_controller: web::Data<AdminController>,
) -> web::Json<serde_json::Value> {
    admin_controller.handle_login(data.into_inner()).await
}

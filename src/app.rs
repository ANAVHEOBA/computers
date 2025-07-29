use actix_web::{web, HttpResponse};
use chrono::Utc;
use mongodb::Database;

use crate::module::{
    user::route as user_routes, 
    admin::route as admin_routes, 
    banner::route as banner_routes, 
    category::route as category_routes,
    brand::route as brand_routes,
    product::route as product_routes  
};
use crate::middleware::{AdminAuthentication, Authentication};

/// Configures all the application services and routes.
pub fn configure_services(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::scope("/api")
            .service(
                // User routes with user Authentication
                web::scope("/users")
                    .wrap(Authentication::new())
                    .configure(user_routes::config)
            )
            // Public admin routes (login)
            .service(admin_routes::public_routes())
            .service(
                // Protected admin routes
                web::scope("/admin")
                    .wrap(AdminAuthentication::new())
                    .configure(admin_routes::protected_routes)
            )
            .configure(banner_routes::config) // Public banner routes
            .configure(category_routes::category_routes) // Category routes
            .configure(brand_routes::brand_routes) // Brand routes
            .configure(product_routes::product_routes) // Product routes 
    )
    .route("/health", web::get().to(health_check))
    .default_service(web::route().to(not_found));
}

// Health check handler
async fn health_check(db: web::Data<Database>) -> HttpResponse {
    match db.list_collection_names().await {
        Ok(_) => HttpResponse::Ok().json(serde_json::json!({
            "status": "ok",
            "timestamp": Utc::now().to_rfc3339(),
            "database": {
                "connected": true,
                "status": "healthy"
            }
        })),
        Err(_) => HttpResponse::InternalServerError().json(serde_json::json!({
            "status": "error",
            "timestamp": Utc::now().to_rfc3339(),
            "database": {
                "connected": false,
                "status": "error"
            }
        })),
    }
}

// 404 handler
async fn not_found() -> HttpResponse {
    HttpResponse::NotFound().json(serde_json::json!({
        "status": "error",
        "message": "Route not found"
    }))
}
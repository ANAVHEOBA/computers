use actix_web::{web, HttpResponse, HttpRequest};
use crate::module::banner::controller::BannerController;
use crate::module::banner::schema::{CreateBannerSchema, UpdateBannerSchema};
use crate::module::banner::interface::BannerService;
use crate::middleware::AdminAuthentication;

pub fn config(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::scope("/banners")
            .route("", web::get().to(get_active_banners))
            .route("/{id}", web::get().to(get_banner))
            // Admin-only routes would be nested under an admin-scoped service
            .route("", web::post().to(create_banner))
            .route("/{id}", web::put().to(update_banner))
            .route("/{id}", web::delete().to(delete_banner))
    );
}

// Public route
async fn get_active_banners(controller: web::Data<BannerController>) -> HttpResponse {
    match controller.get_active_banners().await {
        Ok(banners) => HttpResponse::Ok().json(banners),
        Err(e) => HttpResponse::InternalServerError().body(e.to_string()),
    }
}

// Admin route (example)
async fn create_banner(
    req: HttpRequest,
    controller: web::Data<BannerController>,
    body: web::Json<CreateBannerSchema>,
) -> HttpResponse {
    // Check admin authorization first
    if let Err(e) = AdminAuthentication::check_admin(&req).await {
        return e.error_response();
    }

    match controller.create_banner(body.into_inner()).await {
        Ok(banner) => HttpResponse::Created().json(banner),
        Err(e) => HttpResponse::BadRequest().body(e.to_string()),
    }
}

async fn get_banner(
    controller: web::Data<BannerController>,
    path: web::Path<String>,
) -> HttpResponse {
    match controller.get_banner(&path.into_inner()).await {
        Ok(Some(banner)) => HttpResponse::Ok().json(banner),
        Ok(None) => HttpResponse::NotFound().finish(),
        Err(e) => HttpResponse::InternalServerError().body(e.to_string()),
    }
}

async fn update_banner(
    req: HttpRequest,
    controller: web::Data<BannerController>,
    path: web::Path<String>,
    body: web::Json<UpdateBannerSchema>,
) -> HttpResponse {
    // Check admin authorization first
    if let Err(e) = AdminAuthentication::check_admin(&req).await {
        return e.error_response();
    }

    match controller.update_banner(&path.into_inner(), body.into_inner()).await {
        Ok(banner) => HttpResponse::Ok().json(banner),
        Err(e) => HttpResponse::BadRequest().body(e.to_string()),
    }
}

async fn delete_banner(
    req: HttpRequest,
    controller: web::Data<BannerController>,
    path: web::Path<String>,
) -> HttpResponse {
    // Check admin authorization first
    if let Err(e) = AdminAuthentication::check_admin(&req).await {
        return e.error_response();
    }

    match controller.delete_banner(&path.into_inner()).await {
        Ok(_) => HttpResponse::NoContent().finish(),
        Err(e) => HttpResponse::BadRequest().body(e.to_string()),
    }
}

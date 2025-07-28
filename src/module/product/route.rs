// module/product/route.rs
use actix_web::{web, HttpResponse, Result};
use crate::module::product::{
    schema::CreateProductSchema,
    crud::ProductCrud,
    controller::ProductController,
};

// Route configuration function
pub fn product_routes(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::scope("/api/products")
            .route("", web::post().to(ProductController::create_product_handler))
    );
}
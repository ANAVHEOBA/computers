use actix_web::{web, HttpResponse, Result};
use crate::module::product::{
    schema::CreateProductSchema,
    crud::ProductCrud,
    controller::ProductController,
};

// Route configuration function
pub fn product_routes(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::scope("/products")
            .route("", web::post().to(ProductController::create_product_handler))
            .route("/featured", web::get().to(ProductController::get_featured_products_handler))
            .route("/new", web::get().to(ProductController::get_new_arrivals_handler))
            .route("/best-sellers", web::get().to(ProductController::get_best_sellers_handler))
            .route("/{id}", web::get().to(ProductController::get_product_handler))
            .route("", web::get().to(ProductController::get_products_handler))
    );
}
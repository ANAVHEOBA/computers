use actix_web::{web, HttpResponse, Result};
use crate::module::brand::{
    schema::CreateBrandSchema,
    crud::BrandCrud,
    controller::BrandController,
};

// HTTP handler function - create brand
async fn create_brand_handler(
    brand_: web::Json<CreateBrandSchema>,
    crud: web::Data<BrandCrud>,
) -> Result<HttpResponse> {
    let controller = BrandController::new(crud.get_ref().clone());
    
    match controller.create_brand(brand_.into_inner()).await {
        Ok(brand) => Ok(HttpResponse::Created().json(brand)),
        Err(error) => Ok(HttpResponse::BadRequest().json(serde_json::json!({
            "error": error
        }))),
    }
}

// Add these new handlers:

// GET all brands handler
async fn get_brands_handler(
    crud: web::Data<BrandCrud>,
) -> Result<HttpResponse> {
    let controller = BrandController::new(crud.get_ref().clone());
    
    match controller.get_all_brands().await {
        Ok(brands) => Ok(HttpResponse::Ok().json(brands)),
        Err(error) => Ok(HttpResponse::InternalServerError().json(serde_json::json!({
            "error": error
        }))),
    }
}

// GET single brand handler
async fn get_brand_handler(
    id: web::Path<String>,
    crud: web::Data<BrandCrud>,
) -> Result<HttpResponse> {
    let controller = BrandController::new(crud.get_ref().clone());
    
    match controller.get_brand(&id).await {
        Ok(Some(brand)) => Ok(HttpResponse::Ok().json(brand)),
        Ok(None) => Ok(HttpResponse::NotFound().json(serde_json::json!({
            "error": "Brand not found"
        }))),
        Err(error) => Ok(HttpResponse::InternalServerError().json(serde_json::json!({
            "error": error
        }))),
    }
}

// Route configuration function
pub fn brand_routes(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::scope("/brands")
            .route("", web::post().to(create_brand_handler))
            .route("", web::get().to(get_brands_handler))  // GET all brands
            .route("/{id}", web::get().to(get_brand_handler))  // GET single brand
    );
}
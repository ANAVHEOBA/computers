use actix_web::{web, HttpResponse, Result};
use crate::module::category::{
    schema::CreateCategorySchema,
    crud::CategoryCrud,
    controller::CategoryController,
};

// HTTP handler function - create category
async fn create_category_handler(
    category_: web::Json<CreateCategorySchema>,  // Fixed: added colon
    crud: web::Data<CategoryCrud>,
) -> Result<HttpResponse> {
    let controller = CategoryController::new(crud.get_ref().clone());
    
    match controller.create_category(category_.into_inner()).await {  // Fixed: use correct parameter name
        Ok(category) => Ok(HttpResponse::Created().json(category)),
        Err(error) => Ok(HttpResponse::BadRequest().json(serde_json::json!({
            "error": error
        }))),
    }
}

// HTTP handler function - get all categories
async fn get_categories_handler(
    crud: web::Data<CategoryCrud>,
) -> Result<HttpResponse> {
    let controller = CategoryController::new(crud.get_ref().clone());
    
    match controller.get_all_categories().await {
        Ok(categories) => Ok(HttpResponse::Ok().json(categories)),
        Err(error) => Ok(HttpResponse::InternalServerError().json(serde_json::json!({
            "error": error
        }))),
    }
}

// HTTP handler function - get single category
async fn get_category_handler(
    id: web::Path<String>,
    crud: web::Data<CategoryCrud>,
) -> Result<HttpResponse> {
    let controller = CategoryController::new(crud.get_ref().clone());
    
    match controller.get_category(&id).await {
        Ok(Some(category)) => Ok(HttpResponse::Ok().json(category)),
        Ok(None) => Ok(HttpResponse::NotFound().json(serde_json::json!({
            "error": "Category not found"
        }))),
        Err(error) => Ok(HttpResponse::InternalServerError().json(serde_json::json!({
            "error": error
        }))),
    }
}

// Route configuration function
pub fn category_routes(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::scope("/categories")
            .route("", web::post().to(create_category_handler))
            .route("", web::get().to(get_categories_handler))  // GET all categories
            .route("/{id}", web::get().to(get_category_handler))  // GET single category
    );
}
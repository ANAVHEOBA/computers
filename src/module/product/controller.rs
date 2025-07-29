use validator::Validate;
use actix_web::{web, HttpResponse, Result};
use crate::module::product::{
    model::Product,
    schema::CreateProductSchema,
    crud::ProductCrud,
};

pub struct ProductController {
    crud: ProductCrud,
}

impl ProductController {
    pub fn new(crud: ProductCrud) -> Self {
        Self { crud }
    }
    
    pub async fn create_product(&self, product_: CreateProductSchema) -> Result<Product, String> {
        // Validate the product data
        match product_.validate() {
            Ok(_) => {},
            Err(e) => return Err(format!("Validation error: {}", e))
        }
        
        // Create the product
        let product = self.crud.create_product(product_).await?;
        
        Ok(product)
    }
    
    // Add these new methods:
    
    pub async fn get_product(&self, id: &str) -> Result<Option<Product>, String> {
        self.crud.get_product(id).await
    }
    
    pub async fn get_all_products(&self) -> Result<Vec<Product>, String> {
        self.crud.get_all_products().await
    }
    
    pub async fn get_featured_products(&self, limit: Option<i64>) -> Result<Vec<Product>, String> {
        self.crud.get_featured_products(limit).await
    }
    
    pub async fn get_new_arrivals(&self, limit: Option<i64>) -> Result<Vec<Product>, String> {
        self.crud.get_new_arrivals(limit).await
    }
    
    pub async fn get_best_sellers(&self, limit: Option<i64>) -> Result<Vec<Product>, String> {
        self.crud.get_best_sellers(limit).await
    }
    
    // HTTP handler functions:
    
    // Create product handler
    pub async fn create_product_handler(
        product_: web::Json<CreateProductSchema>,
        crud: web::Data<ProductCrud>,
    ) -> Result<HttpResponse> {
        let controller = ProductController::new(crud.get_ref().clone());
        
        match controller.create_product(product_.into_inner()).await {
            Ok(product) => Ok(HttpResponse::Created().json(product)),
            Err(error) => Ok(HttpResponse::BadRequest().json(serde_json::json!({
                "error": error
            }))),
        }
    }
    
    // GET all products handler
    pub async fn get_products_handler(
        crud: web::Data<ProductCrud>,
    ) -> Result<HttpResponse> {
        let controller = ProductController::new(crud.get_ref().clone());
        
        match controller.get_all_products().await {
            Ok(products) => Ok(HttpResponse::Ok().json(products)),
            Err(error) => Ok(HttpResponse::InternalServerError().json(serde_json::json!({
                "error": error
            }))),
        }
    }
    
    // GET single product handler
    pub async fn get_product_handler(
        id: web::Path<String>,
        crud: web::Data<ProductCrud>,
    ) -> Result<HttpResponse> {
        let controller = ProductController::new(crud.get_ref().clone());
        
        match controller.get_product(&id).await {
            Ok(Some(product)) => Ok(HttpResponse::Ok().json(product)),
            Ok(None) => Ok(HttpResponse::NotFound().json(serde_json::json!({
                "error": "Product not found"
            }))),
            Err(error) => Ok(HttpResponse::InternalServerError().json(serde_json::json!({
                "error": error
            }))),
        }
    }
    
    // GET featured products handler
    pub async fn get_featured_products_handler(
        crud: web::Data<ProductCrud>,
    ) -> Result<HttpResponse> {
        let controller = ProductController::new(crud.get_ref().clone());
        
        match controller.get_featured_products(Some(10)).await {
            Ok(products) => Ok(HttpResponse::Ok().json(products)),
            Err(error) => Ok(HttpResponse::InternalServerError().json(serde_json::json!({
                "error": error
            }))),
        }
    }
    
    // GET new arrivals handler
    pub async fn get_new_arrivals_handler(
        crud: web::Data<ProductCrud>,
    ) -> Result<HttpResponse> {
        let controller = ProductController::new(crud.get_ref().clone());
        
        match controller.get_new_arrivals(Some(10)).await {
            Ok(products) => Ok(HttpResponse::Ok().json(products)),
            Err(error) => Ok(HttpResponse::InternalServerError().json(serde_json::json!({
                "error": error
            }))),
        }
    }
    
    // GET best sellers handler
    pub async fn get_best_sellers_handler(
        crud: web::Data<ProductCrud>,
    ) -> Result<HttpResponse> {
        let controller = ProductController::new(crud.get_ref().clone());
        
        match controller.get_best_sellers(Some(10)).await {
            Ok(products) => Ok(HttpResponse::Ok().json(products)),
            Err(error) => Ok(HttpResponse::InternalServerError().json(serde_json::json!({
                "error": error
            }))),
        }
    }
    
    // Helper method to format price for display
    pub fn format_price(&self, price_in_kobo: i64) -> String {
        let naira = price_in_kobo / 100;
        let kobo = price_in_kobo % 100;
        format!("₦{}.{}", naira, kobo)
    }
    
    // Helper method to check if product is new (created within last 30 days)
    pub fn is_new_product(&self, product: &Product) -> bool {
        use chrono::Utc;
        let now = Utc::now();
        let duration = now.signed_duration_since(product.created_at);
        duration.num_days() <= 30
    }
}
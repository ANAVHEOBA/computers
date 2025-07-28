// module/product/controller.rs
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
    
    pub async fn create_product(&self, product_data: CreateProductSchema) -> Result<Product, String> {
        // Validate the product data
        match product_data.validate() {
            Ok(_) => {},
            Err(e) => return Err(format!("Validation error: {}", e))
        }
        
        // Create the product
        let product = self.crud.create_product(product_data).await?;
        
        Ok(product)
    }
    
    // HTTP handler function - moved here
    pub async fn create_product_handler(
        product_data: web::Json<CreateProductSchema>,
        crud: web::Data<ProductCrud>,
    ) -> Result<HttpResponse> {
        let controller = ProductController::new(crud.get_ref().clone());
        
        match controller.create_product(product_data.into_inner()).await {
            Ok(product) => Ok(HttpResponse::Created().json(product)),
            Err(error) => Ok(HttpResponse::BadRequest().json(serde_json::json!({
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
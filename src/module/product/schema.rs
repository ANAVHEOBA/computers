use serde::{Deserialize, Serialize};
use validator::Validate;

#[derive(Debug, Serialize, Deserialize, Validate, Clone)]
pub struct CreateProductSchema {
    #[validate(length(min = 1, max = 200))]
    pub name: String,
    
    #[validate(length(min = 1, max = 2000))]
    pub description: String,
    
    #[validate(range(min = 1))]
    pub price: i64,
    
    #[validate(range(min = 0))]
    pub sale_price: Option<i64>,
    
    #[validate(length(min = 1, max = 100))]
    pub sku: String,
    
    #[validate(range(min = 0))]
    pub stock_quantity: i32,
    
    pub category_id: String,  // Will be converted to ObjectId
    pub brand_id: String,     // Will be converted to ObjectId
    
    pub is_featured: bool,
    pub is_best_seller: bool,
}

#[derive(Debug, Serialize, Deserialize, Validate, Clone)]
pub struct UpdateProductSchema {
    #[validate(length(min = 1, max = 200))]
    pub name: Option<String>,
    
    #[validate(length(min = 1, max = 2000))]
    pub description: Option<String>,
    
    #[validate(range(min = 1))]
    pub price: Option<i64>,
    
    #[validate(range(min = 0))]
    pub sale_price: Option<i64>,
    
    #[validate(range(min = 0))]
    pub stock_quantity: Option<i32>,
    
    pub category_id: Option<String>,
    pub brand_id: Option<String>,
    
    pub is_active: Option<bool>,
    pub is_featured: Option<bool>,
    pub is_best_seller: Option<bool>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct ProductFilter {
    pub category_id: Option<String>,
    pub brand_id: Option<String>,
    pub is_featured: Option<bool>,
    pub is_new_arrival: Option<bool>,
    pub is_best_seller: Option<bool>,
    pub search: Option<String>,
}
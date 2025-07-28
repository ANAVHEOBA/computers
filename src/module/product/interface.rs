// module/product/interface.rs

// use bson::oid::ObjectId;
use crate::module::product::{model::Product, schema::{CreateProductSchema, UpdateProductSchema, ProductFilter}};

#[async_trait::async_trait]
pub trait ProductInterface {
    async fn create_product(&self, product_data: CreateProductSchema) -> Result<Product, String>;
    async fn get_product(&self, id: &str) -> Result<Option<Product>, String>;
    async fn get_products(&self, filter: ProductFilter, limit: u32, offset: u32) -> Result<Vec<Product>, String>;
    async fn update_product(&self, id: &str, product_data: UpdateProductSchema) -> Result<Option<Product>, String>;
    async fn delete_product(&self, id: &str) -> Result<bool, String>;
    
    // Section-specific queries
    async fn get_featured_products(&self, limit: u32) -> Result<Vec<Product>, String>;
    async fn get_new_arrivals(&self, limit: u32) -> Result<Vec<Product>, String>;
    async fn get_best_sellers(&self, limit: u32) -> Result<Vec<Product>, String>;
    
    // Inventory management
    async fn update_stock(&self, id: &str, quantity: i32) -> Result<Option<Product>, String>;
}
use bson::oid::ObjectId;
use crate::module::brand::{
    model::Brand, 
    schema::{CreateBrandSchema, UpdateBrandSchema}
};

#[async_trait::async_trait]
pub trait BrandInterface {
    async fn create_brand(&self, brand_: CreateBrandSchema) -> Result<Brand, String>;
    async fn get_brand(&self, id: &str) -> Result<Option<Brand>, String>;
    async fn get_brand_by_slug(&self, slug: &str) -> Result<Option<Brand>, String>;
    async fn get_all_brands(&self) -> Result<Vec<Brand>, String>;
    async fn get_active_brands(&self) -> Result<Vec<Brand>, String>;
    async fn update_brand(&self, id: &str, brand_: UpdateBrandSchema) -> Result<Option<Brand>, String>;
    async fn delete_brand(&self, id: &str) -> Result<bool, String>;
    
    // Logo management
    async fn upload_brand_logo(&self, brand_id: &str, image_: Vec<u8>) -> Result<String, String>;
    async fn remove_brand_logo(&self, brand_id: &str) -> Result<bool, String>;
    
    // Search and filter
    async fn search_brands(&self, query: &str) -> Result<Vec<Brand>, String>;
    async fn get_brands_by_letter(&self, letter: char) -> Result<Vec<Brand>, String>;
}
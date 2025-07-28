use bson::oid::ObjectId;
use crate::module::category::{
    model::Category, 
    schema::{CreateCategorySchema, UpdateCategorySchema}
};

#[async_trait::async_trait]
pub trait CategoryInterface {
    async fn create_category(&self, category_data: CreateCategorySchema) -> Result<Category, String>;
    async fn get_category(&self, id: &str) -> Result<Option<Category>, String>;
    async fn get_category_by_slug(&self, slug: &str) -> Result<Option<Category>, String>;
    async fn get_all_categories(&self) -> Result<Vec<Category>, String>;
    async fn get_active_categories(&self) -> Result<Vec<Category>, String>;
    async fn update_category(&self, id: &str, category_data: UpdateCategorySchema) -> Result<Option<Category>, String>;
    async fn delete_category(&self, id: &str) -> Result<bool, String>;
    
    // Helper methods
    async fn get_child_categories(&self, parent_id: &str) -> Result<Vec<Category>, String>;
    async fn get_root_categories(&self) -> Result<Vec<Category>, String>;
}
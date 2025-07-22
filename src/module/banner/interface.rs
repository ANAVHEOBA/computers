use async_trait::async_trait;
use validator::Validate;

use crate::module::banner::{
    model::Banner,
    schema::{CreateBannerSchema, UpdateBannerSchema},
};

pub trait BannerValidation {
    fn validate_create(data: &CreateBannerSchema) -> Result<(), String> {
        if let Err(errors) = data.validate() {
            return Err(errors.to_string());
        }
        Ok(())
    }

    fn validate_update(data: &UpdateBannerSchema) -> Result<(), String> {
        if let Err(errors) = data.validate() {
            return Err(errors.to_string());
        }
        Ok(())
    }
}

#[async_trait]
pub trait BannerService: BannerValidation {
    // Create a new banner
    async fn create_banner(&self, data: CreateBannerSchema) -> Result<Banner, String>;
    
    // Get a single banner by ID
    async fn get_banner(&self, id: &str) -> Result<Option<Banner>, String>;
    
    // Get all active banners
    async fn get_active_banners(&self) -> Result<Vec<Banner>, String>;
    
    // Get all banners (including inactive ones) - for admin use
    async fn get_all_banners(&self) -> Result<Vec<Banner>, String>;
    
    // Update a banner
    async fn update_banner(&self, id: &str, data: UpdateBannerSchema) -> Result<Banner, String>;
    
    // Delete a banner
    async fn delete_banner(&self, id: &str) -> Result<(), String>;
    
    // Update banner order
    async fn update_display_order(&self, id: &str, new_order: i32) -> Result<(), String>;
    
    // Toggle banner active status
    async fn toggle_active_status(&self, id: &str) -> Result<Banner, String>;
}

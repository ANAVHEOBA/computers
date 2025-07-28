use serde::{Deserialize, Serialize};
use validator::Validate;

#[derive(Debug, Serialize, Deserialize, Validate, Clone)]
pub struct CreateBrandSchema {
    #[validate(length(min = 1, max = 100))]
    pub name: String,
    
    #[validate(length(min = 1, max = 500))]
    pub description: String,
    
    #[validate(url)]
    pub website: Option<String>,
    
    // Logo will be uploaded separately, so we don't include it here
    // The logo_url will be set after successful upload
}

#[derive(Debug, Serialize, Deserialize, Validate, Clone)]
pub struct UpdateBrandSchema {
    #[validate(length(min = 1, max = 100))]
    pub name: Option<String>,
    
    #[validate(length(min = 1, max = 500))]
    pub description: Option<String>,
    
    #[validate(url)]
    pub website: Option<String>,
    
    pub logo_url: Option<String>,
    pub is_active: Option<bool>,
    pub display_order: Option<i32>,
    
    #[validate(length(max = 100))]
    pub meta_title: Option<String>,
    
    #[validate(length(max = 200))]
    pub meta_description: Option<String>,
}

#[derive(Debug, Serialize, Deserialize, Validate, Clone)]
pub struct UploadBrandLogoSchema {
    // Base64 encoded image data
    #[validate(length(min = 1))]
    pub image_data: String,
    
    // Brand ID to associate the logo with
    #[validate(length(min = 1))]
    pub brand_id: String,
}
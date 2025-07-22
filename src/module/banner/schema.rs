use serde::{Deserialize, Serialize};
use validator::Validate;
use chrono::{DateTime, Utc};

#[derive(Debug, Serialize, Deserialize, Validate)]
pub struct CreateBannerSchema {
    #[validate(length(min = 1, max = 100, message = "Title must be between 1 and 100 characters"))]
    pub title: String,
    
    #[validate(length(max = 500, message = "Description cannot exceed 500 characters"))]
    pub description: Option<String>,
    
    #[validate(length(min = 1, message = "Image data is required"))]
    pub image_data: String,  // Base64 encoded image
    
    #[validate(url(message = "Invalid URL format"))]
    pub link_url: Option<String>,
    
    pub display_order: Option<i32>,
    pub start_date: Option<DateTime<Utc>>,
    pub end_date: Option<DateTime<Utc>>,
}

#[derive(Debug, Serialize, Deserialize, Validate, Default)]
pub struct UpdateBannerSchema {
    #[validate(length(min = 1, max = 100, message = "Title must be between 1 and 100 characters"))]
    pub title: Option<String>,
    
    #[validate(length(max = 500, message = "Description cannot exceed 500 characters"))]
    pub description: Option<String>,
    
    pub image_data: Option<String>,  // Base64 encoded image
    
    #[validate(url(message = "Invalid URL format"))]
    pub link_url: Option<String>,
    
    pub is_active: Option<bool>,
    pub display_order: Option<i32>,
    pub start_date: Option<DateTime<Utc>>,
    pub end_date: Option<DateTime<Utc>>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct BannerResponse {
    pub id: String,
    pub title: String,
    pub description: Option<String>,
    pub image_url: String,
    pub link_url: Option<String>,
    pub is_active: bool,
    pub display_order: i32,
    pub start_date: Option<DateTime<Utc>>,
    pub end_date: Option<DateTime<Utc>>,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

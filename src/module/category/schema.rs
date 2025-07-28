use serde::{Deserialize, Serialize};
use validator::Validate;

#[derive(Debug, Serialize, Deserialize, Validate, Clone)]
pub struct CreateCategorySchema {
    #[validate(length(min = 1, max = 100))]
    pub name: String,
    
    #[validate(length(min = 1, max = 500))]
    pub description: String,
    
    pub parent_id: Option<String>,
}

#[derive(Debug, Serialize, Deserialize, Validate, Clone)]
pub struct UpdateCategorySchema {
    #[validate(length(min = 1, max = 100))]
    pub name: Option<String>,
    
    #[validate(length(min = 1, max = 500))]
    pub description: Option<String>,
    
    pub parent_id: Option<String>,
    pub is_active: Option<bool>,
    pub display_order: Option<i32>,
}
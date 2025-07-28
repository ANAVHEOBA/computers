use bson::oid::ObjectId;
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Category {
    #[serde(rename = "_id", skip_serializing_if = "Option::is_none")]
    pub id: Option<ObjectId>,
    
    pub name: String,
    pub description: String,
    pub slug: String,
    
    #[serde(skip_serializing_if = "Option::is_none")]
    pub parent_id: Option<ObjectId>,
    
    pub is_active: bool,
    pub display_order: i32,
    
    // Timestamps
    #[serde(with = "bson::serde_helpers::chrono_datetime_as_bson_datetime")]
    pub created_at: DateTime<Utc>,
    #[serde(with = "bson::serde_helpers::chrono_datetime_as_bson_datetime")]
    pub updated_at: DateTime<Utc>,
}

impl Category {
    pub fn new(
        name: String,
        description: String,
        parent_id: Option<ObjectId>,
    ) -> Self {
        let now = Utc::now();
        let slug = name.to_lowercase().replace(" ", "-");
        
        Self {
            id: None,
            name,
            description,
            slug,
            parent_id,
            is_active: true,
            display_order: 0,
            created_at: now,
            updated_at: now,
        }
    }
}
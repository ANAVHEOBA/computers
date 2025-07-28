use bson::oid::ObjectId;
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Brand {
    #[serde(rename = "_id", skip_serializing_if = "Option::is_none")]
    pub id: Option<ObjectId>,
    
    pub name: String,
    pub description: String,
    pub slug: String,
    
    // Brand logo/image URL
    #[serde(skip_serializing_if = "Option::is_none")]
    pub logo_url: Option<String>,
    
    // Website and social links
    #[serde(skip_serializing_if = "Option::is_none")]
    pub website: Option<String>,
    
    // Status
    pub is_active: bool,
    pub display_order: i32,
    
    // SEO fields
    #[serde(skip_serializing_if = "Option::is_none")]
    pub meta_title: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub meta_description: Option<String>,
    
    // Timestamps
    #[serde(with = "bson::serde_helpers::chrono_datetime_as_bson_datetime")]
    pub created_at: DateTime<Utc>,
    #[serde(with = "bson::serde_helpers::chrono_datetime_as_bson_datetime")]
    pub updated_at: DateTime<Utc>,
}

impl Brand {
    pub fn new(
        name: String,
        description: String,
    ) -> Self {
        let now = Utc::now();
        let slug = name.to_lowercase().replace(" ", "-");
        
        Self {
            id: None,
            name,
            description,
            slug,
            logo_url: None,
            website: None,
            is_active: true,
            display_order: 0,
            meta_title: None,
            meta_description: None,
            created_at: now,
            updated_at: now,
        }
    }
}
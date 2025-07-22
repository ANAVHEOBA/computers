use serde::{Deserialize, Serialize};
use chrono::{DateTime, Utc};
use bson::oid::ObjectId;

#[derive(Debug, Serialize, Deserialize)]
pub struct Banner {
    #[serde(rename = "_id", skip_serializing_if = "Option::is_none")]
    pub id: Option<ObjectId>,
    pub title: String,
    pub description: Option<String>,
    pub image_url: String,
    pub link_url: Option<String>,
    pub is_active: bool,
    pub display_order: i32,  // For controlling the order of banners
    pub start_date: Option<DateTime<Utc>>,  // Optional scheduling
    pub end_date: Option<DateTime<Utc>>,    // Optional scheduling
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

impl Banner {
    pub fn new(
        title: String,
        description: Option<String>,
        image_url: String,
        link_url: Option<String>,
        display_order: i32,
        start_date: Option<DateTime<Utc>>,
        end_date: Option<DateTime<Utc>>,
    ) -> Self {
        let now = Utc::now();
        Self {
            id: None,
            title,
            description,
            image_url,
            link_url,
            is_active: true,
            display_order,
            start_date,
            end_date,
            created_at: now,
            updated_at: now,
        }
    }
}

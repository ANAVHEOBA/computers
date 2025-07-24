use serde::{Deserialize, Serialize};
use chrono::{DateTime, Utc};
use bson::{oid::ObjectId, DateTime as BsonDateTime};

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
    pub start_date: Option<BsonDateTime>,  // Optional scheduling
    pub end_date: Option<BsonDateTime>,    // Optional scheduling
    pub created_at: BsonDateTime,
    pub updated_at: BsonDateTime,
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
        let now = BsonDateTime::now();
        Self {
            id: None,
            title,
            description,
            image_url,
            link_url,
            is_active: true,
            display_order,
            start_date: start_date.map(BsonDateTime::from_chrono),
            end_date: end_date.map(BsonDateTime::from_chrono),
            created_at: now,
            updated_at: now,
        }
    }
}

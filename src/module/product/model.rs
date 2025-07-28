use bson::oid::ObjectId;
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Product {
    #[serde(rename = "_id", skip_serializing_if = "Option::is_none")]
    pub id: Option<ObjectId>,
    
    pub name: String,
    pub description: String,
    pub slug: String,
    
    // Pricing in kobo (₦1 = 100 kobo)
    pub price: i64,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sale_price: Option<i64>,
    
    // Inventory
    pub sku: String,
    pub stock_quantity: i32,
    
    // Relationships
    #[serde(rename = "category_id")]
    pub category_id: ObjectId,
    #[serde(rename = "brand_id")]
    pub brand_id: ObjectId,
    
    // Media
    pub images: Vec<String>,
    
    // Status flags for sections
    pub is_active: bool,
    pub is_featured: bool,
    pub is_new_arrival: bool,
    pub is_best_seller: bool,
    
    // Timestamps
    #[serde(with = "bson::serde_helpers::chrono_datetime_as_bson_datetime")]
    pub created_at: DateTime<Utc>,
    #[serde(with = "bson::serde_helpers::chrono_datetime_as_bson_datetime")]
    pub updated_at: DateTime<Utc>,
}

impl Product {
    pub fn new(
        name: String,
        description: String,
        price: i64,
        sku: String,
        category_id: ObjectId,
        brand_id: ObjectId,
    ) -> Self {
        let now = Utc::now();
        let slug = name.to_lowercase().replace(" ", "-");
        
        Self {
            id: None,
            name,
            description,
            slug,
            price,
            sale_price: None,
            sku,
            stock_quantity: 0,
            category_id,
            brand_id,
            images: Vec::new(),
            is_active: true,
            is_featured: false,
            is_new_arrival: true,
            is_best_seller: false,
            created_at: now,
            updated_at: now,
        }
    }
    
    // Helper method to check if product is on sale
    pub fn is_on_sale(&self) -> bool {
        self.sale_price.is_some() && self.sale_price.unwrap() < self.price
    }
    
    // Helper method to get display price
    pub fn display_price(&self) -> i64 {
        self.sale_price.unwrap_or(self.price)
    }
}
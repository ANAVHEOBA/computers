use bson::{doc, oid::ObjectId};
use mongodb::{Collection, Database};
use futures_util::TryStreamExt; 
use crate::module::product::{
    model::Product, 
    schema::CreateProductSchema
};

#[derive(Clone)]
pub struct ProductCrud {
    collection: Collection<Product>,
}

impl ProductCrud {
    pub fn new(db: &Database) -> Self {
        let collection = db.collection("products");
        Self { collection }
    }
    
    pub async fn create_product(&self, product_: CreateProductSchema) -> Result<Product, String> {  // Fixed: added colon
        // Convert string IDs to ObjectIds
        let category_id = ObjectId::parse_str(&product_.category_id)  // Fixed: use correct parameter name
            .map_err(|_| "Invalid category ID".to_string())?;
        let brand_id = ObjectId::parse_str(&product_.brand_id)  // Fixed: use correct parameter name
            .map_err(|_| "Invalid brand ID".to_string())?;
        
        // Create product model
        let mut product = Product::new(
            product_.name,  // Fixed: use correct parameter name
            product_.description,  // Fixed: use correct parameter name
            product_.price,  // Fixed: use correct parameter name
            product_.sku,  // Fixed: use correct parameter name
            category_id,
            brand_id,
        );
        
        // Set optional fields
        product.sale_price = product_.sale_price;  // Fixed: use correct parameter name
        product.stock_quantity = product_.stock_quantity;  // Fixed: use correct parameter name
        product.is_featured = product_.is_featured;  // Fixed: use correct parameter name
        product.is_best_seller = product_.is_best_seller;  // Fixed: use correct parameter name
        
        // Insert into database
        match self.collection.insert_one(&product).await {
            Ok(result) => {
                if let Some(id) = result.inserted_id.as_object_id() {
                    product.id = Some(id);
                }
                Ok(product)
            }
            Err(e) => Err(format!("Failed to create product: {}", e))
        }
    }

    // Add these new methods:
    
    pub async fn get_product(&self, id: &str) -> Result<Option<Product>, String> {
        let object_id = ObjectId::parse_str(id)
            .map_err(|_| "Invalid product ID".to_string())?;
        
        match self.collection.find_one(doc! { "_id": object_id }).await {
            Ok(product) => Ok(product),
            Err(e) => Err(format!("Failed to retrieve product: {}", e))
        }
    }
    
    pub async fn get_all_products(&self) -> Result<Vec<Product>, String> {
        match self.collection.find(doc! { "is_active": true }).await {
            Ok(mut cursor) => {
                let mut products = Vec::new();
                while let Some(product) = cursor.try_next().await
                    .map_err(|e| format!("Failed to retrieve products: {}", e))? {
                    products.push(product);
                }
                Ok(products)
            }
            Err(e) => Err(format!("Failed to retrieve products: {}", e))
        }
    }
    
    // Get featured products
    pub async fn get_featured_products(&self, limit: Option<i64>) -> Result<Vec<Product>, String> {
        let _limit = limit.unwrap_or(10);
        // Note: MongoDB driver version differences, removing options for now
        match self.collection.find(doc! { "is_active": true, "is_featured": true }).await {
            Ok(mut cursor) => {
                let mut products = Vec::new();
                while let Some(product) = cursor.try_next().await
                    .map_err(|e| format!("Failed to retrieve featured products: {}", e))? {
                    products.push(product);
                }
                // In a real implementation, you'd apply the limit here
                Ok(products.into_iter().take(_limit as usize).collect())
            }
            Err(e) => Err(format!("Failed to retrieve featured products: {}", e))
        }
    }
    
    // Get new arrivals (created within last 30 days)
    pub async fn get_new_arrivals(&self, limit: Option<i64>) -> Result<Vec<Product>, String> {
        use chrono::{Utc, Duration};
        let _limit = limit.unwrap_or(10);
        let thirty_days_ago = Utc::now() - Duration::days(30);
        
        // Note: MongoDB driver version differences, removing options for now
        match self.collection.find(
            doc! { 
                "is_active": true, 
                "created_at": { "$gte": thirty_days_ago }
            }
        ).await {
            Ok(mut cursor) => {
                let mut products = Vec::new();
                while let Some(product) = cursor.try_next().await
                    .map_err(|e| format!("Failed to retrieve new arrivals: {}", e))? {
                    products.push(product);
                }
                // In a real implementation, you'd apply the limit here
                Ok(products.into_iter().take(_limit as usize).collect())
            }
            Err(e) => Err(format!("Failed to retrieve new arrivals: {}", e))
        }
    }
    
    // Get best sellers
    pub async fn get_best_sellers(&self, limit: Option<i64>) -> Result<Vec<Product>, String> {
        let _limit = limit.unwrap_or(10);
        // Note: MongoDB driver version differences, removing options for now
        match self.collection.find(doc! { "is_active": true, "is_best_seller": true }).await {
            Ok(mut cursor) => {
                let mut products = Vec::new();
                while let Some(product) = cursor.try_next().await
                    .map_err(|e| format!("Failed to retrieve best sellers: {}", e))? {
                    products.push(product);
                }
                // In a real implementation, you'd apply the limit here
                Ok(products.into_iter().take(_limit as usize).collect())
            }
            Err(e) => Err(format!("Failed to retrieve best sellers: {}", e))
        }
    }
}
use bson::oid::ObjectId;
use mongodb::{Collection, Database};
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
    
    pub async fn create_product(&self, product_data: CreateProductSchema) -> Result<Product, String> {
        // Convert string IDs to ObjectIds
        let category_id = ObjectId::parse_str(&product_data.category_id)
            .map_err(|_| "Invalid category ID".to_string())?;
        let brand_id = ObjectId::parse_str(&product_data.brand_id)
            .map_err(|_| "Invalid brand ID".to_string())?;
        
        // Create product model
        let mut product = Product::new(
            product_data.name,
            product_data.description,
            product_data.price,
            product_data.sku,
            category_id,
            brand_id,
        );
        
        // Set optional fields
        product.sale_price = product_data.sale_price;
        product.stock_quantity = product_data.stock_quantity;
        product.is_featured = product_data.is_featured;
        product.is_best_seller = product_data.is_best_seller;
        
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
}
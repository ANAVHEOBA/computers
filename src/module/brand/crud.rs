use bson::{doc, oid::ObjectId};
use mongodb::{Collection, Database};
use futures_util::TryStreamExt; // Add this import
use crate::module::brand::{
    model::Brand, 
    schema::CreateBrandSchema
};

#[derive(Clone)]
pub struct BrandCrud {
    collection: Collection<Brand>,
}

impl BrandCrud {
    pub fn new(db: &Database) -> Self {
        let collection = db.collection("brands");
        Self { collection }
    }
    
    pub async fn create_brand(&self, brand_: CreateBrandSchema) -> Result<Brand, String> {
        // Create brand model
        let mut brand = Brand::new(
            brand_.name,
            brand_.description,
        );
        
        // Set optional fields
        brand.website = brand_.website;
        
        // Insert into database
        match self.collection.insert_one(&brand).await {
            Ok(result) => {
                if let Some(id) = result.inserted_id.as_object_id() {
                    brand.id = Some(id);
                }
                Ok(brand)
            }
            Err(e) => Err(format!("Failed to create brand: {}", e))
        }
    }

    // Add these new methods:
    
    pub async fn get_brand(&self, id: &str) -> Result<Option<Brand>, String> {
        let object_id = ObjectId::parse_str(id)
            .map_err(|_| "Invalid brand ID".to_string())?;
        
        match self.collection.find_one(doc! { "_id": object_id }).await {
            Ok(brand) => Ok(brand),
            Err(e) => Err(format!("Failed to retrieve brand: {}", e))
        }
    }
    
    pub async fn get_all_brands(&self) -> Result<Vec<Brand>, String> {
        match self.collection.find(doc! { "is_active": true }).await {
            Ok(mut cursor) => {
                let mut brands = Vec::new();
                while let Some(brand) = cursor.try_next().await
                    .map_err(|e| format!("Failed to retrieve brands: {}", e))? {
                    brands.push(brand);
                }
                Ok(brands)
            }
            Err(e) => Err(format!("Failed to retrieve brands: {}", e))
        }
    }
}
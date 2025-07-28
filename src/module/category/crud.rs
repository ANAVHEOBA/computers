use bson::{doc, oid::ObjectId};
use mongodb::{Collection, Database};
use futures_util::TryStreamExt;
use crate::module::category::{
    model::Category, 
    schema::CreateCategorySchema
};

#[derive(Clone)]
pub struct CategoryCrud {
    collection: Collection<Category>,
}

impl CategoryCrud {
    pub fn new(db: &Database) -> Self {
        let collection = db.collection("categories");
        Self { collection }
    }
    
    pub async fn create_category(&self, category_data: CreateCategorySchema) -> Result<Category, String> {  // Fixed this line
        // Convert parent_id if provided
        let parent_id = match category_data.parent_id {
            Some(ref id_str) => {
                Some(ObjectId::parse_str(id_str)
                    .map_err(|_| "Invalid parent category ID".to_string())?)
            }
            None => None
        };
        
        // Create category model
        let mut category = Category::new(
            category_data.name,
            category_data.description,
            parent_id,
        );
        
        // Insert into database
        match self.collection.insert_one(&category).await {
            Ok(result) => {
                if let Some(id) = result.inserted_id.as_object_id() {
                    category.id = Some(id);
                }
                Ok(category)
            }
            Err(e) => Err(format!("Failed to create category: {}", e))
        }
    }

     


    pub async fn get_category(&self, id: &str) -> Result<Option<Category>, String> {
        let object_id = ObjectId::parse_str(id)
            .map_err(|_| "Invalid category ID".to_string())?;
        
            match self.collection.find_one(doc! { "_id": object_id }).await {
            Ok(category) => Ok(category),
            Err(e) => Err(format!("Failed to retrieve category: {}", e))
        }
    }
    
    pub async fn get_all_categories(&self) -> Result<Vec<Category>, String> {
        use mongodb::options::FindOptions;
        
        let options = FindOptions::builder()
            .sort(doc! { "display_order": 1, "name": 1 })
            .build();
        
            match self.collection.find(doc! { "is_active": true }).await {
            Ok(mut cursor) => {
                let mut categories = Vec::new();
                while let Some(category) = cursor.try_next().await
                    .map_err(|e| format!("Failed to retrieve categories: {}", e))? {
                    categories.push(category);
                }
                Ok(categories)
            }
            Err(e) => Err(format!("Failed to retrieve categories: {}", e))
        }
    }


    
}
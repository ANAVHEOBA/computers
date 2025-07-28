use validator::Validate;
use crate::module::category::{
    model::Category,
    schema::CreateCategorySchema,
    crud::CategoryCrud,
};

pub struct CategoryController {
    crud: CategoryCrud,
}

impl CategoryController {
    pub fn new(crud: CategoryCrud) -> Self {
        Self { crud }
    }
    
    pub async fn create_category(&self, category_: CreateCategorySchema) -> Result<Category, String> {  // Fixed: added colon
        // Validate the category data
        match category_.validate() {  // Fixed: use correct parameter name
            Ok(_) => {},
            Err(e) => return Err(format!("Validation error: {}", e))
        }
        
        // Create the category
        let category = self.crud.create_category(category_).await?;  // Fixed: use correct parameter name
        
        Ok(category)
    }




    pub async fn get_category(&self, id: &str) -> Result<Option<Category>, String> {
        self.crud.get_category(id).await
    }



    
    
    pub async fn get_all_categories(&self) -> Result<Vec<Category>, String> {
        self.crud.get_all_categories().await
    }
}
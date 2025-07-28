use validator::Validate;
use actix_web::{web, HttpResponse, Result};
use crate::module::brand::{
    model::Brand,
    schema::CreateBrandSchema,
    crud::BrandCrud,
};

pub struct BrandController {
    crud: BrandCrud,
}

impl BrandController {
    pub fn new(crud: BrandCrud) -> Self {
        Self { crud }
    }
    
    pub async fn create_brand(&self, brand_: CreateBrandSchema) -> Result<Brand, String> {
        // Validate the brand data
        match brand_.validate() {
            Ok(_) => {},
            Err(e) => return Err(format!("Validation error: {}", e))
        }
        
        // Create the brand
        let brand = self.crud.create_brand(brand_).await?;
        
        Ok(brand)
    }

    // Add these new methods:
    
    pub async fn get_brand(&self, id: &str) -> Result<Option<Brand>, String> {
        self.crud.get_brand(id).await
    }
    
    pub async fn get_all_brands(&self) -> Result<Vec<Brand>, String> {
        self.crud.get_all_brands().await
    }
}
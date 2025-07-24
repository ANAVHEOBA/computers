use mongodb::bson::oid::ObjectId;
use crate::module::banner::{
    crud::BannerCrud,
    interface::{BannerService, BannerValidation},
    model::Banner,
    schema::{CreateBannerSchema, UpdateBannerSchema, BannerResponse},
};
use crate::service::upload::banner::BannerUploadService;

pub struct BannerController {
    crud: BannerCrud,
    upload_service: BannerUploadService,
}

impl BannerController {
    pub fn new(crud: BannerCrud) -> Self {
        Self {
            crud,
            upload_service: BannerUploadService::new(),
        }
    }

    fn to_banner_response(&self, banner: Banner) -> BannerResponse {
        BannerResponse {
            id: banner.id.unwrap().to_hex(),
            title: banner.title,
            description: banner.description,
            image_url: banner.image_url,
            link_url: banner.link_url,
            is_active: banner.is_active,
            display_order: banner.display_order,
            start_date: banner.start_date.map(|dt| dt.to_rfc3339_string()),
            end_date: banner.end_date.map(|dt| dt.to_rfc3339_string()),
            created_at: banner.created_at.to_rfc3339_string(),
            updated_at: banner.updated_at.to_rfc3339_string(),
        }
    }
}

impl BannerValidation for BannerController {}

#[async_trait::async_trait]
impl BannerService for BannerController {
    async fn create_banner(&self, data: CreateBannerSchema) -> Result<Banner, String> {
        Self::validate_create(&data)?;
        
        let image_url = self.upload_service.upload_banner(&data.image_data).await
            .map_err(|e| format!("Upload failed: {:?}", e))?;
        
        let new_banner = Banner::new(
            data.title,
            data.description,
            image_url,
            data.link_url,
            data.display_order.unwrap_or(0),
            data.start_date,
            data.end_date,
        );
        self.crud.create_banner(new_banner).await
    }

    async fn get_banner(&self, id: &str) -> Result<Option<Banner>, String> {
        let oid = ObjectId::parse_str(id).map_err(|_| "Invalid ID format".to_string())?;
        self.crud.find_by_id(&oid).await
    }

    async fn get_active_banners(&self) -> Result<Vec<Banner>, String> {
        self.crud.find_active_banners().await
    }
    
    async fn get_all_banners(&self) -> Result<Vec<Banner>, String> {
        self.crud.find_all_banners().await
    }

    async fn update_banner(&self, id: &str, data: UpdateBannerSchema) -> Result<Banner, String> {
        Self::validate_update(&data)?;
        let oid = ObjectId::parse_str(id).map_err(|_| "Invalid ID format".to_string())?;
        self.crud.update_banner(&oid, data).await
    }

    async fn delete_banner(&self, id: &str) -> Result<(), String> {
        let oid = ObjectId::parse_str(id).map_err(|_| "Invalid ID format".to_string())?;
        self.crud.delete_banner(&oid).await
    }

    async fn update_display_order(&self, id: &str, new_order: i32) -> Result<(), String> {
        let oid = ObjectId::parse_str(id).map_err(|_| "Invalid ID format".to_string())?;
        let update_data = UpdateBannerSchema { display_order: Some(new_order), ..Default::default() };
        self.crud.update_banner(&oid, update_data).await?;
        Ok(())
    }

    async fn toggle_active_status(&self, id: &str) -> Result<Banner, String> {
        let oid = ObjectId::parse_str(id).map_err(|_| "Invalid ID format".to_string())?;
        let banner = self.crud.find_by_id(&oid).await?.ok_or("Banner not found")?;
        let update_data = UpdateBannerSchema { is_active: Some(!banner.is_active), ..Default::default() };
        self.crud.update_banner(&oid, update_data).await
    }
}

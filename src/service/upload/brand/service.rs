use cloudinary::upload::{OptionalParameters, Source, Upload};
use std::collections::BTreeSet;
use std::io::Cursor;
use std::io::Error as IoError;
use crate::service::upload::config::get_cloudinary_uploader;
use cloudinary::upload::result::UploadResult;

#[derive(Debug)]
pub enum UploadError {
    IoError(IoError),
    CloudinaryError(String),
}

impl From<IoError> for UploadError {
    fn from(error: IoError) -> Self {
        UploadError::IoError(error)
    }
}

pub struct BrandUploadService {
    uploader: Upload,
}

impl BrandUploadService {
    pub fn new() -> Self {
        Self {
            uploader: get_cloudinary_uploader(),
        }
    }

    pub async fn upload_brand_logo(
        &self,
        image_data: Vec<u8>,
        brand_name: &str,
    ) -> Result<String, UploadError> {
        // Create a unique filename
        let timestamp = chrono::Utc::now().timestamp();
        let filename = format!("brands/{}_{}_logo.png", 
            brand_name.to_lowercase().replace(" ", "_"), 
            timestamp);

        // Convert Vec<u8> to base64 for Cloudinary
        let base64_data = base64::encode(image_data);
        let data_url = format!("data:image/png;base64,{}", base64_data);

        let source = Source::DataUrl(data_url);

        // Set upload options
        let mut options = BTreeSet::new();
        options.insert(OptionalParameters::Folder("brands".to_string()));
        options.insert(OptionalParameters::PublicId(filename));

        // Upload to Cloudinary
        let result = self
            .uploader
            .image(source, &options)
            .await
            .map_err(|e| UploadError::CloudinaryError(e.to_string()))?;

        match result {
            UploadResult::Response(response) => Ok(response.secure_url),
            UploadResult::ResponseWithImageMetadata(response) => Ok(response.secure_url),
            UploadResult::Error(err) => Err(UploadError::CloudinaryError(err.error.message)),
        }
    }

    pub async fn delete_brand_logo(&self, public_id: &str) -> Result<(), UploadError> {
        self.uploader
            .destroy(public_id.to_string())
            .await
            .map_err(|e| UploadError::CloudinaryError(e.to_string()))?;
        Ok(())
    }
}
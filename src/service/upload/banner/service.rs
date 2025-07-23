use cloudinary::upload::{OptionalParameters, Source, Upload};
use std::collections::BTreeSet;
use std::io::Error as IoError;
use std::path::PathBuf;

use crate::service::upload::config::get_cloudinary_uploader;
use cloudinary::upload::result::UploadResult;

pub struct BannerUploadService {
    uploader: Upload,
}

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

impl BannerUploadService {
    pub fn new() -> Self {
        Self {
            uploader: get_cloudinary_uploader(),
        }
    }

    pub async fn upload_banner(&self, image_data: &str) -> Result<String, UploadError> {
        let source = if image_data.starts_with("/") || image_data.starts_with("./") {
            // It's a file path
            Source::Path(PathBuf::from(image_data))
        } else {
            // It's base64 data
            let data_url = if image_data.starts_with("data:image") {
                image_data.to_string()
            } else {
                format!("data:image/png;base64,{}", image_data)
            };
            Source::DataUrl(data_url)
        };

        // Minimize parameters to reduce signature complexity
        let mut options = BTreeSet::new();
        options.insert(OptionalParameters::Folder("banners".to_string()));

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

    pub async fn delete_banner(&self, public_id: &str) -> Result<(), UploadError> {
        self.uploader
            .destroy(public_id.to_string())
            .await
            .map_err(|e| UploadError::CloudinaryError(e.to_string()))?;
        Ok(())
    }
}

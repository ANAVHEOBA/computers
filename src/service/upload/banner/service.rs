use cloudinary::upload::{OptionalParameters, Source, Upload};
use std::collections::BTreeSet;
use std::io::Error as IoError;

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

    pub async fn upload_banner(&self, base64_image: &str) -> Result<String, UploadError> {
        let data_url = if base64_image.starts_with("data:image") {
            base64_image.to_string()
        } else {
            format!("data:image/png;base64,{}", base64_image)
        };

        let options = BTreeSet::from([
            OptionalParameters::Folder("banners".to_string()),
            OptionalParameters::UseFileName(true),
            OptionalParameters::UniqueFilename(true),
        ]);

        let result = self
            .uploader
            .image(Source::DataUrl(data_url), &options)
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

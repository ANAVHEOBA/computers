use cloudinary::upload::Upload;
use std::env;

pub fn get_cloudinary_uploader() -> Upload {
    let cloud_name = env::var("CLOUDINARY_CLOUD_NAME").expect("CLOUDINARY_CLOUD_NAME must be set");
    let api_key = env::var("CLOUDINARY_API_KEY").expect("CLOUDINARY_API_KEY must be set");
    let api_secret = env::var("CLOUDINARY_API_SECRET").expect("CLOUDINARY_API_SECRET must be set");

    Upload::new(api_key, cloud_name, api_secret)
}

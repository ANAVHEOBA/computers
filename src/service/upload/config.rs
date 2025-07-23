use cloudinary::upload::Upload;
use std::env;

pub fn get_cloudinary_uploader() -> Upload {
    // Order matters for signature generation
    let cloud_name = env::var("CLOUDINARY_CLOUD_NAME")
        .expect("CLOUDINARY_CLOUD_NAME must be set")
        .trim()
        .to_string();

    let api_key = env::var("CLOUDINARY_API_KEY")
        .expect("CLOUDINARY_API_KEY must be set")
        .trim()
        .to_string();

    let api_secret = env::var("CLOUDINARY_API_SECRET")
        .expect("CLOUDINARY_API_SECRET must be set")
        .trim()
        .to_string();

    // The order of parameters in Upload::new is: api_key, cloud_name, api_secret
    Upload::new(api_key, cloud_name, api_secret)
}

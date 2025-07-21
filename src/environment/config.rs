use dotenv::dotenv;
use std::env;

pub struct Config {
    pub mongodb_url: String,
}

impl Config {
    pub fn init() -> Config {
        dotenv().ok();

        let mongodb_url = env::var("MONGODB_URL")
            .expect("MONGODB_URL must be set in environment variables");

        Config {
            mongodb_url,
        }
    }
}


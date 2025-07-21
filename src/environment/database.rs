use mongodb::{Client, Database};
use std::env;

pub async fn connect_to_mongodb() -> Result<Database, mongodb::error::Error> {
    let mongo_uri = env::var("MONGODB_URL").expect("MONGODB_URL must be set");
    let database_name = env::var("DATABASE_NAME").unwrap_or_else(|_| "user_registration".to_string());

    let client = Client::with_uri_str(&mongo_uri).await?;
    let db = client.database(&database_name);
    
    println!("Connected to MongoDB successfully!");
    Ok(db)
}

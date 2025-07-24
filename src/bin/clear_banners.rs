
use computers::environment::database::connect_to_mongodb;
use mongodb::bson::doc;
use mongodb::Collection;
use dotenv::dotenv;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    dotenv().ok(); // Load environment variables from .env file
    println!("Connecting to the database to clear banners...");

    let db = connect_to_mongodb().await?;
    let banner_collection: Collection<bson::Document> = db.collection("banners");
    
    let delete_result = banner_collection.delete_many(doc! {}).await?;
    
    println!("\nCleanup complete.");
    println!("{} banners were deleted.", delete_result.deleted_count);
    
    Ok(())
} 
use mongodb::{Collection, Database, bson::{doc, oid::ObjectId, DateTime as BsonDateTime}, options::{FindOptions, FindOneAndUpdateOptions, ReturnDocument}};
use futures_util::stream::TryStreamExt;
use crate::module::banner::{model::Banner, schema::UpdateBannerSchema};
use chrono::Utc;

pub struct BannerCrud {
    collection: Collection<Banner>,
}

impl BannerCrud {
    pub fn new(db: &Database) -> Self {
        Self {
            collection: db.collection("banners"),
        }
    }

    pub async fn create_banner(&self, banner: Banner) -> Result<Banner, String> {
        let result = self.collection.insert_one(banner).await.map_err(|e| e.to_string())?;
        self.collection.find_one(doc! { "_id": result.inserted_id }).await
            .map_err(|e| e.to_string())?
            .ok_or_else(|| "Failed to find created banner".to_string())
    }

    pub async fn find_by_id(&self, id: &ObjectId) -> Result<Option<Banner>, String> {
        self.collection.find_one(doc! { "_id": id }).await.map_err(|e| e.to_string())
    }

    pub async fn find_active_banners(&self) -> Result<Vec<Banner>, String> {
        let filter = doc! { "is_active": true };
        let find_options = FindOptions::builder().sort(doc! { "display_order": 1 }).build();
        let mut cursor = self.collection.find(filter).with_options(find_options).await.map_err(|e| e.to_string())?;
        
        let mut banners: Vec<Banner> = Vec::new();
        while let Some(banner) = cursor.try_next().await.map_err(|e| e.to_string())? {
            banners.push(banner);
        }
        Ok(banners)
    }

    pub async fn find_all_banners(&self) -> Result<Vec<Banner>, String> {
        let find_options = FindOptions::builder().sort(doc! { "display_order": 1 }).build();
        let mut cursor = self.collection.find(doc!{}).with_options(find_options).await.map_err(|e| e.to_string())?;
        
        let mut banners: Vec<Banner> = Vec::new();
        while let Some(banner) = cursor.try_next().await.map_err(|e| e.to_string())? {
            banners.push(banner);
        }
        Ok(banners)
    }

    pub async fn update_banner(&self, id: &ObjectId, data: UpdateBannerSchema) -> Result<Banner, String> {
        let mut update_doc = doc! {};
        if let Some(title) = data.title {
            update_doc.insert("title", title);
        }
        if let Some(description) = data.description {
            update_doc.insert("description", description);
        }
        if let Some(image_url) = data.image_data { // Assuming image_data is the new URL
            update_doc.insert("image_url", image_url);
        }
        if let Some(link_url) = data.link_url {
            update_doc.insert("link_url", link_url);
        }
        if let Some(is_active) = data.is_active {
            update_doc.insert("is_active", is_active);
        }
        if let Some(display_order) = data.display_order {
            update_doc.insert("display_order", display_order);
        }
        if let Some(start_date) = data.start_date {
            update_doc.insert("start_date", BsonDateTime::from_chrono(start_date));
        }
        if let Some(end_date) = data.end_date {
            update_doc.insert("end_date", BsonDateTime::from_chrono(end_date));
        }

        // Add updated_at timestamp
        update_doc.insert("updated_at", BsonDateTime::from_chrono(Utc::now()));

        let update = doc! { "$set": update_doc };
        let options = FindOneAndUpdateOptions::builder()
            .return_document(ReturnDocument::After)
            .build();

        self.collection
            .find_one_and_update(doc! { "_id": id }, update)
            .with_options(options)
            .await
            .map_err(|e| e.to_string())?
            .ok_or_else(|| "Banner not found".to_string())
    }

    pub async fn delete_banner(&self, id: &ObjectId) -> Result<(), String> {
        self.collection.delete_one(doc! { "_id": id }).await.map_err(|e| e.to_string())?;
        Ok(())
    }
}

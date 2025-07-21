use mongodb::{Collection, Database, bson::{doc, DateTime as BsonDateTime}};
use bcrypt::{hash, DEFAULT_COST};
use async_trait::async_trait;
use chrono::{DateTime, Utc};

use crate::module::user::{
    model::User,
    interface::UserRepository,
};

pub struct UserCrud {
    collection: Collection<User>,
}

impl UserCrud {
    pub fn new(db: Database) -> Self {
        let collection = db.collection("users");
        Self { collection }
    }

    pub async fn hash_password(password: &str) -> Result<String, String> {
        hash(password.as_bytes(), DEFAULT_COST)
            .map_err(|e| format!("Password hashing error: {}", e))
    }
}

#[async_trait]
impl UserRepository for UserCrud {
    async fn create_user(&self, user: User) -> Result<User, mongodb::error::Error> {
        let result = self.collection.insert_one(user.clone()).await?;
        Ok(User {
            id: result.inserted_id.as_object_id(),
            ..user
        })
    }

    async fn find_by_email(&self, email: &str) -> Result<Option<User>, mongodb::error::Error> {
        self.collection
            .find_one(doc! { "email": email }).await
    }

    async fn find_by_phone(&self, phone: &str) -> Result<Option<User>, mongodb::error::Error> {
        self.collection
            .find_one(doc! { "phone_number": phone }).await
    }

    async fn update_user_verification_status(&self, email: &str, status: bool) -> Result<(), mongodb::error::Error> {
        let filter = doc! { "email": email };
        let update = doc! { "$set": { "email_verified": status } };
        self.collection.update_one(filter, update).await?;
        Ok(())
    }

    async fn delete_user_by_email(&self, email: &str) -> Result<(), mongodb::error::Error> {
        let filter = doc! { "email": email };
        self.collection.delete_one(filter).await?;
        Ok(())
    }

    async fn update_verification_code(&self, email: &str, code: &str, expires_at: DateTime<Utc>) -> Result<(), mongodb::error::Error> {
        let filter = doc! { "email": email };
        let update = doc! { 
            "$set": { 
                "verification_code": code,
                "verification_code_expires_at": BsonDateTime::from_chrono(expires_at)
            } 
        };
        self.collection.update_one(filter, update).await?;
        Ok(())
    }
}


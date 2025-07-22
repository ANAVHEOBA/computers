use mongodb::{Collection, Database};
use crate::module::admin::model::Admin;
use bcrypt::{hash, DEFAULT_COST};

pub struct AdminCrud {
    collection: Collection<Admin>,
}

impl AdminCrud {
    pub fn new(db: &Database) -> Self {
        Self {
            collection: db.collection("admins"),
        }
    }

    pub async fn find_by_email(&self, email: &str) -> Result<Option<Admin>, String> {
        self.collection
            .find_one(
                mongodb::bson::doc! { "email": email },
            )
            .await
            .map_err(|e| format!("Database error: {}", e))
    }

    pub async fn hash_password(password: &str) -> Result<String, String> {
        hash(password.as_bytes(), DEFAULT_COST)
            .map_err(|e| format!("Failed to hash password: {}", e))
    }

    // For development purposes only - to create initial admin
    pub async fn create_admin(&self, admin: Admin) -> Result<Admin, String> {
        let result = self.collection
            .insert_one(admin)
            .await
            .map_err(|e| format!("Failed to create admin: {}", e))?;

        self.collection
            .find_one(
                mongodb::bson::doc! { "_id": result.inserted_id },
            )
            .await
            .map_err(|e| format!("Failed to fetch created admin: {}", e))?
            .ok_or_else(|| "Created admin not found".to_string())
    }
}

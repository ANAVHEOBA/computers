use mongodb::bson::{self, oid::ObjectId, DateTime as BsonDateTime};
use serde::{Deserialize, Serialize};
use chrono::{DateTime, Utc};

mod datetime_serializer {
    use super::*;
    use serde::{Deserializer, Serializer};

    pub fn serialize<S>(date: &Option<DateTime<Utc>>, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        match date {
            Some(date) => {
                let bson_dt = BsonDateTime::from_chrono(*date);
                bson_dt.serialize(serializer)
            }
            None => serializer.serialize_none(),
        }
    }

    pub fn deserialize<'de, D>(deserializer: D) -> Result<Option<DateTime<Utc>>, D::Error>
    where
        D: Deserializer<'de>,
    {
        Option::<BsonDateTime>::deserialize(deserializer)
            .map(|opt_bson_dt| opt_bson_dt.map(|bson_dt| bson_dt.to_chrono()))
    }
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct User {
    #[serde(rename = "_id", skip_serializing_if = "Option::is_none")]
    pub id: Option<ObjectId>,
    pub first_name: String,
    pub last_name: String,
    pub email: String,
    pub phone_number: String,
    pub password_hash: String,  // We store hashed password, never plain text
    pub profile_picture: Option<String>,
    pub is_active: bool,
    pub bio: Option<String>,
    pub email_verified: bool,
    pub phone_verified: bool,
    pub verification_code: Option<String>,
    #[serde(with = "datetime_serializer")]
    pub verification_code_expires_at: Option<DateTime<Utc>>,
    #[serde(with = "bson::serde_helpers::chrono_datetime_as_bson_datetime")]
    pub created_at: DateTime<Utc>,
    #[serde(with = "bson::serde_helpers::chrono_datetime_as_bson_datetime")]
    pub updated_at: DateTime<Utc>,
}

impl User {
    pub fn new(
        first_name: String,
        last_name: String,
        email: String,
        phone_number: String,
        password_hash: String,
    ) -> Self {
        let now = Utc::now();
        Self {
            id: None,
            first_name,
            last_name,
            email,
            phone_number,
            password_hash,
            profile_picture: None,
            is_active: true,
            bio: None,
            email_verified: false,
            phone_verified: false,
            verification_code: None,
            verification_code_expires_at: None,
            created_at: now,
            updated_at: now,
        }
    }
}


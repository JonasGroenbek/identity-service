use chrono::Utc;
use serde::{Deserialize, Serialize};
use mongodb::bson::{oid::ObjectId};

// You use `serde` to create structs which can serialize & deserialize between BSON:
#[derive(Serialize, Deserialize, Debug)]
struct IdentityPool {
   #[serde(rename = "_id", skip_serializing_if = "Option::is_none")]
   id: Option<ObjectId>,
   #[serde(with = "bson::serde_helpers::chrono_datetime_as_bson_datetime")]
   created_at: chrono::DateTime<Utc>,
}

impl IdentityPool {
  pub fn key<'a>() -> &'a str {
    "id"
  }
}

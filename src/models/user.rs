use chrono::Utc;
use mongodb::bson::{doc,oid::ObjectId};
use serde::{Deserialize,Serialize};


#[derive(Debug, Serialize, Deserialize)]
pub struct User {
#[serde(rename = "_id", skip_serializing_if = "Option::is_none")]
pub _id:Option<ObjectId>,
pub fullname:String,
pub email:String,
pub password:String,
#[serde(with = "bson::serde_helpers::chrono_datetime_as_bson_datetime")]
pub created_at: chrono::DateTime<Utc>,
#[serde(with = "bson::serde_helpers::chrono_datetime_as_bson_datetime")]
pub updated_at: chrono::DateTime<Utc>,
}


// impl User {
//     pub fn new(_id: Option<ObjectId>, fullname: String, password: String, created_at: chrono::DateTime<Utc>, updated_at: chrono::DateTime<Utc>) -> Self { Self { _id, fullname, password, created_at, updated_at } }
// }

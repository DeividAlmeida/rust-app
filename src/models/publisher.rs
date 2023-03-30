use mongodb::bson::oid::ObjectId;
use serde::{Serialize, Deserialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct Publisher {
    #[serde(rename = "_id", skip_serializing_if = "Option::is_none")]
    pub id: Option<ObjectId>,
    pub name: String,
    #[serde(rename = "type")]
    pub r#type: u8,
    pub gender: char,
    pub amount: Option<u32>,
    pub active: Option<bool>,
    pub updated_at: Option<i64>,
    pub created_at: Option<i64>,
}
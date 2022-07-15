use serde::{Deserialize, Serialize};
use serde_json::Value;

#[derive(Debug, Serialize, Deserialize)]
pub struct CloudinaryResponse {
    pub public_id: String,
    version: i64,
    width: i64,
    height: i64,
    format: String,
    created_at: String,
    resource_type: String,
    tags: Vec<Option<Value>>,
    bytes: i64,
    #[serde(rename = "type")]
    welcome_type: String,
    etag: String,
    url: String,
    pub secure_url: String,
    signature: String,
    original_filename: String,
}

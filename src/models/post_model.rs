use mongodb::bson::oid::ObjectId;
use rocket::{fs::TempFile, FromForm};
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct Image {
    pub url: String,
    pub public_id: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Post {
    #[serde(rename = "_id", skip_serializing_if = "Option::is_none")]
    pub id: Option<ObjectId>,
    pub title: String,
    pub description: String,
    pub image: Option<Image>,
}

impl Post {
    pub fn new(title: String, description: String, image: Option<Image>) -> Self {
        Self {
            id: None,
            title,
            description,
            image,
        }
    }
}

#[derive(FromForm)]
pub struct PostFromForm<'r> {
    pub title: String,
    pub description: String,
    pub image: TempFile<'r>,
}

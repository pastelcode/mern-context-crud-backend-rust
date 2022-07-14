use crate::models::post_model::Post;
use mongodb::{Client, Collection};
use std::env;

pub struct MongoRepository {
    pub posts_collection: Collection<Post>,
}

impl MongoRepository {
    pub async fn init() -> Self {
        let uri = match env::var("MONGODB_URI") {
            Ok(value) => value,
            Err(_) => format!(""),
        };
        let client = match Client::with_uri_str(uri).await {
            Ok(client) => client,
            Err(_) => panic!("Couldn't connect to MongoDB"),
        };
        let db = client.database("posts-upload-rust");
        println!("Connected to MongoDB, database = {}", db.name());
        let posts_collection = db.collection::<Post>("posts");
        MongoRepository { posts_collection }
    }
}

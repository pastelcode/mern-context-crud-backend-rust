use crate::models::post_model::Post;
use dotenv::dotenv;
use mongodb::{Client, Collection};
use std::env;

pub struct MongoRepository {
    posts_collection: Collection<Post>,
}

impl MongoRepository {
    pub async fn init() -> Self {
        dotenv().ok();
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

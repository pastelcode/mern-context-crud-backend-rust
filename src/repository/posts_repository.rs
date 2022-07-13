use crate::{models::post_model::Post, repository::mongodb_repository::MongoRepository};
use mongodb::bson::extjson::de::Error;
use rocket::futures::StreamExt;

impl MongoRepository {
    pub async fn get_all_posts(&self) -> Result<Vec<Post>, Error> {
        let cursor = match self.posts_collection.find(None, None).await {
            Ok(cursor) => cursor,
            Err(_) => panic!("Couldn't get all posts from db"),
        };
        let posts = cursor
            .map(|post| post.unwrap())
            .collect::<Vec<Post>>()
            .await;
        Ok(posts)
    }
}

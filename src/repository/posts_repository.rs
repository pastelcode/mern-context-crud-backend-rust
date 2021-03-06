use crate::{models::post_model::Post, repository::mongodb_repository::MongoRepository};
use mongodb::{
    bson::{doc, extjson::de::Error, oid::ObjectId},
    results::InsertOneResult,
};
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

    pub async fn get_post(&self, post_id: &String) -> Result<Post, Error> {
        let parsed_post_id = ObjectId::parse_str(post_id).unwrap();
        let filter = doc! {"_id": parsed_post_id};
        let post = self
            .posts_collection
            .find_one(filter, None)
            .await
            .unwrap()
            .unwrap();
        Ok(post)
    }

    pub async fn create_post(&self, received_post: Post) -> Result<InsertOneResult, Error> {
        let new_document = Post::new(
            received_post.title,
            received_post.description,
            received_post.image,
        );
        let post = self
            .posts_collection
            .insert_one(new_document, None)
            .await
            .unwrap();
        Ok(post)
    }
}

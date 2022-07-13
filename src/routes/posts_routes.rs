use rocket::{get, http::Status, serde::json::Json, State};

use crate::{models::post_model::Post, repository::mongodb_repository::MongoRepository};

#[get("/")]
pub async fn get_all_posts(db: &State<MongoRepository>) -> Result<Json<Vec<Post>>, Status> {
    let posts = db.get_all_posts().await;
    match posts {
        Ok(posts) => Ok(Json(posts)),
        Err(_) => Err(Status::InternalServerError),
    }
}

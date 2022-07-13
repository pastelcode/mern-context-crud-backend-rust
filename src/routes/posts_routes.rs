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

#[get("/<post_id>")]
pub async fn get_post(db: &State<MongoRepository>, post_id: String) -> Result<Json<Post>, Status> {
    if post_id.is_empty() {
        return Err(Status::BadRequest);
    }
    let result = db.get_post(&post_id).await;
    match result {
        Ok(post) => Ok(Json(post)),
        Err(_) => Err(Status::InternalServerError),
    }
}

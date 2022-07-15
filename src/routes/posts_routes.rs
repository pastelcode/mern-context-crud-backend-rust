use mongodb::results::InsertOneResult;
use nanoid::nanoid;
use rocket::{
    form::Form, get, http::Status, post, serde::json::Json, tokio::fs::remove_file, State,
};

use crate::{
    models::post_model::{Post, PostFromForm},
    repository::mongodb_repository::MongoRepository,
    upload_image_to_cloudinary::upload_image_to_cloudinary,
};

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

#[post("/", data = "<post_from_form>")]
pub async fn create_post(
    db: &State<MongoRepository>,
    mut post_from_form: Form<PostFromForm<'_>>,
) -> Result<Json<InsertOneResult>, Status> {
    let new_image_name = nanoid!();
    let image_path = {
        let path_to_persist_image = format!("./tmp/{}", new_image_name);
        // Save image to tmp folder
        post_from_form
            .image
            .persist_to(path_to_persist_image)
            .await
            .unwrap();
        // Get the new image path after save the temporary image to disk
        let image_path = post_from_form
            .image
            .path()
            .unwrap()
            .to_string_lossy()
            .into_owned();
        image_path
    };

    let image = upload_image_to_cloudinary(&image_path, new_image_name).await;
    // Remove the image after saving it to Cloudinary
    remove_file(image_path).await.unwrap();

    let post = Post::new(
        post_from_form.title.to_owned(),
        post_from_form.description.to_owned(),
        Some(image),
    );

    let database_result = db.create_post(post).await;
    match database_result {
        Ok(post) => Ok(Json(post)),
        Err(_) => Err(Status::InternalServerError),
    }
}

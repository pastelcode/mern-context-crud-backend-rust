mod initialize_env_variables;
mod models;
mod repository;
mod routes;
mod services;

use initialize_env_variables::initialize_env_variables;
use repository::mongodb_repository::MongoRepository;
use rocket::{build, launch, routes, tokio::fs::create_dir_all, Build, Rocket};
use routes::posts_routes::{create_post, get_all_posts, get_post};

#[launch]
async fn rocket() -> Rocket<Build> {
    initialize_env_variables();
    // Create `tmp` folder to storage temporary images to upload to Cloudinary
    create_dir_all("./tmp/").await.unwrap();
    let db = MongoRepository::init().await;
    build()
        .manage(db)
        .mount("/posts", routes![get_all_posts, get_post, create_post])
}

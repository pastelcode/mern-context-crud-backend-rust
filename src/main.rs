mod models;
mod repository;
mod routes;

use repository::mongodb_repository::MongoRepository;
use rocket::{build, launch, routes, Build, Rocket};
use routes::posts_routes::get_all_posts;

#[launch]
async fn rocket() -> Rocket<Build> {
    let db = MongoRepository::init().await;
    build().manage(db).mount("/posts", routes![get_all_posts])
}

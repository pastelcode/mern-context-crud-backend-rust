mod models;
mod repository;

use repository::mongodb_repository::MongoRepository;
use rocket::{build, launch, routes, Build, Rocket};

#[launch]
async fn rocket() -> Rocket<Build> {
    let db = MongoRepository::init().await;
    build().manage(db).mount("/", routes![])
}

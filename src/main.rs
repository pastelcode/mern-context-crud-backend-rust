use rocket::{build, launch, routes, Build, Rocket};

#[launch]
async fn rocket() -> Rocket<Build> {
    build().mount("/", routes![])
}

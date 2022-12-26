use mongodb::Client;
use rocket::routes;
use crate::repository::UserRepo;
use crate::system::startup::cli::parse_options;

mod models;
mod handlers;
mod system;
mod repository;


#[rocket::main]
async fn main() -> Result<(), rocket::Error> {
    let options = parse_options();

    let figment = rocket::Config::figment()
        .merge(("host", options.host))
        .merge(("port", options.port));

    let db_client = Client::with_uri_str(format!("mongodb://{}:{}", options.db_host, options.db_port))
        .await.expect("Can't connect to database");
    let db = db_client.database(&options.db_name);

    let user_repo = UserRepo::new(&db);

    let _rocket = rocket::custom(figment)
        .manage(user_repo)
        .mount("/", routes![handlers::index])
        .ignite().await?
        .launch().await?;

    Ok(())
}

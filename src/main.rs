use mongodb::Client;
use rocket::routes;
use crate::core::system::startup::cli::parse_options;
use crate::managers::{GroupManager, UserManager};

mod models;
mod handlers;
mod managers;
mod core;


#[rocket::main]
async fn main() -> Result<(), rocket::Error> {
    let options = parse_options();

    let figment = rocket::Config::figment()
        .merge(("host", options.host))
        .merge(("port", options.port));

    let db_client = Client::with_uri_str(format!("mongodb://{}:{}", options.db_host, options.db_port))
        .await.expect("Can't connect to database");
    let db = db_client.database(&options.db_name);

    let user_manager = UserManager::new(&db).await;
    let group_manager = GroupManager::new(&db).await;

    let _rocket = rocket::custom(figment)
        .manage(user_manager)
        .manage(group_manager)
        .mount("/api/user", handlers::user_routes())
        .ignite().await?
        .launch().await?;

    Ok(())
}

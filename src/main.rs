use mongodb::Client;
use crate::app::project::issue::customfield::register_system_types;
use crate::core::system::startup::cli::parse_options;
use crate::managers::{CustomFieldManager, GroupManager, UserManager};

mod models;
mod handlers;
mod managers;
mod core;
mod app;


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
    let customfield_manager = CustomFieldManager::new(&db).await;

    register_system_types();

    let _rocket = rocket::custom(figment)
        .manage(user_manager)
        .manage(group_manager)
        .manage(customfield_manager)
        .mount("/api/user", handlers::user_routes())
        .mount("/api/customfield", handlers::customfield_routes())
        .ignite().await?
        .launch().await?;

    Ok(())
}

use mongodb::bson::oid;
use rocket::{get, State};
use rocket::response::status::NotFound;
use crate::repository::UserRepo;

#[get("/<id>")]
pub async fn index(id: String, user_repo: &State<UserRepo>) -> Result<String, NotFound<String>> {
    if let Ok(uuid) = oid::ObjectId::parse_str(&id) {
        if let Ok(user) = user_repo.get_user(uuid).await {
            return Ok(format!("{:#?}", user));
        }
    }

    Err(NotFound(format!("User with id = '{}' not found", &id)))
}
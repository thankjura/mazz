use mongodb::bson::oid;
use rocket::{get, post, State};
use rocket::response::status::NotFound;
use crate::managers::UserManager;

#[get("/api/user/<id>")]
pub async fn get_user(id: String, user_manager: &State<UserManager>) -> Result<String, NotFound<String>> {
    if let Ok(uuid) = oid::ObjectId::parse_str(&id) {
        if let Some(user) = user_manager.get_user(&uuid).await {
            return Ok(format!("{:#?}", user));
        }
    }

    Err(NotFound(format!("User with id = '{}' not found", &id)))
}

#[post("/api/user")]
pub async fn create_user(_user_manager: &State<UserManager>) -> Result<String, NotFound<String>> {
    // if let Ok(result) = user_manager.create_user(user).await {
    //     //*user.id = result.inserted_id.as_object_id().unwrap();
    // }

    Err(NotFound(format!("User with id = not found")))
}
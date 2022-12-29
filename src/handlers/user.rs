use mongodb::bson::{Document, oid};
use rocket::{get, post, patch, delete, State};
use rocket::response::status::NotFound;
use rocket::serde::json::Json;
use crate::managers::UserManager;
use crate::models::User;

#[get("/<id>")]
pub async fn get_user(id: &str, user_manager: &State<UserManager>) -> Result<String, NotFound<String>> {
    if let Ok(uuid) = oid::ObjectId::parse_str(id) {
        if let Some(user) = user_manager.get_user(&uuid).await {
            return Ok(format!("{:#?}", user));
        }
    }

    Err(NotFound(format!("User with id = '{}' not found", id)))
}

#[post("/", data = "<user>")]
pub async fn create_user(user: Json<User>, user_manager: &State<UserManager>) -> Result<Json<User>, NotFound<String>> {
    let mut user = user.0;
    user_manager.save_user(&mut user).await;
    Ok(Json::from(user))
}

#[patch("/<id>", data = "<user_data>")]
pub async fn patch_user(id: &str, user_data: Json<Document>, user_manager: &State<UserManager>) -> Result<Json<User>, NotFound<String>> {
    if let Ok(uuid) = oid::ObjectId::parse_str(id) {
        return if let Some(mut user) = user_manager.get_user(&uuid).await {
            if user_data.contains_key("firstname") {
                user.firstname = user_data.get_str("firstname").ok().map(|s| s.to_string());
            }
            if user_data.contains_key("lastname") {
                user.lastname = user_data.get_str("lastname").ok().map(|s| s.to_string());
            }
            user_manager.save_user(&mut user).await;
            Ok(Json::from(user))
        } else {
            Err(NotFound(format!("User with id = {} not found", id)))
        }
    }

    Err(NotFound(format!("User with id = not found")))
}

#[delete("/<id>")]
pub async fn delete_user(id: &str, user_manager: &State<UserManager>) -> Result<String, NotFound<String>> {
    if let Ok(uuid) = oid::ObjectId::parse_str(id) {
        user_manager.delete_user_by_id(&uuid).await;
        return Ok("User deleted".to_string());
    }

    Err(NotFound(format!("User with id = not found")))
}
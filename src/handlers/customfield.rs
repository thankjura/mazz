use mongodb::bson::{Document, oid};
use rocket::{get, post, patch, delete, State};
use rocket::response::status::NotFound;
use rocket::serde::json::Json;
use crate::managers::CustomFieldManager;
use crate::models::CustomField;

#[get("/<id>")]
pub async fn get_customfield(id: &str, customfield_manager: &State<CustomFieldManager>) -> Result<Json<CustomField>, NotFound<String>> {
    if let Ok(uuid) = oid::ObjectId::parse_str(id) {
        if let Some(cf) = customfield_manager.get_customfield(&uuid).await {
            if let Some(cf_type) = cf.customfieldtype() {
                println!("{}", cf_type.name());
            }
            return Ok(Json::from(cf));
        }
    }

    Err(NotFound(format!("CustomField with id = '{}' not found", id)))
}

#[post("/", data = "<cf>")]
pub async fn create_customfield(cf: Json<CustomField>, customfield_manager: &State<CustomFieldManager>) -> Result<Json<CustomField>, NotFound<String>> {
    let mut cf = cf.0;
    customfield_manager.save_customfield(&mut cf).await;
    Ok(Json::from(cf))
}

#[patch("/<id>", data = "<cf_data>")]
pub async fn patch_customfield(id: &str, cf_data: Json<Document>, customfield_manager: &State<CustomFieldManager>) -> Result<Json<CustomField>, NotFound<String>> {
    if let Ok(uuid) = oid::ObjectId::parse_str(id) {
        return if let Some(mut cf) = customfield_manager.get_customfield(&uuid).await {
            if let Ok(name) = cf_data.get_str("name") {
                cf.name = name.to_string();
            }
            customfield_manager.save_customfield(&mut cf).await;
            Ok(Json::from(cf))
        } else {
            Err(NotFound(format!("CustomField with id = {} not found", id)))
        }
    }

    Err(NotFound(format!("CustomField with id = not found")))
}

#[delete("/<id>")]
pub async fn delete_customfield(id: &str, customfield_manager: &State<CustomFieldManager>) -> Result<String, NotFound<String>> {
    if let Ok(uuid) = oid::ObjectId::parse_str(id) {
        customfield_manager.delete_customfield_by_id(&uuid).await;
        return Ok("CustomField deleted".to_string());
    }

    Err(NotFound(format!("CustomField with id = not found")))
}
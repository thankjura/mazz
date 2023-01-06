use mongodb::bson::{Document, oid};
use rocket::{get, post, patch, delete, State};
use rocket::response::status::NotFound;
use rocket::serde::json::Json;
use crate::managers::ProjectManager;
use crate::models::Project;

#[get("/<id_or_key>")]
pub async fn get_project(id_or_key: &str, project_manager: &State<ProjectManager>) -> Result<String, NotFound<String>> {
    if let Ok(uuid) = oid::ObjectId::parse_str(id_or_key) {
        if let Some(project) = project_manager.get_project(&uuid).await {
            return Ok(format!("{:#?}", project));
        }
    } else if let Some(project) = project_manager.get_project_by_key(id_or_key).await {
        return Ok(format!("{:#?}", project));
    }

    Err(NotFound(format!("Project with id/key = '{}' not found", id_or_key)))
}


#[post("/", data = "<project>")]
pub async fn create_project(project: Json<Project>, project_manager: &State<ProjectManager>) -> Result<Json<Project>, NotFound<String>> {
    let mut project = project.0;
    project_manager.save_project(&mut project).await;
    Ok(Json::from(project))
}

#[patch("/<id_or_key>", data = "<project_data>")]
pub async fn patch_project(id_or_key: &str, project_data: Json<Document>, project_manager: &State<ProjectManager>) -> Result<Json<Project>, NotFound<String>> {
    async fn update_project(mut project: Project, project_data: Document, project_manager: &State<ProjectManager>) -> Result<Json<Project>, NotFound<String>> {
        if let Ok(name) = project_data.get_str("name") {
            project.name = name.to_string();
        }
        if project_data.contains_key("description") {
            project.description = project_data.get_str("description").ok().map(|s| s.to_string());
        }
        project_manager.save_project(&mut project).await;
        Ok(Json::from(project))
    }

    if let Ok(uuid) = oid::ObjectId::parse_str(id_or_key) {
        if let Some(project) = project_manager.get_project(&uuid).await {
            return update_project(project, project_data.0, project_manager).await;
        }
    } else if let Some(project) = project_manager.get_project_by_key(id_or_key).await {
        return update_project(project, project_data.0, project_manager).await;
    }

    Err(NotFound(format!("Project with id/key = {} not found", id_or_key)))
}

#[delete("/<id_or_key>")]
pub async fn delete_project(id_or_key: &str, project_manager: &State<ProjectManager>) -> Result<String, NotFound<String>> {
    if let Ok(uuid) = oid::ObjectId::parse_str(id_or_key) {
        project_manager.delete_project_by_id(&uuid).await;
        return Ok("Project deleted".to_string());
    } else if let Some(Project {id: Some(id), ..}) = project_manager.get_project_by_key(id_or_key).await {
        project_manager.delete_project_by_id(&id).await;
        return Ok("Project deleted".to_string());
    }

    Err(NotFound(format!("Project with id/key = {} not found", id_or_key)))
}
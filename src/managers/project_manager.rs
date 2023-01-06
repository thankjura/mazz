use mongodb::bson::oid::ObjectId;
use crate::models::Project;
use crate::core::db::repository::ProjectRepo;

pub struct ProjectManager {
    project_repo: ProjectRepo
}

impl ProjectManager {
    pub(crate) async fn new(database: &mongodb::Database) -> Self {
        let repo = ProjectRepo::new(database).await;
        Self {
            project_repo: repo
        }
    }

    pub async fn save_project(&self, project: &mut Project) {
        if let Ok(project_id) = self.project_repo.save_project(project).await {
            project.id.replace(project_id);
        }
    }

    pub async fn get_project(&self, project_id: &ObjectId) -> Option<Project> {
        if let Ok(project) = self.project_repo.get_project(project_id).await {
            return project;
        }

        None
    }

    pub async fn get_project_by_key(&self, project_key: &str) -> Option<Project> {
        if let Ok(project) = self.project_repo.get_project_by_key(project_key).await {
            return project;
        }

        None
    }

    pub async fn delete_project_by_id(&self, project_id: &ObjectId) {
        if let Err(res) = self.project_repo.delete_project(project_id).await {
            println!("{}", res);
        }
    }

    pub async fn delete_project(&self, project: Project) {
        if let Some(project_id) = project.id {
            if let Err(res) = self.project_repo.delete_project(&project_id).await {
                println!("{}", res);
            }
        }
    }
}
use mongodb::bson::doc;
use mongodb::bson::oid::ObjectId;
use mongodb::Collection;
use crate::core::db::make_indexes;
use crate::models::Project;


pub struct ProjectRepo {
    collection: Collection<Project>
}

impl ProjectRepo {
    pub(crate) async fn new(database: &mongodb::Database) -> Self {
        let collection = database.collection("projects");

        let indexes = vec![
            doc! { "name": 1 },
        ];
        make_indexes(&collection, indexes).await;

        Self {
            collection
        }
    }

    async fn create_project(&self, project: &mut Project) -> Result<ObjectId, &str> {
        match self.collection.insert_one(project, None).await {
            Ok(result) => {
                Ok(result.inserted_id.as_object_id().unwrap())
            }
            Err(_) => {
                Err("Failed to create project")
            }
        }
    }

    pub async fn get_project(&self, project_id: &ObjectId) -> Result<Option<Project>, &str> {
        let filter = doc! {"_id": project_id};
        match self.collection.find_one(filter, None).await {
            Ok(project) => {
                Ok(project)
            }
            Err(_) => {
                Err("Failed to get project")
            }
        }
    }

    pub async fn get_project_by_key(&self, project_key: &str) -> Result<Option<Project>, &str> {
        let filter = doc! {"key": project_key.to_uppercase()};
        match self.collection.find_one(filter, None).await {
            Ok(project) => {
                Ok(project)
            }
            Err(_) => {
                Err("Failed to get project")
            }
        }
    }

    pub async fn save_project(&self, project: &mut Project) -> Result<ObjectId, &str> {
        if let Some(project_id) = project.id() {
            let filter = doc! {"_id": project_id};
            let update = doc! {
                "$set": {
                    "name": project.name(),
                }
            };
            return match self.collection.update_one(filter, update, None).await {
                Ok(result) => {
                    if let Some(Some(res)) = result.upserted_id.map(|s| s.as_object_id()) {
                        Ok(res)
                    } else {
                        Ok(project_id.clone())
                    }
                }
                Err(_) => {
                    Err("Failed to update project")
                }
            }
        } else {
            self.create_project(project).await
        }
    }

    pub async fn delete_project(&self, project_id: &ObjectId) -> Result<u64, &str> {
        let filter = doc! {"_id": project_id};
        return match self.collection.delete_one(filter, None).await {
            Ok(result) => {
                Ok(result.deleted_count)
            }
            Err(_) => {
                Err("Failed to delete project")
            }
        }
    }
}
use mongodb::bson::oid::ObjectId;
use crate::models::Group;
use crate::core::db::repository::GroupRepo;

pub struct GroupManager {
    group_repo: GroupRepo
}

impl GroupManager {
    pub(crate) async fn new(database: &mongodb::Database) -> Self {
        let repo = GroupRepo::new(database).await;
        Self {
            group_repo: repo
        }
    }

    pub async fn create_group(&self, name: &str) -> Option<ObjectId> {
        if let Ok(group_id) = self.group_repo.create_group(name).await {
            return Some(group_id);
        }
        None
    }

    pub async fn get_group(&self, name: &str) -> Option<Group> {
        if let Ok(group) = self.group_repo.get_group(name).await {
            return group;
        }

        None
    }

    pub async fn delete_group(&self, name: &str) {
        if let Err(res) = self.group_repo.delete_group(name).await {
            println!("{}", res)
        }
    }
}
use mongodb::bson::doc;
use mongodb::bson::oid::ObjectId;
use mongodb::Collection;
use crate::models::Group;


pub struct GroupRepo {
    collection: Collection<Group>
}

impl GroupRepo {
    pub fn new(database: &mongodb::Database) -> Self {
        Self {
            collection: database.collection("groups")
        }
    }

    pub async fn create_group(&self, name: &str) -> Result<ObjectId, &str> {
        let group = Group::new(name.to_string());
        match self.collection.insert_one(group, None).await {
            Ok(result) => {
                Ok(result.inserted_id.as_object_id().unwrap())
            }
            Err(_) => {
                Err("Failed to create group")
            }
        }
    }

    pub async fn get_group(&self, group_name: &str) -> Result<Option<Group>, &str> {
        let filter = doc! {"name": group_name};
        match self.collection.find_one(filter, None).await {
            Ok(user) => {
                Ok(user)
            }
            Err(_) => {
                Err("Failed to get group")
            }
        }
    }

    pub async fn delete_group(&self, group_name: &str) -> Result<u64, &str> {
        let filter = doc! {"name": group_name};
        match self.collection.delete_one(filter, None).await {
            Ok(result) => {
                Ok(result.deleted_count)
            }
            Err(_) => {
                Err("Failed to delete group")
            }
        }
    }
}
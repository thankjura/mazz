use mongodb::bson::doc;
use mongodb::bson::oid::ObjectId;
use mongodb::Collection;
use crate::core::db::make_indexes;
use crate::models::Group;


pub struct GroupRepo {
    collection: Collection<Group>,
}

impl GroupRepo {
    pub(crate) async fn new(database: &mongodb::Database) -> Self {
        let indexes = vec![
            doc! { "name": 1 }
        ];
        let collection = database.collection("groups");

        make_indexes(&collection, indexes).await;

        Self {
            collection
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
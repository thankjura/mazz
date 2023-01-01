use mongodb::bson::doc;
use mongodb::bson::oid::ObjectId;
use mongodb::Collection;
use crate::core::db::make_indexes;
use crate::models::CustomField;


pub struct CustomFieldRepo {
    collection: Collection<CustomField>
}

impl CustomFieldRepo {
    pub(crate) async fn new(database: &mongodb::Database) -> Self {
        let collection = database.collection("customfields");

        let indexes = vec![
            doc! { "name": 1 },
            doc! { "cf_type_key": 1 },
        ];
        make_indexes(&collection, indexes).await;

        Self {
            collection
        }
    }

    async fn create_customfield(&self, cf: &mut CustomField) -> Result<ObjectId, &str> {
        match self.collection.insert_one(cf, None).await {
            Ok(result) => {
                Ok(result.inserted_id.as_object_id().unwrap())
            }
            Err(_) => {
                Err("Failed to create customfield")
            }
        }
    }

    pub async fn get_customfield(&self, cf_id: &ObjectId) -> Result<Option<CustomField>, &str> {
        let filter = doc! {"_id": cf_id};
        match self.collection.find_one(filter, None).await {
            Ok(cf) => {
                Ok(cf)
            }
            Err(_) => {
                Err("Failed to get customfield")
            }
        }
    }

    pub async fn save_customfield(&self, cf: &mut CustomField) -> Result<ObjectId, &str> {
        if let Some(cf_id) = cf.id() {
            let filter = doc! {"_id": cf_id};
            let update = doc! {
                "$set": {
                    "name": cf.name(),
                    //"cf_type_key": cf.cf_type_key(),
                }
            };
            return match self.collection.update_one(filter, update, None).await {
                Ok(result) => {
                    if let Some(Some(res)) = result.upserted_id.map(|s| s.as_object_id()) {
                        Ok(res)
                    } else {
                        Ok(cf_id.clone())
                    }
                }
                Err(_) => {
                    Err("Failed to update customfield")
                }
            }
        } else {
            self.create_customfield(cf).await
        }
    }

    pub async fn delete_customfield(&self, cf_id: &ObjectId) -> Result<u64, &str> {
        let filter = doc! {"_id": cf_id};
        return match self.collection.delete_one(filter, None).await {
            Ok(result) => {
                Ok(result.deleted_count)
            }
            Err(_) => {
                Err("Failed to delete customfield")
            }
        }
    }
}
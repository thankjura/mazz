use mongodb::bson::doc;
use mongodb::bson::oid::ObjectId;
use mongodb::Collection;
use crate::core::db::make_indexes;
use crate::models::User;


pub struct UserRepo {
    collection: Collection<User>
}

impl UserRepo {
    pub(crate) async fn new(database: &mongodb::Database) -> Self {
        let collection = database.collection("users");

        let indexes = vec![
            doc! { "login": 1 },
            doc! { "email": 1 },
        ];
        make_indexes(&collection, indexes).await;

        Self {
            collection
        }
    }

    async fn create_user(&self, user: &mut User) -> Result<ObjectId, &str> {
        match self.collection.insert_one(user, None).await {
            Ok(result) => {
                Ok(result.inserted_id.as_object_id().unwrap())
            }
            Err(_) => {
                Err("Failed to create user")
            }
        }
    }

    pub async fn get_user(&self, user_id: &ObjectId) -> Result<Option<User>, &str> {
        let filter = doc! {"_id": user_id};
        match self.collection.find_one(filter, None).await {
            Ok(user) => {
                Ok(user)
            }
            Err(_) => {
                Err("Failed to get user")
            }
        }
    }

    pub async fn save_user(&self, user: &mut User) -> Result<ObjectId, &str> {
        if let Some(user_id) = user.id() {
            let filter = doc! {"_id": user_id};
            let update = doc! {
                "$set": {
                    "login": user.login(),
                    "email": user.email(),
                    "firstname": user.firstname(),
                    "lastname": user.lastname(),
                }
            };
            return match self.collection.update_one(filter, update, None).await {
                Ok(result) => {
                    if let Some(Some(res)) = result.upserted_id.map(|s| s.as_object_id()) {
                        Ok(res)
                    } else {
                        Ok(user_id.clone())
                    }
                }
                Err(_) => {
                    Err("Failed to update user")
                }
            }
        } else {
            self.create_user(user).await
        }
    }

    pub async fn delete_user(&self, user_id: &ObjectId) -> Result<u64, &str> {
        let filter = doc! {"_id": user_id};
        return match self.collection.delete_one(filter, None).await {
            Ok(result) => {
                Ok(result.deleted_count)
            }
            Err(_) => {
                Err("Failed to delete user")
            }
        }
    }
}
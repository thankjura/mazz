use mongodb::bson::doc;
use mongodb::bson::oid::ObjectId;
use mongodb::Collection;
use crate::models::User;


pub struct UserRepo {
    collection: Collection<User>
}

impl UserRepo {
    pub fn new(database: &mongodb::Database) -> Self {
        Self {
            collection: database.collection("users")
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
            match self.collection.update_one(filter, update, None).await {
                Ok(result) => {
                    Ok(result.upserted_id.unwrap().as_object_id().unwrap())
                }
                Err(_) => {
                    Err("Failed to update user")
                }
            }
        } else {
            self.create_user(user).await
        }
    }

    pub async fn delete_user(&self, user: User) -> Result<u64, &str> {
        if let Some(user_id) = user.id() {
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
        Ok(0)
    }
}
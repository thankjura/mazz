use mongodb::bson::{doc, Uuid};
use mongodb::bson::oid::ObjectId;
use mongodb::Collection;
use mongodb::error::Error;
use mongodb::results::InsertOneResult;
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

    pub async fn create_user(&self, user: &mut User) -> Result<InsertOneResult, Error> {
        self.collection.insert_one(user, None).await
    }

    pub async fn get_user(&self, user_id: ObjectId) -> Result<Option<User>, Error> {
        let filter = doc! {"_id": user_id};
        println!("{:#?}", filter);
        self.collection.find_one(filter, None).await
    }
}
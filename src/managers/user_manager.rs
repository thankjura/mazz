use mongodb::bson::oid::ObjectId;
use crate::models::User;
use crate::core::db::repository::UserRepo;

pub struct UserManager {
    user_repo: UserRepo
}

impl UserManager {
    pub(crate) async fn new(database: &mongodb::Database) -> Self {
        let repo = UserRepo::new(database).await;
        Self {
            user_repo: repo
        }
    }

    pub async fn save_user(&self, user: &mut User) {
        if let Ok(user_id) = self.user_repo.save_user(user).await {
            user.id.replace(user_id);
        }
    }

    pub async fn get_user(&self, user_id: &ObjectId) -> Option<User> {
        if let Ok(user) = self.user_repo.get_user(user_id).await {
            return user;
        }

        None
    }

    pub async fn delete_user_by_id(&self, user_id: &ObjectId) {
        if let Err(res) = self.user_repo.delete_user(user_id).await {
            println!("{}", res);
        }
    }

    pub async fn delete_user(&self, user: User) {
        if let Some(user_id) = user.id {
            if let Err(res) = self.user_repo.delete_user(&user_id).await {
                println!("{}", res);
            }
        }
    }
}
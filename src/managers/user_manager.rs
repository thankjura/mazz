use mongodb::bson::oid::ObjectId;
use crate::models::User;
use crate::repository::UserRepo;

pub struct UserManager {
    user_repo: UserRepo
}

impl UserManager {
    pub(crate) fn new(user_repo: UserRepo) -> Self {
        Self {
            user_repo
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
}
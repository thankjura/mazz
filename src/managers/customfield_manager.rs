use mongodb::bson::oid::ObjectId;
use crate::core::db::repository::CustomFieldRepo;
use crate::models::CustomField;

pub struct CustomFieldManager {
    customfield_repo: CustomFieldRepo
}

impl CustomFieldManager {
    pub(crate) async fn new(database: &mongodb::Database) -> Self {
        let repo = CustomFieldRepo::new(database).await;
        Self {
            customfield_repo: repo
        }
    }

    pub async fn save_customfield(&self, cf: &mut CustomField) {
        if let Ok(cf_id) = self.customfield_repo.save_customfield(cf).await {
            cf.id.replace(cf_id);
        }
    }

    pub async fn get_customfield(&self, cf_id: &ObjectId) -> Option<CustomField> {
        if let Ok(cf) = self.customfield_repo.get_customfield(cf_id).await {
            return cf;
        }

        None
    }

    pub async fn delete_customfield_by_id(&self, cf_id: &ObjectId) {
        if let Err(res) = self.customfield_repo.delete_customfield(cf_id).await {
            println!("{}", res);
        }
    }

    pub async fn delete_customfield(&self, cf: CustomField) {
        if let Some(cf_id) = cf.id {
            if let Err(res) = self.customfield_repo.delete_customfield(&cf_id).await {
                println!("{}", res);
            }
        }
    }
}
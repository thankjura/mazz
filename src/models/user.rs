use mongodb::bson::oid::ObjectId;
use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize, Debug)]
pub struct User {
    #[serde(rename = "_id", skip_serializing_if = "Option::is_none")]
    pub(crate) id: Option<ObjectId>,
    pub(crate) login: String,
    pub(crate) email: String,
    pub(crate) firstname: Option<String>,
    pub(crate) lastname: Option<String>,
}

impl User {
    pub fn id(&self) -> Option<&ObjectId> {
        self.id.as_ref()
    }

    pub fn login(&self) -> &str {
        &self.login
    }

    pub fn email(&self) -> &str {
        &self.email
    }

    pub fn firstname(&self) -> Option<&str> {
        self.firstname.as_deref()
    }

    pub fn lastname(&self) -> Option<&str> {
        self.lastname.as_deref()
    }

    pub fn editable_field() -> Vec<&'static str> {
        vec!["firstname", "lastname", "email"]
    }
}
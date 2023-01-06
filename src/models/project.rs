use serde::{Deserialize, Serialize};
use mongodb::bson::oid::ObjectId;

#[derive(Deserialize, Serialize, Debug)]
pub struct Project {
    #[serde(rename = "_id", skip_serializing_if = "Option::is_none")]
    pub(crate) id: Option<ObjectId>,
    pub(crate) name: String,
    pub(crate) description: Option<String>,
    pub(crate) lead_id: Option<ObjectId>
}

impl Project {
    pub fn id(&self) -> Option<&ObjectId> {
        self.id.as_ref()
    }

    pub fn name(&self) -> &str {
        &self.name
    }

    pub fn description(&self) -> Option<&str> {
        self.description.as_deref()
    }

    pub fn lead_id(&self) -> Option<&ObjectId> {
        self.lead_id.as_ref()
    }
}
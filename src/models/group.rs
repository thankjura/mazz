use mongodb::bson::oid::ObjectId;
use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize, Debug)]
pub struct Group {
    #[serde(rename = "_id", skip_serializing_if = "Option::is_none")]
    pub(crate) id: Option<ObjectId>,
    pub(crate) name: String,
}

impl Group {
    pub(crate) fn new(name: String) -> Self {
        Self {
            id: None,
            name,
        }
    }

    pub fn id(&self) -> Option<&ObjectId> {
        self.id.as_ref()
    }

    pub fn name(&self) -> &str {
        &self.name
    }
}
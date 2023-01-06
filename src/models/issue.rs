use serde::{Deserialize, Serialize};
use mongodb::bson::oid::ObjectId;


#[derive(Deserialize, Serialize, Debug)]
pub struct Issue {
    #[serde(rename = "_id", skip_serializing_if = "Option::is_none")]
    pub(crate) id: Option<ObjectId>,
    pub(crate) project_id: ObjectId,
    pub(crate) issuetype_id: ObjectId,
    pub(crate) summary: String,
    pub(crate) description: Option<String>,
    pub(crate) creator_id: Option<ObjectId>,
    pub(crate) author_id: Option<ObjectId>,
    pub(crate) assignee_id: Option<ObjectId>,
}

impl Issue {
    pub fn id(&self) -> Option<&ObjectId> {
        self.id.as_ref()
    }

    pub fn project_id(&self) -> &ObjectId {
        &self.project_id
    }

    pub fn issuetype_id(&self) -> &ObjectId {
        &self.issuetype_id
    }

    pub fn summary(&self) -> &str {
        &self.summary
    }

    pub fn description(&self) -> Option<&str> {
        self.description.as_deref()
    }
}
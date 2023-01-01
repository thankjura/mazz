use std::sync::Arc;
use mongodb::bson::oid::ObjectId;
use serde::{Deserialize, Serialize};
use crate::app::project::issue::customfield::customfieldtype::{CustomFieldType, CUSTOMFIELDTYPE_REGISTRY};


#[derive(Deserialize, Serialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct CustomField {
    #[serde(rename = "_id", skip_serializing_if = "Option::is_none")]
    pub(crate) id: Option<ObjectId>,
    pub(crate) name: String,
    pub(crate) cf_type_key: String,
}


impl CustomField {
    pub fn id(&self) -> Option<&ObjectId> {
        self.id.as_ref()
    }

    pub fn name(&self) -> &str {
        &self.name
    }

    pub fn cf_type_key(&self) -> &str {
        &self.cf_type_key
    }

    pub fn customfieldtype(&self) -> Option<Arc<dyn CustomFieldType>> {
        CUSTOMFIELDTYPE_REGISTRY.lock().unwrap().get_customfieldtype(&self.cf_type_key)
        // if let Some(cf_type) = CUSTOMFIELDTYPE_REGISTRY.lock().unwrap().get_customfieldtype(&self.cf_type_key) {
        //     return Some(cf_type);
        // }
        //
        // None
    }
}
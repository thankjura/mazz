use mongodb::bson::{doc, Document};
use crate::app::project::issue::customfield::customfieldtype::CustomFieldType;

pub struct TextCustomFieldType {
    name: String
}

impl Default for TextCustomFieldType {
    fn default() -> Self {
        Self {
            name: "String type".to_string()
        }
    }
}

impl CustomFieldType for TextCustomFieldType {
    fn name(&self) -> &str {
        &self.name
    }

    fn dumps(&self, doc: Option<Document>) -> Option<Document> {
        if let Some(doc) = doc {
            return Some(doc! { "value": doc });
        }
        None
    }

    fn loads(&self, doc: Option<Document>) -> Option<Document> {
        if let Some(doc) = doc {
            //return doc.get_str("value").ok().map(|s| s.to_string());
        }

        None
    }
}
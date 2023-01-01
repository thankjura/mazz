use std::collections::HashMap;
use std::sync::{Arc, Mutex};
use lazy_static::lazy_static;
use mongodb::bson::Document;


pub struct CustomFieldTypeRegistry {
    registry: HashMap<&'static str, Arc<dyn CustomFieldType>>
}

impl CustomFieldTypeRegistry {
    pub(super) fn new() -> Self {
        Self {
            registry: HashMap::new()
        }
    }

    pub fn register(&mut self, name: &'static str, customfieldtype: impl CustomFieldType) {
        self.registry.insert(name, Arc::new(customfieldtype));
    }

    pub fn unregister(&mut self, name: &str) {
        self.registry.remove(name);
    }

    pub fn get_customfieldtype(&self, name: &str) -> Option<Arc<dyn CustomFieldType>> {
        if let Some(val) = self.registry.get(name) {
            return Some(Arc::clone(val));
        }

        None
        // if let Some(out) = self.registry.get(name) {
        //
        //
        // };
        //
        // None
    }
}

pub trait CustomFieldType: Sync + Send + 'static {
    fn name(&self) -> &str;
    fn dumps(&self, doc: Option<Document>) -> Option<Document>;
    fn loads(&self, doc: Option<Document>) -> Option<Document>;
}

lazy_static! {
    pub static ref CUSTOMFIELDTYPE_REGISTRY: Mutex<CustomFieldTypeRegistry> = Mutex::new(CustomFieldTypeRegistry::new());
}
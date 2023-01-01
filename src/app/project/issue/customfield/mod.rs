use crate::app::project::issue::customfield::customfieldtype::CUSTOMFIELDTYPE_REGISTRY;
use crate::app::project::issue::customfield::imp::TextCustomFieldType;

pub mod customfieldtype;
mod imp;

pub fn register_system_types() {
    CUSTOMFIELDTYPE_REGISTRY.lock().unwrap().register("TextCustomFieldType", TextCustomFieldType::default());
}
use std::sync::Mutex;
use crate::assets::{PicoColor, PicoObject, PicoHeader};
use rlua::{Lua, RegistryKey, Table};

pub struct PicoSave {
    pub header: PicoHeader,
    pub objects: Vec<PicoObject>,
    pub footer: String,
}

impl PicoSave {
    pub fn split_save_string(s: &str) -> (&str, &str, &str) {
        let split_parts: Vec<&str> = s.splitn(2, ' ').collect();
        let (header, objects_and_footer): (&str, &str) = (split_parts.get(0).cloned().unwrap(), split_parts.get(1).cloned().unwrap());
        let split_parts: Vec<&str> = objects_and_footer.rsplitn(2, '%').collect();
        let (objects, footer): (&str, &str) = (split_parts.get(0).cloned().unwrap(), split_parts.get(1).cloned().unwrap());
        (header.trim(), objects.trim(), footer.trim())
    }
}

impl From<String> for PicoSave {
    fn from(s: String) -> Self {
        let header: PicoHeader;
        let objects: Vec<PicoObject>;
        let footer: String;

        let (header_string, objects_string, footer_string): (&str, &str, &str) = PicoSave::split_save_string(s.as_str());

        header = PicoHeader::from(header_string);

        // TODO: create objects from string

        footer = footer_string.to_string();

        todo!()
    }
}



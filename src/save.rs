use crate::assets::{PicoObject, PicoHeader, Serialize};
use rlua::{Lua, Table};

/// Represents a picoCAD savefile and all its contents.
#[derive(Debug, PartialEq)]
pub struct PicoSave {
    pub header: PicoHeader,
    pub objects: Vec<PicoObject>,
    pub footer: String,
}

impl PicoSave {
    /// Splits a savefile represented as a string into header, objects and footer.
    fn split_save_string(s: &str) -> (&str, &str, &str) {
        let split_parts: Vec<&str> = s.splitn(2, '\n').collect();
        let (header, objects_and_footer): (&str, &str) = (split_parts.get(0).cloned().unwrap(), split_parts.get(1).cloned().unwrap());
        let split_parts: Vec<&str> = objects_and_footer.rsplitn(2, '%').collect();
        let (objects, footer): (&str, &str) = (split_parts.get(1).cloned().unwrap(), split_parts.get(0).cloned().unwrap());
        (header.trim(), objects.trim(), footer.trim())
    }

    /// Serializes the save into a string that, when stored in a `.txt` file can be read by picoCAD.
    pub fn to_string(&self) -> String {
        let mut s: String = String::new();

        // header
        s.push_str(format!("{};{};{};{};{}\n",
                           &self.header.identifier,
                           &self.header.name,
                           &self.header.zoom,
                           &self.header.bg_color.to_i32(),
                           &self.header.alpha_color.to_i32()
        ).as_str());

        // objects
        s.push_str("{\n");
        for object in &self.objects {
            s.push_str(object.serialize().as_str());
            s.push(',');
        }
        s = match s.strip_suffix(',') {
            Some(str) => { str }
            None => { "" }
        }.to_string();
        s.push_str("\n}");

        // footer
        s.push_str("%\n");
        s.push_str(&self.footer);

        s
    }
}

impl From<String> for PicoSave {
    fn from(s: String) -> Self {
        let header: PicoHeader;
        let mut objects: Vec<PicoObject> = vec![];
        let footer: String;

        let (header_string, objects_string, footer_string): (&str, &str, &str) = PicoSave::split_save_string(s.as_str());

        header = PicoHeader::from(header_string);

        let lua = Lua::new();
        lua.context(|ctx| {
            let table: Table = ctx.load(objects_string).eval().expect("Failed loading lua table");
            for object_table in table.sequence_values::<Table>() {
                objects.push(PicoObject::from(object_table.expect("Failed to parse object")));
            }
        });

        footer = footer_string.to_string();

        PicoSave {
            header,
            objects,
            footer
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::{fs, env};

    #[test]
    #[ignore]
    fn parse_pico_save() {
        let save = PicoSave::from(fs::read_to_string(format!("{}plane.txt", env::var("picocad_path").unwrap())).expect("Failed to load File"));
        println!("{:#?}", save);
    }

    #[test]
    #[ignore]
    fn serialize_pico_save() {
        let save = PicoSave::from(fs::read_to_string(format!("{}plane.txt", env::var("picocad_path").unwrap())).expect("Failed to load File"));
        println!("{}", save.to_string());
    }
}
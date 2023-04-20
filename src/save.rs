use crate::assets::{PicoMesh, PicoHeader, Serialize, PicoFooter};
use rlua::{Lua, Table};

/// Represents a picoCAD savefile and all its contents.
#[derive(Debug, PartialEq)]
pub struct PicoSave {
    pub header: PicoHeader,
    pub meshes: Vec<PicoMesh>,
    pub footer: PicoFooter,
}

impl PicoSave {
    /// Splits a savefile represented as a string into header, objects and footer.
    fn split_save_string(s: &str) -> (&str, &str, &str) {
        let split_parts: Vec<&str> = s.splitn(2, '\n').collect();
        let (header, meshes_and_footer): (&str, &str) = (split_parts.get(0).cloned().unwrap(), split_parts.get(1).cloned().unwrap());
        let split_parts: Vec<&str> = meshes_and_footer.rsplitn(2, '%').collect();
        let (meshes, footer): (&str, &str) = (split_parts.get(1).cloned().unwrap(), split_parts.get(0).cloned().unwrap());
        (header.trim(), meshes.trim(), footer.trim())
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
        for mesh in &self.meshes {
            s.push_str(mesh.serialize().as_str());
            s.push(',');
        }
        s = match s.strip_suffix(',') {
            Some(str) => { str }
            None => { "" }
        }.to_string();
        s.push_str("\n}");

        // footer
        s.push_str("%\n");
        s.push_str(&self.footer.serialize());

        s
    }
}

impl From<String> for PicoSave {
    fn from(s: String) -> Self {
        let header: PicoHeader;
        let mut meshes: Vec<PicoMesh> = vec![];
        let footer: PicoFooter;

        let (header_string, mesh_string, footer_string): (&str, &str, &str) = PicoSave::split_save_string(s.as_str());

        header = PicoHeader::from(header_string);

        let lua = Lua::new();
        lua.context(|ctx| {
            let table: Table = ctx.load(mesh_string).eval().expect("Failed loading lua table");
            for mesh in table.sequence_values::<Table>() {
                meshes.push(PicoMesh::from(mesh.expect("Failed to parse object")));
            }
        });

        footer = PicoFooter::from(footer_string);

        PicoSave {
            header,
            meshes,
            footer
        }
    }
}

impl From<&str> for PicoSave {
    fn from(s: &str) -> Self {
        PicoSave::from(s.to_string())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::{fs, env};

    // This test requires you to set a env variable called 'picocad_path' as the path to the folder
    // where picoCAD saves file on your system and have a project file called 'plane.txt' there.
    #[test]
    fn parse_pico_save() {
        let save = PicoSave::from(fs::read_to_string(format!("{}plane.txt", env::var("picocad_path").unwrap())).expect("Failed to load File"));
        println!("{:#?}", save);
    }

    #[test]
    fn serialize_pico_save() {
        assert_eq!(
            PicoSave::from(fs::read_to_string(format!("{}plane.txt", env::var("picocad_path").unwrap())).expect("Failed to load File")),
            PicoSave::from(fs::read_to_string(format!("{}plane.txt", env::var("picocad_path").unwrap())).expect("Failed to load File").as_str()),
        )
    }
}
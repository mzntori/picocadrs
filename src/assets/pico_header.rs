use crate::assets::{PicoColor, Serialize};

/// Represents the header of a picoCAD savefile.
#[derive(Debug, PartialEq)]
pub struct PicoHeader {
    pub identifier: String,
    pub name: String,
    pub zoom: i32,
    pub bg_color: PicoColor,
    pub alpha_color: PicoColor,
}

impl Default for PicoHeader {
    fn default() -> Self {
        PicoHeader {
            identifier: "picocad".to_string(),
            name: "unnamed_save".to_string(),
            zoom: 16,
            bg_color: PicoColor::Black,
            alpha_color: PicoColor::DarkBlue,
        }
    }
}

impl From<&str> for PicoHeader {
    fn from(s: &str) -> Self {
        // split header into important data
        let header_data: Vec<&str> = s.trim().split(';').collect();

        Self {
            identifier: header_data.get(0).unwrap().to_string(),
            name: header_data.get(1).unwrap().to_string(),
            zoom: header_data.get(2).unwrap().parse::<i32>().unwrap(),
            bg_color: PicoColor::from(header_data.get(3).unwrap().parse::<i32>().unwrap()),
            alpha_color: PicoColor::from(header_data.get(4).unwrap().parse::<i32>().unwrap()),
        }
    }
}

impl From<String> for PicoHeader {
    fn from(s: String) -> Self {
        PicoHeader::from(s.as_str())
    }
}

impl Serialize for PicoHeader {
    fn serialize(&self) -> String {
        format!("{};{};{};{};{}\n",
                &self.identifier,
                &self.name,
                &self.zoom,
                &self.bg_color.to_i32(),
                &self.alpha_color.to_i32()
        )
    }
}
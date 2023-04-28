use crate::assets::Serialize;

#[derive(Debug, Clone, PartialEq)]
pub struct PicoFooter {
    raw: String,
}

impl Default for PicoFooter {
    fn default() -> Self {
        let mut raw = String::new();

        for _ in 0..120 {
            raw.push_str("00000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000\r\n")
        }

        Self { raw }
    }
}

impl Serialize for PicoFooter {
    fn serialize(&self) -> String {
        self.raw.to_string()
    }
}

impl From<String> for PicoFooter {
    fn from(s: String) -> Self {
        Self { raw: s }
    }
}

impl From<&str> for PicoFooter {
    fn from(s: &str) -> Self {
        PicoFooter::from(s.to_string())
    }
}

//! For the footer of a picoCAD project.
//!
//! <br/>
//!
//! This part of the file saves the texture that is used for uv-mapping.
//! It consists of 120 lines of 128 characters each.
//! Each character is a hex digit and indicates the color at this position.
//! For more information on which characters stands for what color, look at
//! [this](https://pico-8.fandom.com/wiki/Palette#0..15:_Official_base_colors) table.
//!
//! The first character in the first line is at `u=0, v=0`, where `u` extends to the right and
//! `v` downwards.
//! Internally coordinates are stored as float numbers where `0 - 16` are the outer
//! borders for `u` and `0 - 15` for `v`.
//! Any numbers above or below will still be mapped appropriately, but will not return good results
//! in most cases but are not disallowed by picoCAD.

use crate::{assets::color::Color, error::PicoParseError};
use std::fmt::{Display, Formatter};
use std::str::FromStr;

/// Represents the bottom of a picoCAD file.
///
/// <br/>
///
/// This part of the file saves the texture that is used for uv-mapping.
/// It consists of 120 lines of 128 characters each.
///
/// The first character in the first line is at `u=0, v=0`, where `u` extends to the right and
/// `v` downwards.
/// Internally coordinates are stored as float numbers where `0 - 16` are the outer
/// borders for `u` and `0 - 15` for `v`.
/// Any numbers above or below will still be mapped appropriately, but will not return good results
/// in most cases but are not disallowed by this struct.
///
/// <br/>
///
/// This means that the color at `u=1, v=0.25` is represented by the 9th character in the 3rd line.
/// Since indexing by float numbers can be a bit annoying at times this struct has APIs for access
/// via floats and whole numbers.
#[derive(Debug, Clone, Eq, PartialEq)]
pub struct Footer {
    data: Vec<Color>,
}

impl Footer {
    /// Length the private `data` field should have, and the amount of pixels the texture has.
    ///
    /// `120 * 128 = 15360`.
    const DATA_LENGHT: usize = 15360;

    /// Checks if every pixel in the texture has the same color.
    ///
    /// # Example
    ///
    /// ```
    /// use picocadrs::assets::footer::Footer;
    ///
    /// let footer = Footer::default();
    /// assert!(footer.is_solid());
    /// ```
    pub fn is_solid(&self) -> bool {
        let comp = self.data[0];

        for pixel in self.data.iter() {
            if pixel != &comp {
                return false;
            }
        }

        return true;
    }
}

impl Default for Footer {
    /// Creates an empty Footer.
    /// The texture is fully black.
    ///
    /// # Example
    ///
    /// ```
    /// use picocadrs::assets::footer::Footer;
    ///
    /// let footer = Footer::default();
    /// assert!(footer.is_solid());
    /// ```
    fn default() -> Self {
        Footer {
            data: vec![Color::Black; Footer::DATA_LENGHT],
        }
    }
}

impl Display for Footer {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        let mut chars: String = self.data.iter().map(|c| c.as_char()).collect();

        for line in (1..=120).rev() {
            chars.insert(line * 128, '\n');
        }

        write!(f, "{}", chars)
    }
}

impl FromStr for Footer {
    type Err = PicoParseError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let data: Vec<Color> = s
            .chars()
            .filter_map(|c| match c {
                ' ' | '\n' => None,
                _ => Some(Color::from(c)),
            })
            .collect();

        if data.len() != Footer::DATA_LENGHT {
            return Err(PicoParseError::FooterLength(data.len()));
        }

        Ok(Footer { data })
    }
}

#[cfg(test)]
pub mod tests {
    use super::*;

    #[test]
    fn footer_parse() {
        let footer = TEST_FOOTER.parse::<Footer>().unwrap();
    }

    #[test]
    fn footer_serialize() {
        let footer = TEST_FOOTER.parse::<Footer>().unwrap();
        assert_eq!(TEST_FOOTER, footer.to_string());
    }

    #[test]
    fn footer_default() {
        let footer1 = TEST_FOOTER.parse::<Footer>().unwrap();
        let footer2 = Footer::default();

        assert_ne!(footer1, footer2);
        assert!(footer2.is_solid());
        assert!(!footer1.is_solid());
    }

    #[test]
    fn footer_is_solid() {
        let footer1 = TEST_FOOTER.parse::<Footer>().unwrap();
        let footer2 = Footer::default();

        assert_ne!(footer1, footer2);
        assert!(footer2.is_solid());
        assert!(!footer1.is_solid());
    }

    const TEST_FOOTER: &str = r#"00000000eeee8888eeee8888aaaa9999aaaa9999bbbb3333bbbb3333ccccddddccccddddffffeeeeffffeeee7777666677776666555566665555666600000000
00000000eeee8888eeee8888aaaa9999aaaa9999bbbb3333bbbb3333ccccddddccccddddffffeeeeffffeeee7777666677776666555566665555666600000000
00000000eeee8888eeee8888aaaa9999aaaa9999bbbb3333bbbb3333ccccddddccccddddffffeeeeffffeeee7777666677776666555566665555666600000000
00000000eeee8888eeee8888aaaa9999aaaa9999bbbb3333bbbb3333ccccddddccccddddffffeeeeffffeeee7777666677776666555566665555666600000000
000000008888eeee8888eeee9999aaaa9999aaaa3333bbbb3333bbbbddddccccddddcccceeeeffffeeeeffff6666777766667777666655556666555500000000
000000008888eeee8888eeee9999aaaa9999aaaa3333bbbb3333bbbbddddccccddddcccceeeeffffeeeeffff6666777766667777666655556666555500000000
000000008888eeee8888eeee9999aaaa9999aaaa3333bbbb3333bbbbddddccccddddcccceeeeffffeeeeffff6666777766667777666655556666555500000000
000000008888eeee8888eeee9999aaaa9999aaaa3333bbbb3333bbbbddddccccddddcccceeeeffffeeeeffff6666777766667777666655556666555500000000
00000000eeee8888eeee8888aaaa9999aaaa9999bbbb3333bbbb3333cccc4444ccccddddffffeeeeffffeeee7777666677776666555566665555666600000000
00000000eeee8888eeee8888aaaa9999aaaa9999bbbb3333bbbb3333cccc4444ccccddddffffeeeeffffeeee7777666677776666555566665555666600000000
00000000eeee8888eeee8888aaaa9999aaaa9999bbbb3333bbbb3333cccc4444ccccddddffffeeeeffffeeee7777666677776666555566665555666600000000
00000000eeee8888eeee8888aaaa9999aaaa9999bbbb3333bbbb3333cccc4444ccccddddffffeeeeffffeeee7777666677776666555566665555666600000000
000000008888eeee8888eeee9999aaaa9999aaaa3333bbbb3333bbbb2222cccc1111cccceeeeffffeeeeffff6666777766667777666655556666555500000000
000000008888eeee8888eeee9999aaaa9999aaaa3333bbbb3333bbbb2222cccc1111cccceeeeffffeeeeffff6666777766667777666655556666555500000000
000000008888eeee8888eeee9999aaaa9999aaaa3333bbbb3333bbbb2222cccc1111cccceeeeffffeeeeffff6666777766667777666655556666555500000000
000000008888eeee8888eeee9999aaaa9999aaaa3333bbbb3333bbbb2222cccc1111cccceeeeffffeeeeffff6666777766667777666655556666555500000000
00000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000
00000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000
00000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000
00000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000
00000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000
00000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000
00000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000
00000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000
00000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000
00000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000
00000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000
00000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000
00000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000
00000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000
00000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000
00000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000
00000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000
00000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000
00000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000
00000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000
00000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000
00000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000
00000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000
00000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000
00000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000
00000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000
00000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000
00000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000
00000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000
00000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000
00000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000
00000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000
00000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000
00000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000
00000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000
00000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000
00000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000
00000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000
00000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000
00000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000
00000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000
00000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000
00000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000
00000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000
00000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000
00000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000
00000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000
00000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000
00000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000
00000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000
00000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000
00000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000
00000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000
00000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000
00000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000
00000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000
00000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000
00000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000
00000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000
00000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000
00000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000
00000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000
00000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000
00000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000
00000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000
00000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000
00000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000
00000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000
00000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000
00000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000
00000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000
00000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000
00000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000
00000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000
00000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000
00000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000
00000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000
00000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000
00000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000
00000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000
00000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000
00000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000
00000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000
00000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000
00000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000
00000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000
00000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000
00000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000
00000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000
00000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000
00000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000
00000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000
00000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000
00000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000
00000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000
00000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000
00000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000
00000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000
00000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000
00000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000
00000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000
00000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000
00000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000
00000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000
"#;
}

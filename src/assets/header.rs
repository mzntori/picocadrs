//! For dealing with the header of a picoCAD project.
//!
//! <br/>
//!
//! A header consists of 5 parts seperated by semicolons and looks something like this:
//!
//! `picocad;my_project;16;1;0`
//!
//! In order the different parts have the following function:
//! - _identifier_: In order for a text-file to be recognized by picoCAD as a project the header has to start
//! with `"picocad"`.
//! - _project name_: While normally this is the same as the file-name it can actually differ and is what picoCAD
//! displays at the bottom of the window.
//! In this example `"my_project"`.
//! - _zoom_: Level of zoom at last save.
//! Changing this doesn't actually do anything since picoCAD will just use the zoom you currently
//! have when loading a project.
//! In this example `16`.
//! - _background color_: The color of the background in a project.
//! In this example `1` which represents dark-blue.
//! - _alpha color_: The color that will be transparent when uv-mapped onto a face.
//! In this example `0` which represents black.

use super::Color;
use crate::error::PicoError;
use std::{fmt::Display, str::FromStr};

/// Represents the header of a picoCAD project.
/// A header consists of 5 parts seperated by semicolons and looks something like this:
///
/// `picocad;my_project;16;1;0`
///
/// In order the different parts have the following function:
/// - _identifier_: In order for a text-file to be recognized by picoCAD as a project the header has to start
/// with `"picocad"`.
/// - _project name_: While normally this is the same as the file-name it can actually differ and is what picoCAD
/// displays at the bottom of the window.
/// In this example `"my_project"`.
/// - _zoom_: Level of zoom at last save.
/// Changing this doesn't actually do anything since picoCAD will just use the zoom you currently
/// have when loading a project.
/// In this example `16`.
/// - _background color_: The color of the background in a project.
/// In this example `1` which represents dark-blue.
/// - _alpha color_: The color that will be transparent when uv-mapped onto a face.
/// In this example `0` which represents black.
///
/// # Examples
///
/// A header can be parsed, then modified and turned back into a string again.
/// ```
/// use picocadrs::assets::{Header, Color};
///
/// let header = "picocad;unnamed;16;1;4".parse::<Header>().unwrap();
///
/// assert_eq!(header.identifier(), "picocad");
/// assert_eq!(header.name, "unnamed");
/// assert_eq!(header.zoom, 16);
/// assert_eq!(header.background, Color::DarkBlue);
/// assert_eq!(header.alpha, Color::Brown);
///
/// assert_eq!("picocad;unnamed;16;1;4", header.to_string())
/// ```
#[derive(Debug, Clone, Eq, PartialEq)]
pub struct Header {
    identifier: String,
    pub zoom: u8,
    pub name: String,
    pub background: Color,
    pub alpha: Color,
}

impl Header {
    /// Returns the identifier of the header as a [`String`].
    /// In normal use this will always be `"picocad"`.
    ///
    /// # Examples
    ///
    /// ```
    /// use picocadrs::assets::Header;
    ///
    /// assert_eq!(
    ///     "picocad;unnamed;16;1;4".parse::<Header>().unwrap().identifier(),
    ///     "picocad"
    /// );
    /// ```
    pub fn identifier(&self) -> String {
        self.identifier.clone()
    }
}

impl Default for Header {
    /// Creates the header of a picoCAD project the same way picoCAD would when creating a new one.
    /// Project name is always `"unnamed"` however, since this does not count the projects in the
    /// project folder.
    ///
    /// # Examples
    ///
    /// ```
    /// use picocadrs::assets::Header;
    ///
    /// assert_eq!("picocad;unnamed;16;1;0", Header::default().to_string());
    /// ```
    fn default() -> Self {
        Header {
            identifier: "picocad".to_string(),
            zoom: 16,
            name: "unnamed".to_string(),
            background: Color::DarkBlue,
            alpha: Color::Black,
        }
    }
}

impl FromStr for Header {
    type Err = PicoError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let fields: Vec<&str> = s.trim().splitn(5, ';').collect();

        if fields.len() != 5 {
            return Err(PicoError::HeaderLength(fields.len()));
        } else if *fields.first().unwrap() != "picocad" {
            return Err(PicoError::Identifier);
        }

        let identifier: String = fields[0].to_string();
        let name: String = fields[1].to_string();

        let zoom: u8 = if let Ok(value) = fields[2].parse::<u8>() {
            value
        } else {
            return Err(PicoError::HeaderField("zoom".to_string()));
        };

        let background: Color = if let Ok(value) = fields[3].parse::<i32>() {
            Color::from(value)
        } else {
            return Err(PicoError::HeaderField("background".to_string()));
        };

        let alpha: Color = if let Ok(value) = fields[4].parse::<i32>() {
            Color::from(value)
        } else {
            return Err(PicoError::HeaderField("alpha".to_string()));
        };

        let header = Header {
            identifier,
            name,
            zoom,
            background,
            alpha,
        };

        Ok(header)
    }
}

impl Display for Header {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "picocad;{};{};{};{}",
            self.name,
            self.zoom,
            self.background.as_i32(),
            self.alpha.as_i32()
        )
    }
}

#[cfg(test)]
pub mod tests {
    use super::*;

    #[test]
    fn header_display() {
        assert_eq!("picocad;unnamed;16;1;0", Header::default().to_string());
    }

    #[test]
    fn header_parsing() {
        let header = "picocad;unnamed;16;1;4".parse::<Header>().unwrap();

        assert_eq!(header.identifier, "picocad");
        assert_eq!(header.name, "unnamed");
        assert_eq!(header.zoom, 16);
        assert_eq!(header.background, Color::DarkBlue);
        assert_eq!(header.alpha, Color::Brown);

        assert_eq!("picocad;unnamed;16;1;4", header.to_string())
    }
}

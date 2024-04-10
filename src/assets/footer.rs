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

use crate::{
    assets::{color::Color, point::Point2D},
    error::PicoError,
    point,
};
use std::fmt::{Display, Formatter};
use std::ops::{Index, IndexMut};
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

    /// Get a reference to the color at the given index in [`usize`].
    /// This uses the actual pixel position in the texture.
    /// `0, 0` is located in the top left corner.
    ///
    /// Returns [`None`] if coordinates are out of bounds.
    ///
    /// `u` is out of bounds if `>= 128`.
    ///
    /// `v` is out of bounds if `>= 120`.
    ///
    /// <br/>
    ///
    /// Currently, no `get_mut` method as [`Color`] does not have any methods that take a mutable
    /// reference of self.
    ///
    /// # Example
    ///
    /// ```
    /// use picocadrs::assets::{color::Color, point::Point2D, footer::Footer};
    /// use picocadrs::point;
    ///
    /// let mut footer = Footer::default();
    ///
    /// footer.set(point!(3, 2), Color::Lavender).expect("uv index out of range");
    ///
    /// assert_eq!(
    ///     footer.get(point!(3, 2)).unwrap(),
    ///     &Color::Lavender
    /// );
    /// ```
    pub fn get(&self, coords: Point2D<usize>) -> Option<&Color> {
        return if coords.u > 127 || coords.v > 119 {
            None
        } else {
            Some(self.index(coords))
        };
    }

    /// Sets the color at the given index in [`usize`].
    /// This uses the actual pixel position in the texture.
    /// `0, 0` is located in the top left corner.
    ///
    /// Returns a [`PicoError::IndexUSIZE`] if index is out of bounds.
    ///
    /// `u` is out of bounds if `>= 128`.
    ///
    /// `v` is out of bounds if `>= 120`.
    ///
    /// # Example
    ///
    /// ```
    /// use picocadrs::assets::{color::Color, point::Point2D, footer::Footer};
    /// use picocadrs::point;
    ///
    /// let mut footer = Footer::default();
    ///
    /// assert_eq!(
    ///     footer.get(point!(3, 2)).unwrap(),
    ///     &Color::Black
    /// );
    ///
    /// footer.set(point!(3, 2), Color::Lavender).expect("uv index out of range");
    ///
    /// assert_eq!(
    ///     footer.get(point!(3, 2)).unwrap(),
    ///     &Color::Lavender
    /// );
    /// ```
    pub fn set(&mut self, coords: Point2D<usize>, value: Color) -> Result<(), PicoError> {
        return if coords.u > 127 || coords.v > 119 {
            Err(PicoError::IndexUSIZE(coords, point!(128, 120)))
        } else {
            self[coords] = value;
            Ok(())
        };
    }

    /// Reads the color at the given uv coordinates and returns a copy of the color
    /// at the given position.
    /// If you want to index with whole numbers representing pixels consider using [`get`](Footer::get) instead.
    ///
    /// `0.0, 0.0` is located in the top left corner.
    /// Returns [`Color::Invalid`] if coordinates are outside the texture.
    ///
    /// `u` is out of bounds if `-0.0625 > u` or `u >= 15.9375`.
    ///
    /// `v` is out of bounds if `-0.0625 > v` or `v >= 14.9375`.
    ///
    /// This means each pixel "owns" a region of `0.125 x 0.125`.
    ///
    /// # Example
    ///
    /// ```
    /// use picocadrs::assets::{color::Color, point::Point2D, footer::Footer};
    /// use picocadrs::point;
    ///
    /// let mut footer = Footer::default();
    ///
    /// footer.set(point!(6, 4), Color::Lavender).expect("uv index out of range");
    ///
    /// assert_eq!(footer.read(point!(0.75, 0.5)), Color::Lavender);
    /// assert_eq!(footer.read(point!(-0.75, 0.5)), Color::Invalid);
    /// assert_eq!(footer.read(point!(15.95, 0.5)), Color::Invalid);
    /// ```
    pub fn read(&self, coords: Point2D<f64>) -> Color {
        return if -0.0625 > coords.u
            || coords.u >= 15.9375
            || -0.0625 > coords.v
            || coords.v >= 14.9375
        {
            Color::Invalid
        } else {
            self[point!(
                (coords.u * 8.0).round() as usize,
                (coords.v * 8.0).round() as usize
            )]
        };
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
    type Err = PicoError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let data: Vec<Color> = s
            .chars()
            .filter_map(|c| match c {
                ' ' | '\n' => None,
                _ => Some(Color::from(c)),
            })
            .collect();

        if data.len() != Footer::DATA_LENGHT {
            return Err(PicoError::FooterLength(data.len()));
        }

        Ok(Footer { data })
    }
}

impl Index<Point2D<usize>> for Footer {
    type Output = Color;

    /// Panics if `u >= 128` or `v >= 120`.
    ///
    /// # Example
    ///
    /// ```
    /// use picocadrs::assets::{footer::Footer, color::Color, point::Point2D};
    /// use picocadrs::point;
    ///
    /// let footer = Footer::default();
    ///
    /// assert_eq!(footer[point!(0, 0)], Color::Black);
    /// assert_eq!(footer[point!(127, 119)], Color::Black);
    /// // assert_eq!(footer[point!(127, 120)], Color::Black); These panic
    /// // assert_eq!(footer[point!(128, 119)], Color::Black);
    /// ```
    fn index(&self, index: Point2D<usize>) -> &Self::Output {
        if index.u > 127 || index.v > 119 {
            panic!("index out of range");
        }

        let data_index = index.u + index.v * 128;

        self.data.get(data_index).unwrap()
    }
}

impl IndexMut<Point2D<usize>> for Footer {
    /// Panics if `u >= 128` or `v >= 120`.
    ///
    /// # Example
    ///
    /// ```
    /// use picocadrs::assets::{footer::Footer, color::Color, point::Point2D};
    /// use picocadrs::point;
    ///
    /// let footer = Footer::default();
    ///
    /// assert_eq!(footer[point!(0, 0)], Color::Black);
    /// assert_eq!(footer[point!(127, 119)], Color::Black);
    /// // assert_eq!(footer[point!(127, 120)], Color::Black); These panic
    /// // assert_eq!(footer[point!(128, 119)], Color::Black);
    /// ```
    fn index_mut(&mut self, index: Point2D<usize>) -> &mut Self::Output {
        if index.u > 127 || index.v > 119 {
            panic!("index out of range");
        }

        let data_index = index.u + index.v * 128;

        self.data.get_mut(data_index).unwrap()
    }
}

#[cfg(test)]
pub mod tests {
    use super::*;
    use crate::point;

    #[test]
    fn footer_parse() {
        let _footer = TEST_FOOTER.parse::<Footer>().unwrap();
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

    #[test]
    fn footer_index() {
        let footer = TEST_FOOTER.parse::<Footer>().unwrap();

        assert_eq!(footer[point!(0, 0)], Color::Black);
        assert_eq!(footer[point!(13, 4)], Color::from('e'));
        assert_eq!(footer[point!(127, 119)], Color::Black);
        // assert_eq!(footer[point!(127, 120)], Color::Black); These panic
        // assert_eq!(footer[point!(128, 119)], Color::Black);
    }

    #[test]
    fn footer_get() {
        let footer = TEST_FOOTER.parse::<Footer>().unwrap();

        assert_eq!(footer.get(point!(13, 4)).unwrap(), &Color::from('e'));
        assert_eq!(footer.get(point!(0, 0)).unwrap(), &Color::Black);
        assert_eq!(footer.get(point!(128, 1)), None);
        assert_eq!(footer.get(point!(1, 120)), None);
    }

    #[test]
    fn footer_set() {
        let mut footer = TEST_FOOTER.parse::<Footer>().unwrap();

        assert_eq!(footer.get(point!(3, 2)).unwrap(), &Color::Black);

        footer
            .set(point!(3, 2), Color::Lavender)
            .expect("index out of range");
        assert_eq!(footer.get(point!(3, 2)).unwrap(), &Color::Lavender);

        assert!(footer.set(point!(128, 0), Color::Lavender).is_err());
    }

    #[test]
    fn footer_read() {
        let mut footer = TEST_FOOTER.parse::<Footer>().unwrap();

        assert_eq!(footer.read(point!(1.25, 0.75)), Color::from('8'));
        assert_eq!(footer.read(point!(-0.75, 0.5)), Color::Invalid);
        assert_eq!(footer.read(point!(15.95, 0.5)), Color::Invalid);
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

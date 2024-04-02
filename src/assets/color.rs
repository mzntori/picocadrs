//! For dealing with colors in picoCAD.
//!
//! Heavily relies on the pico-8 [color palette](https://pico-8.fandom.com/wiki/Palette).

/// Represents a color in the pico-8 color-theme.
/// picoCAD will only display the 16 official base colors.
///
/// More information on pico8 colors can be found here: https://pico-8.fandom.com/wiki/Palette.
#[derive(Debug, Copy, Clone, Eq, PartialEq)]
pub enum Color {
    Invalid,
    Black,
    DarkBlue,
    DarkPurple,
    DarkGreen,
    Brown,
    DarkGrey,
    LightGrey,
    White,
    Red,
    Orange,
    Yellow,
    Green,
    Blue,
    Lavender,
    Pink,
    LightPeach,
}


impl Color {
    /// Returns the color as a `i32`.
    ///
    /// This representation is used in the header of the project file to represent the background- and alpha color.
    ///
    /// It is also used to represent the color of a face.
    ///
    /// If `self` is `Invalid` returns `0` which is equal to black.
    ///
    /// # Examples
    ///
    /// ```
    /// use picocadrs::assets::color::Color;
    ///
    /// assert_eq!(Color::Lavender.as_i32(), 13);
    /// assert_eq!(Color::LightGrey.as_i32(), 6);
    /// assert_eq!(Color::Invalid.as_i32(), 0);
    /// ```
    pub fn as_i32(&self) -> i32 {
        match self {
            Self::Invalid => 0,
            Self::Black => 0,
            Self::DarkBlue => 1,
            Self::DarkPurple => 2,
            Self::DarkGreen => 3,
            Self::Brown => 4,
            Self::DarkGrey => 5,
            Self::LightGrey => 6,
            Self::White => 7,
            Self::Red => 8,
            Self::Orange => 9,
            Self::Yellow => 10,
            Self::Green => 11,
            Self::Blue => 12,
            Self::Lavender => 13,
            Self::Pink => 14,
            Self::LightPeach => 15,
        }
    }

    /// Returns the color as it's hex code.
    ///
    /// This will return the code with upper case letters and no # at the start.
    ///
    /// If `self` is `Invalid` returns `"000000"` which is equal to black.
    ///
    /// # Examples
    ///
    /// ```
    /// use picocadrs::assets::color::Color;
    ///
    /// assert_eq!(Color::Lavender.as_hex(), "83769C".to_string());
    /// assert_eq!(Color::LightGrey.as_hex(), "C2C3C7".to_string());
    /// assert_eq!(Color::Invalid.as_hex(), "000000".to_string());
    /// ```
    pub fn as_hex(&self) -> String {
        match self {
            Color::Invalid => { "000000" }
            Color::Black => { "000000" }
            Color::DarkBlue => { "1D2B53" }
            Color::DarkPurple => { "7E2553" }
            Color::DarkGreen => { "008751" }
            Color::Brown => { "AB5236" }
            Color::DarkGrey => { "5F574F" }
            Color::LightGrey => { "C2C3C7" }
            Color::White => { "FFF1E8" }
            Color::Red => { "FF004D" }
            Color::Orange => { "FFA300" }
            Color::Yellow => { "FFEC27" }
            Color::Green => { "00E436" }
            Color::Blue => { "29ADFF" }
            Color::Lavender => { "83769C" }
            Color::Pink => { "FF77A8" }
            Color::LightPeach => { "FFCCAA" }
        }.to_string()
    }

    /// Returns the color as a rgb triplet.
    ///
    /// The rgb values are mapped like this: `(r, g, b)`.
    ///
    /// If `self` is `Invalid` returns `(0, 0, 0)` which is equal to black.
    ///
    /// # Examples
    ///
    /// ```
    /// use picocadrs::assets::color::Color;
    ///
    /// assert_eq!(Color::Lavender.as_rgb(), (131, 118, 156));
    /// assert_eq!(Color::LightGrey.as_rgb(), (194, 195, 199));
    /// assert_eq!(Color::Invalid.as_rgb(), (0, 0, 0));
    /// ```
    pub fn as_rgb(&self) -> (u8, u8, u8) {
        match self {
            Color::Invalid => { (0, 0, 0) }
            Color::Black => { (0, 0, 0) }
            Color::DarkBlue => { (29, 43, 83) }
            Color::DarkPurple => { (126, 37, 83) }
            Color::DarkGreen => { (0, 135, 81) }
            Color::Brown => { (171, 82, 54) }
            Color::DarkGrey => { (95, 87, 79) }
            Color::LightGrey => { (194, 195, 199) }
            Color::White => { (255, 241, 232) }
            Color::Red => { (255, 0, 77) }
            Color::Orange => { (255, 163, 0) }
            Color::Yellow => { (255, 236, 39) }
            Color::Green => { (0, 228, 54) }
            Color::Blue => { (41, 173, 255) }
            Color::Lavender => { (131, 118, 156) }
            Color::Pink => { (255, 119, 168) }
            Color::LightPeach => { (255, 204, 170) }
        }
    }

    /// Returns the color represented as a `char`.
    ///
    /// This is required for the UV map which stores colors as the hex representation of their
    /// integer representation. For example lavender is represented by `13` in integer form, so it
    /// is represented by `'d'` as a character.
    ///
    /// If `self` is `Invalid` returns `'0'` which is equal to black.
    ///
    /// # Example
    ///
    /// ```
    /// use picocadrs::assets::color::Color;
    ///
    /// assert_eq!(Color::Lavender.as_char(), 'd');
    /// assert_eq!(Color::LightGrey.as_char(), '6');
    /// assert_eq!(Color::Invalid.as_char(), '0');
    /// ```
    pub fn as_char(&self) -> char {
        match self {
            Color::Invalid => { '0' }
            Color::Black => { '0' }
            Color::DarkBlue => { '1' }
            Color::DarkPurple => { '2' }
            Color::DarkGreen => { '3' }
            Color::Brown => { '4' }
            Color::DarkGrey => { '5' }
            Color::LightGrey => { '6' }
            Color::White => { '7' }
            Color::Red => { '8' }
            Color::Orange => { '9' }
            Color::Yellow => { 'a' }
            Color::Green => { 'b' }
            Color::Blue => { 'c' }
            Color::Lavender => { 'd' }
            Color::Pink => { 'e' }
            Color::LightPeach => { 'f' }
        }
    }


    /// Returns the color picoCAD would use to replace `self` with if it was shadowed.
    ///
    /// Shadow of `Invalid` is still `Invalid`.
    ///
    /// # Example
    ///
    /// ```
    /// use picocadrs::assets::color::Color;
    ///
    /// // Oranges shadow is dark-purple
    /// assert_eq!(Color::Orange.shadow(), Color::DarkPurple);
    /// ```
    pub fn shadow(&self) -> Self {
        match self {
            Color::Invalid                                  => { Color::Invalid }
            Color::Black | Color::DarkBlue |
            Color::DarkPurple | Color::DarkGrey             => { Color::Black }
            Color::DarkGreen | Color::Brown | Color::Red |
            Color::Green | Color::Lavender                  => { Color::DarkBlue }
            Color::LightGrey | Color::Blue                  => { Color::DarkGrey }
            Color::Orange | Color::Pink                     => { Color::DarkPurple }
            Color::Yellow | Color::LightPeach               => { Color::Brown }
            Color::White                                    => { Color::Lavender }
        }
    }


    /// Returns the color picoCAD would replace `self` with while transitioning to being shadowed.
    ///
    /// Shadow in transition of `Invalid` is still `Invalid`.
    ///
    /// # Example
    ///
    /// ```
    /// use picocadrs::assets::color::Color;
    ///
    /// // Orange transitions like `orange -> brown -> dark-purple`
    /// assert_eq!(Color::Orange.shadow_transition(), Color::Brown);
    /// assert_eq!(Color::Orange.shadow(), Color::DarkPurple);
    /// ```
    pub fn shadow_transition(&self) -> Self {
        match self {
            Color::Invalid                      => { Color::Invalid }
            Color::Black | Color::DarkBlue      => { Color::Black }
            Color::DarkPurple | Color::DarkGrey => { Color::DarkBlue }
            Color::DarkGreen | Color::Lavender  => { Color::DarkGrey }
            Color::Brown | Color::Red           => { Color::DarkPurple }
            Color::LightGrey | Color::Blue      => { Color::Lavender }
            Color::Yellow | Color::LightPeach   => { Color::Orange }
            Color::White                        => { Color::LightGrey }
            Color::Orange                       => { Color::Brown }
            Color::Green                        => { Color::DarkGreen }
            Color::Pink                         => { Color::Red }
        }
    }
}


impl From<char> for Color {
    /// Converts `char` into `Color`.
    ///
    /// Only hex values of 0-f as a `char` are valid, any other value will be turned into an `Invalid` color.
    ///
    /// # Example
    ///
    /// ```
    /// use picocadrs::assets::color::Color;
    ///
    /// assert_eq!(Color::Lavender, Color::from('d'));
    /// assert_eq!(Color::LightGrey, Color::from('6'));
    /// assert_eq!(Color::Invalid, Color::from('A'));
    /// ```
    fn from(value: char) -> Self {
        match value {
            '0' => Color::Black,
            '1' => Color::DarkBlue,
            '2' => Color::DarkPurple,
            '3' => Color::DarkGreen,
            '4' => Color::Brown,
            '5' => Color::DarkGrey,
            '6' => Color::LightGrey,
            '7' => Color::White,
            '8' => Color::Red,
            '9' => Color::Orange,
            'a' => Color::Yellow,
            'b' => Color::Green,
            'c' => Color::Blue,
            'd' => Color::Lavender,
            'e' => Color::Pink,
            'f' => Color::LightPeach,
            _ => Color::Invalid
        }
    }
}


impl From<i32> for Color {
    /// Converts `i32` into `Color`.
    ///
    /// Only `i32` values of 0-15 are valid, any other value will be turned into an `Invalid` color.
    ///
    /// # Example
    ///
    /// ```
    /// use picocadrs::assets::color::Color;
    ///
    /// assert_eq!(Color::Lavender, Color::from(13));
    /// assert_eq!(Color::Invalid, Color::from(17));
    /// assert_eq!(Color::Invalid, Color::from(-2));
    /// ```
    fn from(value: i32) -> Self {
        match value.into() {
            0 => Color::Black,
            1 => Color::DarkBlue,
            2 => Color::DarkPurple,
            3 => Color::DarkGreen,
            4 => Color::Brown,
            5 => Color::DarkGrey,
            6 => Color::LightGrey,
            7 => Color::White,
            8 => Color::Red,
            9 => Color::Orange,
            10 => Color::Yellow,
            11 => Color::Green,
            12 => Color::Blue,
            13 => Color::Lavender,
            14 => Color::Pink,
            15 => Color::LightPeach,
            _ => Color::Invalid
        }
    }
}


impl From<(u8, u8, u8)> for Color {
    /// Converts `(u8, u8, u8)` into `Color`.
    /// The tuple should represent the rgb values of the color mapped `(r, g, b)`.
    ///
    /// Only the rgb values in [this](https://pico-8.fandom.com/wiki/Palette#0..15:_Official_base_colors) table will return valid colors.
    ///
    /// # Example
    ///
    /// ```
    /// use picocadrs::assets::color::Color;
    ///
    /// assert_eq!(Color::Lavender, Color::from((131, 118, 156)));
    /// assert_eq!(Color::LightGrey, Color::from((194, 195, 199)));
    /// assert_eq!(Color::Invalid, Color::from((111, 111, 111)));
    /// ```
    fn from(value: (u8, u8, u8)) -> Self {
        match value {
            (0, 0, 0) => Color::Black,
            (29, 43, 83) => Color::DarkBlue,
            (126, 37, 83) => Color::DarkPurple,
            (0, 135, 81) => Color::DarkGreen,
            (171, 82, 54) => Color::Brown,
            (95, 87, 79) => Color::DarkGrey,
            (194, 195, 199) => Color::LightGrey,
            (255, 241, 232) => Color::White,
            (255, 0, 77) => Color::Red,
            (255, 163, 0) => Color::Orange,
            (255, 236, 39) => Color::Yellow,
            (0, 228, 54) => Color::Green,
            (41, 173, 255) => Color::Blue,
            (131, 118, 156) => Color::Lavender,
            (255, 119, 168) => Color::Pink,
            (255, 204, 170) => Color::LightPeach,
            _ => Color::Invalid
        }
    }
}


#[cfg(test)]
pub mod tests {
    use super::*;

    #[test]
    fn color_as_i32() {
        assert_eq!(Color::Lavender.as_i32(), 13);
        assert_eq!(Color::LightGrey.as_i32(), 6);
        assert_eq!(Color::Invalid.as_i32(), 0);
    }

    #[test]
    fn color_as_hex() {
        assert_eq!(Color::Lavender.as_hex(), "83769C".to_string());
        assert_eq!(Color::LightGrey.as_hex(), "C2C3C7".to_string());
        assert_eq!(Color::Invalid.as_hex(), "000000".to_string());
    }

    #[test]
    fn color_as_rgb() {
        assert_eq!(Color::Lavender.as_rgb(), (131, 118, 156));
        assert_eq!(Color::LightGrey.as_rgb(), (194, 195, 199));
        assert_eq!(Color::Invalid.as_rgb(), (0, 0, 0));
    }

    #[test]
    fn color_as_char() {
        assert_eq!(Color::Lavender.as_char(), 'd');
        assert_eq!(Color::LightGrey.as_char(), '6');
        assert_eq!(Color::Invalid.as_char(), '0');
    }

    #[test]
    fn color_from_i32() {
        assert_eq!(Color::Lavender, Color::from(13));
        assert_eq!(Color::Invalid, Color::from(17));
        assert_eq!(Color::Invalid, Color::from(-2));
    }

    #[test]
    fn color_from_char() {
        assert_eq!(Color::Lavender, Color::from('d'));
        assert_eq!(Color::LightGrey, Color::from('6'));
        assert_eq!(Color::Invalid, Color::from('A'));
    }

    #[test]
    fn color_from_tuple() {
        assert_eq!(Color::Lavender, Color::from((131, 118, 156)));
        assert_eq!(Color::LightGrey, Color::from((194, 195, 199)));
        assert_eq!(Color::Invalid, Color::from((111, 111, 111)));
    }

    #[test]
    fn color_shadows() {
        assert_eq!(Color::Orange.shadow_transition(), Color::Brown);
        assert_eq!(Color::Orange.shadow(), Color::DarkPurple);
    }
}
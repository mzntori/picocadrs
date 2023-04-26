/// Enum that represents colors in the pico-8 color palette.
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum PicoColor {
    None = -1,
    Black = 0,
    DarkBlue = 1,
    DarkPurple = 2,
    DarkGreen = 3,
    Brown = 4,
    DarkGrey = 5,
    LightGrey = 6,
    White = 7,
    Red = 8,
    Orange = 9,
    Yellow = 10,
    Green = 11,
    Blue = 12,
    Lavender = 13,
    Pink = 14,
    LightPeach = 15,
}

impl PicoColor {
    /// Returns the Color represented as an integer between 0 and 15.
    /// Returns -1 if its not a valid color.
    pub fn to_i32(&self) -> i32 {
        return match self {
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
            _ => -1
        };
    }

    pub fn to_char(&self) -> char {
        return match self {
            Self::Black => '0',
            Self::DarkBlue => '1',
            Self::DarkPurple => '2',
            Self::DarkGreen => '3',
            Self::Brown => '4',
            Self::DarkGrey => '5',
            Self::LightGrey => '6',
            Self::White => '7',
            Self::Red => '8',
            Self::Orange => '9',
            Self::Yellow => 'a',
            Self::Green => 'b',
            Self::Blue => 'c',
            Self::Lavender => 'd',
            Self::Pink => 'e',
            Self::LightPeach => 'f',
            _ => ' '
        };
    }
}

impl From<i32> for PicoColor {
    fn from(i: i32) -> Self {
        return match i {
            0 => Self::Black,
            1 => Self::DarkBlue,
            2 => Self::DarkPurple,
            3 => Self::DarkGreen,
            4 => Self::Brown,
            5 => Self::DarkGrey,
            6 => Self::LightGrey,
            7 => Self::White,
            8 => Self::Red,
            9 => Self::Orange,
            10 => Self::Yellow,
            11 => Self::Green,
            12 => Self::Blue,
            13 => Self::Lavender,
            14 => Self::Pink,
            15 => Self::LightPeach,
            _ => Self::None
        };
    }
}

impl From<char> for PicoColor {
    fn from(c: char) -> Self {
        return match c {
            '0' => Self::Black,
            '1' => Self::DarkBlue,
            '2' => Self::DarkPurple,
            '3' => Self::DarkGreen,
            '4' => Self::Brown,
            '5' => Self::DarkGrey,
            '6' => Self::LightGrey,
            '7' => Self::White,
            '8' => Self::Red,
            '9' => Self::Orange,
            'a' => Self::Yellow,
            'b' => Self::Green,
            'c' => Self::Blue,
            'd' => Self::Lavender,
            'e' => Self::Pink,
            'f' => Self::LightPeach,
            _ => Self::None
        };
    }
}

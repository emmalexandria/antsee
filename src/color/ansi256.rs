use std::str::FromStr;

use super::{xterm::XtermColors, Color, ColorValue};

///Ansi256 color value represented by a u8. Can be created from [XtermColors] name with
///[FromStr].
#[derive(Clone, Debug, Default, PartialEq, Copy)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct Ansi256(pub u8);

impl Ansi256 {
    /// Create a new Ansi256 from u8
    pub fn new() -> Self {
        Self(0)
    }

    ///Create a new Ansi256 from XtermColors
    pub fn from(color: XtermColors) -> Self {
        Self(color.ansi256())
    }

    /// Set the u8 color code of the color
    pub fn set_code(&mut self, val: u8) {
        self.0 = val
    }

    /// Set the [XtermColors] of the color
    pub fn set_color(&mut self, color: XtermColors) {
        self.0 = color.ansi256()
    }

    ///Set the [XtermColors] of the color
    pub fn color(mut self, color: XtermColors) -> Self {
        self.set_color(color);
        self
    }

    ///Set the u8 color code of the color
    pub fn code(mut self, code: u8) -> Self {
        self.set_code(code);
        self
    }

    ///Get the [XtermColors] of the color
    pub fn get_color(&self) -> XtermColors {
        XtermColors::get_ansi256(self.0)
    }
}

impl From<XtermColors> for Ansi256 {
    fn from(value: XtermColors) -> Self {
        Self(value.ansi256())
    }
}

impl ColorValue for Ansi256 {}

///Possible errors when parsing Ansi256 from string. Required by [FromStr]
pub enum Ansi256Error {
    ///Color name is not in [XtermColors]
    InvalidColorName,
    ///String is not an [XtermColors] string
    InvalidString,
}

impl FromStr for Ansi256 {
    type Err = Ansi256Error;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        if s.starts_with("xterm(") && s.ends_with(")") {
            let color_name = &s[6..s.len() - 1];
            let xterm_color = XtermColors::get_name(color_name);
            if let Some(color) = xterm_color {
                return Ok(Self(color.ansi256()));
            } else {
                return Err(Ansi256Error::InvalidColorName);
            }
        }
        Err(Ansi256Error::InvalidString)
    }
}

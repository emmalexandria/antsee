/** ansi16 provides a representation of the basic ANSI colors as an enum*/
pub mod ansi16;
/** ansi256 provides a representation of the ANSI256 palette, including parsing from [XtermColors] */
pub mod ansi256;
/** rgb provides a representation of RGB colors, including parsing from [CssColors], [XtermColors],
* and hexadecimals */
pub mod rgb;

/** This module serves to provide color names for RGB using css color names */
pub mod css;
/** This module provides color names for the ANSI256 color palette to be used by RGB and ANSI256
* colors */
pub mod xterm;
///Represents a single color in [ANSI16], ANSI256, or [RGB]
pub use {ansi16::Ansi16, ansi256::Ansi256, css::CssColors, rgb::Rgb, xterm::XtermColors};

///Enum representing a generic color of any format.
#[derive(Clone, Copy, PartialEq, Debug)]
#[cfg_attr(feature = "serde", derive(serde::Deserialize, serde::Serialize))]
pub enum Color {
    ///[Ansi16] variant
    Ansi(Ansi16),
    ///[Ansi256] variant
    Ansi256(Ansi256),
    ///[Rgb] variant
    Rgb(Rgb),
}
impl Color {
    ///Retrieve the inner [Ansi16] value if the color has one
    pub fn as_ansi(&self) -> Option<&Ansi16> {
        if let Color::Ansi(value) = self {
            return Some(value);
        }
        None
    }

    ///Retrieve the inner [Ansi256] value if the color has one
    pub fn as_ansi256(&self) -> Option<&Ansi256> {
        if let Color::Ansi256(value) = self {
            return Some(value);
        }
        None
    }

    ///Retrieve the inner [Rgb] value if the color has one
    pub fn as_rgb(&self) -> Option<&Rgb> {
        if let Color::Rgb(value) = self {
            return Some(value);
        }
        None
    }
}

impl From<Ansi16> for Color {
    fn from(value: Ansi16) -> Self {
        Self::Ansi(value)
    }
}

impl From<Ansi256> for Color {
    fn from(value: Ansi256) -> Self {
        Self::Ansi256(value)
    }
}

impl From<Rgb> for Color {
    fn from(value: Rgb) -> Self {
        Self::Rgb(value)
    }
}

///Trait for easily making generics that take Ansi16, Ansi256, or Rgb
pub trait ColorValue: Into<Color> + Default {}

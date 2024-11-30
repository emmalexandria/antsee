/** ansi provides a representation of the basic ANSI colors as an enum*/
pub mod ansi;
/** fixed provides a representation of the ANSI256 palette, including parsing from [XtermColors] */
pub mod fixed;
/** rgb provides a representation of RGB colors, including parsing from [CssColors], [XtermColors],
* and hexadecimals */
pub mod rgb;

/** This module serves to provide color names for RGB using css color names */
pub mod css;
/** This module provides color names for the ANSI256 color palette to be used by RGB and ANSI256
* colors */
pub mod xterm;

use std::fmt::Display;

pub use {ansi::Ansi, fixed::Fixed};
///Represents a single color in [ANSI16], ANSI256, or [RGB]
pub use {css::CssColors, rgb::Rgb, xterm::XtermColors};

///Enum representing a generic color of any format.
#[derive(Clone, PartialEq, Debug)]
#[cfg_attr(
    feature = "serde",
    derive(serde::Deserialize, serde::Serialize),
    serde(untagged)
)]
pub enum Color {
    ///[Ansi16] variant
    Ansi(Ansi),
    ///[Ansi256] variant
    Fixed(Fixed),
    ///[Rgb] variant
    Rgb(Rgb),
}
impl Color {
    ///Retrieve the inner [Ansi] value if the color has one
    pub fn as_ansi(&self) -> Option<&Ansi> {
        if let Color::Ansi(value) = self {
            return Some(value);
        }
        None
    }

    ///Retrieve the inner [Fixed] value if the color has one
    pub fn as_fixed(&self) -> Option<&Fixed> {
        if let Color::Fixed(value) = self {
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

impl From<Ansi> for Color {
    fn from(value: Ansi) -> Self {
        Self::Ansi(value)
    }
}

impl From<Fixed> for Color {
    fn from(value: Fixed) -> Self {
        Self::Fixed(value)
    }
}

impl From<Rgb> for Color {
    fn from(value: Rgb) -> Self {
        Self::Rgb(value)
    }
}

impl Display for Color {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        if let Some(rgb) = self.as_rgb() {
            write!(f, "{:#}", rgb)?;
        }
        if let Some(fixed) = self.as_fixed() {
            write!(f, "{:#}", fixed)?;
        }
        if let Some(ansi) = self.as_ansi() {
            write!(f, "{:#}", ansi)?;
        }
        Ok(())
    }
}

///Trait defining common functions for macro based colour libraries ([xterm], [css])
pub trait ColorLibrary
where
    Self: Sized,
{
    ///The function style wrapper which identifies a value as being from this color library
    const WRAPPER: &str;

    ///Wrap a string in the style wrapper
    fn wrap_name(s: &str) -> String;
    ///Extract a string from the style wrapper
    fn unwrap_name(s: &str) -> &str;

    ///Get a color by name
    fn get_name(s: &str) -> Option<Self>;

    ///Get the name of a color
    fn color_name(&self) -> &'static str;

    ///Get the RGB value of a colour
    fn rgb(&self) -> [u8; 3];
}

///Trait for easily making generics that take Ansi16, Ansi256, or Rgb
pub trait ColorValue: Into<Color> + Default {}

///Trait which enables management of "sources", which are what get serialized when the data
///representation isnt
pub trait ColorSource {
    type ExternalSource;

    ///Set an external source on the colour
    fn set_external_source(&mut self, value: Self::ExternalSource);
    ///Use the external source for serialization
    fn source_external(&mut self);
    ///Use the internal source (data representation) for serialization
    fn source_internal(&mut self);
}

#[derive(Clone, Copy, PartialEq, Debug)]
enum Source<S> {
    Active(S),
    Inactive(S),
}

impl<S> Source<S> {
    fn inactive(self) -> Self {
        match self {
            Self::Inactive(s) => Self::Inactive(s),
            Self::Active(s) => Self::Inactive(s),
        }
    }

    fn active(self) -> Self {
        match self {
            Source::Active(s) => Self::Active(s),
            Source::Inactive(s) => Self::Active(s),
        }
    }
}

impl<S> Default for Source<S>
where
    S: Default,
{
    fn default() -> Self {
        Self::Inactive(S::default())
    }
}

#[cfg(test)]
mod color_tests {
    use serde_test::{assert_tokens, Token};

    use super::*;

    #[test]
    fn test_serialize_ansi() {
        let color: Color = Ansi::Red.into();

        assert_tokens(&color, &[Token::Str("Red")])
    }

    #[test]
    fn test_serialize_xterm() {
        let color = Rgb::from(XtermColors::Seafoam);

        assert_tokens(&color, &[Token::Str("xterm(Seafoam)")])
    }
}

/*!
# Color sources
[Rgb] and [Fixed] both store a string value called the 'external source'. This value is set whenever these color formats are constructed from a color name or hex value. This is what allows for the serialisation system.

*/

/** ansi provides a representation of the basic ANSI colors as an enum*/
mod ansi;
/** fixed provides a representation of the ANSI256 palette, including parsing from [XtermColors] */
mod fixed;
/** rgb provides a representation of RGB colors, including parsing from [CssColors], [XtermColors],
* and hexadecimals */
mod rgb;

///libraries is the module containing the CSS and Xterm color libraries
pub mod libraries;
use std::fmt::Display;

#[doc(inline)]
pub use {ansi::Ansi, fixed::Fixed, rgb::Rgb};

///This enum defines various errors that may be encountered when parsing a colour from a string
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum ColorFromStrError {
    ///Value is not a valid name in the colour library
    InvalidName,
    ///Colour value is invalid (e.g. invalid Hex)
    InvalidValue,
    ///String value could not be matched to any color source
    InvalidString,
}

///Represents a single color in [ANSI16], ANSI256, or [RGB]
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

///Trait allowing color formats to be turned into [Color] automatically
pub trait ColorValue: Into<Color> + Default {}

///Trait which enables management of "color sources" -- the value which gets
///serialised with the color
pub trait ColorSource {
    type ExternalSource;

    ///Set an external source on the color
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
    use libraries::XtermColors;
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

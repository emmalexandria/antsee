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
pub use {ansi::Ansi, fixed::Fixed};
///Represents a single color in [ANSI16], ANSI256, or [RGB]
pub use {css::CssColors, rgb::Rgb, xterm::XtermColors};

///Enum representing a generic color of any format.
#[derive(Clone, PartialEq, Debug)]
#[cfg_attr(feature = "serde", derive(serde::Deserialize, serde::Serialize))]
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

///Trait for easily making generics that take Ansi16, Ansi256, or Rgb
pub trait ColorValue: Into<Color> + Default {}

trait ColorSource {
    type ExternalSource: ?Sized;

    fn set_external_source(&mut self, value: Self::ExternalSource);
    fn source_external(&mut self);
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
        return Self::Inactive(S::default());
    }
}

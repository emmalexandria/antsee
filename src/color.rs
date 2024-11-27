/** ansi16 provides basic terminal colors */
pub mod ansi16;
/** This module defines a representation of ANSI256 colors */
pub mod ansi256;

/** This module serves to provide color names for RGB using css color names */
pub mod css;
pub mod rgb;
/** This module provides color names for the ANSI256 color palette to be used by RGB and ANSI256
* colors */
pub mod xterm;
///Represents a single color in [ANSI16], ANSI256, or [RGB]
pub use {ansi16::Ansi16, ansi256::Ansi256, css::CssColors, rgb::Rgb, xterm::XtermColors};

#[derive(Clone, Copy, PartialEq, Debug)]
#[cfg_attr(feature = "serde", derive(serde::Deserialize, serde::Serialize))]
pub enum Color {
    Ansi(Ansi16),
    Ansi256(Ansi256),
    Rgb(Rgb),
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

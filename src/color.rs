use ansi16::Ansi16;
use ansi256::Ansi256;
use rgb::Rgb;

/** ansi16 provides basic terminal colors */
pub mod ansi16;
pub mod ansi256;

/** This module serves to provide color names for RGB using css color names */
pub mod css;
pub mod rgb;
///Represents a single color in [ANSI16], ANSI256, or [RGB]

#[cfg_attr(feature = "serde", derive(serde::Deserialize, serde::Serialize))]
pub enum Color {
    Ansi(Ansi16),
    Ansi256(Ansi256),
    Rgb(Rgb),
}

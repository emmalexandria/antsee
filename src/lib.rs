mod ansi;
mod color;
mod display;
mod style;

const ANSI_ESCAPE: &str = "\x1b[";

pub use color::{Color, ColorLevels, ColorVal, ANSI16, RGB};
pub use style::{Property, Style, Styleable};

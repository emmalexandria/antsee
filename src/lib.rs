mod ansi;
mod color;
mod display;
mod style;

const ANSI_ESCAPE: &str = "\x1b[";

pub use color::{Color, ColorLevels, ColorValue, ANSI16, RGB};
pub use style::Property;
pub use style::Style;

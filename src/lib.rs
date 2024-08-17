mod ansi;
mod color;
mod display;
mod style;

const ANSI_ESCAPE: &str = "\x1b[";

pub use color::{AdaptiveMultiColor, Color, MultiColor, TrueColor};
pub use style::Style;

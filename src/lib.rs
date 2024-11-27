/** Antsee is a terminal output library which doesn't output anything to the terminal. It's designed
* for writing highly customizeable and user friendly configuration formats */
pub mod color;
pub mod style;

pub use color::Color;
pub use color::{Ansi16, Ansi256, Rgb};

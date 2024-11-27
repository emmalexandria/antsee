#![warn(missing_docs)]

/*! Antsee is a terminal output library which doesn't output anything to the terminal. It's designed
* for writing highly customizeable and user friendly configuration formats */

/** color is responsible for the representation of colors, such as formats ([Rgb]), names
* ([color::CssColors]) and their interoperation */
pub mod color;

/** style holds the [Style] type and the [Attributes] type. */
pub mod style;

pub use color::Color;
pub use color::{Ansi16, Ansi256, Rgb};
pub use style::{Attributes, Style};

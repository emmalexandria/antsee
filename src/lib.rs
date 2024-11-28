#![warn(missing_docs)]

/*! Antsee is a terminal output library which doesn't output anything to the terminal. It's designed
* for writing highly customizeable and user friendly configuration formats */

/** color is responsible for the representation of colors, such as formats ([Rgb]), names
* ([color::CssColors]) and their interoperation */
pub mod color;

/** style holds the [Style] type and the [Attributes] type. */
pub mod style;

#[doc(inline)]
pub use color::Color;
#[doc(inline)]
pub use color::{Ansi, Fixed, Rgb};
#[doc(inline)]
pub use style::{Attributes, Style};

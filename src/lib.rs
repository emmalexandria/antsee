#![warn(missing_docs)]

/*! This crate provides types representing terminal colors and styles built for building
configuration files. It also provides named colour libraries for both CSS color names
and xterm color names.


# Overview

The primary types of this crate are [Color] and [Style].

[Color] is an enum which contains one of the three color formats ([Rgb], [Fixed], [Ansi]).

[Style] contains a foreground color, background color, and [Attributes]

## Colors

Color formats in this crate can be parsed from strings, or set from color libraries. As an overview:
- [Ansi] -- Can be parsed from a basic name string (`BrightRed` or `brightred`)
- [Fixed] -- Can be parsed or set from [XtermColors] or set as a [u8]
- [Rgb] -- Can be parsed or set from [XtermColors] or [CssColors], parsed from a hex string, or set as an RGB value.

### Libraries
This crate provides two color libraries. [CssColors] provides CSS color names and [XtermColors] provides names for the ANSI256 ([Fixed]) palette.

When being parsed from a string or used in a configuration file, they are identified by a 'function' style syntax.
```rust
use std::str::FromStr;
let css_color = antsee::Rgb::from_str("css(red)").unwrap();
let xterm_color = antsee::Fixed::from_str("xterm(Seafoam)").unwrap();
```

## Serde
  While `serde` is an optional feature, most of the useful functionality of this crate
lies in its custom implementations of `Serialize` and `Deserialize`. These custom implementations
provide the following functionality:
- Serialization and deserialisation from hex values, color names, etc
- Colors which "remember" the last value they were set with and serialise to that value


```rust
use antsee::{Color, Rgb};
    //This will serialise as #324450 instead of the value of Rgb
let color: Color = Rgb::new().hex("#324450").unwrap().into();
```



*/

/** color defines the [Color] type, color formats ([Rgb], [Fixed], [Ansi]) and [color libraries](color::libraries)
 */
pub mod color;

/** style holds the [Style] type and the [Attributes] type. */
pub mod style;

pub use color::libraries::{CssColors, XtermColors};
pub use color::{Ansi, Color, Fixed, Rgb};
pub use style::{Attributes, Style};

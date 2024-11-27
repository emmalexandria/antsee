# antsee

`antsee` is a terminal colour and style library which does not handle escape codes, color or format text, or interact with the terminal in any way. 

## Motivation 
Practically every single Rust ANSI/terminal style library is useless for configuration. As far as I can tell, out of `owo-colors`, `colorize`, `ansi_term`, `yansi`, etc, only `ansi_term` supports `serde` at all. That support goes no further than deriving `Serialize` and `Deserialize`.

If you need your users to be able to configure styles and colours, you're likely stuck with separate types anyway. Why not do it well? This aims to provide
a library built for configuration, with high quality `serde` support including custom Deserialize and Serialize implementations.

It also aims to fix other small annoyances, like attributes directly on style types, making it a pain to define custom style types with 
additional colours. `antsee` aims to stay small, having no default dependencies. 

## Features

### User friendly options for setting colours
RGB colours can be set with a list of three u8 values, a hex string, a CSS colour name, or from a set of names for the ANSI256 palette (which can also be used for ANSI256 of course).

ANSI16 colours can be set by both PascalCase, such as `DarkLight`, or lowercase such as `darklight`. Colour deserialization will first attempt to create an 
RGB colour, then an ANSI256 colour, and then finally an ANSI16 colour. 

### Easy custom style types 
If your configuration needs go beyond `fg`, `bg`, and `attributes`, text attributes and color have been kept seperate from the `Style` type. This makes it easy to define custom style types with the same benefits as the built in one.

## Roadmap

### Conversion libraries for common ANSI libraries
I plan to create crates which allow for easy conversion between `antsee`'s types and those of popular terminal output libraries like `crossterm`, `owo-colors`, `colorize`, and `ansi_term`. 

### Generic macros and traits for defining new color names
Currently, the `xterm` and `css` color names are created by specialised macros and loaded specially in each colour class. I plan to eventually create a generic 
macro alongside traits for defining new sets of colour variables.


# antsee

`antsee` is a terminal colour and style library which does not handle escape codes, color or format text, or interact with the terminal in any way. 

## Motivation 
Practically every single Rust ANSI/terminal style library is useless for configuration. As far as I can tell, out of `owo-colors`, `colorize`, `ansi_term`, `yansi`, etc, only `ansi_term` supports `serde` at all. That support goes no further than deriving `Serialize` and `Deserialize`.

If you need your users to be able to configure styles and colours, you're likely stuck with separate types anyway. Why not do it well? This aims to provide
a library built for configuration, with high quality `serde` support including custom Deserialize and Serialize implementations.

It also aims to fix other small annoyances, like attributes directly on style types, making it a pain to define custom style types with 
additional colours. `antsee` aims to stay small, having no default dependencies. 

## Features

### Extensive colour libraries 
`antsee` provides full libraries of CSS and xterm colours which can be used to set the values of RGB or Fixed (ANSI256) colours. These libraries 
contain the colour names, and RGB values. In addition, RGB colours can be set with hex values, and ANSI16 colours can be set with string names such as 
BrightGreen or brightgreen. 

### Custom Serde magic 
Every colour type in `antsee` has custom Serialize and Deserialize definitions. This allows for two main connected features:
1. CSS/xterm color names and hex values will be deserialized
2. Every color type in `antsee` keeps track of the last value used to set it. It will then serialize with this value instead of a strict data representation.

#### Example
If I deserialize an RGB colour from a hex value, the RGB colour will store that hex value alongside its data representation. If I were to reserialize the RGB colour, it would serialize to the hex value. However, if I then set the RGB colour to `css(red)`, it will then serialize to `css(red)` automatically. 

*If you want to serialize raw data values, either set the colour with those raw data values or import the `ColorSource` trait and call `source_internal` on the color variable*

### Easy custom style types 
If your configuration needs go beyond `fg`, `bg`, and `attributes`, text attributes and color have been kept seperate from the `Style` type. This makes it easy to define custom style types with the same benefits as the built in one.

## Examples
The best place to see `antsee` at work is in the `/examples` directory.

## Roadmap

- Unify implementation of xterm and CSS colour libraries
- Create conversion libraries for popular ANSI output libraries
- Introduce the ability to set attributes with a shorthand (e.g.) bsu would set bold, strikethrough, and hidden


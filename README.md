# antsee

`antsee` is a crate which aims to provide color and style types designed for configuration files. 

[View the docs](https://docs.rs/antsee/latest/antsee/)

*This library is currently in its very early stages. I am only promoting it as a full release now because I have become blind to what I'm sure are many obvious issues with it. Feedback is hugely appreciated*

## Motivation 
Practically every single Rust ANSI/terminal style library is useless for configuration files. As far as I can tell, out of `owo-colors`, `colorize`, `ansi_term`, `yansi`, etc, only `ansi_term` supports `serde` at all. That support goes no further than deriving `Serialize` and `Deserialize`. This crate aims to fix that gap.

It also aims to fix other small annoyances, like attributes directly on style types.

## Features

### Extensive colour libraries 
`antsee` provides full libraries of CSS and xterm colour names which can be used to set the values of RGB or Fixed (ANSI256) colours. These libraries 
contain the colour names, RGB values, and ANSI256 index in the case of the xterm library. In addition, RGB colours can be set with hex values, and ANSI16 colours can be set with string names such as 
BrightGreen or brightgreen. 

### Flexible colour parsing
Every color type in `antsee` has a `FromStr` implementation. RGB colors can be set with hex values, RGB values, or colour library names. ANSI256 (Fixed) colors can be set with xterm colour names or `u8` indices, and ANSI16 colors can be set using string color names (e.g. `BrightRed`). When being parsed from a string, the colour libraries are distinguished with function like wrappers: 
```
css(red)
xterm(Seafoam)
```

### Custom Serde implementations 
Every colour type in `antsee` has custom Serialize and Deserialize definitions. This allows for two main connected features:
1. CSS/xterm color names and hex values will be deserialized
2. Every color type in `antsee` keeps track of the last value used to set it. It will then serialize with this value instead of a strict data representation.

#### Example
If I deserialize an RGB colour from a hex value, the RGB colour will store that hex value alongside its data representation. If I were to reserialize the RGB colour, it would serialize to the hex value. However, if I then set the RGB colour to `css(red)`, it will then serialize to `css(red)` automatically. 

*If you want to serialize raw data values, either set the colour with those raw data values or import the `ColorSource` trait and call `source_internal` on the color variable*




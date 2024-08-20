# antsee

## Goals

`antsee` is a lightweight and dependency free ANSI library with the primary goal of allowing for flexibility in styling. This is achieved primarily through the way it models colours. A `Color` in `antsee` represents two colors. One is for use on dark terminal backgrounds, and the other is for use on light terminal backgrounds. Further, each colour can contain three colour values for terminals that support ANSI16, ANSI256, and RGB. If given the appropriate information, `Color` will choose the appropriate color for the environment the application is running in.

This crate does not aim to do the work of detecting the color support of the current terminal, nor whether the background is dark or light.

## Planned features
- Windows support (automatically enabling ANSI support when on windows)
- Strings that contain multiple styles for each block of text that remain easily editable
- Support for non-standard properties (e.g. hyperlinks)

## Example usage

```rust
let styled_str = "Hi there"
        .to_styled_string()
        .with_foreground(RGB::rgb(134, 100, 50))
        .with_background(ANSI16::BrightBlack)
        .with_property(Property::Underline);
```

```rust
let style = Style::default()
        .with_foreground(Color::from(ANSI16::Red))
        .with_background(Color::from(ANSI16::Black))
        .with_property(Property::Bold)
        .with_property(Property::Underline);

let output = style.paint("Hello");
```

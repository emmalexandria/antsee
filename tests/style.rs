use antsee::ColorVal;
use antsee::Property;
use antsee::Styleable;
use antsee::RGB;
use antsee::{Color, Style, ANSI16};

#[test]
fn test_style() {
    let style = Style::default()
        .with_foreground(Color::from(ANSI16::Red))
        .with_background(Color::from(ANSI16::Black))
        .with_property(Property::Bold)
        .with_property(Property::Underline);

    let output = style.paint("Hello");

    assert_eq!(output, "\x1b[31;40;1;4mHello\x1b[0m");
}

#[test]
fn test_styleable() {
    let styled_str = "Hi there"
        .to_styled_string()
        .with_foreground(RGB::rgb(134, 100, 50))
        .with_background(ANSI16::BrightBlack)
        .with_property(Property::Underline);

    println!("{styled_str}");
}

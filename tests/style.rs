use antsee::ColorValue;
use antsee::Property;
use antsee::{Color, Style, ANSI16};

#[test]
fn test_style() {
    let style = Style::default()
        .with_foreground(Color::default().with_color(ColorValue::Base(ANSI16::Red)))
        .with_background(Color::default().with_color(ColorValue::Base(ANSI16::Black)))
        .with_property(Property::Bold)
        .with_property(Property::Underline);

    let output = style.paint("Hello");

    assert_eq!(output, "\x1b[31;40;1;4mHello\x1b[0m");
}

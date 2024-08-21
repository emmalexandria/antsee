use antsee::ColorVal;
use antsee::Property;
use antsee::Styleable;
use antsee::RGB;
use antsee::{Color, Style, ANSI16};

#[test]
fn test_style() {
    let style = Style::default()
        .with_fg(Color::from(ANSI16::Red))
        .with_bg(Color::from(ANSI16::Black))
        .with_prop(Property::Bold)
        .with_prop(Property::Underline);

    let output = style.paint("Hello");

    assert_eq!(output, "\x1b[31;40;1;4mHello\x1b[0m");
}

use antsee::{Attributes, Color, Rgb, Style};
use toml::Serializer;

struct MultiColorStyle {
    foreground: Vec<Color>,
    background: Option<Color>,
    attributes: Attributes,
}

struct Config {
    quote: MultiColorStyle,
}

fn main() {
    let config = Config {
        quote: {
            foreground: vec![Color::Rgb(Rgb::from_str("blue"))]
        }
    }
}

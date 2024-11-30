use bat::PrettyPrinter;
use std::fmt::Display;
use std::str::FromStr;

use antsee::{
    color::{Ansi, Fixed, Rgb},
    Attributes, Color, Style, XtermColors,
};

#[derive(serde::Serialize, serde::Deserialize, Debug)]
struct SerdeExample {
    output_text: Style,

    border_color: Color,
    background_rgb: Rgb,
}

impl Display for SerdeExample {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        writeln!(f, "output_text: {:#}", self.output_text)?;
        writeln!(f, "border_color: {:#}", self.border_color)?;
        writeln!(f, "background_rgb: {:#}", self.background_rgb)?;
        Ok(())
    }
}

impl Default for SerdeExample {
    fn default() -> Self {
        Self {
            output_text: Style::default()
                .fg(Ansi::Red)
                .attributes(Attributes::new().bold()),

            border_color: Fixed::from(XtermColors::Pinky).into(),
            background_rgb: Rgb::from_str("#342398").unwrap(),
        }
    }
}

fn main() {
    serialize()
}

fn serialize() {
    let config = SerdeExample::default();
    let output = toml::to_string_pretty(&config).unwrap();

    println!(
        "{}",
        nu_ansi_term::Style::new()
            .bold()
            .underline()
            .paint("Outputted the following configuration format:")
    );
    print_highlighted_debug(&config);

    println!(
        "{}",
        nu_ansi_term::Style::new()
            .bold()
            .underline()
            .paint("As the following TOML:")
    );
    print_highlighted(&output, Some("toml"));
}

fn print_highlighted(text: &str, language: Option<&str>) {
    let mut printer = PrettyPrinter::new();
    printer.input_from_bytes(text.as_bytes());
    if let Some(lang) = language {
        printer.language(lang);
    }
    printer.line_numbers(true);
    printer.theme("ansi");
    printer.print();
}

fn print_highlighted_debug<T>(value: &T)
where
    T: std::fmt::Debug,
{
    let debug = format!("{:#?}", value);
    print_highlighted(debug.as_str(), None)
}

fn print_highlighted_json<T>(value: &T)
where
    T: serde::Serialize,
{
    let pretty_json = serde_json::to_string_pretty(value).unwrap_or("".to_owned());
    print_highlighted(&pretty_json, Some("json"));
}

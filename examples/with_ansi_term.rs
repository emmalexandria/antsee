use antsee::{Attributes, Fixed, Style, XtermColors};

struct Config {
    output_style: Style,
}

impl Default for Config {
    fn default() -> Self {
        Self {
            output_style: Style::default()
                .bg(Fixed::from(XtermColors::Seafoam))
                .attributes(Attributes::new().bold()),
        }
    }
}

fn main() {
    let config = Config::default();
    let bg_color_index = config
        .output_style
        .bg
        .unwrap()
        .as_fixed()
        .unwrap()
        .to_owned();
    let ansi_term_style = nu_ansi_term::Style::new()
        .on(nu_ansi_term::Color::Fixed(bg_color_index.0))
        .bold();

    println!("{}", ansi_term_style.paint("Hello this is my output style"))
}

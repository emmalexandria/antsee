use antsee::style::Style;

struct Config {
    outputStyle: Style,
}

impl Default for Config {
    fn default() -> Self {
        Self {
            outputStyle: Style::default().fg(Color),
        }
    }
}

fn main() {}

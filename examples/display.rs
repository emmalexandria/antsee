use antsee::*;

fn main() {
    let intro_style = Style::new().with_prop(Property::Bold);

    println!(
        "{} {} {}",
        "Hello! This is an example application showing the capabilities of the"
            .with_style(&intro_style),
        "antsee"
            .to_styled()
            .with_fg(ANSI16::BrightWhite)
            .with_prop(Property::Bold),
        "library.".with_style(&intro_style)
    );

    println!("It has full support for the 256 color palette");
    let mut col = 0;
    for i in 0..=255 {
        print!("{}", "H".to_styled().with_bg(ColorVal::Fixed(i)));
        col += 1;
        if col == 32 {
            col = 0;
            print!("\n");
        }
    }
}

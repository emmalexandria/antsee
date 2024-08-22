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
        let mut num_str = i.to_string();
        pad_with_zeros(&mut num_str);

        print!("{}", num_str.to_styled().with_bg(i).with_fg(255));
        col += 1;
        if col == 32 {
            col = 0;
            print!("\n");
        }
    }

    println!("And it can display RGB colors");
    col = 0;
    for i in 0..=255 {
        print!("{}", " ".to_styled().with_bg(RGB::rgb(i, 0, 0)));

        col += 1;
        if col == 32 {
            col = 0;
            print!("\n");
        }
    }
}

fn pad_with_zeros(num: &mut String) {
    while num.len() < 3 {
        num.insert(0, '0');
    }
    num.insert(0, ' ');
    num.push(' ');
}

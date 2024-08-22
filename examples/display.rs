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

    println!(
        "{}",
        "It has full support for the 256 color palette:".with_style(&intro_style)
    );
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

    println!(
        "\n{}",
        "And it can display RGB colors:".with_style(&intro_style)
    );

    let mut curr_col = 0;
    let mut abs_row = 0;
    for _ in 0..32u32.pow(2) {
        print!(
            "{}",
            "  ".to_styled().with_bg(RGB::rgb(
                map_in_rgb_range(abs_row, 0, 32),
                0,
                map_in_rgb_range(curr_col, 0, 32)
            ))
        );
        curr_col += 1;
        if curr_col == 32 {
            curr_col = 0;
            print!("\n");
            abs_row += 1;
        }
    }

    println!("");
}

fn pad_with_zeros(num: &mut String) {
    while num.len() < 3 {
        num.insert(0, '0');
    }
    num.insert(0, ' ');
    num.push(' ');
}

fn map_in_rgb_range(val: u8, min: u8, max: u8) -> u8 {
    let range = max - min;
    return (255.0 * (f32::from(val) / f32::from(range))) as u8;
}

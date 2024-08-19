use std::fmt::Write;

use crate::{
    display::StyledString,
    style::{Property, Style},
    ANSI_ESCAPE,
};

pub trait ANSICode: Clone + Default {
    fn get_codes(&self, bg: Option<bool>) -> Vec<u32>;
    fn get_reset_code(&self, bg: Option<bool>) -> u32;
}

impl Style {
    pub fn paint<S: ToString>(&self, s: S) -> String {
        let mut output_str = String::new();
        self.prefix_codes(&mut output_str).unwrap();
        output_str.push_str(&s.to_string());
        Self::write_reset(&mut output_str).unwrap();

        output_str
    }

    fn prefix_codes<W: Write>(&self, writer: &mut W) -> std::fmt::Result {
        let mut has_written = false;

        if self.prefix_with_reset {
            Self::write_reset(writer)?;
        }
        writer.write_str(ANSI_ESCAPE)?;
        Self::write_code(&self.foreground, writer, Some(false), &mut has_written)?;
        Self::write_code(&self.background, writer, Some(true), &mut has_written)?;

        for prop in self.get_properties().iter() {
            Self::write_code(prop, writer, None, &mut has_written)?;
        }

        writer.write_char('m');

        Ok(())
    }

    fn write_code<W: Write>(
        prop: &impl ANSICode,
        w: &mut W,
        bg: Option<bool>,
        has_written: &mut bool,
    ) -> std::fmt::Result {
        for num in prop.get_codes(bg) {
            if *has_written {
                w.write_char(';')?;
            }

            w.write_str(&num.to_string())?;

            if !*has_written {
                *has_written = true
            }
        }

        Ok(())
    }

    fn write_reset<W: Write>(w: &mut W) -> std::fmt::Result {
        w.write_str(&format!(
            "{}{}m",
            ANSI_ESCAPE,
            Property::Reset.get_reset_code(None)
        ))
    }
}

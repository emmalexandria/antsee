use std::fmt::Write;

use crate::{
    display::StyledString,
    style::{Property, Style},
    ANSI_ESCAPE,
};

pub trait ANSICode: Clone + Default {
    fn get_codes(&self, bg: Option<bool>) -> Vec<u32>;
    fn get_reset_code(&self) -> u32;
}

impl<C: ANSICode> StyledString<C> {
    fn render(&self) {}
}

impl<C: ANSICode> Style<C> {
    fn prefix_codes<W: Write>(&self, writer: &mut W) -> std::fmt::Result {
        let mut has_written = false;

        if self.prefix_with_reset {
            writer.write_str(&format!(
                "{}{}m",
                ANSI_ESCAPE,
                Property::Reset.get_reset_code()
            ))?;
        }

        Self::write_code(&self.foreground, writer, Some(false), &mut has_written)?;
        Self::write_code(&self.background, writer, Some(true), &mut has_written)?;

        for prop in self.get_properties().iter() {
            Self::write_code(prop, writer, None, &mut has_written)?;
        }

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
}

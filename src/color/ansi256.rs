use std::str::FromStr;

use super::xterm::XtermColors;

#[derive(Clone, Debug, PartialEq, Copy)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct Ansi256(u8);

pub enum Ansi256Error {
    InvalidColorName,
    InvalidString,
}

impl FromStr for Ansi256 {
    type Err = Ansi256Error;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        if s.starts_with("xterm(") && s.ends_with(")") {
            let color_name = &s[6..s.len() - 1];
            let xterm_color = XtermColors::get_name(color_name);
            if let Some(color) = xterm_color {
                return Ok(Self(color.ansi256()));
            } else {
                return Err(Ansi256Error::InvalidColorName);
            }
        }
        Err(Ansi256Error::InvalidString)
    }
}

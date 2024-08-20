use super::ansi::ANSICode;
use crate::{style::Property, ANSI_ESCAPE};
use std::fmt::Display;

///Represents a single color in ANSI16, ANSI256, or RGB

#[derive(Clone, Copy, Eq, PartialEq, PartialOrd, Hash, Debug)]
pub enum ColorVal {
    Base(ANSI16),
    ANSI256(u8),
    RGB(RGB),
}

impl Default for ColorVal {
    fn default() -> Self {
        Self::Base(ANSI16::Default)
    }
}

impl ANSICode for ColorVal {
    fn get_codes(&self, bg: Option<bool>) -> Vec<u32> {
        match self {
            Self::Base(c) => c.get_codes(bg),
            Self::ANSI256(c) => Self::get_ansi_256_codes(c, bg),
            Self::RGB(c) => c.get_codes(bg),
        }
    }

    fn get_reset_code(&self, bg: Option<bool>) -> u32 {
        ANSI16::Default.get_reset_code(bg)
    }
}

impl ColorVal {
    fn get_ansi_256_codes(val: &u8, bg: Option<bool>) -> Vec<u32> {
        match bg {
            Some(false) | None => vec![38, 5, *val as u32],
            Some(true) => vec![48, 5, *val as u32],
        }
    }
}

///A color with representations in ANSI16 and optionally ANSI256 and RGB)

#[derive(Clone, Copy, Eq, PartialEq, PartialOrd, Hash, Debug, Default)]
pub struct ColorLevels(ColorVal, Option<ColorVal>, Option<ColorVal>);

impl From<ColorVal> for ColorLevels {
    fn from(value: ColorVal) -> Self {
        let mut ret_val = Self::default();
        match value {
            ColorVal::Base(_) => ret_val.0 = value,
            ColorVal::ANSI256(_) => ret_val.1 = Some(value),
            ColorVal::RGB(_) => ret_val.2 = Some(value),
        }

        ret_val
    }
}

///Color (optionally) represents color values for terminals with
///support for ANSI16, ANSI256, or Truecolor. It can hold another set of colors for
///terminals with light backgrounds.

#[derive(Clone, Copy, Eq, PartialEq, PartialOrd, Hash, Debug, Default)]
pub struct Color {
    ///Default colors for dark backgrounds
    pub color: ColorLevels,
    ///Colors for light backgrounds
    pub light_color: Option<ColorLevels>,
    ///The support (if determined) of the terminal (ANSI16, ANSI256, Truecolor)
    pub color_support: Option<(bool, bool, bool)>,
    ///Determines behaviour if color support is unknown. Optimistic colors will always select the
    ///"highest" level of support.
    pub optimistic_colors: bool,
    ///Whether the background of the terminal is light or not.
    pub is_light_bg: Option<bool>,
}

impl From<ColorVal> for Color {
    fn from(value: ColorVal) -> Self {
        let mut ret_val = Self::default();
        ret_val.color = ColorLevels::from(value);

        ret_val
    }
}

impl From<ANSI16> for Color {
    fn from(value: ANSI16) -> Self {
        Self::from(ColorVal::Base(value))
    }
}

impl From<RGB> for Color {
    fn from(value: RGB) -> Self {
        Self::from(ColorVal::RGB(value))
    }
}

impl Color {
    pub fn new(color: ColorVal) -> Self {
        Self {
            color: ColorLevels(color.clone(), None, None),
            light_color: Some(ColorLevels(color, None, None)),
            color_support: None,
            optimistic_colors: false,
            is_light_bg: None,
        }
    }

    pub fn with_color_support(mut self, support: (bool, bool, bool)) -> Self {
        self.color_support = Some(support);
        self
    }

    pub fn with_color(mut self, color: ColorVal) -> Self {
        match color {
            ColorVal::RGB(_) => self.color.2 = Some(color),
            ColorVal::ANSI256(_) => self.color.1 = Some(color),
            ColorVal::Base(_) => self.color.0 = color,
        }
        self
    }

    pub fn with_light_color(mut self, color: ColorVal) -> Self {
        let mut range = self.light_color.unwrap_or_default();
        match color {
            ColorVal::RGB(_) => range.0 = color,
            ColorVal::ANSI256(_) => range.1 = Some(color),
            ColorVal::Base(_) => range.2 = Some(color),
        }

        self.light_color = Some(range);
        self
    }

    pub fn with_colors(mut self, levels: ColorLevels) -> Self {
        self.color = levels;
        self
    }

    pub fn with_light_colors(mut self, levels: ColorLevels) -> Self {
        self.light_color = Some(levels);
        self
    }

    pub fn with_background_light(mut self, light: Option<bool>) -> Self {
        self.is_light_bg = light;
        self
    }

    pub fn with_optimistic_colors(mut self, optimistic: bool) -> Self {
        self.optimistic_colors = optimistic;
        self
    }
}

impl ANSICode for Color {
    fn get_codes(&self, bg: Option<bool>) -> Vec<u32> {
        self.get_highest_supported_code(bg)
    }

    fn get_reset_code(&self, bg: Option<bool>) -> u32 {
        return ANSI16::Default.get_reset_code(bg);
    }
}

impl Color {
    fn get_highest_supported_code(&self, bg: Option<bool>) -> Vec<u32> {
        let mut color = &self.color;
        if self.is_light_bg.is_some_and(|v| v) {
            color = self.light_color.as_ref().unwrap()
        }

        if self.color_support.is_none() {
            return self.get_color_without_support_info(color).get_codes(bg);
        }

        let support = self.color_support.unwrap();
        if support.2 && color.2.is_some() {
            return color.2.as_ref().unwrap().get_codes(bg);
        } else if support.1 && color.1.is_some() {
            return color.1.as_ref().unwrap().get_codes(bg);
        }

        return color.0.get_codes(bg);
    }

    fn get_color_without_support_info(&self, levels: &ColorLevels) -> ColorVal {
        if self.optimistic_colors {
            if levels.2.is_some() {
                return levels.2.as_ref().unwrap().clone();
            }
            if levels.1.is_some() {
                return levels.1.as_ref().unwrap().clone();
            }
        }

        return levels.0.clone();
    }
}

///An RGB value
#[derive(Clone, Copy, Eq, PartialEq, PartialOrd, Hash, Debug, Default)]
pub struct RGB {
    pub r: u8,
    pub g: u8,
    pub b: u8,
}

impl RGB {
    ///Construct a new RGB value from the red, green, and blue components
    pub fn rgb(r: u8, g: u8, b: u8) -> Self {
        Self { r, g, b }
    }

    ///Construct a new RGB value from a hex string
    pub fn hex<S: AsRef<str>>(hex: S) -> Option<Self> {
        let s = hex.as_ref().trim_start_matches("#");
        if s.len() > 6 {
            return None;
        }

        let r = u8::from_str_radix(&s[0..=1], 16).ok()?;
        let g = u8::from_str_radix(&s[2..=3], 16).ok()?;
        let b = u8::from_str_radix(&s[4..=5], 16).ok()?;

        Some(Self { r, g, b })
    }
}

impl From<[u8; 3]> for RGB {
    fn from(value: [u8; 3]) -> Self {
        Self {
            r: value[0],
            g: value[1],
            b: value[2],
        }
    }
}

impl ANSICode for RGB {
    fn get_codes(&self, bg: Option<bool>) -> Vec<u32> {
        let mut codes: Vec<u32> = Vec::new();
        match bg {
            None | Some(false) => codes.push(38),
            Some(true) => codes.push(48),
        }

        codes.push(2);
        codes.push(self.r.into());
        codes.push(self.g.into());
        codes.push(self.b.into());

        codes
    }

    fn get_reset_code(&self, bg: Option<bool>) -> u32 {
        return ANSI16::Default.get_reset_code(bg);
    }
}

///The standard 16 terminal colors

#[derive(Clone, Copy, Eq, PartialEq, PartialOrd, Hash, Debug, Default)]
pub enum ANSI16 {
    Black,
    BrightBlack,
    Red,
    BrightRed,
    Green,
    BrightGreen,
    Yellow,
    BrightYellow,
    Blue,
    BrightBlue,
    Magenta,
    BrightMagenta,
    Cyan,
    BrightCyan,
    White,
    BrightWhite,
    #[default]
    Default,
}

impl ANSICode for ANSI16 {
    fn get_codes(&self, bg: Option<bool>) -> Vec<u32> {
        vec![match bg {
            Some(true) => self.get_background_code(),
            None | Some(false) => self.get_foreground_code(),
        }]
    }

    fn get_reset_code(&self, bg: Option<bool>) -> u32 {
        match bg {
            Some(true) => Self::Default.get_background_code(),
            Some(false) | None => Self::Default.get_foreground_code(),
        }
    }
}

impl ANSI16 {
    fn get_foreground_code(&self) -> u32 {
        match self {
            Self::Black => 30,
            Self::Red => 31,
            Self::Green => 32,
            Self::Yellow => 33,
            Self::Blue => 34,
            Self::Magenta => 35,
            Self::Cyan => 36,
            Self::White => 37,
            Self::BrightBlack => 90,
            Self::BrightRed => 91,
            Self::BrightGreen => 92,
            Self::BrightYellow => 93,
            Self::BrightBlue => 94,
            Self::BrightMagenta => 95,
            Self::BrightCyan => 96,
            Self::BrightWhite => 97,
            Self::Default => 39,
        }
    }

    fn get_background_code(&self) -> u32 {
        //Background values are just offset by 10
        self.get_foreground_code() + 10
    }
}

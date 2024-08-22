use super::ansi::ANSICode;
use crate::{style::Property, ANSI_ESCAPE};
use std::fmt::Display;

///Represents a single color in [ANSI16], ANSI256, or [RGB]

#[derive(Clone, Copy, Eq, PartialEq, PartialOrd, Hash, Debug)]
pub enum ColorVal {
    Base(ANSI16),
    Fixed(u8),
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
            Self::Fixed(c) => Self::get_ansi_256_codes(c, bg),
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

impl From<ANSI16> for ColorVal {
    fn from(value: ANSI16) -> Self {
        Self::Base(value)
    }
}

impl From<u8> for ColorVal {
    fn from(value: u8) -> Self {
        Self::Fixed(value)
    }
}

impl From<RGB> for ColorVal {
    fn from(value: RGB) -> Self {
        Self::RGB(value)
    }
}

///Holds [Color] representations for [ANSI16], ANSI256 and [RGB]
#[derive(Clone, Copy, Eq, PartialEq, PartialOrd, Hash, Debug, Default)]
pub struct ColorLevels(Option<ColorVal>, Option<ColorVal>, Option<ColorVal>);

impl From<ColorVal> for ColorLevels {
    fn from(value: ColorVal) -> Self {
        let mut ret_val = Self::default();
        match value {
            ColorVal::Base(_) => ret_val.0 = Some(value),
            ColorVal::Fixed(_) => ret_val.1 = Some(value),
            ColorVal::RGB(_) => ret_val.2 = Some(value),
        }

        ret_val
    }
}

///Color (optionally) represents color values for terminals with
///support for [ANSI16], ANSI256, or [RGB]. It can hold another set of [Color](Colors) for
///terminals with light backgrounds.
#[derive(Clone, Copy, Eq, PartialEq, PartialOrd, Hash, Debug, Default)]
pub struct Color {
    ///Default [ColorLevels] for dark backgrounds
    pub color: ColorLevels,
    ///Optional [ColorLevels] for light backgrounds
    pub light_color: Option<ColorLevels>,
    ///The support (if determined) of the terminal ([ANSI16], ANSI256, [RGB])
    pub color_support: Option<(bool, bool, bool)>,
    ///Determines behaviour if color support is unknown. Optimistic colors will always select the
    ///"highest" level of support.
    pub optimistic_colors: bool,
    ///Whether the background of the terminal is light or not.
    pub is_light_bg: Option<bool>,
}

impl<C: Into<ColorVal>> From<C> for Color {
    fn from(value: C) -> Self {
        let mut ret_val = Self::default();
        ret_val.color = ColorLevels::from(value.into());

        ret_val
    }
}

impl Color {
    pub fn new<C: Into<ColorVal>>(color: C) -> Self {
        Self {
            color: ColorLevels::from(color.into()),
            light_color: None,
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
            ColorVal::Fixed(_) => self.color.1 = Some(color),
            ColorVal::Base(_) => self.color.0 = Some(color),
        }
        self
    }

    pub fn with_light_color(mut self, color: ColorVal) -> Self {
        let mut range = self.light_color.unwrap_or_default();
        match color {
            ColorVal::RGB(_) => range.0 = Some(color),
            ColorVal::Fixed(_) => range.1 = Some(color),
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

    pub fn with_bg_light(mut self, light: Option<bool>) -> Self {
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

        return color.0.unwrap_or_default().get_codes(bg);
    }

    fn get_color_without_support_info(&self, levels: &ColorLevels) -> ColorVal {
        if self.optimistic_colors {
            if levels.2.is_some() {
                return levels.2.as_ref().unwrap().clone();
            }
            if levels.1.is_some() {
                return levels.1.as_ref().unwrap().clone();
            }
            if levels.0.is_some() {
                return levels.0.as_ref().unwrap().clone();
            }
        } else {
            if levels.0.is_some() {
                return levels.0.as_ref().unwrap().clone();
            }
            if levels.1.is_some() {
                return levels.1.as_ref().unwrap().clone();
            }
            if levels.2.is_some() {
                return levels.2.as_ref().unwrap().clone();
            }
        }

        return levels.0.unwrap_or_default().clone();
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

#[cfg(test)]
mod color_tests {
    use super::*;

    fn create_test_color() -> Color {
        Color::new(ANSI16::Red).with_colors(ColorLevels(
            Some(ANSI16::Red.into()),
            Some(160.into()),
            Some(RGB::rgb(255, 0, 0).into()),
        ))
    }

    #[test]
    fn test_optimistic_colors() {
        let mut color = create_test_color().with_optimistic_colors(true);

        //Should return RGB color
        assert!(color.get_codes(Some(false)).len() == 5);

        color.optimistic_colors = false;

        //Should return ANSI16 color
        assert!(color.get_codes(Some(false)).len() == 1);
    }

    #[test]
    fn test_color_degradation() {
        let mut color = create_test_color().with_color_support((true, true, false));

        assert!(color.get_codes(Some(false)).len() == 3);

        color.color_support = Some((true, true, true));

        assert!(color.get_codes(Some(false)).len() == 5);

        color.color_support = Some((true, false, false));

        assert!(color.get_codes(Some(false)).len() == 1);
    }

    #[test]
    fn test_rgb_from_hex() {
        let rgb_lime = RGB::hex("#a6fc35");
        assert_eq!(rgb_lime.unwrap(), RGB::rgb(166, 252, 53));

        let dark_blue = RGB::hex("#021e5b");
        assert_eq!(dark_blue.unwrap(), RGB::rgb(2, 30, 91));
    }

    #[test]
    fn test_rgb_codes() {
        let val = ColorVal::RGB(RGB::rgb(166, 252, 53));

        let fg_codes = val.get_codes(Some(false));
        let bg_codes = val.get_codes(Some(true));

        assert_eq!(fg_codes, vec![38, 2, 166, 252, 53]);
        assert_eq!(bg_codes, vec![48, 2, 166, 252, 53]);
    }

    #[test]
    fn test_fixed_codes() {
        let val = ColorVal::Fixed(32);

        let fg_codes = val.get_codes(Some(false));
        let bg_codes = val.get_codes(Some(true));

        assert_eq!(fg_codes, vec![38, 5, 32]);
        assert_eq!(bg_codes, vec![48, 5, 32]);
    }
}

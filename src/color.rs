use super::ansi::ANSICode;
use crate::{style::Property, ANSI_ESCAPE};
use std::fmt::Display;

#[derive(Clone)]
enum ColorSupport {
    ANSI16,
    ANSI256,
    TrueColor,
}

#[derive(Clone, Default)]
pub struct Color<C: ANSICode> {
    color: C,
}

impl<C: ANSICode> ANSICode for Color<C> {
    fn get_codes(&self, bg: Option<bool>) -> Vec<u32> {
        self.color.get_codes(bg)
    }

    fn get_reset_code(&self) -> u32 {
        self.color.get_reset_code()
    }
}

impl<C: ANSICode> Color<C> {
    fn new(color: C) -> Self {
        Self { color }
    }

    fn set(&mut self, color: C) {
        self.color = color;
    }
}

///Color that can change based on a boolean value representing the determined color of the terminal
///background (defaults to dark)
#[derive(Clone, Default)]
pub struct AdaptiveColor<T>
where
    T: ANSICode,
{
    is_dark: Option<bool>,
    light: T,
    dark: T,
}

impl AdaptiveColor<ANSI16> {
    pub fn default() -> Self {
        return Self {
            is_dark: None,
            light: ANSI16::White,
            dark: ANSI16::Black,
        };
    }
}

impl AdaptiveColor<ANSI256> {
    pub fn default() -> Self {
        return Self {
            is_dark: None,
            light: ANSI256::Grey0,
            dark: ANSI256::Grey0,
        };
    }
}

impl AdaptiveColor<TrueColor> {
    pub fn default() -> Self {
        return Self {
            is_dark: None,
            light: TrueColor::default(),
            dark: TrueColor::default(),
        };
    }
}

impl<T: ANSICode> AdaptiveColor<T> {
    pub fn new(light: T, dark: T, is_dark: Option<bool>) -> Self {
        Self {
            is_dark,
            light,
            dark,
        }
    }

    pub fn with_light_color(mut self, color: T) -> Self {
        self.light = color;
        return self;
    }

    pub fn with_dark_color(mut self, color: T) -> Self {
        self.dark = color;
        return self;
    }

    pub fn with_terminal_dark(mut self, b: Option<bool>) -> Self {
        self.is_dark = b;
        return self;
    }
}

impl<C: ANSICode> ANSICode for AdaptiveColor<C> {
    fn get_codes(&self, bg: Option<bool>) -> Vec<u32> {
        match self.is_dark() {
            true => self.dark.get_codes(bg),
            false => self.light.get_codes(bg),
        }
    }

    fn get_reset_code(&self) -> u32 {
        return Property::Reset.get_reset_code();
    }
}

impl<C: ANSICode> AdaptiveColor<C> {
    fn is_dark(&self) -> bool {
        match self.is_dark {
            Some(true) | None => true,
            Some(false) => false,
        }
    }
}

#[derive(Clone, Default)]
pub struct MultiColor {
    truecolor: TrueColor,
    ansi256: ANSI256,
    ansi16: ANSI16,

    detected_support_level: Option<ColorSupport>,
}

impl ANSICode for MultiColor {
    fn get_codes(&self, bg: Option<bool>) -> Vec<u32> {
        if self.detected_support_level.is_none() {
            return self.ansi16.get_codes(bg);
        }
        match self.detected_support_level.clone().unwrap() {
            ColorSupport::TrueColor => self.truecolor.get_codes(bg),
            ColorSupport::ANSI256 => self.ansi256.get_codes(bg),
            ColorSupport::ANSI16 => self.ansi16.get_codes(bg),
        }
    }

    fn get_reset_code(&self) -> u32 {
        if self.detected_support_level.is_none() {
            return self.ansi16.get_reset_code();
        }
        match self.detected_support_level.clone().unwrap() {
            ColorSupport::TrueColor => self.truecolor.get_reset_code(),
            ColorSupport::ANSI256 => self.ansi256.get_reset_code(),
            ColorSupport::ANSI16 => self.ansi16.get_reset_code(),
        }
    }
}

#[derive(Clone, Default)]
pub struct AdaptiveMultiColor {
    color: AdaptiveColor<MultiColor>,
}

impl ANSICode for AdaptiveMultiColor {
    fn get_codes(&self, bg: Option<bool>) -> Vec<u32> {
        return self.color.get_codes(bg);
    }

    fn get_reset_code(&self) -> u32 {
        return self.color.get_reset_code();
    }
}

#[derive(Clone, Default)]
pub struct RGB {
    pub r: u8,
    pub g: u8,
    pub b: u8,
}

impl RGB {
    pub fn rgb(r: u8, g: u8, b: u8) -> Self {
        Self { r, g, b }
    }

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

#[derive(Clone, Default)]
pub struct TrueColor(RGB);

impl ANSICode for TrueColor {
    fn get_codes(&self, bg: Option<bool>) -> Vec<u32> {
        let mut codes: Vec<u32> = Vec::new();
        match bg {
            None | Some(false) => codes.push(38),
            Some(true) => codes.push(48),
        }

        codes.push(2);
        codes.push(self.0.r.into());
        codes.push(self.0.g.into());
        codes.push(self.0.b.into());

        codes
    }

    fn get_reset_code(&self) -> u32 {
        return Property::Reset.get_reset_code();
    }
}

impl TrueColor {
    fn new(color: RGB) -> Self {
        Self(color)
    }
}

//This was painful to do. Anyone who uses the ANSI256 color names should pay me
//money for my toil
#[repr(u8)]
#[derive(Clone)]
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
    Default,
    ///Resets all colors and styles
    Reset,
}

impl Default for ANSI16 {
    fn default() -> Self {
        Self::Default
    }
}

impl ANSICode for ANSI16 {
    fn get_codes(&self, bg: Option<bool>) -> Vec<u32> {
        vec![match bg {
            Some(true) => self.get_background_code(),
            None | Some(false) => self.get_foreground_code(),
        }]
    }

    fn get_reset_code(&self) -> u32 {
        return Property::Reset.get_reset_code();
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
            Self::Reset => 0,
        }
    }

    fn get_background_code(&self) -> u32 {
        //Background values are just offset by 10
        self.get_foreground_code() + 10
    }
}

#[repr(u8)]
#[derive(Clone)]
pub enum ANSI256 {
    ANSI16(ANSI16),
    Grey0 = 16,
}

impl Default for ANSI256 {
    fn default() -> Self {
        Self::ANSI16(ANSI16::White)
    }
}

impl ANSICode for ANSI256 {
    fn get_codes(&self, bg: Option<bool>) -> Vec<u32> {
        todo!()
    }

    fn get_reset_code(&self) -> u32 {
        Property::Reset.get_reset_code()
    }
}

#[cfg(test)]
mod color_tests {
    use super::*;
}

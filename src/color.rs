use crate::ANSI_ESCAPE;

pub trait Color: Clone {
    fn get_escape_code(&self, background: bool) -> String;
}

#[derive(Clone)]
enum ColorSupport {
    ANSI16,
    ANSI256,
    TrueColor,
}

///Color that can change based on a boolean value representing the determined color of the terminal
///background (defaults to dark)
#[derive(Clone)]
pub struct AdaptiveColor<T>
where
    T: Color,
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

impl<T: Color> AdaptiveColor<T> {
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

impl<T: Color> Color for AdaptiveColor<T> {
    fn get_escape_code(&self, background: bool) -> String {
        if self.is_dark.is_some_and(|v| v == false) {
            return self.light.get_escape_code(background);
        }

        return self.dark.get_escape_code(background);
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

#[derive(Clone, Default)]
pub struct TrueColor(RGB);

impl Color for TrueColor {
    fn get_escape_code(&self, bg: bool) -> String {
        let start_code: &str;
        match bg {
            true => start_code = "[38;2;",
            false => start_code = "[48;2;",
        }

        let rgb = &self.0;
        return format!(
            "{}{}{};{};{}m",
            ANSI_ESCAPE, start_code, rgb.r, rgb.g, rgb.b
        );
    }
}

#[derive(Clone)]
pub struct MultiColor {
    truecolor: TrueColor,
    ansi256: ANSI256,
    ansi16: ANSI16,

    detected_support_level: Option<ColorSupport>,
}

impl Color for MultiColor {
    fn get_escape_code(&self, background: bool) -> String {
        if self.detected_support_level.is_none() {
            return self.ansi16.get_escape_code(background);
        }
        match self.detected_support_level.clone().unwrap() {
            ColorSupport::TrueColor => self.truecolor.get_escape_code(background),
            ColorSupport::ANSI256 => self.ansi256.get_escape_code(background),
            ColorSupport::ANSI16 => self.ansi16.get_escape_code(background),
        }
    }
}

#[derive(Clone)]
pub struct AdaptiveMultiColor {
    light: MultiColor,
    dark: MultiColor,
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

impl Color for ANSI16 {
    fn get_escape_code(&self, background: bool) -> String {
        match background {
            true => format!("{}[{}m", ANSI_ESCAPE, self.get_background_code_num()),
            false => format!("{}[{}m", ANSI_ESCAPE, self.get_foreground_code_num()),
        }
    }
}

impl ANSI16 {
    fn get_foreground_code_num(&self) -> u32 {
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

    fn get_background_code_num(&self) -> u32 {
        match self {
            Self::Black => 40,
            Self::Red => 41,
            Self::Green => 42,
            Self::Yellow => 43,
            Self::Blue => 44,
            Self::Magenta => 45,
            Self::Cyan => 46,
            Self::White => 47,
            Self::BrightBlack => 100,
            Self::BrightRed => 101,
            Self::BrightGreen => 102,
            Self::BrightYellow => 103,
            Self::BrightBlue => 104,
            Self::BrightMagenta => 105,
            Self::BrightCyan => 106,
            Self::BrightWhite => 107,
            Self::Default => 49,
            Self::Reset => 0,
        }
    }
}

#[repr(u8)]
#[derive(Clone)]
pub enum ANSI256 {
    ANSI16(ANSI16),
    Grey0 = 16,
}

impl Color for ANSI256 {
    fn get_escape_code(&self, background: bool) -> String {
        match self {
            Self::ANSI16(c) => c.get_escape_code(background),
            _ => todo!(),
        }
    }
}

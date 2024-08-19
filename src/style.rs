use std::{
    borrow::Cow,
    fmt::{Display, Write},
};

use crate::color::Color;

use super::ansi::ANSICode;
use super::display::StyledString;

pub trait Styleable<S> {
    ///Converts any string type to a new styled string. Resets a styled strings style if called on
    ///one.
    fn to_styled(&self) -> StyledString;
}

impl<S: Display> Styleable<S> for S {
    fn to_styled(&self) -> StyledString {
        return StyledString::new(self.to_string());
    }
}

#[derive(Clone, Default)]
pub struct Style {
    pub foreground: Color,
    pub background: Color,

    properties: Vec<Property>,

    pub prefix_with_reset: bool,
}

impl<'a> Style {
    pub fn with_foreground(mut self, foreground: Color) -> Self {
        self.foreground = foreground;
        self
    }

    pub fn with_background(mut self, background: Color) -> Self {
        self.background = background;
        self
    }

    pub fn with_property(mut self, prop: Property) -> Self {
        if !&self.properties.contains(&prop) {
            self.properties.push(prop);
        }
        self
    }

    pub fn reset_style(&mut self) {
        self.properties.clear();
        self.foreground = Color::default();
        self.background = Color::default();
    }

    pub fn get_properties(&self) -> Cow<Vec<Property>> {
        return Cow::Borrowed(&self.properties);
    }
}

#[derive(Clone, PartialEq)]
pub enum Property {
    Reset,
    Bold,
    Dim,
    Italic,
    Underline,
    Blinking,
    Inverse,
    Hidden,
    Strikethrough,
}

impl Default for Property {
    fn default() -> Self {
        Self::Reset
    }
}

impl ANSICode for Property {
    fn get_codes(&self, _bg: Option<bool>) -> Vec<u32> {
        vec![match self {
            Self::Reset => 0,
            Self::Bold => 1,
            Self::Dim => 2,
            Self::Italic => 3,
            Self::Underline => 4,
            Self::Blinking => 5,
            Self::Inverse => 7,
            Self::Hidden => 8,
            Self::Strikethrough => 9,
        }]
    }

    fn get_reset_code(&self, _bg: Option<bool>) -> u32 {
        match self {
            Self::Reset => 0,
            Self::Bold | Self::Dim => 22,
            //With the exception of bold and dim, the reset codes for properties are just offset
            //by 20
            _ => self.get_codes(None)[0] + 20,
        }
    }
}

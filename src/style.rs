use std::{
    borrow::Cow,
    fmt::{Display, Write},
};

use crate::color::Color;

use super::ansi::ANSICode;
use super::display::StyledString;

pub trait Styleable<S> {
    ///Converts to a new styled string. Resets a styled strings style if called on
    ///one.
    fn to_styled_string(&self) -> StyledString;

    ///Converts to a styled string with an associated style
    fn with_style(&self, style: Style) -> StyledString;
}

impl<S: Display> Styleable<S> for S {
    fn to_styled_string(&self) -> StyledString {
        return StyledString::new(self.to_string(), None);
    }

    fn with_style(&self, style: Style) -> StyledString {
        return StyledString::new(self.to_string(), Some(style));
    }
}

///A style is a combination of foreground, background, and a list of properties
#[derive(Clone, Default)]
pub struct Style {
    pub foreground: Color,
    pub background: Color,

    properties: Vec<Property>,

    pub prefix_with_reset: bool,
}

impl<'a> Style {
    ///Set the foreground
    pub fn with_foreground<C: Into<Color>>(mut self, foreground: C) -> Self {
        self.foreground = foreground.into();
        self
    }

    ///Set the background
    pub fn with_background<C: Into<Color>>(mut self, background: C) -> Self {
        self.background = background.into();
        self
    }

    ///Set a property
    pub fn with_property(mut self, prop: Property) -> Self {
        if !&self.properties.contains(&prop) {
            self.properties.push(prop);
        }
        self
    }

    pub fn add_property(&mut self, prop: Property) {
        if !&self.properties.contains(&prop) {
            self.properties.push(prop);
        }
    }

    pub fn remove_property(&mut self, prop: Property) {
        self.properties = self
            .properties
            .iter()
            .cloned()
            .filter(|v| return *v != prop)
            .collect()
    }

    pub fn toggle_property(&mut self, prop: Property) {
        if self.properties.contains(&prop) {
            self.remove_property(prop)
        } else {
            self.add_property(prop)
        }
    }

    ///Reset the style
    pub fn reset_style(&mut self) {
        self.properties.clear();
        self.foreground = Color::default();
        self.background = Color::default();
    }

    pub fn get_properties(&self) -> Cow<Vec<Property>> {
        return Cow::Borrowed(&self.properties);
    }
}

///A property represents things like bold text, underlines, etc.
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

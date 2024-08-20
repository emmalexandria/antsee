use crate::{
    color::{Color, ANSI16},
    Property,
};

use super::{
    ansi::ANSICode,
    style::{Style, Styleable},
};

use std::fmt::Display;

///A string with an associated [Style]
#[derive(Clone, Eq, PartialEq, PartialOrd, Hash, Debug)]
pub struct StyledString {
    pub content: String,
    pub style: Style,
}

impl Display for StyledString {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(&self.style.paint(&self.content))
    }
}

impl AsRef<str> for StyledString {
    fn as_ref(&self) -> &str {
        return &self.content;
    }
}

impl StyledString {
    pub fn new<S: AsRef<str>>(content: S, style: Option<Style>) -> Self {
        Self {
            content: content.as_ref().to_string(),
            style: style.unwrap_or_default(),
        }
    }

    pub fn with_foreground<C: Into<Color>>(mut self, color: C) -> Self {
        self.style.foreground = color.into();
        self
    }

    pub fn with_background<C: Into<Color>>(mut self, color: C) -> Self {
        self.style.background = color.into();
        self
    }

    pub fn with_property(mut self, prop: Property) -> Self {
        self.style.add_property(prop);
        self
    }

    pub fn with_properties<V: Into<Vec<Property>>>(mut self, props: V) -> Self {
        for p in props.into() {
            self.style.add_property(p);
        }
        self
    }

    pub fn foreground<C: Into<Color>>(&mut self, color: C) {
        self.style.foreground = color.into();
    }

    pub fn background<C: Into<Color>>(&mut self, color: C) {
        self.style.foreground = color.into();
    }
}

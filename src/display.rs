use crate::color::{Color, ANSI16};

use super::{
    ansi::ANSICode,
    style::{Style, Styleable},
};

use std::fmt::Display;

pub struct StyledString {
    content: String,
    style: Style,
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
    pub fn new<S: AsRef<str>>(content: S) -> Self {
        Self {
            content: content.as_ref().to_string(),
            style: Style::default(),
        }
    }

    pub fn foreground(mut self, color: Color) -> Self {
        self.style.foreground = color;
        self
    }

    pub fn background(mut self, color: Color) -> Self {
        self.style.background = color;
        self
    }
}

pub struct StyledStringList {
    content: Vec<StyledString>,
}

impl StyledString {}

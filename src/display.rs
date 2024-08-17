use crate::color::{Color, ANSI16};

use super::{
    ansi::ANSICode,
    style::{Style, Styleable},
};

use std::fmt::Display;

pub struct StyledString<C: ANSICode> {
    content: String,
    style: Style<C>,
}

impl<C: ANSICode> Display for StyledString<C> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(&self.style.paint(&self.content))
    }
}

impl<C: ANSICode> AsRef<str> for StyledString<C> {
    fn as_ref(&self) -> &str {
        return &self.content;
    }
}

impl<C: ANSICode> StyledString<C> {
    pub fn new<S: AsRef<str>>(content: S) -> Self {
        Self {
            content: content.as_ref().to_string(),
            style: Style::default(),
        }
    }

    pub fn foreground(mut self, color: Color<C>) -> Self {
        self.style.foreground = color;
        self
    }

    pub fn background(mut self, color: Color<C>) -> Self {
        self.style.background = color;
        self
    }
}

pub struct StyledStringList<C: ANSICode> {
    content: Vec<StyledString<C>>,
}

impl<C: ANSICode> StyledString<C> {}

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

    pub fn with_fg<C: Into<Color>>(mut self, color: C) -> Self {
        self.style.foreground = color.into();
        self
    }

    pub fn with_bg<C: Into<Color>>(mut self, color: C) -> Self {
        self.style.background = color.into();
        self
    }

    pub fn with_prop(mut self, prop: Property) -> Self {
        self.style.add_prop(prop);
        self
    }

    pub fn with_props<V: Into<Vec<Property>>>(mut self, props: V) -> Self {
        for p in props.into() {
            self.style.add_prop(p);
        }
        self
    }

    pub fn fg<C: Into<Color>>(&mut self, color: C) {
        self.style.foreground = color.into();
    }

    pub fn bg<C: Into<Color>>(&mut self, color: C) {
        self.style.foreground = color.into();
    }

    pub fn dim(mut self) -> StyledString {
        self.style.add_prop(Property::Dim);
        self
    }

    pub fn bold(mut self) -> StyledString {
        self.style.add_prop(Property::Bold);
        self
    }

    pub fn italic(mut self) -> StyledString {
        self.style.add_prop(Property::Italic);
        self
    }

    pub fn underline(mut self) -> StyledString {
        self.style.add_prop(Property::Underline);
        self
    }

    pub fn strikethrough(mut self) -> StyledString {
        self.style.add_prop(Property::Strikethrough);
        self
    }

    pub fn hidden(mut self) -> StyledString {
        self.style.add_prop(Property::Hidden);
        self
    }

    pub fn inverse(mut self) -> StyledString {
        self.style.add_prop(Property::Inverse);
        self
    }

    pub fn blinking(mut self) -> StyledString {
        self.style.add_prop(Property::Blinking);
        self
    }
}

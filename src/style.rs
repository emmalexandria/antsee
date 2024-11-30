/*!


*/

use std::{borrow::Cow, fmt::Display, rc::Rc};

mod attributes;

pub use attributes::Attribute;
pub use attributes::Attributes;

use crate::color::{Color, ColorValue};

#[derive(Default, Clone, Debug)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
///Defines a text style consisting of a foreground [Color], background [Color], and [Attributes]
pub struct Style {
    ///Text foreground color. Default if set to [None]
    pub fg: Option<Color>,
    ///Text background color. Default if set to [None]
    pub bg: Option<Color>,
    ///Text attributes such as bold and underline
    pub attributes: Attributes,
}

impl Style {
    ///Create a new Style struct
    pub fn new<C: ColorValue>(
        fg: Option<C>,
        bg: Option<C>,
        attributes: Option<Attributes>,
    ) -> Self {
        let ffg = fg.unwrap_or_default();
        let bgf = bg.unwrap_or_default();
        Self {
            fg: Some(ffg.into()),
            bg: Some(bgf.into()),
            attributes: attributes.unwrap_or_default(),
        }
    }
    ///Set the text foreground color
    pub fn fg<C: ColorValue>(mut self, fg: C) -> Self {
        self.fg = Some(fg.into());
        self
    }
    ///Set the text background color
    pub fn bg<C: ColorValue>(mut self, bg: C) -> Self {
        self.bg = Some(bg.into());
        self
    }

    ///Set the attributes of the style while still receiving self back
    pub fn attributes(mut self, attributes: Attributes) -> Self {
        self.attributes = attributes;
        self
    }
}

impl Display for Style {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        if let Some(foreground) = self.fg.clone() {
            writeln!(f, "Foreground: {:#}", foreground)?;
        }
        if let Some(background) = self.bg.clone() {
            writeln!(f, "Background: {:#}", background)?;
        }
        write!(f, "{:#}", self.attributes)
    }
}

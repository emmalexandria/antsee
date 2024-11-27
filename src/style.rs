use crate::color::{Color, ColorValue};

#[derive(Default, Clone, Copy, Debug)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
///Basic style struct holding text color and attributes
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

#[derive(Default, Clone, Copy, PartialEq, Debug)]
#[cfg_attr(
    feature = "serde",
    derive(serde::Serialize, serde::Deserialize),
    serde(default)
)]
///Text attributes. Control how text is rendered besides color
pub struct Attributes {
    ///Whether text is bold
    pub bold: bool,
    ///Whether text is dimmed
    pub dimmed: bool,
    ///Whether text is italic
    pub italic: bool,
    ///Whether text is underlined
    pub underline: bool,
    ///Whether text is blinking
    pub blink: bool,
    ///Whether text has reversed colors
    pub reverse: bool,
    ///Whether text is hidden
    pub hidden: bool,
    ///Whether text is struck through
    pub strikethrough: bool,
}

impl Attributes {
    ///Create an empty attributes object
    pub fn new() -> Attributes {
        Attributes::default()
    }
    ///Reset all attributes to false
    pub fn reset(&mut self) {
        self.bold = false;
        self.dimmed = false;
        self.italic = false;
        self.underline = false;
        self.blink = false;
        self.reverse = false;
        self.hidden = false;
        self.strikethrough = false;
    }
    ///Returns whether all attributes are false
    pub fn is_plain(&self) -> bool {
        !self.bold
            && !self.dimmed
            && !self.italic
            && !self.underline
            && !self.blink
            && !self.reverse
            && !self.hidden
            && !self.strikethrough
    }

    ///Set the bold attribute to true
    pub fn bold(mut self) -> Self {
        self.bold = true;
        self
    }
    ///Set the dimmed attribute to true
    pub fn dim(mut self) -> Self {
        self.dimmed = true;
        self
    }
    ///Set the italic attribute to true
    pub fn italic(mut self) -> Self {
        self.italic = true;
        self
    }
    ///Set the underline attribute to true
    pub fn underline(mut self) -> Self {
        self.underline = true;
        self
    }
    ///Set the blink attribute to true
    pub fn blink(mut self) -> Self {
        self.blink = true;
        self
    }
    ///Set the reverse attribute to true
    pub fn reverse(mut self) -> Self {
        self.reverse = true;
        self
    }
    ///Set the strikethrough attribute to true
    pub fn strikethrough(mut self) -> Self {
        self.strikethrough = true;
        self
    }
}

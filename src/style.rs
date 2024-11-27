use crate::color::Color;

#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct Style {
    pub fg: Option<Color>,
    pub bg: Option<Color>,
    pub attributes: Attributes,
}

#[derive(Default, Clone, Copy, PartialEq)]
#[cfg_attr(
    feature = "serde",
    derive(serde::Serialize, serde::Deserialize),
    serde(default)
)]
pub struct Attributes {
    pub bold: bool,
    pub dimmed: bool,
    pub italic: bool,
    pub underline: bool,
    pub blink: bool,
    pub reverse: bool,
    pub hidden: bool,
    pub strikethrough: bool,
}

impl Attributes {
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
}

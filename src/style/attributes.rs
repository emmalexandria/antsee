use std::fmt::Display;

///Attribute represents a boolean text attribute
#[derive(Default, Clone, Copy, PartialEq, Debug)]
pub struct Attribute(pub bool);

impl Attribute {
    pub fn new(value: bool) -> Self {
        Self(value)
    }

    pub fn set(mut self, val: bool) -> Self {
        self.0 = val;
        self
    }

    pub fn on(mut self) -> Self {
        self.0 = true;
        self
    }

    pub fn off(mut self) -> Self {
        self.0 = false;
        self
    }
}

#[cfg(feature = "serde")]
impl serde::Serialize for Attribute {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_bool(self.0)
    }
}

#[cfg(feature = "serde")]
impl<'de> serde::Deserialize<'de> for Attribute {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        struct AttributeVisitor;

        impl<'de> serde::de::Visitor<'de> for AttributeVisitor {
            type Value = Attribute;
            fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
                write!(formatter, "Expecting boolean")
            }

            fn visit_bool<E>(self, v: bool) -> Result<Self::Value, E>
            where
                E: serde::de::Error,
            {
                return Ok(Attribute(v));
            }
        }
        deserializer.deserialize_bool(AttributeVisitor)
    }
}

#[cfg_attr(feature = "serde", derive(serde::Deserialize, serde::Serialize))]
#[derive(Debug, Clone, PartialEq, Default)]
///Stores text attributes like bold, strikethrough, etc.
pub struct Attributes {
    ///Whether text is bold
    pub bold: Attribute,
    ///Whether text is dimmed
    pub dimmed: Attribute,
    ///Whether text is italic
    pub italic: Attribute,
    ///Whether text is underlined
    pub underline: Attribute,
    ///Whether text is blinking
    pub blink: Attribute,
    ///Whether text has reversed colors
    pub reverse: Attribute,
    ///Whether text is hidden
    pub hidden: Attribute,
    ///Whether text is struck through
    pub strikethrough: Attribute,
}

impl Attributes {
    ///Create an empty attributes object
    pub fn new() -> Attributes {
        Attributes::default()
    }
    ///Reset all attributes to false
    pub fn reset(&mut self) {
        self.bold = Attribute::new(false);
        self.dimmed = Attribute::new(false);
        self.italic = Attribute::new(false);
        self.underline = Attribute::new(false);
        self.blink = Attribute::new(false);
        self.reverse = Attribute::new(false);
        self.hidden = Attribute::new(false);
        self.strikethrough = Attribute::new(false);
    }
    ///Returns whether all attributes are false
    pub fn is_plain(&self) -> bool {
        !self.bold.0
            && !self.dimmed.0
            && !self.italic.0
            && !self.underline.0
            && !self.blink.0
            && !self.reverse.0
            && !self.hidden.0
            && !self.strikethrough.0
    }

    pub fn list(&self) -> Vec<&Attribute> {
        return vec![
            &self.bold,
            &self.dimmed,
            &self.italic,
            &self.underline,
            &self.blink,
            &self.reverse,
            &self.hidden,
            &self.strikethrough,
        ];
    }

    ///Set the bold attribute to true
    pub fn bold(mut self) -> Self {
        self.bold = self.bold.on();
        self
    }
    ///Set the dimmed attribute to true
    pub fn dim(mut self) -> Self {
        self.dimmed = self.dimmed.on();
        self
    }
    ///Set the italic attribute to true
    pub fn italic(mut self) -> Self {
        self.italic = self.italic.on();
        self
    }
    ///Set the underline attribute to true
    pub fn underline(mut self) -> Self {
        self.underline = self.underline.on();
        self
    }
    ///Set the blink attribute to true
    pub fn blink(mut self) -> Self {
        self.blink = self.blink.on();
        self
    }
    ///Set the reverse attribute to true
    pub fn reverse(mut self) -> Self {
        self.reverse = self.reverse.on();
        self
    }
    ///Set the strikethrough attribute to true
    pub fn strikethrough(mut self) -> Self {
        self.strikethrough = self.strikethrough.on();
        self
    }
}

impl Display for Attributes {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        fn display_if_true(
            attribute: &Attribute,
            name: &str,
            f: &mut std::fmt::Formatter<'_>,
        ) -> std::fmt::Result {
            if attribute.0 {
                writeln!(f, "{}:{}", name, attribute.0)?;
            }
            Ok(())
        }
        display_if_true(&self.bold, "Bold", f)?;
        display_if_true(&self.italic, "Italic", f)?;
        display_if_true(&self.underline, "Underlined", f)?;
        display_if_true(&self.hidden, "Hidden", f)?;
        display_if_true(&self.strikethrough, "Struckthrough", f)?;
        display_if_true(&self.reverse, "Reversed", f)?;
        display_if_true(&self.blink, "Blinking", f)?;
        display_if_true(&self.dimmed, "Dimmed", f)?;

        Ok(())
    }
}

use std::fmt::Display;
use std::rc::Rc;
use std::{borrow::Cow, str::FromStr};

use super::libraries::ColorLibrary;
use super::libraries::XtermColors;
use super::{Color, ColorFromStrError, ColorSource, ColorValue, Source};

///Ansi256 color value represented by a u8. Can be created from [XtermColors] name with
///[FromStr].
#[derive(Clone, Debug, Default, PartialEq)]
pub struct Fixed(pub u8, Source<Rc<str>>);

impl Fixed {
    /// Create a new Ansi256 from u8
    pub fn new() -> Self {
        Self(0, Source::Inactive(Rc::from("")))
    }

    ///Create a new Ansi256 from XtermColors
    pub fn from(color: XtermColors) -> Self {
        Self(
            color.ansi256(),
            Source::Active(Rc::from(color.color_name())),
        )
    }

    /// Set the u8 color code of the color
    pub fn set_code(&mut self, val: u8) {
        self.0 = val
    }

    /// Set the [XtermColors] of the color
    pub fn set_color(&mut self, color: XtermColors) {
        self.0 = color.ansi256();
        self.1 = Source::Active(Rc::from(color.color_name()));
    }

    ///Set the [XtermColors] of the color
    pub fn color(mut self, color: XtermColors) -> Self {
        self.set_color(color);
        self
    }

    ///Set the u8 color code of the color
    pub fn code(mut self, code: u8) -> Self {
        self.set_code(code);
        self
    }

    ///Get the [XtermColors] of the color
    pub fn get_color(&self) -> XtermColors {
        XtermColors::get_ansi256(self.0)
    }
}

impl Display for Fixed {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:?}", self)
    }
}

impl ColorSource for Fixed {
    type ExternalSource = String;
    fn set_external_source(&mut self, value: Self::ExternalSource) {
        self.1 = Source::Active(Rc::from(value))
    }
    fn source_external(&mut self) {
        self.1 = self.1.clone().active()
    }
    fn source_internal(&mut self) {
        self.1 = self.1.clone().active()
    }
}

impl From<XtermColors> for Fixed {
    fn from(value: XtermColors) -> Self {
        Self(
            value.ansi256(),
            Source::Active(Rc::from(value.color_name())),
        )
    }
}

impl ColorValue for Fixed {}

impl FromStr for Fixed {
    type Err = ColorFromStrError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        if s.starts_with("xterm(") && s.ends_with(")") {
            let color_name = XtermColors::unwrap_name(s);
            let xterm_color = XtermColors::get_name(color_name);
            if let Some(color) = xterm_color {
                return Ok(Self(
                    color.ansi256(),
                    Source::Active(Rc::from(color.color_name())),
                ));
            } else {
                return Err(ColorFromStrError::InvalidName);
            }
        }
        Err(ColorFromStrError::InvalidString)
    }
}

#[cfg(feature = "serde")]
impl serde::Serialize for Fixed {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        if let Source::Active(s) = self.1.clone() {
            return serializer.serialize_str(&XtermColors::wrap_name(&s));
        }
        serializer.serialize_u8(self.0)
    }
}

#[cfg(feature = "serde")]
struct FixedVisitor;

#[cfg(feature = "serde")]
impl<'de> serde::de::Visitor<'de> for FixedVisitor {
    type Value = Fixed;
    fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(formatter, "Expecting either an xterm color name or a u8")
    }

    fn visit_u8<E>(self, v: u8) -> Result<Self::Value, E>
    where
        E: serde::de::Error,
    {
        Ok(Fixed(v, Source::Inactive(Rc::from(""))))
    }

    fn visit_str<E>(self, v: &str) -> Result<Self::Value, E>
    where
        E: serde::de::Error,
    {
        if v.trim().starts_with("xterm(") && v.trim().ends_with(")") {
            let name = &v[6..v.len() - 1];
            if let Some(color) = XtermColors::get_name(name) {
                return Ok(Fixed(
                    color.ansi256(),
                    Source::Active(Rc::from(color.color_name())),
                ));
            }
        }
        Err(E::custom("Invalid xterm theme"))
    }
}

#[cfg(feature = "serde")]
impl<'de> serde::Deserialize<'de> for Fixed {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        deserializer.deserialize_str(FixedVisitor)
    }
}

#[cfg(test)]
mod fixed_tests {
    use serde_test::{assert_de_tokens, assert_ser_tokens, assert_tokens, Token};

    use super::*;
    #[test]
    fn test_fixed_serialize_u8() {
        let mut fixed = Fixed::new();
        fixed.0 = 5;
        assert_tokens(&fixed, &[Token::U8(5)])
    }

    #[test]
    fn test_fixed_serialize_source() {
        let fixed = Fixed::new().color(XtermColors::Seafoam);
        assert_tokens(&fixed, &[Token::Str("xterm(Seafoam)")]);
    }
}

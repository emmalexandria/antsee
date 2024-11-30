use std::str::FromStr;

use super::{ColorFromStrError, ColorValue};

#[derive(Clone, Copy, PartialEq, Eq, Debug, Default)]
pub enum Ansi {
    #[default]
    /// Default color (foreground code `39`, background code `49`)
    Default,
    ///Color #0 (foreground code `30`, background code `40`)
    ///
    ///This is not necessarily the background color
    Black,
    /// Color #0 (foreground code `90`, background code `100`)
    DarkGray,
    /// Color #1 (foreground code `31`, background code `41`)
    Red,
    /// Color #1 (foreground code `91`, background code `101`)
    BrightRed,
    /// Color #2 (foreground code `32`, background code `42`).
    Green,
    /// Color #2 (foreground code `92`, background code `102`).
    BrightGreen,
    /// Color #3 (foreground code `33`, background code `43`)
    Yellow,
    /// Color #3 (foreground code `93`, background code `103`)
    BrightYellow,
    /// Color #4 (foreground code `34`, background code `44`)
    Blue,
    /// Color #4 (foreground code `94`, background code `104`)
    BrightBlue,
    /// Color #5 (foreground code `35`, background code `45`)
    Magenta,
    /// Color #5 (foreground code `95`, background code `105`)
    BrightMagenta,
    /// Color #6 (foreground code `36`, background code `46`)
    Cyan,
    /// Color #6 (foreground code `96`, background code `106`)
    BrightCyan,
    /// Color #7 (foreground code `37`, background code `47`)
    ///
    /// As with Black, not necessarily the foreground color
    White,
    /// Color #7 (foreground code `97`, background code `107`)
    LightGray,
}

impl ColorValue for Ansi {}

impl FromStr for Ansi {
    type Err = ColorFromStrError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let variants = [
            Self::Black,
            Self::Red,
            Self::Green,
            Self::Yellow,
            Self::Blue,
            Self::Magenta,
            Self::Cyan,
            Self::White,
            Self::DarkGray,
            Self::BrightRed,
            Self::BrightGreen,
            Self::BrightYellow,
            Self::BrightBlue,
            Self::BrightMagenta,
            Self::BrightCyan,
            Self::LightGray,
        ];
        for v in variants {
            if s == v.to_string() {
                return Ok(v);
            }
        }

        Err(ColorFromStrError::InvalidName)
    }
}

impl std::fmt::Display for Ansi {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        //Hacky displayk implementation using the auto derived debug
        write!(f, "{:?}", self)
    }
}

#[cfg(feature = "serde")]
impl<'de> serde::Deserialize<'de> for Ansi {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        struct AnsiVisitor;

        impl<'de> serde::de::Visitor<'de> for AnsiVisitor {
            type Value = Ansi;

            fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
                return write!(formatter, "Expecting ANSI16 color name in PascalCase");
            }

            fn visit_str<E>(self, v: &str) -> Result<Self::Value, E>
            where
                E: serde::de::Error,
            {
                let ansi16 = Ansi::from_str(v);
                if let Ok(color) = ansi16 {
                    return Ok(color);
                }
                Err(E::custom("Invalid color name"))
            }
        }
        deserializer.deserialize_string(AnsiVisitor)
    }
}

#[cfg(feature = "serde")]
impl serde::Serialize for Ansi {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(&self.to_string())
    }
}

#[cfg(test)]
mod ansi_tests {
    use serde_test::{assert_tokens, Token};

    use super::*;

    #[test]
    fn test_serialisation_basic() {
        let color = Ansi::BrightRed;

        assert_tokens(&color, &[Token::Str("BrightRed")]);
    }
}

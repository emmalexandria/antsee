use std::str::FromStr;

use super::{ColorFromStrError, ColorValue};

#[repr(u8)]
#[derive(Clone, Copy, PartialEq, Eq, Debug, Default)]
pub enum Ansi {
    #[default]
    Default = 0,
    Black = 1,
    Red = 2,
    Green = 3,
    Yellow = 4,
    Blue = 5,
    Magenta = 6,
    Cyan = 7,
    White = 8,
    BrightBlack = 9,
    BrightRed = 10,
    BrightGreen = 11,
    BrightYellow = 12,
    BrightBlue = 13,
    BrightMagenta = 14,
    BrightCyan = 15,
    BrightWhite = 16,
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
            Self::BrightBlack,
            Self::BrightRed,
            Self::BrightGreen,
            Self::BrightYellow,
            Self::BrightBlue,
            Self::BrightMagenta,
            Self::BrightCyan,
            Self::BrightWhite,
        ];
        for v in variants {
            if generate_name_variants(v).contains(&s.to_owned()) {
                return Ok(v);
            }
        }

        Err(ColorFromStrError::InvalidName)
    }
}

fn generate_name_variants<S: ToString>(name: S) -> [String; 2] {
    let base = name.to_string();
    let lowercase = base.to_lowercase();

    [base, lowercase]
}

impl std::fmt::Display for Ansi {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        //Hacky display implementation using the auto derived debug
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
                return write!(formatter, "Expecting ANSI16 color name in PascalCase or lowercase (e.g. BrightMagenta or brightmagenta");
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

            fn visit_string<E>(self, v: String) -> Result<Self::Value, E>
            where
                E: serde::de::Error,
            {
                let ansi16 = Ansi::from_str(&v);
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

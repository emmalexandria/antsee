use std::str::FromStr;

#[repr(u8)]
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
#[cfg_attr(feature = "serde", derive(serde::Serialize))]
pub enum Ansi16 {
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

#[derive(Debug, PartialEq)]
pub enum Ansi16Error {
    InvalidName,
    U8TooLarge,
}

impl Into<u8> for Ansi16 {
    fn into(self) -> u8 {
        self as u8
    }
}

impl TryFrom<u8> for Ansi16 {
    type Error = Ansi16Error;
    fn try_from(value: u8) -> Result<Ansi16, Self::Error> {
        match value {
            _ => Err(Ansi16Error::U8TooLarge),
        }
    }
}

impl FromStr for Ansi16 {
    type Err = Ansi16Error;

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

        Err(Ansi16Error::InvalidName)
    }
}

fn generate_name_variants<S: ToString>(name: S) -> [String; 2] {
    let base = name.to_string();
    let lowercase = base.to_lowercase();

    [base, lowercase]
}

impl std::fmt::Display for Ansi16 {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        //Hacky display implementation using the auto derived debug
        write!(f, "{:?}", self)
    }
}

#[cfg(feature = "serde")]
struct Ansi16Visitor;

#[cfg(feature = "serde")]
impl<'de> serde::de::Visitor<'de> for Ansi16Visitor {
    type Value = Ansi16;

    fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
        return write!(formatter, "Expecting ANSI16 color name in PascalCase or lowercase (e.g. BrightMagenta or brightmagenta");
    }

    fn visit_str<E>(self, v: &str) -> Result<Self::Value, E>
    where
        E: serde::de::Error,
    {
        let ansi16 = Ansi16::from_str(v);
        if let Ok(color) = ansi16 {
            return Ok(color);
        }
        Err(E::custom("Invalid color name"))
    }

    fn visit_string<E>(self, v: String) -> Result<Self::Value, E>
    where
        E: serde::de::Error,
    {
        let ansi16 = Ansi16::from_str(&v);
        if let Ok(color) = ansi16 {
            return Ok(color);
        }
        Err(E::custom("Invalid color name"))
    }
}

#[cfg(feature = "serde")]
impl<'de> serde::Deserialize<'de> for Ansi16 {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        deserializer.deserialize_string(Ansi16Visitor)
    }
}

#[cfg(test)]
mod ansi_tests {
    use super::*;

    #[test]
    fn test_ansi16_from_str() {
        assert_eq!(Ansi16::from_str("brightblack"), Ok(Ansi16::BrightBlack));
        assert_eq!(
            Ansi16::from_str("bright black"),
            Err(Ansi16Error::InvalidName)
        );
    }
}
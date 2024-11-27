use std::str::FromStr;

#[cfg(feature = "serde")]
use serde::{de::Visitor, ser::SerializeSeq, Deserialize, Serialize};

use super::css::{self, CssColors};

/** The RGB colour type, containing a simple u8 array to represent the color value */
#[derive(Debug, Clone, Copy, Default, PartialEq)]
pub struct Rgb([u8; 3]);

/** Defines potential errors when parsing an [Rgb] value from a string. Required by
* the [FromStr] trait. */
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum RgbError {
    ///Hex value is not 6 characters. This library currently does not support 3 character hex
    ///values.
    HexWrongLength,
    ///Hex value contains characters outside the 0-f range
    InvalidHexValue,
    ///String doesn't match any method of parsing
    InvalidString,
}

impl Rgb {
    ///Construct a new Rgb instance from an array of u8s
    pub fn new(val: [u8; 3]) -> Rgb {
        Rgb(val)
    }

    ///Set the RGB color with a hexadecimal color string
    pub fn set_hex(&mut self, hex: &str) -> Result<(), RgbError> {
        let new = Self::from_hex(hex)?;
        self.0 = new.0;
        Ok(())
    }

    fn from_hex(hex: &str) -> Result<Self, RgbError> {
        let mut hex = hex;
        if hex.starts_with('#') {
            hex = &hex[1..]
        }
        if hex.len() != 6 {
            return Err(RgbError::HexWrongLength);
        }
        let r = u8::from_str_radix(&hex[0..2], 16).map_err(|_| RgbError::InvalidHexValue)?;
        let g = u8::from_str_radix(&hex[2..4], 16).map_err(|_| RgbError::InvalidHexValue)?;
        let b = u8::from_str_radix(&hex[4..6], 16).map_err(|_| RgbError::InvalidHexValue)?;

        Ok(Self([r, g, b]))
    }
}

impl From<CssColors> for Rgb {
    fn from(value: CssColors) -> Self {
        Rgb(value.rgb())
    }
}

impl FromStr for Rgb {
    type Err = RgbError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        if s.starts_with('#') {
            return Self::from_hex(s);
        }
        if s.starts_with("css(") && s.ends_with(')') {
            let color_name = &s[4..s.len() - 1];
            let css_color = CssColors::get_name(color_name);
            if let Some(color) = css_color {
                return Ok(Self::from(color));
            }
        }
        Err(RgbError::InvalidString)
    }
}

#[cfg(feature = "serde")]
impl Serialize for Rgb {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        let mut seq = serializer.serialize_seq(Some(3))?;
        seq.serialize_element(&self.0[0])?;
        seq.serialize_element(&self.0[1])?;
        seq.serialize_element(&self.0[2])?;
        seq.end()
    }
}

#[cfg(feature = "serde")]
struct RgbVisitor;

#[cfg(feature = "serde")]
impl<'de> Visitor<'de> for RgbVisitor {
    type Value = Rgb;
    fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
        formatter.write_str("A sequence of 3 u8 values, a css color name, or a hex value")
    }

    fn visit_str<E>(self, v: &str) -> Result<Self::Value, E>
    where
        E: serde::de::Error,
    {
        let rgb = Rgb::from_str(v);
        if let Ok(color) = rgb {
            return Ok(color);
        }
        Err(E::custom(format!("No string match found")))
    }

    fn visit_string<E>(self, v: String) -> Result<Self::Value, E>
    where
        E: serde::de::Error,
    {
        let rgb = Rgb::from_str(&v);
        if let Ok(color) = rgb {
            return Ok(color);
        }
        Err(E::custom(format!("No string match found")))
    }

    fn visit_seq<A>(self, mut seq: A) -> Result<Self::Value, A::Error>
    where
        A: serde::de::SeqAccess<'de>,
    {
        let mut rgb = Rgb([0, 0, 0]);
        let r = seq.next_element()?;
        if let Some(red) = r {
            rgb.0[0] = red;
        }
        let g = seq.next_element()?;
        if let Some(green) = g {
            rgb.0[1] = green;
        }
        let b = seq.next_element()?;
        if let Some(blue) = b {
            rgb.0[2] = blue;
        }
        Ok(rgb)
    }
}

#[cfg(feature = "serde")]
impl<'de> Deserialize<'de> for Rgb {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        deserializer.deserialize_any(RgbVisitor)
    }
}

#[cfg(test)]
mod rgb_tests {
    use super::*;

    #[test]
    fn test_rgb_from_hex() {
        let rgb = Rgb::from_str("#324582");
        assert!(rgb.is_ok());
        assert_eq!(rgb.unwrap(), Rgb([50, 69, 130]));
    }

    #[test]
    fn test_rgb_from_css() {
        let rgb = Rgb::from_str("css(red)");
        assert!(rgb.is_ok());
        assert_eq!(rgb.unwrap(), Rgb(CssColors::Red.rgb()));
    }

    #[test]
    fn test_invalid_strings() {
        let rgb = Rgb::from_str("#3245");
        assert!(rgb.is_err_and(|e| e == RgbError::HexWrongLength));
        let rgb = Rgb::from_str("#jjjjjj");
        assert!(rgb.is_err_and(|e| e == RgbError::InvalidHexValue));
        let rgb = Rgb::from_str("css(fakeasscolor)");
        assert!(rgb.is_err_and(|e| e == RgbError::InvalidString));
    }
}

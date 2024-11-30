use std::{fmt::Display, rc::Rc, str::FromStr};

#[cfg(feature = "serde")]
use serde::{de::Visitor, ser::SerializeSeq, Deserialize, Serialize};

use super::{
    libraries::{ColorLibrary, CssColors, XtermColors},
    ColorFromStrError, ColorSource, ColorValue, Source,
};

/** The RGB colour type, containing a simple u8 array to represent the color value */
#[derive(Debug, Clone, Default, PartialEq)]
pub struct Rgb([u8; 3], Source<Rc<str>>);

impl Rgb {
    ///Construct a new Rgb instance from an array of u8s
    pub fn new() -> Rgb {
        Rgb([0; 3], Source::Inactive(Rc::from("")))
    }

    ///Set the value of the colour with RGB
    pub fn rgb(mut self, val: [u8; 3]) -> Self {
        self.0 = val;
        self
    }

    ///Set the value of the colour with a hex string
    pub fn hex(mut self, hex: &str) -> Result<Self, ColorFromStrError> {
        self = Self::from_hex(hex)?;
        Ok(self)
    }

    ///Set the RGB color with a hexadecimal color string
    pub fn set_hex(&mut self, hex: &str) -> Result<(), ColorFromStrError> {
        let new = Self::from_hex(hex)?;
        self.0 = new.0;
        self.1 = Source::Active(Rc::from(hex));
        Ok(())
    }

    ///Set the RGB color with a value from [CssColors] or [XtermColors]
    pub fn color<C: ColorLibrary>(mut self, color: C) -> Self {
        self.0 = color.rgb();
        self.1 = Source::Active(Rc::from(color.color_name()));
        self
    }

    ///Set the RGB color with a value from [CssColors] or [XtermColors]
    pub fn set_color<C: ColorLibrary>(&mut self, color: C) {
        self.0 = color.rgb();
        self.1 = Source::Active(Rc::from(color.color_name()))
    }

    fn from_hex(hex: &str) -> Result<Self, ColorFromStrError> {
        let fullhex = hex;
        let mut hex = hex;
        if hex.starts_with('#') {
            hex = &hex[1..]
        }
        if hex.len() != 6 {
            return Err(ColorFromStrError::InvalidValue);
        }
        let r = u8::from_str_radix(&hex[0..2], 16).map_err(|_| ColorFromStrError::InvalidValue)?;
        let g = u8::from_str_radix(&hex[2..4], 16).map_err(|_| ColorFromStrError::InvalidValue)?;
        let b = u8::from_str_radix(&hex[4..6], 16).map_err(|_| ColorFromStrError::InvalidValue)?;

        Ok(Self([r, g, b], Source::Active(Rc::from(fullhex))))
    }
}

impl<C> From<C> for Rgb
where
    C: ColorLibrary,
{
    fn from(value: C) -> Self {
        Rgb(value.rgb(), Source::Active(Rc::from(value.color_name())))
    }
}

impl ColorValue for Rgb {}

impl FromStr for Rgb {
    type Err = ColorFromStrError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        if s.starts_with('#') {
            return Self::from_hex(s);
        }
        if s.starts_with("css(") && s.ends_with(')') {
            let color_name = CssColors::unwrap_name(s);
            let css_color = CssColors::get_name(color_name);
            if let Some(color) = css_color {
                return Ok(Self::from(color));
            }
        }
        if s.starts_with("xterm(") && s.ends_with(")") {
            let color_name = XtermColors::unwrap_name(s);
            let xterm_color = XtermColors::get_name(color_name);
            if let Some(color) = xterm_color {
                return Ok(Self::from(color));
            }
        }
        Err(ColorFromStrError::InvalidString)
    }
}

#[cfg(feature = "serde")]
impl Serialize for Rgb {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        if let Source::Active(s) = self.1.clone() {
            if let Some(c) = CssColors::get_name(&s) {
                return serializer.serialize_str(&CssColors::wrap_name(&s));
            }
            if let Some(c) = XtermColors::get_name(&s) {
                return serializer.serialize_str(&XtermColors::wrap_name(&s));
            }
            return serializer.serialize_str(&s);
        }
        let mut seq = serializer.serialize_seq(Some(3))?;
        seq.serialize_element(&self.0[0])?;
        seq.serialize_element(&self.0[1])?;
        seq.serialize_element(&self.0[2])?;
        seq.end()
    }
}

impl Display for Rgb {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        if let Source::Active(c) = self.clone().1 {
            if CssColors::get_name(&c).is_some() {
                write!(f, "css({})", c)?;
                return Ok(());
            }
            if XtermColors::get_name(&c).is_some() {
                write!(f, "xterm({})", c)?;
                return Ok(());
            }
        }
        write!(f, "rgb({},{},{})", self.0[0], self.0[1], self.0[2])?;
        Ok(())
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

    fn visit_seq<A>(self, mut seq: A) -> Result<Self::Value, A::Error>
    where
        A: serde::de::SeqAccess<'de>,
    {
        let mut rgb = Rgb([0, 0, 0], Source::Inactive(Rc::from("")));
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
    use serde_test::{assert_tokens, Token};

    use super::*;

    #[test]
    fn test_rgb_from_hex() {
        let rgb = Rgb::from_str("#324582");
        assert!(rgb.is_ok());
        assert_eq!(
            rgb.unwrap(),
            Rgb([50, 69, 130], Source::Active(Rc::from("#324582")))
        );
    }

    #[test]
    fn test_rgb_from_css() {
        let rgb = Rgb::from_str("css(red)");
        assert!(rgb.is_ok());
        assert_eq!(
            rgb.unwrap(),
            Rgb(
                CssColors::Red.rgb(),
                Source::Active(Rc::from(CssColors::Red.color_name()))
            )
        );
    }

    #[test]
    fn test_serialize_array() {
        let rgb = Rgb::new().rgb([32, 45, 0]);

        assert_tokens(
            &rgb,
            &[
                Token::Seq { len: Some(3) },
                Token::U8(32),
                Token::U8(45),
                Token::U8(0),
                Token::SeqEnd,
            ],
        )
    }

    #[test]
    fn test_serialize_hex() {
        let rgb = Rgb::from_str("#453209").unwrap();

        assert_tokens(&rgb, &[Token::Str("#453209")])
    }

    #[test]
    fn test_serialize_css() {
        let rgb = Rgb::from_str("css(red)").unwrap();
        assert_tokens(&rgb, &[Token::Str("css(red)")])
    }
}

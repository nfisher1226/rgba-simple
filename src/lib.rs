#![warn(clippy::all, clippy::pedantic)]
//! `Rgba_simple` is a small library for storing colors in RGBA and Hex notation.
//! It includes functions to convert to and from Hex and RGBA. All of the internal
//! formats can be serialized and deserialized with `serde`. If compiled with the
//! `gtk` feature, all of it's internal representations can also be converted to
//! and from `gtk::gdk::RGBA`, making one use case storing colors generated from
//! a Gtk+ gui in a config file, using one of the many formats with `serde`
//! support.
#[cfg(feature = "gtk")]
use gtk::gdk;
use serde::{Deserialize, Serialize};
use std::fmt;

pub mod hexcolor;
pub use hexcolor::HexColor;
pub mod reduced;
pub use reduced::ReducedRGBA;
pub mod rgba;
pub use rgba::RGBA;
#[cfg(feature = "gtk")]
pub mod gdk_impl;

#[derive(Clone, Deserialize, Debug, Serialize)]
#[serde(tag = "ColorType")]
pub enum Color {
    Hex(HexColor),
    Reduced(ReducedRGBA),
    Rgba(RGBA),
}

#[derive(Clone, Debug, PartialEq)]
pub enum ColorError {
    OutsideBoundsNegative,
    OutsideBoundsHigh,
    InvalidHex,
}

impl fmt::Display for ColorError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{:?}", self)
    }
}

pub trait Convert {
    type Err;
    fn to_hex(&self) -> Result<HexColor, Self::Err>;
    fn to_rgba(&self) -> Result<RGBA, Self::Err>;
    fn to_reduced_rgba(&self) -> Result<ReducedRGBA, Self::Err>;
    #[cfg(feature = "gtk")]
    fn to_gdk(&self) -> Result<gdk::RGBA, Self::Err>;
}

pub trait Primary {
    fn black() -> Self;
    fn white() -> Self;
    fn red() -> Self;
    fn green() -> Self;
    fn blue() -> Self;
    fn yellow() -> Self;
    fn magenta() -> Self;
    fn cyan() -> Self;
}

pub trait ToHex {
    type Err;
    fn to_hex(&self) -> Result<HexColor, Self::Err>;
}

impl ToHex for Color {
    type Err = ColorError;

    fn to_hex(&self) -> Result<HexColor, ColorError> {
        match self {
            Color::Hex(c) => Ok(HexColor {
                color: c.color.clone(),
                alpha: c.alpha,
            }),
            Color::Rgba(c) => c.to_hex(),
            Color::Reduced(c) => c.to_hex(),
        }
    }
}

pub trait ToRGBA {
    fn to_rgba(&self) -> RGBA;
}

pub trait ToReducedRGBA {
    type Err;
    fn to_reduced_rgba(&self) -> Result<ReducedRGBA, Self::Err>;
}

#[cfg(feature = "gtk")]
pub trait ToGdk {
    fn to_gdk(&self) -> gdk::RGBA;
}

impl Color {
    #[cfg(feature = "gtk")]
    pub fn to_gdk(&self) -> Result<gdk::RGBA, ColorError> {
        match self {
            Color::Hex(c) => c.to_gdk(),
            Color::Rgba(c) => c.to_gdk(),
            Color::Reduced(c) => c.to_gdk(),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn black() {
        assert_eq!(RGBA::black().to_hex().unwrap().color, HexColor::black().color);
        assert_eq!(ReducedRGBA::black().to_hex().unwrap().color, HexColor::black().color);
    }

    #[test]
    fn white() {
        assert_eq!(RGBA::white().to_hex().unwrap().color, HexColor::white().color);
        assert_eq!(ReducedRGBA::white().to_hex().unwrap().color, HexColor::white().color);
    }

    #[test]
    fn red() {
        assert_eq!(RGBA::red().to_hex().unwrap().color, HexColor::red().color);
        assert_eq!(ReducedRGBA::red().to_hex().unwrap().color, HexColor::red().color);
    }

    #[test]
    fn green() {
        assert_eq!(RGBA::green().to_hex().unwrap().color, HexColor::green().color);
        assert_eq!(ReducedRGBA::green().to_hex().unwrap().color, HexColor::green().color);
    }

    #[test]
    fn blue() {
        assert_eq!(RGBA::blue().to_hex().unwrap().color, HexColor::blue().color);
        assert_eq!(ReducedRGBA::blue().to_hex().unwrap().color, HexColor::blue().color);
    }

    #[test]
    fn from_hex() {
        let red_hex = HexColor {
            color: String::from("#ff0000"),
            alpha: 1.0,
        };
        let red = red_hex.to_reduced_rgba().unwrap();
        assert_eq!(red.red, 255);
        assert_eq!(red.green, 0);
        assert_eq!(red.blue, 0);
        assert_eq!(red.alpha, 255);
    }

    #[cfg(feature = "gtk")]
    #[test]
    fn rgba_to_gdk() {
        let red = RGBA::red();
        let gdk_red = red.to_gdk().unwrap();
        assert_eq!(red.red, gdk_red.red);
        assert_eq!(red.green, gdk_red.green);
        assert_eq!(red.blue, gdk_red.blue);
        assert_eq!(red.alpha, gdk_red.alpha);
    }

    #[cfg(feature = "gtk")]
    #[test]
    fn rgba_from_gdk() {
        let gdk_red = gdk::RGBA {
            red: 1.0,
            green: 0.0,
            blue: 0.0,
            alpha: 1.0,
        };
        let red = gdk_red.to_rgba().unwrap();
        assert_eq!(red.red, gdk_red.red);
        assert_eq!(red.green, gdk_red.green);
        assert_eq!(red.blue, gdk_red.blue);
        assert_eq!(red.alpha, gdk_red.alpha);
    }
}

#![warn(clippy::all, clippy::pedantic)]
//! `Rgba_simple` is a small library for storing colors in RGBA and Hex notation.
//! It includes functions to convert to and from Hex and RGBA. All of the internal
//! formats can be serialized and deserialized with `serde`. If compiled with the
//! `gdk` feature, all of it's internal representations can also be converted to
//! and from `gdk::RGBA`, making one use case storing colors generated from
//! a Gtk+ gui in a config file, using one of the many formats with `serde`
//! support.
//!
//! Use this library if your color needs are simple and you don't require
//! addressing colors in otherr color spaces, such as CMYK or HSL.
//!
//! ## Examples
//! ```Rust
//! use rgba_simple::{ReducedRGBA, HexColor, Primary, Convert}
//!
//! let red = ReducedRGBA::red();
//! let red_hex = HexColor::red();
//! assert_eq!(red.to_hex().unwrap(), red_hex);
//! ```
use num::Num;
use serde::{Deserialize, Serialize};
use std::fmt;

mod hexcolor;
pub use hexcolor::HexColor;
//mod reduced;
//pub use reduced::ReducedRGBA;
pub mod float;
pub mod int;
#[cfg(feature = "gdk")]
mod gdk_impl;

pub struct Int;
pub struct Float;

pub trait NumType {
    type Num;
}

impl NumType for f64 {
    type Num = Float;
}

impl NumType for f32 {
    type Num = Float;
}

impl NumType for u8 {
    type Num = Int;
}

impl NumType for u16 {
    type Num = Int;
}

impl NumType for u32 {
    type Num = Int;
}

impl NumType for u64 {
    type Num = Int;
}

pub trait ToHex {
    type Err;

    fn to_hex(&self) -> Result<HexColor, Self::Err>;
}

pub trait ToIntRGBA<T>
where T: Num + NumType {
    type Err;

    fn to_int_rgba(&self) -> Result<int::RGBA<T>, Self::Err>;
}

pub trait ToFloatRGBA<T>
where T: Num + NumType {
    type Err;

    fn to_float_rgba(&self) -> Result<int::RGBA<T>, Self::Err>;
}


#[cfg(feature = "gdk")]
pub trait ToGdk {
    type Err;

    fn to_gdk(&self) -> Result<gdk::RGBA, Self::Err>;
}

/// An `enum` which can represent one of the three color storage types provided
/// by this library. Implements the `Convert` and `Primary` traits.
#[derive(Clone, Deserialize, Debug, Serialize)]
#[serde(tag = "ColorType")]
pub enum Color<T: Num + NumType> {
    Hex(HexColor),
    Reduced(int::RGBA<T>),
    Rgba(float::RGBA<T>),
}

/// Errors which might occur when validating or converting colors
#[derive(Clone, Debug, PartialEq)]
pub enum ColorError {
    OutsideBoundsNegative,
    OutsideBoundsHigh,
    TruncatedHexString,
    HexStringOverflow,
    InvalidHexCharacter,
}

impl fmt::Display for ColorError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{:?}", self)
    }
}

impl<T> fmt::Display for Color<T>
where T: fmt::Display + Num + NumType {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Color({})", self)
    }
}

/// Checks that the stored values represent a valid color
pub trait Validate {
    type Err;

    /// # Errors
    ///
    /// Will return `ColorError` if the color fails
    /// to validate
    fn validate(&self) -> Result<(), Self::Err>;
}

/// An enumeration of primary and secondary colors
pub enum PrimaryColor {
    Black,
    White,
    Red,
    Green,
    Blue,
    Yellow,
    Magenta,
    Cyan,
}

/// Initializes the color storage type with the specified color
pub trait Primary {
    fn primary(color: PrimaryColor) -> Self;
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn to_hex() {
        let color = Color::Rgba(RGBA::primary(PrimaryColor::Cyan));
        assert_eq!(color.to_hex(), Ok(HexColor::primary(PrimaryColor::Cyan)));
    }

    #[test]
    fn to_rgba() {
        let color = Color::Reduced(ReducedRGBA::primary(PrimaryColor::Yellow));
        assert_eq!(color.to_rgba(), Ok(RGBA::primary(PrimaryColor::Yellow)));
    }

    #[test]
    fn to_reduced() {
        let color = Color::Hex(HexColor::primary(PrimaryColor::Magenta));
        assert_eq!(
            color.to_reduced_rgba(),
            Ok(ReducedRGBA::primary(PrimaryColor::Magenta))
        );
    }
}

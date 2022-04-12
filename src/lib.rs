#![warn(clippy::all, clippy::pedantic)]
#![allow(clippy::cast_sign_loss)]
#![allow(clippy::cast_possible_truncation)]
#![allow(clippy::cast_precision_loss)]
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
//! # Example
//! ```Rust
//! use rgba_simple::*;
//!
//! let red_hex = String::from("#ff0000");
//! let red: RGB::<u8> = RGB::primary(PrimaryColor::Red);
//! assert_eq!(RGB::<u8>::from_hex(&red_hex), red);
//!```

pub(crate) mod channel;
pub(crate) use channel::Channel;
mod colorerror;
pub use colorerror::ColorError;
mod hex;
pub use hex::Hex;
mod primary;
pub use primary::{Primary, PrimaryColor};
mod rgb;
pub use rgb::RGB;
mod rgba;
pub use rgba::RGBA;
#[cfg(feature = "gdk")]
mod gdk_impl;
#[cfg(feature = "gdk")]
pub use gdk_impl::{FromGdk, ToGdk};

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn from_hex() {
        let red_hex = String::from("#ff0000");
        let red: RGB<u8> = RGB::primary(PrimaryColor::Red);
        assert_eq!(red, RGB::<u8>::from_hex(&red_hex).unwrap());
    }

    #[test]
    fn to_hex() {
        let blue_hex = String::from("#0000ff");
        let blue: RGB<u8> = RGB::primary(PrimaryColor::Blue);
        assert_eq!(blue.to_hex(), blue_hex);
    }

    #[test]
    fn from_hex_float() {
        let red_hex = String::from("#ff0000");
        let red: RGBA<f64> = RGBA::primary(PrimaryColor::Red);
        assert_eq!(red, RGBA::<f64>::from_hex(&red_hex).unwrap());
    }

    #[test]
    fn to_hex_float() {
        let blue_hex = String::from("#0000ff");
        let blue: RGB<f64> = RGB::primary(PrimaryColor::Blue);
        assert_eq!(blue.to_hex(), blue_hex);
    }
}

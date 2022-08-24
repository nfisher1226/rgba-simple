#![warn(clippy::all, clippy::pedantic)]
#![allow(clippy::cast_sign_loss)]
#![allow(clippy::cast_possible_truncation)]
#![allow(clippy::cast_precision_loss)]
#![doc = include_str!("../README.md")]
mod channel;
pub(crate) use channel::Channel;
mod colorerror;
pub use colorerror::ColorError;
mod hex;
pub use hex::Hex;
mod rgb;
pub use rgb::RGB;
mod rgba;
pub use rgba::RGBA;
#[cfg(feature = "gdk")]
mod gdk_impl;

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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn from_hex() {
        let red_hex = String::from("#ff0000");
        let red: RGB<u8> = PrimaryColor::Red.into();
        assert_eq!(red, RGB::<u8>::from_hex(&red_hex).unwrap());
    }

    #[test]
    fn to_hex() {
        let blue_hex = String::from("#0000ff");
        let blue: RGB<u8> = PrimaryColor::Blue.into();
        assert_eq!(blue.to_hex(), blue_hex);
    }

    #[test]
    fn from_hex_float() {
        let red_hex = String::from("#ff0000");
        let red: RGBA<f64> = RGBA::from(PrimaryColor::Red);
        assert_eq!(red, RGBA::<f64>::from_hex(&red_hex).unwrap());
    }

    #[test]
    fn to_hex_float() {
        let blue_hex = String::from("#0000ff");
        let blue: RGB<f64> = RGB::from(PrimaryColor::Blue);
        assert_eq!(blue.to_hex(), blue_hex);
    }
}

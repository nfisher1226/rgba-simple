use crate::ColorError;

pub(crate) fn validate_hex_string(hex: &str) -> Result<(), ColorError> {
    match &hex.len() {
        x if *x < 7 => Err(ColorError::TruncatedHexString),
        x if *x > 7 => Err(ColorError::HexStringOverflow),
        _ => if &hex[0..1] == "#" {
            Ok(())
        } else {
            Err(ColorError::InvalidHexCharacter)
        }
    }
}

/// Transformations to and from hexadecimal notation (base 16)
pub trait Hex {
    type Err;
    /// Represent a value as a hex string
    fn to_hex(&self) -> String;
    /// Convert a hex string to a value
    /// # Errors
    /// Returns error if the hex string is not valid or value is out of bounds
    fn from_hex(_: &str) -> Result<Self, Self::Err>
    where
        Self: Sized;
}

#[doc(hidden)]
impl Hex for u8 {
    type Err = ColorError;

    fn to_hex(&self) -> String {
        format!("{:02x}", self)
    }

    fn from_hex(hex: &str) -> Result<Self, Self::Err> {
        match Self::from_str_radix(hex, 16) {
            Ok(c) => Ok(c),
            Err(_) => Err(ColorError::InvalidHexCharacter),
        }
    }
}

#[doc(hidden)]
impl Hex for u16 {
    type Err = ColorError;

    fn to_hex(&self) -> String {
        format!("{:02x}", self)
    }

    fn from_hex(hex: &str) -> Result<Self, Self::Err> {
        match Self::from_str_radix(hex, 16) {
            Ok(c) => Ok(c),
            Err(_) => Err(ColorError::InvalidHexCharacter),
        }
    }
}

#[doc(hidden)]
impl Hex for u32 {
    type Err = ColorError;

    fn to_hex(&self) -> String {
        format!("{:02x}", self)
    }

    fn from_hex(hex: &str) -> Result<Self, Self::Err> {
        match Self::from_str_radix(hex, 16) {
            Ok(c) => Ok(c),
            Err(_) => Err(ColorError::InvalidHexCharacter),
        }
    }
}

#[doc(hidden)]
impl Hex for u64 {
    type Err = ColorError;

    fn to_hex(&self) -> String {
        format!("{:02x}", self)
    }

    fn from_hex(hex: &str) -> Result<Self, Self::Err> {
        match Self::from_str_radix(hex, 16) {
            Ok(c) => Ok(c),
            Err(_) => Err(ColorError::InvalidHexCharacter),
        }
    }
}

#[doc(hidden)]
impl Hex for i16 {
    type Err = ColorError;

    fn to_hex(&self) -> String {
        format!("{:02x}", self)
    }

    fn from_hex(hex: &str) -> Result<Self, Self::Err> {
        match Self::from_str_radix(hex, 16) {
            Ok(c) => Ok(c),
            Err(_) => Err(ColorError::InvalidHexCharacter),
        }
    }
}

#[doc(hidden)]
impl Hex for i32 {
    type Err = ColorError;

    fn to_hex(&self) -> String {
        format!("{:02x}", self)
    }

    fn from_hex(hex: &str) -> Result<Self, Self::Err> {
        match Self::from_str_radix(hex, 16) {
            Ok(c) => Ok(c),
            Err(_) => Err(ColorError::InvalidHexCharacter),
        }
    }
}

#[doc(hidden)]
impl Hex for i64 {
    type Err = ColorError;

    fn to_hex(&self) -> String {
        format!("{:02x}", self)
    }

    fn from_hex(hex: &str) -> Result<Self, Self::Err> {
        match Self::from_str_radix(hex, 16) {
            Ok(c) => Ok(c),
            Err(_) => Err(ColorError::InvalidHexCharacter),
        }
    }
}

#[doc(hidden)]
impl Hex for f32 {
    type Err = ColorError;

    fn to_hex(&self) -> String {
        format!("{:02x}", (self * 255.0).round() as u8)
    }

    fn from_hex(hex: &str) -> Result<Self, Self::Err> {
        match u8::from_str_radix(hex, 16) {
            Ok(c) => Ok(f32::from(c) / 255.0),
            Err(_) => Err(ColorError::InvalidHexCharacter),
        }
    }
}

#[doc(hidden)]
impl Hex for f64 {
    type Err = ColorError;

    fn to_hex(&self) -> String {
        format!("{:02x}", (self * 255.0).round() as u8)
    }

    fn from_hex(hex: &str) -> Result<Self, Self::Err> {
        match u8::from_str_radix(hex, 16) {
            Ok(c) => Ok(f64::from(c) / 255.0),
            Err(_) => Err(ColorError::InvalidHexCharacter),
        }
    }
}

use crate::ColorError;

pub trait ToHex {
    fn to_hex(&self) -> String;
}

pub trait FromHex {
    type Err;

    fn from_hex(_: &str) -> Result<Self, Self::Err> where Self: Sized;
}

impl ToHex for u8 {
    fn to_hex(&self) -> String {
        format!("{:02x}", self)
    }
}

impl ToHex for u16 {
    fn to_hex(&self) -> String {
        format!("{:02x}", self)
    }
}

impl ToHex for u32 {
    fn to_hex(&self) -> String {
        format!("{:02x}", self)
    }
}

impl ToHex for u64 {
    fn to_hex(&self) -> String {
        format!("{:02x}", self)
    }
}

impl ToHex for i16 {
    fn to_hex(&self) -> String {
        format!("{:02x}", self)
    }
}

impl ToHex for i32 {
    fn to_hex(&self) -> String {
        format!("{:02x}", self)
    }
}

impl ToHex for i64 {
    fn to_hex(&self) -> String {
        format!("{:02x}", self)
    }
}

impl ToHex for f32 {
    fn to_hex(&self) -> String {
        format!("{:02x}", (self * 255.0).round() as u8)
    }
}

impl ToHex for f64 {
    fn to_hex(&self) -> String {
        format!("{:02x}", (self * 255.0).round() as u8)
    }
}

impl FromHex for u8 {
    type Err = ColorError;

    fn from_hex(hex: &str) -> Result<Self, Self::Err> {
        match Self::from_str_radix(hex, 16) {
            Ok(c) => Ok(c),
            Err(_) => Err(ColorError::InvalidHexCharacter),
        }
    }
}

impl FromHex for u16 {
    type Err = ColorError;

    fn from_hex(hex: &str) -> Result<Self, Self::Err> {
        match Self::from_str_radix(hex, 16) {
            Ok(c) => Ok(c),
            Err(_) => Err(ColorError::InvalidHexCharacter),
        }
    }
}

impl FromHex for u32 {
    type Err = ColorError;

    fn from_hex(hex: &str) -> Result<Self, Self::Err> {
        match Self::from_str_radix(hex, 16) {
            Ok(c) => Ok(c),
            Err(_) => Err(ColorError::InvalidHexCharacter),
        }
    }
}

impl FromHex for u64 {
    type Err = ColorError;

    fn from_hex(hex: &str) -> Result<Self, Self::Err> {
        match Self::from_str_radix(hex, 16) {
            Ok(c) => Ok(c),
            Err(_) => Err(ColorError::InvalidHexCharacter),
        }
    }
}

impl FromHex for i16 {
    type Err = ColorError;

    fn from_hex(hex: &str) -> Result<Self, Self::Err> {
        match Self::from_str_radix(hex, 16) {
            Ok(c) => Ok(c),
            Err(_) => Err(ColorError::InvalidHexCharacter),
        }
    }
}

impl FromHex for i32 {
    type Err = ColorError;

    fn from_hex(hex: &str) -> Result<Self, Self::Err> {
        match Self::from_str_radix(hex, 16) {
            Ok(c) => Ok(c),
            Err(_) => Err(ColorError::InvalidHexCharacter),
        }
    }
}

impl FromHex for i64 {
    type Err = ColorError;

    fn from_hex(hex: &str) -> Result<Self, Self::Err> {
        match Self::from_str_radix(hex, 16) {
            Ok(c) => Ok(c),
            Err(_) => Err(ColorError::InvalidHexCharacter),
        }
    }
}

impl FromHex for f32 {
    type Err = ColorError;

    fn from_hex(hex: &str) -> Result<Self, Self::Err> {
        match u8::from_str_radix(hex, 16) {
            Ok(c) => Ok(c as f32 / 255.0),
            Err(_) => Err(ColorError::InvalidHexCharacter),
        }
    }
}

impl FromHex for f64 {
    type Err = ColorError;

    fn from_hex(hex: &str) -> Result<Self, Self::Err> {
        match u8::from_str_radix(hex, 16) {
            Ok(c) => Ok(c as f64 / 255.0),
            Err(_) => Err(ColorError::InvalidHexCharacter),
        }
    }
}

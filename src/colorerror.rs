use std::{error::Error, fmt::Display};

/// Errors which might occur when validating or converting colors
#[derive(Clone, Debug, Eq, PartialEq)]
pub enum ColorError {
    OutsideBoundsNegative,
    OutsideBoundsHigh,
    TruncatedHexString,
    HexStringOverflow,
    InvalidHexCharacter,
}

impl Display for ColorError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}",
            match self {
                Self::OutsideBoundsNegative => "Value out of bounds: negative",
                Self::OutsideBoundsHigh => "Value out of bounds: too high",
                Self::TruncatedHexString => "Hex string is truncated",
                Self::HexStringOverflow => "Extra characters in hex string",
                Self::InvalidHexCharacter => "Invalid character in hex string",
            }
        )
    }
}

impl Error for ColorError {}

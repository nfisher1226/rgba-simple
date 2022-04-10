/// Errors which might occur when validating or converting colors
#[derive(Clone, Debug, PartialEq)]
pub enum ColorError {
    OutsideBoundsNegative,
    OutsideBoundsHigh,
    TruncatedHexString,
    HexStringOverflow,
    InvalidHexCharacter,
}

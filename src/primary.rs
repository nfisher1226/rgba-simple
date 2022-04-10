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

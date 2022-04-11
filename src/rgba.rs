use {
    crate::{ColorChannel, ColorError, Hex, Primary, PrimaryColor, PrimaryColor::*},
    serde::{Deserialize, Serialize},
    std::fmt,
};

/// Represents a color as red, green and blue channels with an alpha channel
/// for transparency
#[derive(Clone, Copy, Debug, Default, Deserialize, PartialEq, Serialize)]
pub struct RGBA<T>
where
    T: ColorChannel,
{
    pub red: T,
    pub green: T,
    pub blue: T,
    pub alpha: T,
}

impl<T> RGBA<T>
where
    T: ColorChannel,
{
    /// Creates a new instance
    ///
    /// # Errors
    /// Returns `ColorError::OutsideBoundsNegative` if any channel is less than `ColorChannel::MIN`
    /// Returns `ColorError::OutsideBoundsHigh` if any channel is greater than `ColorChannel::MAX`
    pub fn try_new(red: T, green: T, blue: T, alpha: T) -> Result<Self, ColorError> {
        for c in [red, green, blue, alpha] {
            if c < ColorChannel::MIN {
                return Err(ColorError::OutsideBoundsNegative);
            } else if c > ColorChannel::MAX {
                return Err(ColorError::OutsideBoundsHigh);
            }
        }
        Ok(Self {
            red,
            green,
            blue,
            alpha,
        })
    }

    /// Creates a new instance infallibly. If any of the arguments are outside of
    /// the bounds `ColorChannel::MIN` and `ColorChannel::MAX`, that channels value
    /// will be either the minimum or the maximum, respectively.
    pub fn new(red: T, green: T, blue: T, alpha: T) -> Self {
        Self {
            red: if red < ColorChannel::MIN {
                ColorChannel::MIN
            } else if red > ColorChannel::MAX {
                ColorChannel::MAX
            } else {
                red
            },
            green: if green < ColorChannel::MIN {
                ColorChannel::MIN
            } else if green > ColorChannel::MAX {
                ColorChannel::MAX
            } else {
                green
            },
            blue: if blue < ColorChannel::MIN {
                ColorChannel::MIN
            } else if blue > ColorChannel::MAX {
                ColorChannel::MAX
            } else {
                blue
            },
            alpha: if alpha < ColorChannel::MIN {
                ColorChannel::MIN
            } else if alpha > ColorChannel::MAX {
                ColorChannel::MAX
            } else {
                alpha
            },
        }
    }
}

impl<T> fmt::Display for RGBA<T>
where
    T: ColorChannel,
{
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "RGBA({}, {}, {}, {})",
            self.red.channel_display(),
            self.green.channel_display(),
            self.blue.channel_display(),
            self.alpha.channel_display(),
        )
    }
}

impl<T> Primary for RGBA<T>
where
    T: ColorChannel,
{
    fn primary(color: PrimaryColor) -> Self {
        Self {
            red: match color {
                Black | Green | Blue | Cyan => ColorChannel::MIN,
                _ => ColorChannel::MAX,
            },
            green: match color {
                Black | Red | Blue | Magenta => ColorChannel::MIN,
                _ => ColorChannel::MAX,
            },
            blue: match color {
                Black | Red | Green | Yellow => ColorChannel::MIN,
                _ => ColorChannel::MAX,
            },
            alpha: ColorChannel::MAX,
        }
    }
}

impl<T> Hex for RGBA<T>
where
    T: ColorChannel + Hex,
{
    type Err = ColorError;

    fn to_hex(&self) -> String {
        format!(
            "#{}{}{}",
            self.red.to_hex(),
            self.green.to_hex(),
            self.blue.to_hex(),
        )
    }

    fn from_hex(hex: &str) -> Result<Self, Self::Err> {
        crate::hex::validate_hex_string(hex)?;
        let red = match T::from_hex(&hex[1..3]) {
            Ok(r) => r,
            Err(_) => return Err(ColorError::InvalidHexCharacter),
        };
        let green = match T::from_hex(&hex[3..5]) {
            Ok(g) => g,
            Err(_) => return Err(ColorError::InvalidHexCharacter),
        };
        let blue = match T::from_hex(&hex[5..7]) {
            Ok(b) => b,
            Err(_) => return Err(ColorError::InvalidHexCharacter),
        };
        for channel in [red, green, blue] {
            if channel < ColorChannel::MIN {
                return Err(ColorError::OutsideBoundsNegative);
            } else if channel > ColorChannel::MAX {
                return Err(ColorError::OutsideBoundsHigh);
            }
        }
        Ok(Self {
            red,
            green,
            blue,
            alpha: ColorChannel::MAX,
        })
    }
}

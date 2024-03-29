#[allow(clippy::enum_glob_use)]
use {
    crate::{hex, Channel, ColorError, Hex, PrimaryColor, PrimaryColor::*},
    std::fmt,
};

#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};

/// Represents a color as red, green and blue channels with an alpha channel
/// for transparency
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]
pub struct RGBA<T>
where
    T: Channel,
{
    pub red: T,
    pub green: T,
    pub blue: T,
    pub alpha: T,
}

impl<T> RGBA<T>
where
    T: Channel,
{
    /// Creates a new instance
    ///
    /// # Errors
    /// Returns `ColorError::OutsideBoundsNegative` if any channel is less than `ColorChannel::MIN`
    /// Returns `ColorError::OutsideBoundsHigh` if any channel is greater than `ColorChannel::MAX`
    pub fn try_new(red: T, green: T, blue: T, alpha: T) -> Result<Self, ColorError> {
        for c in [red, green, blue, alpha] {
            if c < Channel::MIN {
                return Err(ColorError::OutsideBoundsNegative);
            } else if c > Channel::MAX {
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
            red: if red < Channel::MIN {
                Channel::MIN
            } else if red > Channel::MAX {
                Channel::MAX
            } else {
                red
            },
            green: if green < Channel::MIN {
                Channel::MIN
            } else if green > Channel::MAX {
                Channel::MAX
            } else {
                green
            },
            blue: if blue < Channel::MIN {
                Channel::MIN
            } else if blue > Channel::MAX {
                Channel::MAX
            } else {
                blue
            },
            alpha: if alpha < Channel::MIN {
                Channel::MIN
            } else if alpha > Channel::MAX {
                Channel::MAX
            } else {
                alpha
            },
        }
    }
}

impl<T> fmt::Display for RGBA<T>
where
    T: Channel,
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

impl<T> From<PrimaryColor> for RGBA<T>
where
    T: Channel,
{
    fn from(color: PrimaryColor) -> Self {
        Self {
            red: match color {
                Black | Green | Blue | Cyan => Channel::MIN,
                _ => Channel::MAX,
            },
            green: match color {
                Black | Red | Blue | Magenta => Channel::MIN,
                _ => Channel::MAX,
            },
            blue: match color {
                Black | Red | Green | Yellow => Channel::MIN,
                _ => Channel::MAX,
            },
            alpha: Channel::MAX,
        }
    }
}

impl<T> Hex for RGBA<T>
where
    T: Channel + Hex,
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
        hex::validate_hex_string(hex)?;
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
            if channel < Channel::MIN {
                return Err(ColorError::OutsideBoundsNegative);
            } else if channel > Channel::MAX {
                return Err(ColorError::OutsideBoundsHigh);
            }
        }
        Ok(Self {
            red,
            green,
            blue,
            alpha: Channel::MAX,
        })
    }
}

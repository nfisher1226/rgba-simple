use {
    crate::{ColorChannel, ColorError, Hex, Primary, PrimaryColor, PrimaryColor::*},
    serde::{Deserialize, Serialize},
    std::fmt,
};

#[derive(Clone, Copy, Deserialize, Debug, PartialEq, Serialize)]
pub struct RGBA<T>
where T: ColorChannel + Copy + PartialOrd {
    red: T,
    green: T,
    blue: T,
    alpha: T,
}

impl<T> fmt::Display for RGBA<T>
where T: ColorChannel + Copy + PartialOrd {
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
where T: ColorChannel + Copy + PartialOrd {
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
where T: ColorChannel + Hex + Copy + PartialOrd {
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

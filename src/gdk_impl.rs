#![allow(clippy::enum_glob_use)]
use crate::{Channel, ColorError, Hex, PrimaryColor, PrimaryColor::*, RGB, RGBA};

impl<T> From<RGB<T>> for gdk::RGBA
where
    T: Channel,
{
    fn from(color: RGB<T>) -> gdk::RGBA {
        gdk::builders::RGBABuilder::new()
            .red(color.red.to_percent())
            .green(color.green.to_percent())
            .blue(color.blue.to_percent())
            .alpha(1.0)
            .build()
    }
}

impl<T> From<RGBA<T>> for gdk::RGBA
where
    T: Channel,
{
    fn from(color: RGBA<T>) -> gdk::RGBA {
        gdk::builders::RGBABuilder::new()
            .red(color.red.to_percent())
            .green(color.green.to_percent())
            .blue(color.blue.to_percent())
            .alpha(color.alpha.to_percent())
            .build()
    }
}

impl<T> From<gdk::RGBA> for RGB<T>
where
    T: Channel,
{
    fn from(color: gdk::RGBA) -> Self {
        Self {
            red: T::from_percent(color.red()),
            green: T::from_percent(color.green()),
            blue: T::from_percent(color.blue()),
        }
    }
}

impl<T> From<gdk::RGBA> for RGBA<T>
where
    T: Channel,
{
    fn from(color: gdk::RGBA) -> Self {
        Self {
            red: T::from_percent(color.red()),
            green: T::from_percent(color.green()),
            blue: T::from_percent(color.blue()),
            alpha: T::from_percent(color.alpha()),
        }
    }
}

impl Hex for gdk::RGBA {
    type Err = ColorError;

    fn to_hex(&self) -> String {
        format!(
            "#{}{}{}",
            self.red().to_hex(),
            self.green().to_hex(),
            self.blue().to_hex(),
        )
    }

    fn from_hex(hex: &str) -> Result<Self, Self::Err> {
        crate::hex::validate_hex_string(hex)?;
        let red = match f32::from_hex(&hex[1..3]) {
            Ok(r) => r,
            Err(_) => return Err(ColorError::InvalidHexCharacter),
        };
        let green = match f32::from_hex(&hex[3..5]) {
            Ok(g) => g,
            Err(_) => return Err(ColorError::InvalidHexCharacter),
        };
        let blue = match f32::from_hex(&hex[5..7]) {
            Ok(b) => b,
            Err(_) => return Err(ColorError::InvalidHexCharacter),
        };
        for channel in [red, green, blue] {
            if channel < 0.0 {
                return Err(ColorError::OutsideBoundsNegative);
            } else if channel > 1.0 {
                return Err(ColorError::OutsideBoundsHigh);
            }
        }
        Ok(gdk::builders::RGBABuilder::new()
            .red(red)
            .green(green)
            .blue(blue)
            .alpha(1.0)
            .build())
    }
}

impl From<PrimaryColor> for gdk::RGBA {
    fn from(color: PrimaryColor) -> Self {
        gdk::builders::RGBABuilder::new()
            .red(match color {
                Black | Green | Blue | Cyan => 0.0,
                _ => 1.0,
            })
            .green(match color {
                Black | Red | Blue | Magenta => 0.0,
                _ => 1.0,
            })
            .blue(match color {
                Black | Red | Green | Yellow => 0.0,
                _ => 1.0,
            })
            .alpha(1.0)
            .build()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn u8_to_gdk() {
        let red: gdk::RGBA = RGBA::<u8>::try_new(255, 0, 0, 0).unwrap().into();
        assert_eq!(1.0, red.red());
        assert_eq!(0.0, red.blue());
    }
}

use crate::{ColorChannel, ColorError, Hex, RGB, RGBA, Primary, PrimaryColor, PrimaryColor::*};

/// Convert to the `gdk::RGBA` color representation
pub trait ToGdk {
    fn to_gdk(&self) -> gdk::RGBA;
}

/// Convert from the `gdk::RGBA` color representation
pub trait FromGdk {
    fn from_gdk(_: gdk::RGBA) -> Self;
}

impl<T> ToGdk for RGB<T>
where
    T: ColorChannel,
{
    fn to_gdk(&self) -> gdk::RGBA {
        gdk::builders::RGBABuilder::new()
            .red(self.red.to_percent())
            .green(self.green.to_percent())
            .blue(self.blue.to_percent())
            .alpha(1.0)
            .build()
    }
}

impl<T> ToGdk for RGBA<T>
where
    T: ColorChannel,
{
    fn to_gdk(&self) -> gdk::RGBA {
        gdk::builders::RGBABuilder::new()
            .red(self.red.to_percent())
            .green(self.green.to_percent())
            .blue(self.blue.to_percent())
            .alpha(self.alpha.to_percent())
            .build()
    }
}

impl<T> FromGdk for RGB<T>
where
    T: ColorChannel,
{
    fn from_gdk(color: gdk::RGBA) -> Self {
        Self {
            red: T::from_percent(color.red()),
            green: T::from_percent(color.green()),
            blue: T::from_percent(color.blue()),
        }
    }
}

impl<T> FromGdk for RGBA<T>
where
    T: ColorChannel,
{
    fn from_gdk(color: gdk::RGBA) -> Self {
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
            .build()
        )
    }
}

impl Primary for gdk::RGBA {
    fn primary(color: PrimaryColor) -> Self {
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
        let red = RGBA::<u8>::try_new(255, 0, 0, 0).unwrap().to_gdk();
        assert_eq!(1.0, red.red());
        assert_eq!(0.0, red.blue());
    }
}

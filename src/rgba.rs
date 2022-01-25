use crate::{ColorError, Convert, HexColor, Primary, PrimaryColor, ReducedRGBA, Validate};
#[cfg(feature = "gdk")]
use gdk;
use serde::{Deserialize, Serialize};

/// This struct represents colors in floating point precision as separate
/// Red, Green, and Blue channels plus a separate Alpha (Opacity) channel
#[derive(Clone, Copy, Deserialize, Debug, PartialEq, Serialize)]
pub struct RGBA {
    pub red: f32,
    pub green: f32,
    pub blue: f32,
    pub alpha: f32,
}

impl Validate for RGBA {
    type Err = ColorError;

    /// # Errors
    ///
    /// Will return `ColorError` if any field is less than 0 or greater
    /// than 1.0
    fn validate(&self) -> Result<(), ColorError> {
        if self.red < 0.0 || self.green < 0.0 || self.blue < 0.0 {
            Err(ColorError::OutsideBoundsNegative)
        } else if self.red > 1.0 || self.green > 1.0 || self.blue > 1.0 {
            Err(ColorError::OutsideBoundsHigh)
        } else {
            Ok(())
        }
    }
}

/// > Note: some of these operations are lossy
impl Convert for RGBA {
    type Err = ColorError;

    /// Converts an [RGBA] color (red, green, blue plus alpha) to a struct
    /// containing a hex color string and an opacity value, suitable for
    /// web usage.
    /// > Note: this operation is lossy
    /// # Errors
    ///
    /// Will return `ColorError` if any field is less than 0 or greater
    /// than 1.0
    #[allow(clippy::cast_sign_loss, clippy::cast_possible_truncation)]
    fn to_hex(&self) -> Result<HexColor, Self::Err> {
        self.validate()?;
        Ok(HexColor {
            color: format!(
                "#{:02x}{:02x}{:02x}",
                (self.red * 255.0) as u8,
                (self.green * 255.0) as u8,
                (self.blue * 255.0) as u8,
            ),
            alpha: self.alpha,
        })
    }

    /// # Errors
    ///
    /// Will return `ColorError` if any field is less than 0 or greater
    /// than 1.0
    fn to_rgba(&self) -> Result<Self, Self::Err> {
        self.validate()?;
        Ok(*self)
    }

    /// Converts an `RGBA` struct into a `ReducedRGBA` struct, using `u8` for
    /// storage of the color channels rather than f64.
    /// > Note: this operation is lossy
    /// # Errors
    ///
    /// Will return `ColorError` if any field is less than 0 or greater
    /// than 1.0
    #[allow(clippy::cast_sign_loss, clippy::cast_possible_truncation)]
    fn to_reduced_rgba(&self) -> Result<ReducedRGBA, Self::Err> {
        self.validate()?;
        Ok(ReducedRGBA {
            red: (self.red * 255.0) as u8,
            green: (self.green * 255.0) as u8,
            blue: (self.blue * 255.0) as u8,
            alpha: (self.alpha * 255.0) as u8,
        })
    }

    /// # Errors
    ///
    /// Will return `ColorError` if any field is less than 0 or greater
    /// than 1.0
    #[cfg(feature = "gdk")]
    fn to_gdk(&self) -> Result<gdk::RGBA, Self::Err> {
        self.validate()?;
        Ok(gdk::builders::RGBABuilder::new()
            .red(self.red)
            .green(self.green)
            .blue(self.blue)
            .alpha(self.alpha)
            .build())
    }
}

impl Primary for RGBA {
    fn primary(color: PrimaryColor) -> Self {
        Self {
            red: match color {
                PrimaryColor::Black
                | PrimaryColor::Green
                | PrimaryColor::Blue
                | PrimaryColor::Cyan => 0.0,
                _ => 1.0,
            },
            green: match color {
                PrimaryColor::Black
                | PrimaryColor::Red
                | PrimaryColor::Blue
                | PrimaryColor::Magenta => 0.0,
                _ => 1.0,
            },
            blue: match color {
                PrimaryColor::Black
                | PrimaryColor::Red
                | PrimaryColor::Green
                | PrimaryColor::Yellow => 0.0,
                _ => 1.0,
            },
            alpha: 1.0,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn black() {
        let k = RGBA::primary(PrimaryColor::Black);
        assert_eq!(
            k,
            RGBA {
                red: 0.0,
                green: 0.0,
                blue: 0.0,
                alpha: 1.0,
            }
        );
    }

    #[test]
    fn white() {
        let w = RGBA::primary(PrimaryColor::White);
        assert_eq!(
            w,
            RGBA {
                red: 1.0,
                green: 1.0,
                blue: 1.0,
                alpha: 1.0,
            }
        );
    }

    #[test]
    fn red() {
        let red = RGBA::primary(PrimaryColor::Red);
        assert_eq!(
            red,
            RGBA {
                red: 1.0,
                green: 0.0,
                blue: 0.0,
                alpha: 1.0,
            }
        );
    }

    #[test]
    fn green() {
        let grn = RGBA::primary(PrimaryColor::Green);
        assert_eq!(
            grn,
            RGBA {
                red: 0.0,
                green: 1.0,
                blue: 0.0,
                alpha: 1.0,
            }
        );
    }

    #[test]
    fn blue() {
        let blue = RGBA::primary(PrimaryColor::Blue);
        assert_eq!(
            blue,
            RGBA {
                red: 0.0,
                green: 0.0,
                blue: 1.0,
                alpha: 1.0,
            }
        );
    }

    #[test]
    fn yellow() {
        let yel = RGBA::primary(PrimaryColor::Yellow);
        assert_eq!(
            yel,
            RGBA {
                red: 1.0,
                green: 1.0,
                blue: 0.0,
                alpha: 1.0,
            }
        );
    }

    #[test]
    fn magenta() {
        let mag = RGBA::primary(PrimaryColor::Magenta);
        assert_eq!(
            mag,
            RGBA {
                red: 1.0,
                green: 0.0,
                blue: 1.0,
                alpha: 1.0,
            }
        );
    }

    #[test]
    fn cyan() {
        let c = RGBA::primary(PrimaryColor::Cyan);
        assert_eq!(
            c,
            RGBA {
                red: 0.0,
                green: 1.0,
                blue: 1.0,
                alpha: 1.0,
            }
        );
    }

    #[test]
    fn to_hex() {
        let red = RGBA::primary(PrimaryColor::Red).to_hex();
        assert_eq!(red, Ok(HexColor::primary(PrimaryColor::Red)));
    }

    #[test]
    fn to_hex_negative() {
        let invalid = RGBA {
            red: 1.0,
            green: 1.0,
            blue: -1.0,
            alpha: 1.0,
        };
        assert_eq!(invalid.to_hex(), Err(ColorError::OutsideBoundsNegative));
    }

    #[test]
    fn to_hex_high() {
        let invalid = RGBA {
            red: 1.1,
            green: 1.0,
            blue: 1.0,
            alpha: 1.0,
        };
        assert_eq!(invalid.to_hex(), Err(ColorError::OutsideBoundsHigh));
    }

    #[test]
    fn to_rgba() {
        let red = RGBA::primary(PrimaryColor::Red);
        let rgba = red.to_rgba().unwrap();
        assert_eq!(red, rgba);
    }

    #[test]
    fn to_rgba_negative() {
        let invalid = RGBA {
            red: 1.0,
            green: 1.0,
            blue: -1.0,
            alpha: 1.0,
        };
        assert_eq!(invalid.to_rgba(), Err(ColorError::OutsideBoundsNegative));
    }

    #[test]
    fn to_rgba_high() {
        let invalid = RGBA {
            red: 1.1,
            green: 1.0,
            blue: 1.0,
            alpha: 1.0,
        };
        assert_eq!(invalid.to_rgba(), Err(ColorError::OutsideBoundsHigh));
    }

    #[test]
    fn to_reduced_rgba() {
        let red = RGBA::primary(PrimaryColor::Red).to_reduced_rgba();
        assert_eq!(red, Ok(ReducedRGBA::primary(PrimaryColor::Red)));
    }

    #[test]
    fn to_reduced_negative() {
        let invalid = RGBA {
            red: 1.0,
            green: 1.0,
            blue: -1.0,
            alpha: 1.0,
        };
        assert_eq!(
            invalid.to_reduced_rgba(),
            Err(ColorError::OutsideBoundsNegative)
        );
    }

    #[test]
    fn to_reduced_high() {
        let invalid = RGBA {
            red: 1.1,
            green: 1.0,
            blue: 1.0,
            alpha: 1.0,
        };
        assert_eq!(
            invalid.to_reduced_rgba(),
            Err(ColorError::OutsideBoundsHigh)
        );
    }

    #[cfg(feature = "gdk")]
    #[test]
    fn rgba_to_gdk() {
        let red = RGBA::primary(PrimaryColor::Red);
        let gdk_red = red.to_gdk().unwrap();
        assert_eq!(red.red, gdk_red.red());
        assert_eq!(red.green, gdk_red.green());
        assert_eq!(red.blue, gdk_red.blue());
        assert_eq!(red.alpha, gdk_red.alpha());
    }

    #[cfg(feature = "gdk")]
    #[test]
    fn to_gdk_negative() {
        let invalid = RGBA {
            red: 1.0,
            green: 1.0,
            blue: -1.0,
            alpha: 1.0,
        };
        assert_eq!(invalid.to_gdk(), Err(ColorError::OutsideBoundsNegative));
    }

    #[cfg(feature = "gdk")]
    #[test]
    fn to_gdk_high() {
        let invalid = RGBA {
            red: 1.1,
            green: 1.0,
            blue: 1.0,
            alpha: 1.0,
        };
        assert_eq!(invalid.to_gdk(), Err(ColorError::OutsideBoundsHigh));
    }
}

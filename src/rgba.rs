#[cfg(feature = "gtk")]
use gtk::gdk;
use serde::{Deserialize, Serialize};
use crate::{ColorError, Convert, HexColor, Primary, ReducedRGBA};

/// This struct represents colors in floating point precision as separate
/// Red, Green, and Blue channels plus a separate Alpha (Opacity) channel
#[derive(Clone, Deserialize, Debug, PartialEq, Serialize)]
pub struct RGBA {
    pub red: f32,
    pub green: f32,
    pub blue: f32,
    pub alpha: f32,
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
        if self.red < 0.0 || self.green < 0.0 || self.blue < 0.0 {
            Err(ColorError::OutsideBoundsNegative)
        } else if self.red > 1.0 || self.green > 1.0 || self.blue> 1.0 {
            Err(ColorError::OutsideBoundsHigh)
        } else {
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
    }

    /// # Errors
    ///
    /// Will return `ColorError` if any field is less than 0 or greater
    /// than 1.0
    fn to_rgba(&self) -> Result<Self, Self::Err> {
        if self.red < 0.0 || self.green < 0.0 || self.blue < 0.0 {
            Err(ColorError::OutsideBoundsNegative)
        } else if self.red > 1.0 || self.green > 1.0 || self.blue > 1.0 {
            Err(ColorError::OutsideBoundsHigh)
        } else {
            Ok(self.clone())
        }
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
        if self.red < 0.0 || self.green < 0.0 || self.blue < 0.0 {
            Err(ColorError::OutsideBoundsNegative)
        } else if self.red > 1.0 || self.green > 1.0 || self.blue > 1.0 {
            Err(ColorError::OutsideBoundsHigh)
        } else {
            Ok(ReducedRGBA {
                red: (self.red * 255.0) as u8,
                green: (self.green * 255.0) as u8,
                blue: (self.blue * 255.0) as u8,
                alpha: (self.alpha * 255.0) as u8,
            })
        }
    }

    /// # Errors
    ///
    /// Will return `ColorError` if any field is less than 0 or greater
    /// than 1.0
    #[cfg(feature = "gtk")]
    fn to_gdk(&self) -> Result<gdk::RGBA, Self::Err> {
        if self.red < 0.0 || self.green < 0.0 || self.blue < 0.0 {
            Err(ColorError::OutsideBoundsNegative)
        } else if self.red > 1.0 || self.green > 1.0 || self.blue > 1.0 {
            Err(ColorError::OutsideBoundsHigh)
        } else {
            Ok(gdk::RGBA {
                red: self.red,
                green: self.green,
                blue: self.blue,
                alpha: self.alpha,
            })
        }
    }
}

impl Primary for RGBA {
    fn black() -> Self {
        Self {
            red: 0.0,
            green: 0.0,
            blue: 0.0,
            alpha: 1.0,
        }
    }

    fn white() -> Self {
        Self {
            red: 1.0,
            green: 1.0,
            blue: 1.0,
            alpha: 1.0,
        }
    }

    /// Returns "red" as an [RGBA] struct
    fn red() -> Self {
        Self {
            red: 1.0,
            green: 0.0,
            blue: 0.0,
            alpha: 1.0,
        }
    }

    /// Returns "green" as an [RGBA] struct
    fn green() -> Self {
        Self {
            red: 0.0,
            green: 1.0,
            blue: 0.0,
            alpha: 1.0,
        }
    }

    /// Returns "blue" as an [RGBA] struct
    fn blue() -> Self {
        Self {
            red: 0.0,
            green: 0.0,
            blue: 1.0,
            alpha: 1.0,
        }
    }

    /// Returns "yellow" as an [RGBA] struct
    fn yellow() -> Self {
        Self {
            red: 1.0,
            green: 1.0,
            blue: 0.0,
            alpha: 1.0,
        }
    }

    /// Returns "magenta" as an [RGBA] struct
    fn magenta() -> Self {
        Self {
            red: 1.0,
            green: 0.0,
            blue: 1.0,
            alpha: 1.0,
        }
    }

    /// Returns "cyan" as an [RGBA] struct
    fn cyan() -> Self {
        Self {
            red: 0.0,
            green: 1.0,
            blue: 1.0,
            alpha: 1.0,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn black() {
        let k = RGBA::black();
        assert_eq!(k, RGBA {
            red: 0.0,
            green: 0.0,
            blue: 0.0,
            alpha: 1.0,
        });
    }

    #[test]
    fn white() {
        let w = RGBA::white();
        assert_eq!(w, RGBA {
            red: 1.0,
            green: 1.0,
            blue: 1.0,
            alpha: 1.0,
        });
    }

    #[test]
    fn red() {
        let red = RGBA::red();
        assert_eq!(red, RGBA {
            red: 1.0,
            green: 0.0,
            blue: 0.0,
            alpha: 1.0,
        });
    }

    #[test]
    fn green() {
        let grn = RGBA::green();
            assert_eq!(grn, RGBA {
            red: 0.0,
            green: 1.0,
            blue: 0.0,
            alpha: 1.0,
        });
    }

    #[test]
    fn blue() {
        let blue = RGBA::blue();
            assert_eq!(blue, RGBA {
            red: 0.0,
            green: 0.0,
            blue: 1.0,
            alpha: 1.0,
        });
    }

    #[test]
    fn yellow() {
        let yel = RGBA::yellow();
            assert_eq!(yel, RGBA {
            red: 1.0,
            green: 1.0,
            blue: 0.0,
            alpha: 1.0,
        });
    }

    #[test]
    fn magenta() {
        let mag = RGBA::magenta();
            assert_eq!(mag, RGBA {
            red: 1.0,
            green: 0.0,
            blue: 1.0,
            alpha: 1.0,
        });
    }

    #[test]
    fn cyan() {
        let c = RGBA::cyan();
            assert_eq!(c, RGBA {
            red: 0.0,
            green: 1.0,
            blue: 1.0,
            alpha: 1.0,
        });
    }

    #[test]
    fn to_hex() {
        let red = RGBA::red().to_hex();
        assert_eq!(red, Ok(HexColor::red()));
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
        let red = RGBA::red();
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
        let red = RGBA::red().to_reduced_rgba();
        assert_eq!(red, Ok(ReducedRGBA::red()));
    }

    #[test]
    fn to_reduced_negative() {
        let invalid = RGBA {
            red: 1.0,
            green: 1.0,
            blue: -1.0,
            alpha: 1.0,
        };
        assert_eq!(invalid.to_reduced_rgba(), Err(ColorError::OutsideBoundsNegative));
    }

    #[test]
    fn to_reduced_high() {
        let invalid = RGBA {
            red: 1.1,
            green: 1.0,
            blue: 1.0,
            alpha: 1.0,
        };
        assert_eq!(invalid.to_reduced_rgba(), Err(ColorError::OutsideBoundsHigh));
    }

    #[cfg(feature = "gtk")]
    #[test]
    fn rgba_to_gdk() {
        let red = RGBA::red();
        let gdk_red = red.to_gdk().unwrap();
        assert_eq!(red.red, gdk_red.red);
        assert_eq!(red.green, gdk_red.green);
        assert_eq!(red.blue, gdk_red.blue);
        assert_eq!(red.alpha, gdk_red.alpha);
    }

    #[cfg(feature = "gtk")]
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

    #[cfg(feature = "gtk")]
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

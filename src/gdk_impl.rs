use gtk::gdk;
use crate::{ColorError, Convert, HexColor, Primary, ReducedRGBA, RGBA};

/// Note: some of these operations are lossy
impl Convert for gdk::RGBA {
    type Err = ColorError;

    /// > Note: this operation is lossy
    /// # Errors
    ///
    /// Will return `ColorError` if any field is less than 0 or greater
    /// than 1.0
    #[allow(clippy::cast_sign_loss, clippy::cast_possible_truncation)]
    fn to_hex(&self) -> Result<HexColor, Self::Err> {
        if self.red < 0.0 || self.green < 0.0 || self.blue < 0.0 {
            Err(ColorError::OutsideBoundsNegative)
        } else if self.red > 1.0 || self.green > 1.0 || self.blue > 1.0 {
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
    fn to_rgba(&self) -> Result<RGBA, Self::Err> {
        if self.red < 0.0 || self.green < 0.0 || self.blue < 0.0 {
            Err(ColorError::OutsideBoundsNegative)
        } else if self.red > 1.0 || self.green > 1.0 || self.blue > 1.0 {
            Err(ColorError::OutsideBoundsHigh)
        } else {
            Ok(RGBA {
                red: self.red,
                green: self.green,
                blue: self.blue,
                alpha: self.alpha,
            })
        }
    }

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
    fn to_gdk(&self) -> Result<Self, Self::Err> {
        if self.red < 0.0 || self.green < 0.0 || self.blue < 0.0 {
            Err(ColorError::OutsideBoundsNegative)
        } else if self.red > 1.0 || self.green > 1.0 || self.blue > 1.0 {
            Err(ColorError::OutsideBoundsHigh)
        } else {
            Ok(*self)
        }
    }
}

impl Primary for gdk::RGBA {
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

    fn red() -> Self {
        Self {
            red: 1.0,
            green: 0.0,
            blue: 0.0,
            alpha: 1.0,
        }
    }

    fn green() -> Self {
        Self {
            red: 0.0,
            green: 1.0,
            blue: 0.0,
            alpha: 1.0,
        }
    }

    fn blue() -> Self {
        Self {
            red: 0.0,
            green: 0.0,
            blue: 1.0,
            alpha: 1.0,
        }
    }

    fn yellow() -> Self {
        Self {
            red: 1.0,
            green: 1.0,
            blue: 0.0,
            alpha: 1.0,
        }
    }

    fn magenta() -> Self {
        Self {
            red: 1.0,
            green: 0.0,
            blue: 1.0,
            alpha: 1.0,
        }
    }

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
        let k = gdk::RGBA::black();
        assert_eq!(k, gdk::RGBA {
            red: 0.0,
            green: 0.0,
            blue: 0.0,
            alpha: 1.0,
        });
    }

    #[test]
    fn white() {
        let w = gdk::RGBA::white();
        assert_eq!(w, gdk::RGBA {
            red: 1.0,
            green: 1.0,
            blue: 1.0,
            alpha: 1.0,
        });
    }

    #[test]
    fn red() {
        let red = gdk::RGBA::red();
        assert_eq!(red, gdk::RGBA {
            red: 1.0,
            green: 0.0,
            blue: 0.0,
            alpha: 1.0,
        });
    }

    #[test]
    fn green() {
        let grn = gdk::RGBA::green();
            assert_eq!(grn, gdk::RGBA {
            red: 0.0,
            green: 1.0,
            blue: 0.0,
            alpha: 1.0,
        });
    }

    #[test]
    fn blue() {
        let blue = gdk::RGBA::blue();
            assert_eq!(blue, gdk::RGBA {
            red: 0.0,
            green: 0.0,
            blue: 1.0,
            alpha: 1.0,
        });
    }

    #[test]
    fn yellow() {
        let yel = gdk::RGBA::yellow();
            assert_eq!(yel, gdk::RGBA {
            red: 1.0,
            green: 1.0,
            blue: 0.0,
            alpha: 1.0,
        });
    }

    #[test]
    fn magenta() {
        let mag = gdk::RGBA::magenta();
            assert_eq!(mag, gdk::RGBA {
            red: 1.0,
            green: 0.0,
            blue: 1.0,
            alpha: 1.0,
        });
    }

    #[test]
    fn cyan() {
        let c = gdk::RGBA::cyan();
            assert_eq!(c, gdk::RGBA {
            red: 0.0,
            green: 1.0,
            blue: 1.0,
            alpha: 1.0,
        });
    }

    #[test]
    fn to_hex() {
        let red = gdk::RGBA::red();
        let red_hex = red.to_hex().unwrap();
        assert_eq!(red_hex.color, String::from("#ff0000"));
        assert_eq!(red_hex.alpha, 1.0);
    }

    #[test]
    fn to_hex_negative() {
        let invalid = gdk::RGBA {
            red: 1.0,
            green: 1.0,
            blue: -1.0,
            alpha: 1.0,
        };
        assert_eq!(invalid.to_hex(), Err(ColorError::OutsideBoundsNegative));
    }

    #[test]
    fn to_hex_high() {
        let invalid = gdk::RGBA {
            red: 1.1,
            green: 1.0,
            blue: 1.0,
            alpha: 1.0,
        };
        assert_eq!(invalid.to_hex(), Err(ColorError::OutsideBoundsHigh));
    }

    #[test]
    fn to_rgba() {
        let red = gdk::RGBA::red();
        let rgba = red.to_rgba();
        assert_eq!(rgba, Ok(RGBA::red()));
    }

    #[test]
    fn to_rgba_negative() {
        let invalid = gdk::RGBA {
            red: 1.0,
            green: 1.0,
            blue: -1.0,
            alpha: 1.0,
        };
        assert_eq!(invalid.to_rgba(), Err(ColorError::OutsideBoundsNegative));
    }

    #[test]
    fn to_rgba_high() {
        let invalid = gdk::RGBA {
            red: 1.1,
            green: 1.0,
            blue: 1.0,
            alpha: 1.0,
        };
        assert_eq!(invalid.to_rgba(), Err(ColorError::OutsideBoundsHigh));
    }

    #[test]
    fn to_reduced_rgba() {
        let red = gdk::RGBA::red().to_reduced_rgba();
        assert_eq!(red, Ok(ReducedRGBA::red()));
    }

    #[test]
    fn to_reduced_negative() {
        let invalid = gdk::RGBA {
            red: 1.0,
            green: 1.0,
            blue: -1.0,
            alpha: 1.0,
        };
        assert_eq!(invalid.to_reduced_rgba(), Err(ColorError::OutsideBoundsNegative));
    }

    #[test]
    fn to_reduced_high() {
        let invalid = gdk::RGBA {
            red: 1.1,
            green: 1.0,
            blue: 1.0,
            alpha: 1.0,
        };
        assert_eq!(invalid.to_reduced_rgba(), Err(ColorError::OutsideBoundsHigh));
    }

    #[test]
    fn rgba_to_gdk() {
        let red = gdk::RGBA::red();
        let gdk_red = red.to_gdk().unwrap();
        assert_eq!(red, gdk_red);
    }

    #[test]
    fn to_gdk_negative() {
        let invalid = gdk::RGBA {
            red: 1.0,
            green: 1.0,
            blue: -1.0,
            alpha: 1.0,
        };
        assert_eq!(invalid.to_gdk(), Err(ColorError::OutsideBoundsNegative));
    }

    #[test]
    fn to_gdk_high() {
        let invalid = gdk::RGBA {
            red: 1.1,
            green: 1.0,
            blue: 1.0,
            alpha: 1.0,
        };
        assert_eq!(invalid.to_gdk(), Err(ColorError::OutsideBoundsHigh));
    }
}

use crate::{ColorError, Float, HexColor, NumType, Primary, PrimaryColor::*, Validate, ToHex};
#[cfg(feature = "gdk")]
use {crate::ToGdk, gdk};
use num::{Num, FromPrimitive, ToPrimitive};
use serde::{Deserialize, Serialize};
use std::fmt;

/// This struct represents colors in floating point precision as separate
/// Red, Green, and Blue channels plus a separate Alpha (Opacity) channel
#[derive(Clone, Copy, Deserialize, Debug, PartialEq, Serialize)]
pub struct RGBA<T: NumType> {
    pub red: T,
    pub green: T,
    pub blue: T,
    pub alpha: T,
}

impl<T> fmt::Display for RGBA<T>
where T: fmt::Display + Num + NumType<Num = Float> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "RGBA({:.3}, {:.3}, {:.3}, {:.3})", self.red, self.green, self.blue, self.alpha)
    }
}

impl<T> Validate for RGBA<T>
where T: num::Float + FromPrimitive + NumType {
    type Err = ColorError;

    /// # Errors
    ///
    /// Will return `ColorError` if any field is less than 0 or greater
    /// than 1.0
    fn validate(&self) -> Result<(), ColorError> {
        if self.red < FromPrimitive::from_f64(0.0).unwrap()
            || self.green < FromPrimitive::from_f64(0.0).unwrap()
            || self.blue < FromPrimitive::from_f64(0.0).unwrap() {
            Err(ColorError::OutsideBoundsNegative)
        } else if self.red > FromPrimitive::from_f64(1.0).unwrap()
            || self.green > FromPrimitive::from_f64(1.0).unwrap()
            || self.blue > FromPrimitive::from_f64(1.0).unwrap() {
            Err(ColorError::OutsideBoundsHigh)
        } else {
            Ok(())
        }
    }
}

impl<T> ToHex for RGBA<T>
where T: num::Num + ToPrimitive + NumType<Num = Float> {
    type Err = ColorError;

    fn to_hex(&self) -> Result<HexColor, Self::Err> {
        Ok(HexColor {
            color: format!(
                "#{:02x}{:02x}{:02x}",
                (self.red.to_f32().unwrap() * 255.0).round() as u8,
                (self.green.to_f32().unwrap() * 255.0).round() as u8,
                (self.blue.to_f32().unwrap() * 255.0).round() as u8,
            ),
            alpha: self.alpha.to_f32().unwrap()
        })
    }
}

#[cfg(feature = "gdk")]
impl<T> ToGdk for RGBA<T>
where T: num::Num + ToPrimitive + NumType<Num = Float> {
    type Err = ColorError;

    fn to_gdk(&self) -> Result<gdk::RGBA, Self::Err> {
        Ok(gdk::builders::RGBABuilder::new()
            .red(self.red.to_f32().unwrap())
            .green(self.green.to_f32().unwrap())
            .blue(self.blue.to_f32().unwrap())
            .alpha(self.alpha.to_f32().unwrap())
            .build())
    }
}

impl<T> Primary for RGBA<T>
where T: num::Float + FromPrimitive +NumType {
    fn primary(color: crate::PrimaryColor) -> Self {
        Self {
            red: match color {
                Black | Green | Blue | Cyan => FromPrimitive::from_f64(0.0).unwrap(),
                _ => FromPrimitive::from_f64(1.0).unwrap(),
            },
            green: match color {
                Black | Red | Blue | Magenta => FromPrimitive::from_f64(0.0).unwrap(),
                _ => FromPrimitive::from_f64(1.0).unwrap(),
            },
            blue: match color {
                Black | Red | Green | Yellow => FromPrimitive::from_f64(0.0).unwrap(),
                _ => FromPrimitive::from_f64(1.0).unwrap(),
            },
            alpha: FromPrimitive::from_f64(1.0).unwrap(),
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
    fn to_string() {
        let blk = RGBA::primary(PrimaryColor::Black).to_string();
        assert_eq!(blk, "RGBA(0.000, 0.000, 0.000, 1.000)");
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

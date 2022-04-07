use crate::{ColorError, Int, HexColor, NumType, Primary, PrimaryColor::*, ToHex};
#[cfg(feature = "gdk")]
use {gdk, crate::ToGdk};
use num::ToPrimitive;
use serde::{Deserialize, Serialize};
use std::fmt;

/// This struct represents colors in 8-bit precision as separate
/// Red, Green, and Blue channels
#[derive(Clone, Copy, Deserialize, Debug, PartialEq, Serialize)]
pub struct RGBA<T: num::Num + NumType> {
    pub red: T,
    pub green: T,
    pub blue: T,
    pub alpha: T,
}

impl<T> fmt::Display for RGBA<T>
where T: fmt::Display + num::Num + NumType<Num = Int> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "RGBA({}, {}, {}, {})", self.red, self.green, self.blue, self.alpha)
    }
}

impl<T> ToHex for RGBA<T>
where T: num::Num + ToPrimitive + NumType<Num = Int> {
    type Err = ColorError;

    fn to_hex(&self) -> Result<HexColor, Self::Err> {
        Ok(HexColor {
            color: format!(
                "#{:02x}{:02x}{:02x}",
                self.red.to_u8().unwrap(),
                self.green.to_u8().unwrap(),
                self.blue.to_u8().unwrap()
            ),
            alpha: self.alpha.to_f32().unwrap() / 255.0,
        })
    }
}

#[cfg(feature = "gdk")]
impl<T> ToGdk for RGBA<T>
where T: num::Num + ToPrimitive + NumType<Num = Int> {
    type Err = ColorError;

    fn to_gdk(&self) -> Result<gdk::RGBA, Self::Err> {
        Ok(gdk::builders::RGBABuilder::new()
            .red(self.red.to_f32().unwrap() / 255.0)
            .green(self.green.to_f32().unwrap() / 255.0)
            .blue(self.blue.to_f32().unwrap() / 255.0)
            .alpha(self.alpha.to_f32().unwrap() / 255.0)
            .build())
    }
}

/*
/// > Note: none of these operations are lossy
impl Convert for ReducedRGBA {
    type Err = ColorError;

    fn to_hex(&self) -> Result<HexColor, Self::Err> {
        Ok(HexColor {
            color: format!("#{:02x}{:02x}{:02x}", self.red, self.green, self.blue,),
            alpha: f32::from(self.alpha) / 255.0,
        })
    }

    fn to_rgba(&self) -> Result<RGBA, Self::Err> {
        Ok(RGBA {
            red: f32::from(self.red) / 255.0,
            green: f32::from(self.green) / 255.0,
            blue: f32::from(self.blue) / 255.0,
            alpha: f32::from(self.alpha) / 255.0,
        })
    }

    #[allow(clippy::cast_sign_loss)]
    fn to_reduced_rgba(&self) -> Result<Self, Self::Err> {
        Ok(*self)
    }

    #[cfg(feature = "gdk")]
    fn to_gdk(&self) -> Result<gdk::RGBA, Self::Err> {
        Ok(gdk::builders::RGBABuilder::new()
            .red(f32::from(self.red) / 255.0)
            .green(f32::from(self.green) / 255.0)
            .blue(f32::from(self.blue) / 255.0)
            .alpha(f32::from(self.alpha) / 255.0)
            .build())
    }
}
*/

impl<T> Primary for RGBA<T>
where T: num::Integer + num::FromPrimitive +NumType {
    fn primary(color: crate::PrimaryColor) -> Self {
        Self {
            red: match color {
                Black | Green | Blue | Cyan => num::FromPrimitive::from_u8(0).unwrap(),
                _ => num::FromPrimitive::from_u8(255).unwrap(),
            },
            green: match color {
                Black | Red | Blue | Magenta => num::FromPrimitive::from_u8(0).unwrap(),
                _ => num::FromPrimitive::from_u8(255).unwrap(),
            },
            blue: match color {
                Black | Red | Green | Yellow => num::FromPrimitive::from_u8(0).unwrap(),
                _ => num::FromPrimitive::from_u8(255).unwrap(),
            },
            alpha: num::FromPrimitive::from_u8(255).unwrap(),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn black() {
        let k = ReducedRGBA::primary(PrimaryColor::Black);
        assert_eq!(
            k,
            ReducedRGBA {
                red: 0,
                green: 0,
                blue: 0,
                alpha: 255,
            }
        );
    }

    #[test]
    fn white() {
        let w = ReducedRGBA::primary(PrimaryColor::White);
        assert_eq!(
            w,
            ReducedRGBA {
                red: 255,
                green: 255,
                blue: 255,
                alpha: 255,
            }
        );
    }

    #[test]
    fn red() {
        let red = ReducedRGBA::primary(PrimaryColor::Red);
        assert_eq!(
            red,
            ReducedRGBA {
                red: 255,
                green: 0,
                blue: 0,
                alpha: 255,
            }
        );
    }

    #[test]
    fn green() {
        let grn = ReducedRGBA::primary(PrimaryColor::Green);
        assert_eq!(
            grn,
            ReducedRGBA {
                red: 0,
                green: 255,
                blue: 0,
                alpha: 255,
            }
        );
    }

    #[test]
    fn blue() {
        let blue = ReducedRGBA::primary(PrimaryColor::Blue);
        assert_eq!(
            blue,
            ReducedRGBA {
                red: 0,
                green: 0,
                blue: 255,
                alpha: 255,
            }
        );
    }

    #[test]
    fn yellow() {
        let yel = ReducedRGBA::primary(PrimaryColor::Yellow);
        assert_eq!(
            yel,
            ReducedRGBA {
                red: 255,
                green: 255,
                blue: 0,
                alpha: 255,
            }
        );
    }

    #[test]
    fn magenta() {
        let mag = ReducedRGBA::primary(PrimaryColor::Magenta);
        assert_eq!(
            mag,
            ReducedRGBA {
                red: 255,
                green: 0,
                blue: 255,
                alpha: 255,
            }
        );
    }

    #[test]
    fn cyan() {
        let c = ReducedRGBA::primary(PrimaryColor::Cyan);
        assert_eq!(
            c,
            ReducedRGBA {
                red: 0,
                green: 255,
                blue: 255,
                alpha: 255,
            }
        );
    }

    #[test]
    fn to_string() {
        let blk = ReducedRGBA::primary(PrimaryColor::Black).to_string();
        assert_eq!(blk, "ReducedRGBA(0, 0, 0, 255)");
    }

    #[test]
    fn to_hex() {
        let red = ReducedRGBA::primary(PrimaryColor::Red);
        let red_hex = red.to_hex().unwrap();
        assert_eq!(red_hex.color, String::from("#ff0000"));
        assert_eq!(red_hex.alpha, 1.0);
    }

    #[test]
    fn to_rgba() {
        let red = ReducedRGBA::primary(PrimaryColor::Red).to_rgba();
        assert_eq!(red, Ok(RGBA::primary(PrimaryColor::Red)));
    }

    #[test]
    fn to_reduced_rgba() {
        let red = ReducedRGBA::primary(PrimaryColor::Red);
        assert_eq!(red, red.to_reduced_rgba().unwrap());
    }

    #[cfg(feature = "gdk")]
    #[test]
    fn rgba_to_gdk() {
        let red = ReducedRGBA::primary(PrimaryColor::Red).to_gdk().unwrap();
        assert_eq!(red.red(), 1.0);
        assert_eq!(red.green(), 0.0);
        assert_eq!(red.blue(), 0.0);
        assert_eq!(red.alpha(), 1.0);
    }
}

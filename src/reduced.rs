use crate::{ColorError, Convert, HexColor, Primary, PrimaryColor, RGBA};
#[cfg(feature = "gdk")]
use gdk;
use serde::{Deserialize, Serialize};

/// This struct represents colors in 8-bit precision as separate
/// Red, Green, and Blue channels
#[derive(Clone, Copy, Deserialize, Debug, PartialEq, Serialize)]
pub struct ReducedRGBA {
    pub red: u8,
    pub green: u8,
    pub blue: u8,
    pub alpha: u8,
}

impl ToString for ReducedRGBA {
    fn to_string(&self) -> String {
        format!("ReducedRGBA({}, {}, {}, {})", self.red, self.green, self.blue, self.alpha)
    }
}

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

impl Primary for ReducedRGBA {
    fn primary(color: PrimaryColor) -> Self {
        Self {
            red: match color {
                PrimaryColor::Black
                | PrimaryColor::Green
                | PrimaryColor::Blue
                | PrimaryColor::Cyan => 0,
                _ => 255,
            },
            green: match color {
                PrimaryColor::Black
                | PrimaryColor::Red
                | PrimaryColor::Blue
                | PrimaryColor::Magenta => 0,
                _ => 255,
            },
            blue: match color {
                PrimaryColor::Black
                | PrimaryColor::Red
                | PrimaryColor::Green
                | PrimaryColor::Yellow => 0,
                _ => 255,
            },
            alpha: 255,
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
        let blk = ReducedRGBA::black().to_string();
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

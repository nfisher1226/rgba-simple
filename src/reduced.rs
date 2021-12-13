#[cfg(feature = "gtk")]
use gtk::gdk;
use serde::{Deserialize, Serialize};
use crate::{ColorError, Convert, HexColor, Primary, RGBA};

/// This struct represents colors in 8-bit precision as separate
/// Red, Green, and Blue channels
#[derive(Clone, Copy, Deserialize, Debug, PartialEq, Serialize)]
pub struct ReducedRGBA {
    pub red: u8,
    pub green: u8,
    pub blue: u8,
    pub alpha: u8,
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

    #[cfg(feature = "gtk")]
    fn to_gdk(&self) -> Result<gdk::RGBA, Self::Err> {
        Ok(gdk::RGBA {
            red: f32::from(self.red) / 255.0,
            green: f32::from(self.green) / 255.0,
            blue: f32::from(self.blue) / 255.0,
            alpha: f32::from(self.alpha) / 255.0,
        })
    }
}

impl Primary for ReducedRGBA {
    fn black() -> Self {
        Self {
            red: 0,
            green: 0,
            blue: 0,
            alpha: 255,
        }
    }

    fn white() -> Self {
        Self {
            red: 255,
            green: 255,
            blue: 255,
            alpha: 255,
        }
    }

    fn red() -> Self {
        Self {
            red: 255,
            green: 0,
            blue: 0,
            alpha: 255,
        }
    }

    fn green() -> Self {
        Self {
            red: 0,
            green: 255,
            blue: 0,
            alpha: 255,
        }
    }

    fn blue() -> Self {
        Self {
            red: 0,
            green: 0,
            blue: 255,
            alpha: 255,
        }
    }

    fn yellow() -> Self {
        Self {
            red: 255,
            green: 255,
            blue: 0,
            alpha: 255,
        }
    }

    fn magenta() -> Self {
        Self {
            red: 255,
            green: 0,
            blue: 255,
            alpha: 255,
        }
    }

    fn cyan() -> Self {
        Self {
            red: 0,
            green: 255,
            blue: 255,
            alpha: 255,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn black() {
        let k = ReducedRGBA::black();
        assert_eq!(k, ReducedRGBA {
            red: 0,
            green: 0,
            blue: 0,
            alpha: 255,
        });
    }

    #[test]
    fn white() {
        let w = ReducedRGBA::white();
        assert_eq!(w, ReducedRGBA {
            red: 255,
            green: 255,
            blue: 255,
            alpha: 255,
        });
    }

    #[test]
    fn red() {
        let red = ReducedRGBA::red();
        assert_eq!(red, ReducedRGBA {
            red: 255,
            green: 0,
            blue: 0,
            alpha: 255,
        });
    }

    #[test]
    fn green() {
        let grn = ReducedRGBA::green();
            assert_eq!(grn, ReducedRGBA {
            red: 0,
            green: 255,
            blue: 0,
            alpha: 255,
        });
    }

    #[test]
    fn blue() {
        let blue = ReducedRGBA::blue();
            assert_eq!(blue, ReducedRGBA {
            red: 0,
            green: 0,
            blue: 255,
            alpha: 255,
        });
    }

    #[test]
    fn yellow() {
        let yel = ReducedRGBA::yellow();
            assert_eq!(yel, ReducedRGBA {
            red: 255,
            green: 255,
            blue: 0,
            alpha: 255,
        });
    }

    #[test]
    fn magenta() {
        let mag = ReducedRGBA::magenta();
            assert_eq!(mag, ReducedRGBA {
            red: 255,
            green: 0,
            blue: 255,
            alpha: 255,
        });
    }

    #[test]
    fn cyan() {
        let c = ReducedRGBA::cyan();
            assert_eq!(c, ReducedRGBA {
            red: 0,
            green: 255,
            blue: 255,
            alpha: 255,
        });
    }

    #[test]
    fn to_hex() {
        let red = ReducedRGBA::red();
        let red_hex = red.to_hex().unwrap();
        assert_eq!(red_hex.color, String::from("#ff0000"));
        assert_eq!(red_hex.alpha, 1.0);
    }

    #[test]
    fn to_rgba() {
        let red = ReducedRGBA::red().to_rgba();
        assert_eq!(red, Ok(RGBA::red()));
    }

    #[test]
    fn to_reduced_rgba() {
        let red = ReducedRGBA::red();
        assert_eq!(red, red.to_reduced_rgba().unwrap());
    }

    #[cfg(feature = "gtk")]
    #[test]
    fn rgba_to_gdk() {
        let red = ReducedRGBA::red().to_gdk().unwrap();
        assert_eq!(red.red, 1.0);
        assert_eq!(red.green, 0.0);
        assert_eq!(red.blue, 0.0);
        assert_eq!(red.alpha, 1.0);
    }
}

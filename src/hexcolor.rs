#[cfg(feature = "gtk")]
use gtk::gdk;
use serde::{Deserialize, Serialize};
use crate::{ColorError, Convert, Primary, ReducedRGBA, RGBA};

/// This struct contains a color represented in hex notation plus an opacity
/// value. This is necessary to represent colors in an SVG image
#[derive(Clone, Deserialize, Debug, PartialEq, Serialize)]
pub struct HexColor {
    pub color: String,
    pub alpha: f32,
}

/// > Note: none of these operations are lossy
impl Convert for HexColor {
    type Err = ColorError;

    fn to_hex(&self) -> Result<Self, Self::Err> {
        if self.alpha < 0.0 {
            Err(ColorError::OutsideBoundsNegative)
        } else if self.alpha > 1.0 {
            Err(ColorError::OutsideBoundsHigh)
        } else if self.to_reduced_rgba().is_err() {
            Err(ColorError::InvalidHex)
        } else {
            Ok(self.clone())
        }
    }

    fn to_rgba(&self) -> Result<RGBA, Self::Err> {
        if self.alpha < 0.0 {
            Err(ColorError::OutsideBoundsNegative)
        } else if self.alpha > 1.0 {
            Err(ColorError::OutsideBoundsHigh)
        } else if let Ok(color) = self.to_reduced_rgba() {
            color.to_rgba()
        } else {
            Err(ColorError::InvalidHex)
        }
    }

    #[allow(clippy::cast_sign_loss, clippy::cast_possible_truncation)]
    fn to_reduced_rgba(&self) -> Result<ReducedRGBA, Self::Err> {
        if self.alpha < 0.0 {
            Err(ColorError::OutsideBoundsNegative)
        } else if self.alpha > 1.0 {
            Err(ColorError::OutsideBoundsHigh)
        } else if let Ok(buf) = hex::decode(&self.color[1..]) {
            Ok(ReducedRGBA {
                red: buf[0],
                green: buf[1],
                blue: buf[2],
                alpha: (self.alpha * 255.0) as u8,
            })
        } else {
            Err(ColorError::InvalidHex)
        }
    }

    #[cfg(feature = "gtk")]
    fn to_gdk(&self) -> Result<gdk::RGBA, Self::Err> {
        if self.alpha < 0.0 {
            Err(ColorError::OutsideBoundsNegative)
        } else if self.alpha > 1.0 {
            Err(ColorError::OutsideBoundsHigh)
        } else if let Ok(color) = self.to_reduced_rgba() {
            color.to_gdk()
        } else {
            Err(ColorError::InvalidHex)
        }
    }
}

impl Primary for HexColor {
    fn black() -> Self {
        Self {
            color: String::from("#000000"),
            alpha: 1.0,
        }
    }

    fn white() -> Self {
        Self {
            color: String::from("#ffffff"),
            alpha: 1.0,
        }
    }

    fn red() -> Self {
        Self {
            color: String::from("#ff0000"),
            alpha: 1.0,
        }
    }

    fn green() -> Self {
        Self {
            color: String::from("#00ff00"),
            alpha: 1.0,
        }
    }

    fn blue() -> Self {
        Self {
            color: String::from("#0000ff"),
            alpha: 1.0,
        }
    }

    fn yellow() -> Self {
        Self {
            color: String::from("#ffff00"),
            alpha: 1.0,
        }
    }

    fn magenta() -> Self {
        Self {
            color: String::from("#ff00ff"),
            alpha: 1.0,
        }
    }

    fn cyan() -> Self {
        Self {
            color: String::from("#00ffff"),
            alpha: 1.0,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn black() {
        let k = HexColor::black();
        assert_eq!(k, HexColor {
            color: String::from("#000000"),
            alpha: 1.0,
        });
    }

    #[test]
    fn white() {
        let w = HexColor::white();
        assert_eq!(w, HexColor {
            color: String::from("#ffffff"),
            alpha: 1.0,
        });
    }

    #[test]
    fn red() {
        let red = HexColor::red();
        assert_eq!(red, HexColor {
            color: String::from("#ff0000"),
            alpha: 1.0,
        });
    }

    #[test]
    fn green() {
        let grn = HexColor::green();
            assert_eq!(grn, HexColor {
            color: String::from("#00ff00"),
            alpha: 1.0,
        });
    }

    #[test]
    fn blue() {
        let blue = HexColor::blue();
            assert_eq!(blue, HexColor {
            color: String::from("#0000ff"),
            alpha: 1.0,
        });
    }

    #[test]
    fn yellow() {
        let yel = HexColor::yellow();
            assert_eq!(yel, HexColor {
            color: String::from("#ffff00"),
            alpha: 1.0,
        });
    }

    #[test]
    fn magenta() {
        let mag = HexColor::magenta();
            assert_eq!(mag, HexColor {
            color: String::from("#ff00ff"),
            alpha: 1.0,
        });
    }

    #[test]
    fn cyan() {
        let c = HexColor::cyan();
            assert_eq!(c, HexColor {
            color: String::from("#00ffff"),
            alpha: 1.0,
        });
    }

    #[test]
    fn to_hex() {
        let foo = HexColor::blue();
        let bar = foo.to_hex().unwrap();
        assert_eq!(foo, bar);
    }

    #[test]
    fn to_hex_invalid() {
        let foo = HexColor {
            color: String::from("#ggffee"),
            alpha: 1.0,
        };
        let bar = foo.to_hex();
        assert_eq!(bar, Err(ColorError::InvalidHex));
    }

    #[test]
    fn to_hex_negative() {
        let foo = HexColor {
            color: String::from("#343434"),
            alpha: -0.5,
        };
        let bar = foo.to_hex();
        assert_eq!(bar, Err(ColorError::OutsideBoundsNegative));
    }

    #[test]
    fn to_hex_high() {
        let foo = HexColor {
            color: String::from("#343434"),
            alpha: 1.1,
        };
        let bar = foo.to_hex();
        assert_eq!(bar, Err(ColorError::OutsideBoundsHigh));
    }

    #[test]
    fn to_rgba() {
        let foo = HexColor::green();
        let bar = foo.to_rgba();
        assert_eq!(bar, Ok(RGBA::green()));
    }

    #[test]
    fn to_rgba_invalid() {
        let foo = HexColor {
            color: String::from("#00fpee"),
            alpha: 1.0,
        };
        let bar = foo.to_rgba();
        assert_eq!(bar, Err(ColorError::InvalidHex));
    }

    #[test]
    fn to_rgba_negative() {
        let foo = HexColor {
            color: String::from("#343434"),
            alpha: -0.5,
        };
        let bar = foo.to_rgba();
        assert_eq!(bar, Err(ColorError::OutsideBoundsNegative));
    }

    #[test]
    fn to_rgba_high() {
        let foo = HexColor {
            color: String::from("#343434"),
            alpha: 1.1,
        };
        let bar = foo.to_rgba();
        assert_eq!(bar, Err(ColorError::OutsideBoundsHigh));
    }

    #[test]
    fn to_reduced() {
        let red_hex = HexColor::red();
        let red = red_hex.to_reduced_rgba();
        assert_eq!(red, Ok(ReducedRGBA::red()));
    }

    #[test]
    fn to_reduced_invalid() {
        let foo = HexColor {
            color: String::from("#ggffee"),
            alpha: 1.0,
        };
        let bar = foo.to_reduced_rgba();
        assert_eq!(bar, Err(ColorError::InvalidHex));
    }

    #[test]
    fn to_reduced_negative() {
        let foo = HexColor {
            color: String::from("#343434"),
            alpha: -0.5,
        };
        let bar = foo.to_reduced_rgba();
        assert_eq!(bar, Err(ColorError::OutsideBoundsNegative));
    }

    #[test]
    fn to_reduced_high() {
        let foo = HexColor {
            color: String::from("#343434"),
            alpha: 1.1,
        };
        let bar = foo.to_reduced_rgba();
        assert_eq!(bar, Err(ColorError::OutsideBoundsHigh));
    }

    #[cfg(feature = "gtk")]
    #[test]
    fn to_gdk() {
        let red = HexColor::red().to_gdk();
        assert_eq!(red, Ok(gdk::RGBA {
            red: 1.0,
            green: 0.0,
            blue: 0.0,
            alpha: 1.0,
        }));
    }

    #[cfg(feature = "gtk")]
    #[test]
    fn to_gdk_invalid() {
        let foo = HexColor {
            color: String::from("#00fpee"),
            alpha: 1.0,
        };
        let bar = foo.to_gdk();
        assert_eq!(bar, Err(ColorError::InvalidHex));
    }

    #[cfg(feature = "gtk")]
    #[test]
    fn to_gdk_negative() {
        let foo = HexColor {
            color: String::from("#343434"),
            alpha: -0.5,
        };
        let bar = foo.to_gdk();
        assert_eq!(bar, Err(ColorError::OutsideBoundsNegative));
    }

    #[cfg(feature = "gtk")]
    #[test]
    fn to_gdk_high() {
        let foo = HexColor {
            color: String::from("#343434"),
            alpha: 1.1,
        };
        let bar = foo.to_gdk();
        assert_eq!(bar, Err(ColorError::OutsideBoundsHigh));
    }
}

use crate::{ColorError, Primary, PrimaryColor, Validate};
#[cfg(feature = "gdk")]
use {
    crate::ToGdk,
    gdk,
};
use serde::{Deserialize, Serialize};
use std::u8;
use std::fmt;

/// This struct contains a color represented in hex notation plus an opacity
/// value. This is necessary to represent colors in an SVG image
#[derive(Clone, Deserialize, Debug, PartialEq, Serialize)]
pub struct HexColor {
    /// A seven character string beginning with '#' representing the three
    /// colors red, green and blue in hexadecimal
    pub color: String,
    /// The opacity of the color
    pub alpha: f32,
}

impl fmt::Display for HexColor {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "color: {}, alpha: {:.3}", self.color, self.alpha)
    }
}

fn parse_hex(hex: &str) -> Result<(u8, u8, u8), ColorError> {
    validate_hex_string(hex)?;
    Ok((
        match u8::from_str_radix(&hex[1..3], 16) {
            Ok(c) => c,
            Err(_) => return Err(ColorError::InvalidHexCharacter),
        },
        match u8::from_str_radix(&hex[3..5], 16) {
            Ok(c) => c,
            Err(_) => return Err(ColorError::InvalidHexCharacter),
        },
        match u8::from_str_radix(&hex[5..7], 16) {
            Ok(c) => c,
            Err(_) => return Err(ColorError::InvalidHexCharacter),
        },
    ))
}

fn validate_hex_string(hex: &str) -> Result<(), ColorError> {
    match &hex.len() {
        x if *x < 7 => return Err(ColorError::TruncatedHexString),
        x if *x > 7 => return Err(ColorError::HexStringOverflow),
        _ => {}
    };
    if &hex[0..1] != "#" {
        return Err(ColorError::InvalidHexCharacter);
    }
    Ok(())
}

impl Validate for HexColor {
    type Err = ColorError;

    /// # Errors
    ///
    /// Will return `ColorError` if any field is less than 0 or greater
    /// than 1.0
    fn validate(&self) -> Result<(), ColorError> {
        if self.alpha < 0.0 {
            Err(ColorError::OutsideBoundsNegative)
        } else if self.alpha > 1.0 {
            Err(ColorError::OutsideBoundsHigh)
        } else {
            match parse_hex(&self.color) {
                Err(e) => Err(e),
                Ok(_) => Ok(()),
            }
        }
    }
}

#[cfg(feature = "gdk")]
impl ToGdk for HexColor {
    type Err = ColorError;

    fn to_gdk(&self) -> Result<gdk::RGBA, Self::Err> {
        self.validate()?;
        let color = parse_hex(&self.color)?;
        Ok(gdk::builders::RGBABuilder::new()
            .red(color.0 as f32 / 255.0)
            .green(color.1 as f32 / 255.0)
            .blue(color.2 as f32 / 255.0)
            .alpha(255.0)
            .build()
        )
    }
}

impl Primary for HexColor {
    fn primary(color: PrimaryColor) -> Self {
        Self {
            color: String::from(match color {
                PrimaryColor::Black => "#000000",
                PrimaryColor::White => "#ffffff",
                PrimaryColor::Red => "#ff0000",
                PrimaryColor::Green => "#00ff00",
                PrimaryColor::Blue => "#0000ff",
                PrimaryColor::Yellow => "#ffff00",
                PrimaryColor::Magenta => "#ff00ff",
                PrimaryColor::Cyan => "#00ffff",
            }),
            alpha: 1.0,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn black() {
        let k = HexColor::primary(PrimaryColor::Black);
        assert_eq!(
            k,
            HexColor {
                color: String::from("#000000"),
                alpha: 1.0,
            }
        );
    }

    #[test]
    fn white() {
        let w = HexColor::primary(PrimaryColor::White);
        assert_eq!(
            w,
            HexColor {
                color: String::from("#ffffff"),
                alpha: 1.0,
            }
        );
    }

    #[test]
    fn red() {
        let red = HexColor::primary(PrimaryColor::Red);
        assert_eq!(
            red,
            HexColor {
                color: String::from("#ff0000"),
                alpha: 1.0,
            }
        );
    }

    #[test]
    fn green() {
        let grn = HexColor::primary(PrimaryColor::Green);
        assert_eq!(
            grn,
            HexColor {
                color: String::from("#00ff00"),
                alpha: 1.0,
            }
        );
    }

    #[test]
    fn blue() {
        let blue = HexColor::primary(PrimaryColor::Blue);
        assert_eq!(
            blue,
            HexColor {
                color: String::from("#0000ff"),
                alpha: 1.0,
            }
        );
    }

    #[test]
    fn yellow() {
        let yel = HexColor::primary(PrimaryColor::Yellow);
        assert_eq!(
            yel,
            HexColor {
                color: String::from("#ffff00"),
                alpha: 1.0,
            }
        );
    }

    #[test]
    fn magenta() {
        let mag = HexColor::primary(PrimaryColor::Magenta);
        assert_eq!(
            mag,
            HexColor {
                color: String::from("#ff00ff"),
                alpha: 1.0,
            }
        );
    }

    #[test]
    fn cyan() {
        let c = HexColor::primary(PrimaryColor::Cyan);
        assert_eq!(
            c,
            HexColor {
                color: String::from("#00ffff"),
                alpha: 1.0,
            }
        );
    }

    #[test]
    fn to_string() {
        let red = HexColor::primary(PrimaryColor::Red).to_string();
        assert_eq!(red, "color: #ff0000, alpha: 1.000");
    }

    #[test]
    fn to_hex() {
        let foo = HexColor::primary(PrimaryColor::Blue);
        let bar = foo.to_hex();
        assert_eq!(bar, Ok(HexColor::primary(PrimaryColor::Blue)));
    }

    #[test]
    fn to_hex_invalid() {
        let foo = HexColor {
            color: String::from("#ggffee"),
            alpha: 1.0,
        };
        let bar = foo.to_hex();
        assert_eq!(bar, Err(ColorError::InvalidHexCharacter));
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
        let foo = HexColor::primary(PrimaryColor::Green);
        let bar = foo.to_rgba();
        assert_eq!(bar, Ok(RGBA::primary(PrimaryColor::Green)));
    }

    #[test]
    fn to_rgba_invalid() {
        let foo = HexColor {
            color: String::from("#00fpee"),
            alpha: 1.0,
        };
        let bar = foo.to_rgba();
        assert_eq!(bar, Err(ColorError::InvalidHexCharacter));
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
        let red_hex = HexColor::primary(PrimaryColor::Red);
        let red = red_hex.to_reduced_rgba();
        assert_eq!(red, Ok(ReducedRGBA::primary(PrimaryColor::Red)));
    }

    #[test]
    fn to_reduced_invalid() {
        let foo = HexColor {
            color: String::from("#ggffee"),
            alpha: 1.0,
        };
        let bar = foo.to_reduced_rgba();
        assert_eq!(bar, Err(ColorError::InvalidHexCharacter));
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

    #[cfg(feature = "gdk")]
    #[test]
    fn to_gdk() {
        let red = HexColor::primary(PrimaryColor::Red).to_gdk().unwrap();
        assert_eq!(red.red(), 1.0);
        assert_eq!(red.green(), 0.0);
        assert_eq!(red.blue(), 0.0);
        assert_eq!(red.alpha(), 1.0);
    }

    #[cfg(feature = "gdk")]
    #[test]
    fn to_gdk_invalid() {
        let foo = HexColor {
            color: String::from("#00fpee"),
            alpha: 1.0,
        };
        let bar = foo.to_gdk();
        assert_eq!(bar, Err(ColorError::InvalidHexCharacter));
    }

    #[cfg(feature = "gdk")]
    #[test]
    fn to_gdk_negative() {
        let foo = HexColor {
            color: String::from("#343434"),
            alpha: -0.5,
        };
        let bar = foo.to_gdk();
        assert_eq!(bar, Err(ColorError::OutsideBoundsNegative));
    }

    #[cfg(feature = "gdk")]
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

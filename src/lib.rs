#![warn(clippy::all, clippy::pedantic)]
#[cfg(feature = "gtk")]
use gtk::gdk;
use serde::{Deserialize, Serialize};
/// This struct contains a color represented in hex notation plus an opacity
/// value. This is necessary to represent colors in an SVG image
#[derive(Clone, Deserialize, Debug, Serialize)]
pub struct HexColor {
    pub color: String,
    pub alpha: f32,
}

/// This struct represents colors in floating point precision as separate
/// Red, Green, and Blue channels plus a separate Alpha (Opacity) channel
#[derive(Clone, Deserialize, Debug, Serialize)]
pub struct RGBA {
    pub red: f32,
    pub green: f32,
    pub blue: f32,
    pub alpha: f32,
}

/// This struct represents colors in 8-bit precision as separate
/// Red, Green, and Blue channels
#[derive(Clone, Deserialize, Debug, Serialize)]
pub struct ReducedRGBA {
    pub red: u8,
    pub green: u8,
    pub blue: u8,
    pub alpha: u8,
}

#[derive(Clone, Deserialize, Debug, Serialize)]
#[serde(tag = "ColorType")]
pub enum Color {
    Hex(HexColor),
    Reduced(ReducedRGBA),
    Rgba(RGBA),
}

impl Color {
    pub fn to_hex(&self) -> HexColor {
        match self {
            Color::Hex(c) => HexColor {
                color: c.color.clone(),
                alpha: c.alpha.clone(),
            },
            Color::Rgba(c) => c.to_hex(),
            Color::Reduced(c) => c.to_hex(),
        }
    }

    #[cfg(feature = "gtk")]
    pub fn to_gdk(&self) -> Option<gdk::RGBA> {
        match self {
            Color::Hex(c) => c.to_gdk(),
            Color::Rgba(c) => Some(c.to_gdk()),
            Color::Reduced(c) => Some(c.to_gdk()),
        }
    }
}

impl HexColor {
    pub fn to_reduced(&self) -> Option<ReducedRGBA> {
        if let Ok(buf) = hex::decode(&self.color[1..]) {
            return Some(ReducedRGBA {
                red: buf[0],
                green: buf[1],
                blue: buf[2],
                alpha: (self.alpha * 255.0) as u8,
            });
        }
        None
    }

    pub fn to_rgba(&self) -> Option<RGBA> {
        if let Some(color) = self.to_reduced() {
            Some(color.to_rgba())
        } else {
            None
        }
    }

    #[cfg(feature = "gtk")]
    pub fn to_gdk(&self) -> Option<gdk::RGBA> {
        if let Some(color) = self.to_reduced() {
            Some(color.to_gdk())
        } else {
            None
        }
    }

    pub fn black() -> HexColor {
        HexColor {
            color: String::from("#000000"),
            alpha: 1.0,
        }
    }

    pub fn white() -> HexColor {
        HexColor {
            color: String::from("#ffffff"),
            alpha: 1.0,
        }
    }

    pub fn red() -> HexColor {
        HexColor {
            color: String::from("#ff0000"),
            alpha: 1.0,
        }
    }

    pub fn green() -> HexColor {
        HexColor {
            color: String::from("#00ff00"),
            alpha: 1.0,
        }
    }

    pub fn blue() -> HexColor {
        HexColor {
            color: String::from("#0000ff"),
            alpha: 1.0,
        }
    }
}

impl ReducedRGBA {
    pub fn to_hex(&self) -> HexColor {
        HexColor {
            color: format!("#{:02x}{:02x}{:02x}", self.red, self.green, self.blue,),
            alpha: self.alpha as f32 / 255.0,
        }
    }

    pub fn to_rgba(&self) -> RGBA {
        RGBA {
            red: self.red as f32 / 255.0,
            green: self.green as f32 / 255.0,
            blue: self.blue as f32 / 255.0,
            alpha: self.alpha as f32 / 255.0,
        }
    }

    #[cfg(feature = "gtk")]
    pub fn to_gdk(&self) -> gdk::RGBA {
        gdk::RGBA {
            red: self.red as f32 / 255.0,
            green: self.green as f32 / 255.0,
            blue: self.blue as f32 / 255.0,
            alpha: self.alpha as f32 / 255.0,
        }
    }

    #[cfg(feature = "gtk")]
    pub fn from_gdk(color: &gdk::RGBA) -> ReducedRGBA {
        RGBA {
            red: color.red,
            green: color.green,
            blue: color.blue,
            alpha: color.alpha,
        }
        .to_reduced()
    }

    pub fn black() -> ReducedRGBA {
        ReducedRGBA {
            red: 0,
            green: 0,
            blue: 0,
            alpha: 255,
        }
    }

    pub fn white() -> ReducedRGBA {
        ReducedRGBA {
            red: 255,
            green: 255,
            blue: 255,
            alpha: 255,
        }
    }

    pub fn red() -> ReducedRGBA {
        ReducedRGBA {
            red: 255,
            green: 0,
            blue: 0,
            alpha: 255,
        }
    }

    pub fn green() -> ReducedRGBA {
        ReducedRGBA {
            red: 0,
            green: 255,
            blue: 0,
            alpha: 255,
        }
    }

    pub fn blue() -> ReducedRGBA {
        ReducedRGBA {
            red: 0,
            green: 0,
            blue: 255,
            alpha: 255,
        }
    }
}

impl RGBA {
    /// Converts an [RGBA] color (red, green, blue plus alpha) to a struct
    /// containing a hex color string and an opacity value, suitable for
    /// embedding into an svg image
    pub fn to_hex(&self) -> HexColor {
        HexColor {
            color: format!(
                "#{:02x}{:02x}{:02x}",
                (self.red * 255.0) as u8,
                (self.green * 255.0) as u8,
                (self.blue * 255.0) as u8,
            ),
            alpha: self.alpha,
        }
    }

    pub fn to_reduced(&self) -> ReducedRGBA {
        ReducedRGBA {
            red: (self.red * 255.0) as u8,
            green: (self.green * 255.0) as u8,
            blue: (self.blue * 255.0) as u8,
            alpha: (self.alpha * 255.0) as u8,
        }
    }

    #[cfg(feature = "gtk")]
    pub fn to_gdk(&self) -> gdk::RGBA {
        gdk::RGBA {
            red: self.red,
            green: self.green,
            blue: self.blue,
            alpha: self.alpha,
        }
    }

    #[cfg(feature = "gtk")]
    pub fn from_gdk(color: &gdk::RGBA) -> RGBA {
        RGBA {
            red: color.red,
            green: color.green,
            blue: color.blue,
            alpha: color.alpha,
        }
    }

    /// Returns "black" as an [RGBA] struct
    pub fn black() -> RGBA {
        RGBA {
            red: 0.0,
            green: 0.0,
            blue: 0.0,
            alpha: 1.0,
        }
    }

    /// Returns "white" as an [RGBA] struct
    pub fn white() -> RGBA {
        RGBA {
            red: 1.0,
            green: 1.0,
            blue: 1.0,
            alpha: 1.0,
        }
    }

    /// Returns "red" as an [RGBA] struct
    pub fn red() -> RGBA {
        RGBA {
            red: 1.0,
            green: 0.0,
            blue: 0.0,
            alpha: 1.0,
        }
    }

    /// Returns "green" as an [RGBA] struct
    pub fn green() -> RGBA {
        RGBA {
            red: 0.0,
            green: 1.0,
            blue: 0.0,
            alpha: 1.0,
        }
    }

    /// Returns "blue" as an [RGBA] struct
    pub fn blue() -> RGBA {
        RGBA {
            red: 0.0,
            green: 0.0,
            blue: 1.0,
            alpha: 1.0,
        }
    }

    /// Returns "yellow" as an [RGBA] struct
    pub fn yellow() -> RGBA {
        RGBA {
            red: 1.0,
            green: 1.0,
            blue: 0.0,
            alpha: 1.0,
        }
    }

    /// Returns "cyan" as an [RGBA] struct
    pub fn cyan() -> RGBA {
        RGBA {
            red: 0.0,
            green: 1.0,
            blue: 1.0,
            alpha: 1.0,
        }
    }

    /// Returns "magenta" as an [RGBA] struct
    pub fn magenta() -> RGBA {
        RGBA {
            red: 1.0,
            green: 0.0,
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
        assert_eq!(RGBA::black().to_hex().color, HexColor::black().color);
        assert_eq!(ReducedRGBA::black().to_hex().color, HexColor::black().color);
    }

    #[test]
    fn white() {
        assert_eq!(RGBA::white().to_hex().color, HexColor::white().color);
        assert_eq!(ReducedRGBA::white().to_hex().color, HexColor::white().color);
    }

    #[test]
    fn red() {
        assert_eq!(RGBA::red().to_hex().color, HexColor::red().color);
        assert_eq!(ReducedRGBA::red().to_hex().color, HexColor::red().color);
    }

    #[test]
    fn green() {
        assert_eq!(RGBA::green().to_hex().color, HexColor::green().color);
        assert_eq!(ReducedRGBA::green().to_hex().color, HexColor::green().color);
    }

    #[test]
    fn blue() {
        assert_eq!(RGBA::blue().to_hex().color, HexColor::blue().color);
        assert_eq!(ReducedRGBA::blue().to_hex().color, HexColor::blue().color);
    }

    #[test]
    fn from_hex() {
        let red_hex = HexColor {
            color: String::from("#ff0000"),
            alpha: 1.0,
        };
        let red = red_hex.to_reduced().unwrap();
        assert_eq!(red.red, 255);
        assert_eq!(red.green, 0);
        assert_eq!(red.blue, 0);
        assert_eq!(red.alpha, 255);
    }

    #[cfg(feature = "gtk")]
    #[test]
    fn rgba_to_gdk() {
        let red = RGBA::red();
        let gdk_red = red.to_gdk();
        assert_eq!(red.red, gdk_red.red);
        assert_eq!(red.green, gdk_red.green);
        assert_eq!(red.blue, gdk_red.blue);
        assert_eq!(red.alpha, gdk_red.alpha);
    }

    #[cfg(feature = "gtk")]
    #[test]
    fn rgba_from_gdk() {
        let gdk_red = gdk::RGBA {
            red: 1.0,
            green: 0.0,
            blue: 0.0,
            alpha: 1.0,
        };
        let red = RGBA::from_gdk(&gdk_red);
        assert_eq!(red.red, gdk_red.red);
        assert_eq!(red.green, gdk_red.green);
        assert_eq!(red.blue, gdk_red.blue);
        assert_eq!(red.alpha, gdk_red.alpha);
    }
}

#[cfg(feature = "gtk")]
use gtk::gdk;
use serde::{Deserialize, Serialize};
use crate::{ColorError, Convert, Primary, ReducedRGBA, RGBA};

/// This struct contains a color represented in hex notation plus an opacity
/// value. This is necessary to represent colors in an SVG image
#[derive(Clone, Deserialize, Debug, Serialize)]
pub struct HexColor {
    pub color: String,
    pub alpha: f32,
}

impl Convert for HexColor {
    type Err = ColorError;

    fn to_hex(&self) -> Result<Self, Self::Err> {
        if let Ok(_) = self.to_reduced_rgba() {
            Ok(self.clone())
        } else {
            Err(ColorError::InvalidHex)
        }
    }

    fn to_rgba(&self) -> Result<RGBA, Self::Err> {
        if let Ok(color) = self.to_reduced_rgba() {
            color.to_rgba()
        } else {
            Err(ColorError::InvalidHex)
        }
    }

    fn to_reduced_rgba(&self) -> Result<ReducedRGBA, Self::Err> {
        if let Ok(buf) = hex::decode(&self.color[1..]) {
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
        if let Ok(color) = self.to_reduced_rgba() {
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

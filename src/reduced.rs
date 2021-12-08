#[cfg(feature = "gtk")]
use gtk::gdk;
use serde::{Deserialize, Serialize};
use crate::{ColorError, Convert, HexColor, Primary, RGBA};

/// This struct represents colors in 8-bit precision as separate
/// Red, Green, and Blue channels
#[derive(Clone, Deserialize, Debug, Serialize)]
pub struct ReducedRGBA {
    pub red: u8,
    pub green: u8,
    pub blue: u8,
    pub alpha: u8,
}

impl Convert for ReducedRGBA {
    type Err = ColorError;

    fn to_hex(&self) -> Result<HexColor, Self::Err> {
        Ok(HexColor {
            color: format!("#{:02x}{:02x}{:02x}", self.red, self.green, self.blue,),
            alpha: self.alpha as f32 / 255.0,
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

    fn to_reduced_rgba(&self) -> Result<ReducedRGBA, Self::Err> {
        Ok(self.clone())
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

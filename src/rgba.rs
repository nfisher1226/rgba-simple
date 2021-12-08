#[cfg(feature = "gtk")]
use gtk::gdk;
use serde::{Deserialize, Serialize};
use crate::{ColorError, Convert, HexColor, Primary, ReducedRGBA};

/// This struct represents colors in floating point precision as separate
/// Red, Green, and Blue channels plus a separate Alpha (Opacity) channel
#[derive(Clone, Deserialize, Debug, Serialize)]
pub struct RGBA {
    pub red: f32,
    pub green: f32,
    pub blue: f32,
    pub alpha: f32,
}

impl Convert for RGBA {
    type Err = ColorError;

    /// Converts an [RGBA] color (red, green, blue plus alpha) to a struct
    /// containing a hex color string and an opacity value, suitable for
    /// embedding into an svg image
    /// # Errors
    ///
    /// Will return `ColorError` if any field is less than 0 or greater
    /// than 1.0
    #[allow(clippy::cast_sign_loss, clippy::cast_possible_truncation)]
    fn to_hex(&self) -> Result<HexColor, Self::Err> {
        if self.red < 0.0 || self.green < 0.0 || self.blue < 0.0 {
            Err(ColorError::OutsideBoundsNegative)
        } else if self.red > 1.0 || self.green > 1.0 || self.blue> 1.0 {
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
    fn to_rgba(&self) -> Result<Self, Self::Err> {
        if self.red < 0.0 || self.green < 0.0 || self.blue < 0.0 {
            Err(ColorError::OutsideBoundsNegative)
        } else if self.red > 1.0 || self.green > 1.0 || self.blue > 1.0 {
            Err(ColorError::OutsideBoundsHigh)
        } else {
            Ok(self.clone())
        }
    }

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
    #[cfg(feature = "gtk")]
    fn to_gdk(&self) -> Result<gdk::RGBA, Self::Err> {
        if self.red < 0.0 || self.green < 0.0 || self.blue < 0.0 {
            Err(ColorError::OutsideBoundsNegative)
        } else if self.red > 1.0 || self.green > 1.0 || self.blue > 1.0 {
            Err(ColorError::OutsideBoundsHigh)
        } else {
            Ok(gdk::RGBA {
                red: self.red,
                green: self.green,
                blue: self.blue,
                alpha: self.alpha,
            })
        }
    }
}

impl Primary for RGBA {
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

    /// Returns "red" as an [RGBA] struct
    fn red() -> Self {
        Self {
            red: 1.0,
            green: 0.0,
            blue: 0.0,
            alpha: 1.0,
        }
    }

    /// Returns "green" as an [RGBA] struct
    fn green() -> Self {
        Self {
            red: 0.0,
            green: 1.0,
            blue: 0.0,
            alpha: 1.0,
        }
    }

    /// Returns "blue" as an [RGBA] struct
    fn blue() -> Self {
        Self {
            red: 0.0,
            green: 0.0,
            blue: 1.0,
            alpha: 1.0,
        }
    }

    /// Returns "yellow" as an [RGBA] struct
    fn yellow() -> Self {
        Self {
            red: 1.0,
            green: 1.0,
            blue: 0.0,
            alpha: 1.0,
        }
    }

    /// Returns "magenta" as an [RGBA] struct
    fn magenta() -> Self {
        Self {
            red: 1.0,
            green: 0.0,
            blue: 1.0,
            alpha: 1.0,
        }
    }

    /// Returns "cyan" as an [RGBA] struct
    fn cyan() -> Self {
        Self {
            red: 0.0,
            green: 1.0,
            blue: 1.0,
            alpha: 1.0,
        }
    }
}

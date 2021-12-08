use gtk::gdk;
use crate::{ColorError, Convert, HexColor, Primary, ReducedRGBA, RGBA};

impl Convert for gdk::RGBA {
    type Err = ColorError;

    /// # Errors
    ///
    /// Will return `ColorError` if any field is less than 0 or greater
    /// than 1.0
    fn to_hex(&self) -> Result<HexColor, Self::Err> {
        if self.red < 0.0 || self.green < 0.0 || self.blue < 0.0 {
            Err(ColorError::OutsideBoundsNegative)
        } else if self.red > 1.0 || self.green > 1.0 || self.blue > 1.0 {
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
    fn to_rgba(&self) -> Result<RGBA, Self::Err> {
        if self.red < 0.0 || self.green < 0.0 || self.blue < 0.0 {
            Err(ColorError::OutsideBoundsNegative)
        } else if self.red > 1.0 || self.green > 1.0 || self.blue > 1.0 {
            Err(ColorError::OutsideBoundsHigh)
        } else {
            Ok(RGBA {
                red: self.red.clone(),
                green: self.green.clone(),
                blue: self.blue.clone(),
                alpha: self.alpha.clone(),
            })
        }
    }

    /// # Errors
    ///
    /// Will return `ColorError` if any field is less than 0 or greater
    /// than 1.0
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
    fn to_gdk(&self) -> Result<Self, Self::Err> {
        if self.red < 0.0 || self.green < 0.0 || self.blue < 0.0 {
            Err(ColorError::OutsideBoundsNegative)
        } else if self.red > 1.0 || self.green > 1.0 || self.blue > 1.0 {
            Err(ColorError::OutsideBoundsHigh)
        } else {
            Ok(self.clone())
        }
    }
}

impl Primary for gdk::RGBA {
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

    fn red() -> Self {
        Self {
            red: 1.0,
            green: 0.0,
            blue: 0.0,
            alpha: 1.0,
        }
    }

    fn green() -> Self {
        Self {
            red: 0.0,
            green: 1.0,
            blue: 0.0,
            alpha: 1.0,
        }
    }

    fn blue() -> Self {
        Self {
            red: 0.0,
            green: 0.0,
            blue: 1.0,
            alpha: 1.0,
        }
    }

    fn yellow() -> Self {
        Self {
            red: 1.0,
            green: 1.0,
            blue: 0.0,
            alpha: 1.0,
        }
    }

    fn magenta() -> Self {
        Self {
            red: 1.0,
            green: 0.0,
            blue: 1.0,
            alpha: 1.0,
        }
    }

    fn cyan() -> Self {
        Self {
            red: 0.0,
            green: 1.0,
            blue: 1.0,
            alpha: 1.0,
        }
    }
}

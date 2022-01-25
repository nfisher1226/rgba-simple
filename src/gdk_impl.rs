use crate::{ColorError, Convert, HexColor, Primary, PrimaryColor, ReducedRGBA, Validate, RGBA};
use gdk;

impl Validate for gdk::RGBA {
    type Err = ColorError;

    /// # Errors
    ///
    /// Will return `ColorError` if any field is less than 0 or greater
    /// than 1.0
    fn validate(&self) -> Result<(), ColorError> {
        if self.red() < 0.0 || self.green() < 0.0 || self.blue() < 0.0 {
            Err(ColorError::OutsideBoundsNegative)
        } else if self.red() > 1.0 || self.green() > 1.0 || self.blue() > 1.0 {
            Err(ColorError::OutsideBoundsHigh)
        } else {
            Ok(())
        }
    }
}

/// Note: some of these operations are lossy
impl Convert for gdk::RGBA {
    type Err = ColorError;

    /// > Note: this operation is lossy
    /// # Errors
    ///
    /// Will return `ColorError` if any field is less than 0 or greater
    /// than 1.0
    #[allow(clippy::cast_sign_loss, clippy::cast_possible_truncation)]
    fn to_hex(&self) -> Result<HexColor, Self::Err> {
        self.validate()?;
        Ok(HexColor {
            color: format!(
                "#{:02x}{:02x}{:02x}",
                (self.red() * 255.0) as u8,
                (self.green() * 255.0) as u8,
                (self.blue() * 255.0) as u8,
            ),
            alpha: self.alpha(),
        })
    }

    /// # Errors
    ///
    /// Will return `ColorError` if any field is less than 0 or greater
    /// than 1.0
    fn to_rgba(&self) -> Result<RGBA, Self::Err> {
        self.validate()?;
        Ok(RGBA {
            red: self.red(),
            green: self.green(),
            blue: self.blue(),
            alpha: self.alpha(),
        })
    }

    /// > Note: this operation is lossy
    /// # Errors
    ///
    /// Will return `ColorError` if any field is less than 0 or greater
    /// than 1.0
    #[allow(clippy::cast_sign_loss, clippy::cast_possible_truncation)]
    fn to_reduced_rgba(&self) -> Result<ReducedRGBA, Self::Err> {
        self.validate()?;
        Ok(ReducedRGBA {
            red: (self.red() * 255.0) as u8,
            green: (self.green() * 255.0) as u8,
            blue: (self.blue() * 255.0) as u8,
            alpha: (self.alpha() * 255.0) as u8,
        })
    }

    /// # Errors
    ///
    /// Will return `ColorError` if any field is less than 0 or greater
    /// than 1.0
    fn to_gdk(&self) -> Result<Self, Self::Err> {
        self.validate()?;
        Ok(*self)
    }
}

impl Primary for gdk::RGBA {
    fn primary(color: PrimaryColor) -> Self {
        gdk::builders::RGBABuilder::new()
            .red(match color {
                PrimaryColor::Black
                | PrimaryColor::Green
                | PrimaryColor::Blue
                | PrimaryColor::Cyan => 0.0,
                _ => 1.0,
            })
            .green(match color {
                PrimaryColor::Black
                | PrimaryColor::Red
                | PrimaryColor::Blue
                | PrimaryColor::Magenta => 0.0,
                _ => 1.0,
            })
            .blue(match color {
                PrimaryColor::Black
                | PrimaryColor::Red
                | PrimaryColor::Green
                | PrimaryColor::Yellow => 0.0,
                _ => 1.0,
            })
            .alpha(1.0)
            .build()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn black() {
        let k = gdk::RGBA::primary(PrimaryColor::Black);
        assert_eq!(k.red(), 0.0);
        assert_eq!(k.green(), 0.0);
        assert_eq!(k.blue(), 0.0);
        assert_eq!(k.alpha(), 1.0);
    }

    #[test]
    fn white() {
        let w = gdk::RGBA::primary(PrimaryColor::White);
        assert_eq!(w.red(), 1.0);
        assert_eq!(w.green(), 1.0);
        assert_eq!(w.blue(), 1.0);
        assert_eq!(w.alpha(), 1.0);
    }

    #[test]
    fn red() {
        let r = gdk::RGBA::primary(PrimaryColor::Red);
        assert_eq!(r.red(), 1.0);
        assert_eq!(r.green(), 0.0);
        assert_eq!(r.blue(), 0.0);
        assert_eq!(r.alpha(), 1.0);
    }

    #[test]
    fn green() {
        let grn = gdk::RGBA::primary(PrimaryColor::Green);
        assert_eq!(grn.red(), 0.0);
        assert_eq!(grn.green(), 1.0);
        assert_eq!(grn.blue(), 0.0);
        assert_eq!(grn.alpha(), 1.0);
    }

    #[test]
    fn blue() {
        let blue = gdk::RGBA::primary(PrimaryColor::Blue);
        assert_eq!(blue.red(), 0.0);
        assert_eq!(blue.green(), 0.0);
        assert_eq!(blue.blue(), 1.0);
        assert_eq!(blue.alpha(), 1.0);
    }

    #[test]
    fn yellow() {
        let yel = gdk::RGBA::primary(PrimaryColor::Yellow);
        assert_eq!(yel.red(), 1.0);
        assert_eq!(yel.green(), 1.0);
        assert_eq!(yel.blue(), 0.0);
        assert_eq!(yel.alpha(), 1.0);
    }

    #[test]
    fn magenta() {
        let mag = gdk::RGBA::primary(PrimaryColor::Magenta);
        assert_eq!(mag.red(), 1.0);
        assert_eq!(mag.green(), 0.0);
        assert_eq!(mag.blue(), 1.0);
        assert_eq!(mag.alpha(), 1.0);
    }

    #[test]
    fn cyan() {
        let c = gdk::RGBA::primary(PrimaryColor::Cyan);
        assert_eq!(c.red(), 0.0);
        assert_eq!(c.green(), 1.0);
        assert_eq!(c.blue(), 1.0);
        assert_eq!(c.alpha(), 1.0);
    }

    #[test]
    fn to_hex() {
        let red = gdk::RGBA::primary(PrimaryColor::Red);
        let red_hex = red.to_hex().unwrap();
        assert_eq!(red_hex.color, String::from("#ff0000"));
        assert_eq!(red_hex.alpha, 1.0);
    }

    #[test]
    fn to_hex_negative() {
        let invalid = gdk::builders::RGBABuilder::new()
            .red(1.0)
            .green(1.0)
            .blue(-1.0)
            .alpha(1.0)
            .build();
        assert_eq!(invalid.to_hex(), Err(ColorError::OutsideBoundsNegative));
    }

    #[test]
    fn to_hex_high() {
        let invalid = gdk::builders::RGBABuilder::new()
            .red(1.1)
            .green(1.0)
            .blue(1.0)
            .alpha(1.0)
            .build();
        assert_eq!(invalid.to_hex(), Err(ColorError::OutsideBoundsHigh));
    }

    #[test]
    fn to_rgba() {
        let red = gdk::RGBA::primary(PrimaryColor::Red);
        let rgba = red.to_rgba();
        assert_eq!(rgba, Ok(RGBA::primary(PrimaryColor::Red)));
    }

    #[test]
    fn to_rgba_negative() {
        let invalid = gdk::builders::RGBABuilder::new()
            .red(1.0)
            .green(1.0)
            .blue(-1.0)
            .alpha(1.0)
            .build();
        assert_eq!(invalid.to_rgba(), Err(ColorError::OutsideBoundsNegative));
    }

    #[test]
    fn to_rgba_high() {
        let invalid = gdk::builders::RGBABuilder::new()
            .red(1.1)
            .green(1.0)
            .blue(1.0)
            .alpha(1.0)
            .build();
        assert_eq!(invalid.to_rgba(), Err(ColorError::OutsideBoundsHigh));
    }

    #[test]
    fn to_reduced_rgba() {
        let red = gdk::RGBA::primary(PrimaryColor::Red).to_reduced_rgba();
        assert_eq!(red, Ok(ReducedRGBA::primary(PrimaryColor::Red)));
    }

    #[test]
    fn to_reduced_negative() {
        let invalid = gdk::builders::RGBABuilder::new()
            .red(1.0)
            .green(1.0)
            .blue(-1.0)
            .alpha(1.0)
            .build();
        assert_eq!(
            invalid.to_reduced_rgba(),
            Err(ColorError::OutsideBoundsNegative)
        );
    }

    #[test]
    fn to_reduced_high() {
        let invalid = gdk::builders::RGBABuilder::new()
            .red(1.1)
            .green(1.0)
            .blue(1.0)
            .alpha(1.0)
            .build();
        assert_eq!(
            invalid.to_reduced_rgba(),
            Err(ColorError::OutsideBoundsHigh)
        );
    }

    #[test]
    fn rgba_to_gdk() {
        let red = gdk::RGBA::primary(PrimaryColor::Red);
        let gdk_red = red.to_gdk().unwrap();
        assert_eq!(red, gdk_red);
    }

    #[test]
    fn to_gdk_negative() {
        let invalid = gdk::builders::RGBABuilder::new()
            .red(1.0)
            .green(1.0)
            .blue(-1.0)
            .alpha(1.0)
            .build();
        assert_eq!(invalid.to_gdk(), Err(ColorError::OutsideBoundsNegative));
    }

    #[test]
    fn to_gdk_high() {
        let invalid = gdk::builders::RGBABuilder::new()
            .red(1.1)
            .green(1.0)
            .blue(1.0)
            .alpha(1.0)
            .build();
        assert_eq!(invalid.to_gdk(), Err(ColorError::OutsideBoundsHigh));
    }
}

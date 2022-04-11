use crate::{ColorChannel, RGB, RGBA};

/// Convert to the `gdk::RGBA` color representation
pub trait ToGdk {
    fn to_gdk(&self) -> gdk::RGBA;
}

/// Convert from the `gdk::RGBA` color representation
pub trait FromGdk {
    fn from_gdk(_: gdk::RGBA) -> Self;
}

impl<T> ToGdk for RGB<T>
where
    T: ColorChannel,
{
    fn to_gdk(&self) -> gdk::RGBA {
        gdk::builders::RGBABuilder::new()
            .red(self.red.to_percent())
            .green(self.green.to_percent())
            .blue(self.blue.to_percent())
            .alpha(1.0)
            .build()
    }
}

impl<T> ToGdk for RGBA<T>
where
    T: ColorChannel,
{
    fn to_gdk(&self) -> gdk::RGBA {
        gdk::builders::RGBABuilder::new()
            .red(self.red.to_percent())
            .green(self.green.to_percent())
            .blue(self.blue.to_percent())
            .alpha(self.alpha.to_percent())
            .build()
    }
}

impl<T> FromGdk for RGB<T>
where
    T: ColorChannel,
{
    fn from_gdk(color: gdk::RGBA) -> Self {
        Self {
            red: T::from_percent(color.red()),
            green: T::from_percent(color.green()),
            blue: T::from_percent(color.blue()),
        }
    }
}

impl<T> FromGdk for RGBA<T>
where
    T: ColorChannel,
{
    fn from_gdk(color: gdk::RGBA) -> Self {
        Self {
            red: T::from_percent(color.red()),
            green: T::from_percent(color.green()),
            blue: T::from_percent(color.blue()),
            alpha: T::from_percent(color.alpha()),
        }
    }
}

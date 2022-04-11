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
            .red(self.red.to_gdk())
            .green(self.green.to_gdk())
            .blue(self.blue.to_gdk())
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
            .red(self.red.to_gdk())
            .green(self.green.to_gdk())
            .blue(self.blue.to_gdk())
            .alpha(self.alpha.to_gdk())
            .build()
    }
}

impl FromGdk for RGB<u8> {
    fn from_gdk(color: gdk::RGBA) -> Self {
        Self {
            red: (color.red() * 255.0) as u8,
            green: (color.green() * 255.0) as u8, 
            blue: (color.blue() * 255.0) as u8,
        }
    }
}

impl FromGdk for RGB<u16> {
    fn from_gdk(color: gdk::RGBA) -> Self {
        Self {
            red: (color.red() * 255.0) as u16,
            green: (color.green() * 255.0) as u16, 
            blue: (color.blue() * 255.0) as u16,
        }
    }
}

impl FromGdk for RGB<u32> {
    fn from_gdk(color: gdk::RGBA) -> Self {
        Self {
            red: (color.red() * 255.0) as u32,
            green: (color.green() * 255.0) as u32, 
            blue: (color.blue() * 255.0) as u32,
        }
    }
}

impl FromGdk for RGB<u64> {
    fn from_gdk(color: gdk::RGBA) -> Self {
        Self {
            red: (color.red() * 255.0) as u64,
            green: (color.green() * 255.0) as u64, 
            blue: (color.blue() * 255.0) as u64,
        }
    }
}

impl FromGdk for RGB<i16> {
    fn from_gdk(color: gdk::RGBA) -> Self {
        Self {
            red: (color.red() * 255.0) as i16,
            green: (color.green() * 255.0) as i16, 
            blue: (color.blue() * 255.0) as i16,
        }
    }
}

impl FromGdk for RGB<i32> {
    fn from_gdk(color: gdk::RGBA) -> Self {
        Self {
            red: (color.red() * 255.0) as i32,
            green: (color.green() * 255.0) as i32, 
            blue: (color.blue() * 255.0) as i32,
        }
    }
}

impl FromGdk for RGB<i64> {
    fn from_gdk(color: gdk::RGBA) -> Self {
        Self {
            red: (color.red() * 255.0) as i64,
            green: (color.green() * 255.0) as i64, 
            blue: (color.blue() * 255.0) as i64,
        }
    }
}

impl FromGdk for RGB<f32> {
    fn from_gdk(color: gdk::RGBA) -> Self {
        Self {
            red: color.red(),
            green: color.green(), 
            blue: color.blue(),
        }
    }
}

impl FromGdk for RGB<f64> {
    fn from_gdk(color: gdk::RGBA) -> Self {
        Self {
            red: color.red() as f64,
            green: color.green() as f64, 
            blue: color.blue() as f64,
        }
    }
}

impl FromGdk for RGBA<u8> {
    fn from_gdk(color: gdk::RGBA) -> Self {
        Self {
            red: (color.red() * 255.0) as u8,
            green: (color.green() * 255.0) as u8,
            blue: (color.blue() * 255.0) as u8,
            alpha: (color.alpha() * 255.0) as u8,
        }
    }
}

impl FromGdk for RGBA<u16> {
    fn from_gdk(color: gdk::RGBA) -> Self {
        Self {
            red: (color.red() * 255.0) as u16,
            green: (color.green() * 255.0) as u16, 
            blue: (color.blue() * 255.0) as u16,
            alpha: (color.blue() * 255.0) as u16,
        }
    }
}

impl FromGdk for RGBA<u32> {
    fn from_gdk(color: gdk::RGBA) -> Self {
        Self {
            red: (color.red() * 255.0) as u32,
            green: (color.green() * 255.0) as u32, 
            blue: (color.blue() * 255.0) as u32,
            alpha: (color.blue() * 255.0) as u32,
        }
    }
}

impl FromGdk for RGBA<u64> {
    fn from_gdk(color: gdk::RGBA) -> Self {
        Self {
            red: (color.red() * 255.0) as u64,
            green: (color.green() * 255.0) as u64, 
            blue: (color.blue() * 255.0) as u64,
            alpha: (color.blue() * 255.0) as u64,
        }
    }
}

impl FromGdk for RGBA<i16> {
    fn from_gdk(color: gdk::RGBA) -> Self {
        Self {
            red: (color.red() * 255.0) as i16,
            green: (color.green() * 255.0) as i16, 
            blue: (color.blue() * 255.0) as i16,
            alpha: (color.blue() * 255.0) as i16,
        }
    }
}

impl FromGdk for RGBA<i32> {
    fn from_gdk(color: gdk::RGBA) -> Self {
        Self {
            red: (color.red() * 255.0) as i32,
            green: (color.green() * 255.0) as i32, 
            blue: (color.blue() * 255.0) as i32,
            alpha: (color.blue() * 255.0) as i32,
        }
    }
}

impl FromGdk for RGBA<i64> {
    fn from_gdk(color: gdk::RGBA) -> Self {
        Self {
            red: (color.red() * 255.0) as i64,
            green: (color.green() * 255.0) as i64, 
            blue: (color.blue() * 255.0) as i64,
            alpha: (color.blue() * 255.0) as i64,
        }
    }
}

impl FromGdk for RGBA<f32> {
    fn from_gdk(color: gdk::RGBA) -> Self {
        Self {
            red: color.red(),
            green: color.green(), 
            blue: color.blue(),
            alpha: color.alpha(),
        }
    }
}

impl FromGdk for RGBA<f64> {
    fn from_gdk(color: gdk::RGBA) -> Self {
        Self {
            red: color.red() as f64,
            green: color.green() as f64, 
            blue: color.blue() as f64,
            alpha: color.alpha() as f64,
        }
    }
}

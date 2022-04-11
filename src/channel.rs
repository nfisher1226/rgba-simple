/// Required trait for a type to be able to represent a color channel
pub trait ColorChannel: Copy + PartialOrd {
    const MAX: Self;
    const MIN: Self;

    fn channel_display(&self) -> String;
    fn from_percent(_: f32) -> Self;
    fn to_percent(&self) -> f32;
}

impl ColorChannel for u8 {
    const MAX: Self = 255;
    const MIN: Self = 0;

    fn channel_display(&self) -> String {
        format!("{}", self)
    }

    fn from_percent(percent: f32) -> Self {
        (percent * 255.0).round() as u8
    }

    fn to_percent(&self) -> f32 {
        *self as f32 / 255.0
    }
}

impl ColorChannel for u16 {
    const MAX: Self = 255;
    const MIN: Self = 0;

    fn channel_display(&self) -> String {
        format!("{}", self)
    }

    fn from_percent(percent: f32) -> Self {
        (percent * 255.0).round() as u16
    }

    fn to_percent(&self) -> f32 {
        *self as f32 / 255.0
    }
}

impl ColorChannel for u32 {
    const MAX: Self = 255;
    const MIN: Self = 0;

    fn channel_display(&self) -> String {
        format!("{}", self)
    }

    fn from_percent(percent: f32) -> Self {
        (percent * 255.0).round() as u32
    }

    fn to_percent(&self) -> f32 {
        *self as f32 / 255.0
    }
}

impl ColorChannel for u64 {
    const MAX: Self = 255;
    const MIN: Self = 0;

    fn channel_display(&self) -> String {
        format!("{}", self)
    }

    fn from_percent(percent: f32) -> Self {
        (percent * 255.0).round() as u64
    }

    fn to_percent(&self) -> f32 {
        *self as f32 / 255.0
    }
}

impl ColorChannel for i16 {
    const MAX: Self = 255;
    const MIN: Self = 0;

    fn channel_display(&self) -> String {
        format!("{}", self)
    }

    fn from_percent(percent: f32) -> Self {
        (percent * 255.0).round() as i16
    }

    fn to_percent(&self) -> f32 {
        *self as f32 / 255.0
    }
}

impl ColorChannel for i32 {
    const MAX: Self = 255;
    const MIN: Self = 0;

    fn channel_display(&self) -> String {
        format!("{}", self)
    }

    fn from_percent(percent: f32) -> Self {
        (percent * 255.0).round() as i32
    }

    fn to_percent(&self) -> f32 {
        *self as f32 / 255.0
    }
}

impl ColorChannel for i64 {
    const MAX: Self = 255;
    const MIN: Self = 0;

    fn channel_display(&self) -> String {
        format!("{}", self)
    }

    fn from_percent(percent: f32) -> Self {
        (percent * 255.0).round() as i64
    }

    fn to_percent(&self) -> f32 {
        *self as f32 / 255.0
    }
}

impl ColorChannel for f32 {
    const MAX: Self = 1.0;
    const MIN: Self = 0.0;

    fn channel_display(&self) -> String {
        format!("{:.3}", self)
    }

    fn from_percent(percent: f32) -> Self {
        percent
    }

    fn to_percent(&self) -> f32 {
        *self
    }
}

impl ColorChannel for f64 {
    const MAX: Self = 1.0;
    const MIN: Self = 0.0;

    fn channel_display(&self) -> String {
        format!("{:.3}", self)
    }

    fn from_percent(percent: f32) -> Self {
        percent as f64
    }

    fn to_percent(&self) -> f32 {
        *self as f32
    }
}

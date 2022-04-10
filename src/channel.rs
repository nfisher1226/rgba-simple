pub trait ColorChannel {
    const MAX: Self;
    const MIN: Self;

    fn channel_display(&self) -> String;
}

impl ColorChannel for u8 {
    const MAX: Self = 255;
    const MIN: Self = 0;

    fn channel_display(&self) -> String {
        format!("{}", self)
    }
}

impl ColorChannel for u16 {
    const MAX: Self = 255;
    const MIN: Self = 0;

    fn channel_display(&self) -> String {
        format!("{}", self)
    }
}

impl ColorChannel for u32 {
    const MAX: Self = 255;
    const MIN: Self = 0;

    fn channel_display(&self) -> String {
        format!("{}", self)
    }
}

impl ColorChannel for u64 {
    const MAX: Self = 255;
    const MIN: Self = 0;

    fn channel_display(&self) -> String {
        format!("{}", self)
    }
}

impl ColorChannel for i16 {
    const MAX: Self = 255;
    const MIN: Self = 0;

    fn channel_display(&self) -> String {
        format!("{}", self)
    }
}

impl ColorChannel for i32 {
    const MAX: Self = 255;
    const MIN: Self = 0;

    fn channel_display(&self) -> String {
        format!("{}", self)
    }
}

impl ColorChannel for i64 {
    const MAX: Self = 255;
    const MIN: Self = 0;

    fn channel_display(&self) -> String {
        format!("{}", self)
    }
}

impl ColorChannel for f32 {
    const MAX: Self = 1.0;
    const MIN: Self = 0.0;

    fn channel_display(&self) -> String {
        format!("{:.3}", self)
    }
}

impl ColorChannel for f64 {
    const MAX: Self = 1.0;
    const MIN: Self = 0.0;

    fn channel_display(&self) -> String {
        format!("{:.3}", self)
    }
}

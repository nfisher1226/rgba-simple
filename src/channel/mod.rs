pub trait ColorChannel {
    const MAX: Self;
    const MIN: Self;
}

impl ColorChannel for u8 {
    const MAX: Self = 255;
    const MIN: Self = 0;
}

impl ColorChannel for u16 {
    const MAX: Self = 255;
    const MIN: Self = 0;
}

impl ColorChannel for u32 {
    const MAX: Self = 255;
    const MIN: Self = 0;
}

impl ColorChannel for u64 {
    const MAX: Self = 255;
    const MIN: Self = 0;
}

impl ColorChannel for i16 {
    const MAX: Self = 255;
    const MIN: Self = 0;
}

impl ColorChannel for i32 {
    const MAX: Self = 255;
    const MIN: Self = 0;
}

impl ColorChannel for i64 {
    const MAX: Self = 255;
    const MIN: Self = 0;
}

impl ColorChannel for f32 {
    const MAX: Self = 1.0;
    const MIN: Self = 0.0;
}

impl ColorChannel for f64 {
    const MAX: Self = 1.0;
    const MIN: Self = 0.0;
}

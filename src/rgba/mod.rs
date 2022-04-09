use {
    crate::{ColorChannel, Primary, PrimaryColor, PrimaryColor::*},
    serde::{Deserialize, Serialize},
};

#[derive(Clone, Copy, Deserialize, Debug, PartialEq, Serialize)]
pub struct RGBA<T>
where T: ColorChannel {
    red: T,
    green: T,
    blue: T,
    alpha: T,
}

impl<T> Primary for RGBA<T>
where T: ColorChannel {
    fn primary(color: PrimaryColor) -> Self {
        Self {
            red: match color {
                Black | Green | Blue | Cyan => ColorChannel::MIN,
                _ => ColorChannel::MAX,
            },
            green: match color {
                Black | Red | Blue | Magenta => ColorChannel::MIN,
                _ => ColorChannel::MAX,
            },
            blue: match color {
                Black | Red | Green | Yellow => ColorChannel::MIN,
                _ => ColorChannel::MAX,
            },
            alpha: ColorChannel::MAX,
        }
    }
}

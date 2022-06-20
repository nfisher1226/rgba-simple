`Rgba_simple` is a small library for storing colors in RGBA and Hex notation.
It includes functions to convert to and from Hex and RGBA. All of the internal
formats can be serialized and deserialized with `serde`. If compiled with the
`gdk` feature, all of it's internal representations can also be converted to
and from `gdk::RGBA`, making one use case storing colors generated from
a Gtk+ gui in a config file, using one of the many formats with `serde`
support.

Use this library if your color needs are simple and you don't require
addressing colors in otherr color spaces, such as CMYK or HSL.

# Example
```Rust
use rgba_simple::*;

let red_hex = String::from("#ff0000");
let red: RGB::<u8> = PrimaryColor::Red.into();
assert_eq!(RGB::<u8>::from_hex(&red_hex), red);
```

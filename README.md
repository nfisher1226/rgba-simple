# Simple RGBA
<!-- cargo-sync-readme start -->

`Rgba_simple` is a small library for storing colors in RGBA and Hex notation.
It includes functions to convert to and from Hex and RGBA. All of the internal
formats can be serialized and deserialized with `serde`. If compiled with the
`gtk` feature, all of it's internal representations can also be converted to
and from `gtk::gdk::RGBA`, making one use case storing colors generated from
a Gtk+ gui in a config file, using one of the many formats with `serde`
support.

Use this library if your color needs are simple and you don't require
addressing colors in otherr color spaces, such as CMYK or HSL.

## Examples
```Rust
use rgba_simple::{ReducedRGBA, HexColor, Primary, Convert}

let red = ReducedRGBA::red();
let red_hex = HexColor::red();
assert_eq!(red.to_hex().unwrap(), red_hex);
```

<!-- cargo-sync-readme end -->

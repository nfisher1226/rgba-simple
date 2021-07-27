# Simple RGBA
Simple RGBA represents colors in RGBA color space, using either u8 or f64 types,
can convert those color to and from each other and to and from hexadecimal
strings, and can optionally do conversions to and from `gdk::RGBA` colors.

Use this library if your needs are very simple and you don't require addressing
color spaces other than RGBA. It is also useful for storing colors which can be
easily moved to and from `gdk` color representation, as the structs can be
serialized and deserialized using `serde`

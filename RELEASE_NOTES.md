# Contents
* [Unreleased](#unreleased)
* [0.6.0](#0.6.0-release)

## Unreleased
* Make Serde support optional

## 0.6.0 Release
Complete rewrite using generics. This should be essentially the stable interface
going forward, modulo minor additions, and is much cleaner and more Rustic.
* Removed `HexColor` and `ReducedRGBA` structs
* Removed `Color` enum
* Made RGBA generic over most numeric primitives
  * unsigned integers u8 - u64
  * signed integers i16 - i64
  * 32 and 64 bit floats
* New `RGB` struct which is the same as `RGBA` without the alpha channel
* New public `Hex` trait for converting to and from hexadecimal Strings
* Conversions to and from gdk are now in `ToGdk` and `FromGdk` traits

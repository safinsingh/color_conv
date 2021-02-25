# color_conv

[color_conv](https://crates.io/crates/color_conv) is a simple and lightweight helper library for easily and programmatically converting between the `RGB`, `CMYK`, `HSL`, and `hex` color formats.

```toml
[dependencies]
color_conv = "0.2"
```

# Example

```rust
use color_conv::Color;
use color_conv::Cmyk;
use color_conv::Rgb;

let cyan = Cmyk::new_unchecked(100, 0, 0, 0);
let cyan_rgb = cyan.to_rgb();

assert_eq!(Rgb::new(0, 255, 255), cyan_rgb);
```

use crate::{Cmyk, Color, Hsl};
use std::fmt;

#[derive(Clone, Copy, Debug, PartialEq)]
/// A representation of the RGB (red, green, blue) color format.
pub struct Rgb {
	/// Red value
	pub red: u8,
	/// Green value
	pub green: u8,
	/// Blue value
	pub blue: u8,
}

impl Rgb {
	/// Returns a new Rgb object given red, green, and blue values.
	///
	/// # Arguments
	///
	/// * `red` - the red value of the color
	/// * `green` - the green value of the color
	/// * `blue` - the blue value of the color
	///
	/// # Examples
	///
	/// ```
	/// use color_conv::Rgb;
	/// let cyan = Rgb::new(0, 255, 255);
	/// // ...
	/// ```
	///
	/// # Note
	///
	/// * `red`, `green`, and `blue` are all 8-bit integers with a maximum value
	///   of 255.
	pub fn new(red: u8, green: u8, blue: u8) -> Self {
		Self { red, green, blue }
	}

	fn _to_cmyk(self) -> (u8, u8, u8, u8) {
		let r_prime = self.red as f64 / 255.;
		let g_prime = self.green as f64 / 255.;
		let b_prime = self.blue as f64 / 255.;

		let key = 1.
			- [r_prime, g_prime, b_prime]
				.iter()
				.cloned()
				.fold(f64::NAN, f64::max);

		let apply = |v: f64| (((1. - v - key) / (1. - key)) * 100.).round();
		let cyan = apply(r_prime);
		let magenta = apply(g_prime);
		let yellow = apply(b_prime);

		(cyan as u8, magenta as u8, yellow as u8, (key * 100.) as u8)
	}
}

impl fmt::Display for Rgb {
	fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
		write!(f, "rgb({}, {}, {})", self.red, self.green, self.blue)
	}
}

impl Color for Rgb {
	fn to_rgb(self) -> Rgb {
		self
	}

	fn to_hex_string(self) -> String {
		format!("#{:0>2x}{:0>2x}{:0>2x}", self.red, self.green, self.blue)
	}

	fn to_cmyk(self) -> Cmyk {
		let (c, m, y, k) = self._to_cmyk();
		Cmyk::new_unchecked(c, m, y, k)
	}

	fn to_hsl(self) -> Hsl {
		let Self { red, green, blue } = self;

		let r_prime = red as f64 / 255.;
		let g_prime = green as f64 / 255.;
		let b_prime = blue as f64 / 255.;

		let c_max = [red, green, blue].iter().max().cloned().unwrap() as f64 / 255.;
		let c_min = [red, green, blue].iter().min().cloned().unwrap() as f64 / 255.;

		let delta = c_max - c_min;

		let hue = if (delta - 0.) < f64::EPSILON {
			0
		} else {
			match c_max {
				x if x == r_prime => 60. * (((g_prime - b_prime) / delta) % 6.),
				x if x == g_prime => 60. * (((b_prime - r_prime) / delta) + 2.),
				x if x == b_prime => 60. * (((r_prime - g_prime) / delta) + 4.),
				_ => panic!("Invalid hue calculation!"),
			}
			.round() as u16
		};

		let lightness = (c_max + c_min) / 2.;

		let saturation = if (delta - 0.) < f64::EPSILON {
			0
		} else {
			(delta / (1. - ((2. * lightness) - 1.)) * 100.).round() as u8
		};

		Hsl::new_unchecked(hue, saturation, (lightness * 100.).round() as u8)
	}
}

#[cfg(test)]
mod test {
	use super::*;

	#[test]
	fn test_to_string() {
		let rgb = Rgb::new(30, 50, 60);
		assert_eq!(rgb.to_string(), String::from("rgb(30, 50, 60)"));
	}

	#[test]
	fn test_to_hex_string() {
		let hex = Rgb::new(30, 50, 60).to_hex_string();
		assert_eq!(hex, String::from("#1e323c"));
	}

	#[test]
	fn test_to_cmyk() {
		let rgb = Rgb::new(30, 50, 60).to_cmyk();
		assert_eq!(rgb, Cmyk::new_unchecked(50, 17, 0, 76));
	}

	#[test]
	fn test_to_hsl() {
		let hsl = Rgb::new(204, 153, 102).to_hsl();
		assert_eq!(hsl, Hsl::new_unchecked(30, 50, 60));
	}
}

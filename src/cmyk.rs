use crate::{Color, Error, Hsl, Rgb};
use std::fmt;

#[derive(Clone, Copy, Debug, PartialEq)]
/// A representation of the CMYK (cyan, magenta, yellow, key) color format.
pub struct Cmyk {
	/// Cyan value (percentage)
	pub cyan: u8,
	/// Magenta value (percentage)
	pub magenta: u8,
	/// Yellow value (percentage)
	pub yellow: u8,
	/// Key value (percentage)
	pub key: u8,
}

impl Cmyk {
	/// Returns a Result containing a new Cmyk object given cyan, magenta,
	/// yellow, and key values. Will return an [`Error`] if any of the arguments
	/// are larger than 100 due to the fact that they represent percentages.
	///
	/// # Arguments
	///
	/// * `cyan` - the cyan value of the color
	/// * `magenta` - the magenta value of the color
	/// * `yellow` - the yellow value of the color
	/// * `key` - the key value of the color
	///
	/// # Examples
	///
	/// ```
	/// use color_conv::Cmyk;
	/// let cyan = Cmyk::new(100, 0, 0, 0)?;
	/// # Ok::<(), color_conv::Error>(())
	/// ```
	pub fn new(cyan: u8, magenta: u8, yellow: u8, key: u8) -> Result<Self, Error> {
		if ![cyan, magenta, yellow, key].iter().all(|val| *val <= 100) {
			return Err(Error::PercentageOverflow);
		}

		Ok(Self::new_unchecked(cyan, magenta, yellow, key))
	}

	/// See [`Cmyk::new`](self::Cmyk::new). Does not perform check to ensure
	/// that all parameters are less than or equal to 100. This is useful for
	/// when you know more than the compiler about which values are being passed
	/// to the method.
	///
	/// # Examples
	///
	/// ```
	/// use color_conv::Cmyk;
	/// let cyan = Cmyk::new_unchecked(100, 0, 0, 0);
	/// ```
	pub fn new_unchecked(cyan: u8, magenta: u8, yellow: u8, key: u8) -> Self {
		Self {
			cyan,
			magenta,
			yellow,
			key,
		}
	}
}

impl fmt::Display for Cmyk {
	fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
		write!(
			f,
			"cmyk({}%, {}%, {}%, {}%)",
			self.cyan, self.magenta, self.yellow, self.key
		)
	}
}

impl Color for Cmyk {
	fn to_rgb(self) -> Rgb {
		let apply =
			|v| (255. * (1f64 - v as f64 / 100.) * (1. - self.key as f64 / 100.)).round() as u8;

		let red = apply(self.cyan);
		let green = apply(self.magenta);
		let blue = apply(self.yellow);

		Rgb { red, green, blue }
	}

	fn to_cmyk(self) -> Cmyk {
		self
	}

	fn to_hex_string(self) -> String {
		Rgb::to_hex_string(self.to_rgb())
	}

	fn to_hsl(self) -> Hsl {
		self.to_rgb().to_hsl()
	}
}

#[cfg(test)]
mod test {
	use super::*;

	#[test]
	fn test_to_string() {
		let cmyk = Cmyk::new(30, 50, 60, 40).unwrap();
		assert_eq!(cmyk.to_string(), String::from("cmyk(30%, 50%, 60%, 40%)"));
	}

	#[test]
	fn test_to_hex_string() {
		let hex = Cmyk::new(30, 50, 60, 40).unwrap().to_hex_string();
		assert_eq!(hex, String::from("#6b4d3d"));
	}

	#[test]
	fn test_to_rgb() {
		let hex = Cmyk::new(30, 50, 60, 40).unwrap().to_rgb();
		assert_eq!(hex, Rgb::new(107, 77, 61));
	}

	#[should_panic]
	#[test]
	fn test_checked_cmyk() {
		Cmyk::new(255, 255, 255, 255).unwrap();
	}
}

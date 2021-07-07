#[allow(unused_imports)]
use crate::prelude::*;
use crate::{Cmyk, Color, Error, Float, Rgb};
use core::fmt;

///
/// A representation of the HSL (cyan, magenta, yellow, key) color format.
///
#[derive(Clone, Copy, PartialEq, Eq, Debug, Hash)]
pub struct Hsl {
	/// Hue value (in degrees)
	pub hue: u16,
	/// Saturation percentage
	pub saturation: u8,
	/// Lightness percentage
	pub lightness: u8,
}

impl Hsl {
	///
	/// Returns a Result containing a new Hsl object given hue, saturation,
	/// and lightness values. Will return an [`Error`] if either the saturation
	/// or lightness are larger than 100 due to the fact that they represent
	/// percentages or the hue is greater than 360 because it represents a
	/// degree value.
	///
	/// # Arguments
	///
	/// * `hue` - the hue value of the color
	/// * `saturation` - the saturation value of the color
	/// * `lightness` - the lightness value of the color
	///
	/// # Examples
	///
	/// ```
	/// use color_conv::Hsl;
	/// let cyan = Hsl::new(180, 100, 50)?;
	/// # Ok::<(), color_conv::Error>(())
	/// ```
	///
	pub fn new(hue: u16, saturation: u8, lightness: u8) -> Result<Self, Error> {
		if !(saturation <= 100 && lightness <= 100) {
			return Err(Error::PercentageOverflow);
		}

		if hue > 360 {
			return Err(Error::DegreeOverflow);
		}

		Ok(Self::new_unchecked(hue, saturation, lightness))
	}

	///
	/// See [`Hsl::new`](self::Hsl::new). Does not perform check to ensure
	/// that all parameters are valid. This is useful for when you know more
	/// than the compiler about which values are being passed to the method.
	///
	/// # Examples
	///
	/// ```
	/// use color_conv::Hsl;
	/// let cyan = Hsl::new_unchecked(180, 100, 50);
	/// ```
	///
	pub fn new_unchecked(hue: u16, saturation: u8, lightness: u8) -> Self {
		Self {
			hue,
			saturation,
			lightness,
		}
	}
}

impl fmt::Display for Hsl {
	fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
		write!(
			f,
			"hsl({}°, {}%, {}%)",
			self.hue, self.saturation, self.lightness
		)
	}
}

macro_rules! exclusive_range_workaround {
	(
		$self:ident,
		$($range:expr => $tup:expr),*
	) => ({
		match $self.hue {
			$(
				h if ($range).contains(&h) => $tup,
			)*
			_ => panic!("Unexpected hue: {}, larger than 360!", $self.hue),
		}
	});
}

impl Color for Hsl {
	fn to_rgb(self) -> Rgb {
		let c = (1. - ((2. * (self.lightness as Float / 100.)) - 1.).abs())
			* (self.saturation as Float / 100.);
		let x = c * (1. - ((((self.hue as Float) / 60.) % 2.) - 1.).abs());
		let m = (self.lightness as Float / 100.) - (c / 2.);

		let (r_prime, g_prime, b_prime) = exclusive_range_workaround! { self,
			0..60 => (c, x, 0.),
			60..120 => (x, c, 0.),
			120..180 => (0., c, x),
			180..240 => (0., x, c),
			240..300 => (x, 0., c),
			300..360 => (c, 0., x)
		};

		let apply = |v: Float| ((v + m) * 255.).round() as u8;
		let red = apply(r_prime);
		let green = apply(g_prime);
		let blue = apply(b_prime);

		Rgb { red, green, blue }
	}

	fn to_cmyk(self) -> Cmyk {
		self.to_rgb().to_cmyk()
	}

	fn to_hex_string(self) -> String {
		self.to_rgb().to_hex_string()
	}

	fn to_hsl(self) -> Hsl {
		self
	}
}

#[cfg(test)]
mod test {
	use super::*;

	#[test]
	fn test_to_string() {
		let hsl = Hsl::new_unchecked(100, 100, 100);
		assert_eq!(hsl.to_string(), String::from("hsl(100°, 100%, 100%)"));
	}

	#[test]
	fn test_to_hex_string() {
		let hex = Hsl::new_unchecked(30, 50, 60).to_hex_string();
		assert_eq!(hex, String::from("#cc9966"));
	}

	#[test]
	fn test_to_rgb() {
		let rgb = Hsl::new_unchecked(30, 50, 60).to_rgb();
		assert_eq!(rgb, Rgb::new(204, 153, 102));
	}

	#[should_panic]
	#[test]
	fn test_checked_hsl() {
		Hsl::new(361, 101, 101).unwrap();
	}
}

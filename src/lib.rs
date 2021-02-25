#![deny(
	missing_docs,
	missing_doc_code_examples,
	trivial_casts,
	trivial_numeric_casts
)]

//! `color_conv` is a helper library for easily and programmatically converting
//! between the `RGB`, `CMYK`, `HSL`, and `hex` color formats.
//!
//! ```toml
//! [dependencies]
//! color_conv = "0.1"
//! ```
//!
//! # Example
//!
//! ```
//! use color_conv::Color;
//! use color_conv::Cmyk;
//! use color_conv::Rgb;
//!
//! let cyan = Cmyk::new_unchecked(100, 0, 0, 0);
//! let cyan_rgb = cyan.to_rgb();
//!
//! assert_eq!(Rgb::new(0, 255, 255), cyan_rgb);
//! ```

/// CMYK-specific structures
pub mod cmyk;
/// Crate-wide errors
pub mod error;
/// HSL-specific strucures
pub mod hsl;
/// RGB-specific strucures
pub mod rgb;

pub use self::{cmyk::Cmyk, error::Error, hsl::Hsl, rgb::Rgb};

/// Unifying `Color` trait which encompasses each of the structs provided by
/// this crate.
pub trait Color {
	/// Convert to [`Rgb`]
	///
	/// # Examples
	///
	/// ```
	/// use color_conv::Color;
	/// use color_conv::Cmyk;
	/// use color_conv::Rgb;
	///
	/// let cyan = Cmyk::new_unchecked(100, 0, 0, 0);
	/// let cyan_rgb = cyan.to_rgb();
	///
	/// assert_eq!(Rgb::new(0, 255, 255), cyan_rgb);
	/// # Ok::<(), color_conv::Error>(())
	/// ```
	fn to_rgb(self) -> Rgb;

	/// Convert to [`Cmyk`] with the possibility of failing if any of the
	/// percentage values are above 100
	///
	/// # Examples
	///
	/// ```
	/// use color_conv::Color;
	/// use color_conv::Rgb;
	/// use color_conv::Cmyk;
	///
	/// let cyan = Rgb::new(0, 255, 255);
	/// let cyan_cmyk = cyan.to_cmyk();
	///
	/// assert_eq!(cyan_cmyk, Cmyk::new_unchecked(100, 0, 0, 0));
	/// ```
	fn to_cmyk(self) -> Cmyk;

	/// Convert to [`Hsl`] with the possibility of failing if any of the
	/// percentage values are above 100 or degree values are above 360
	///
	/// # Examples
	///
	/// ```
	/// use color_conv::Color;
	/// use color_conv::Hsl;
	/// use color_conv::Cmyk;
	///
	/// let cyan = Hsl::new_unchecked(180, 100, 50);
	/// let cyan_cmyk = cyan.to_cmyk();
	///
	/// assert_eq!(cyan_cmyk, Cmyk::new_unchecked(100, 0, 0, 0));
	/// ```
	fn to_hsl(self) -> Hsl;

	/// Convert to a [`String`] containing the hex code of the color prefixed
	/// with a hashtag (`#`)
	///
	/// # Examples
	///
	/// ```
	/// use color_conv::Color;
	/// use color_conv::Rgb;
	///
	/// let cyan = Rgb::new(0, 255, 255);
	/// let cyan_hex = cyan.to_hex_string();
	///
	/// assert_eq!(cyan_hex, String::from("#00ffff"));
	/// ```
	fn to_hex_string(self) -> String;
}

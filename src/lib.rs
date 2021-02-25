#![deny(
	missing_docs,
	missing_doc_code_examples,
	trivial_casts,
	trivial_numeric_casts
)]

//!
//! `color_conv` is a helper library for easily and programmatically converting
//! between the `RGB`, `CMYK`, `HSL`, and `hex` color formats.
//!
//! ```toml
//! [dependencies]
//! color_conv = "0.2.1"
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
//!

/// CMYK-specific structures
pub mod cmyk;
/// HSL-specific strucures
pub mod hsl;
/// RGB-specific strucures
pub mod rgb;

pub use self::{cmyk::Cmyk, hsl::Hsl, rgb::Rgb};
use thiserror::Error as ThisError;

#[derive(ThisError, Debug)]
///
/// Crate-wide Error type.
///
pub enum Error {
	///
	/// Occurs when a parameter representing a percentage value is greater than
	/// 100. This error can be thrown by [`Cmyk::new`](crate::Cmyk::new) or
	/// [`Hsl::new`](crate::Hsl::new), both of which perform this check.
	///
	#[error("Percentage overflow: value is larger than 100!")]
	PercentageOverflow,
	///
	/// Occurs when a parameter representing a degree value is greater than 360.
	/// 100. This error can be thrown by  [`Hsl::new`](crate::Hsl::new), which
	/// performs this check.
	///
	#[error("Degree overflow: value is larger than 360!")]
	DegreeOverflow,
}

///
/// Unifying `Color` trait which encompasses each of the structs provided by
/// this crate.
///
pub trait Color {
	///
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
	///
	fn to_rgb(self) -> Rgb;

	///
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
	///
	fn to_cmyk(self) -> Cmyk;

	///
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
	///
	fn to_hsl(self) -> Hsl;

	///
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
	///
	fn to_hex_string(self) -> String;
}

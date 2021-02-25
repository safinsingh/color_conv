use thiserror::Error as ThisError;

#[derive(ThisError, Debug)]
/// Crate-wide Error type.
pub enum Error {
	#[error("Percentage overflow: value is larger than 100!")]
	/// Occurs when a parameter representing a percentage value is greater than
	/// 100. This error can be thrown by [`Cmyk::new`](crate::Cmyk::new) or
	/// [`Hsl::new`](crate::Hsl::new), both of which perform this check.
	PercentageOverflow,
	/// Occurs when a parameter representing a degree value is greater than 360.
	/// 100. This error can be thrown by  [`Hsl::new`](crate::Hsl::new), which
	/// performs this check.
	#[error("Degree overflow: value is larger than 360!")]
	DegreeOverflow,
}

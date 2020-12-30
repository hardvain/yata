use crate::core::{Action, ValueType};
use std::fmt;

#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};

/// Every `Indicator` proceed an input of [`OHLCV`](crate::core::OHLCV) and returns an `IndicatorResult` which consist of some returned raw values.
///
/// `Indicator` may return up to 4 raw values at each step
#[derive(Clone, Copy)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct IndicatorResult {
	values: [ValueType; IndicatorResult::SIZE],
	length: u8,
}

impl IndicatorResult {
	/// Size of pre-allocated result array
	/// For the most of cases it should not be used anywhere outside this crate
	pub const SIZE: usize = 4;

	/// Returns a slice of raw indicator values of current indicator result
	#[must_use]
	pub fn values(&self) -> &[ValueType] {
		let len = self.length as usize;
		&self.values[..len]
	}

	/// Returns count of raw values
	#[must_use]
	pub const fn values_length(&self) -> u8 {
		self.length
	}

	/// Returns a count of raw values
	#[must_use]
	pub const fn size(&self) -> u8 {
		self.length
	}

	/// Returns a raw value at given index
	#[inline]
	#[must_use]
	pub fn value(&self, index: usize) -> ValueType {
		debug_assert!(index < self.length as usize);
		self.values[index]
	}


	/// Creates a new instance of `IndicatorResult` with provided *values*
	#[inline]
	#[must_use]
	pub fn new(values_slice: &[ValueType]) -> Self {
		let mut values = [0 as ValueType; Self::SIZE];

		let values_length = Self::SIZE.min(values_slice.len());
		values[..values_length].copy_from_slice(&values_slice[..values_length]);

		#[allow(clippy::cast_possible_truncation)]
		let length = values_length as u8;

		Self {
			values,
			length,
		}
	}
}

impl fmt::Debug for IndicatorResult {
	fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
		let values: Vec<String> = self
			.values
			.iter()
			.take(self.length as usize)
			.map(|&x| format!("{:>7.4}", x))
			.collect();

		write!(
			f,
			"Values: [{:}]",
			values.join(", ")
		)
	}
}

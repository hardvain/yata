#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};

use crate::core::{Error, Method, PeriodType, Source, OHLCV};
use crate::core::{IndicatorConfig, IndicatorInstance, IndicatorResult};
use crate::methods::{ReversalSignal, HMA};

/// Hull Moving Average indicator
///
/// ## Links
///
/// * <https://www.fidelity.com/learning-center/trading-investing/technical-analysis/technical-indicator-guide/hull-moving-average>
///
/// # 1 value
///
/// * `HMA value`
///
/// Range of values is the same as the range of the `source` values.
///
/// # 1 signal
///
/// * When `HMA value` reverses upwards, gives full positive signal.
/// When `HMA value` reverses downwards, gives full negative signal.
/// Otherwise returns no signal.
#[derive(Debug, Clone, Copy)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct HullMovingAverage {
	/// HMA period. Default is `9`.
	///
	/// Range in \[`3`; [`PeriodType::MAX`](crate::core::PeriodType)\).
	pub period: PeriodType,

	/// Left lag for reverse point detection. Default is `3`.
	///
	/// Range in \[`1`; [`PeriodType::MAX`](crate::core::PeriodType)/`2`\]
	pub left: PeriodType,

	/// Right lag for reverse point detection. Default is `2`.
	///
	/// Range in \[`1`; [`PeriodType::MAX`](crate::core::PeriodType)/`2`\].
	pub right: PeriodType,

	/// Source type of values. Default is [`Close`](crate::core::Source::Close)
	pub source: Source,
}

impl IndicatorConfig for HullMovingAverage {
	type Instance = HullMovingAverageInstance;

	const NAME: &'static str = "HullMovingAverage";

	fn init<T: OHLCV>(self, candle: &T) -> Result<Self::Instance, Error> {
		if !self.validate() {
			return Err(Error::WrongConfig);
		}

		let cfg = self;
		let src = candle.source(cfg.source);

		Ok(Self::Instance {
			hma: HMA::new(cfg.period, src)?,
			cfg,
		})
	}

	fn validate(&self) -> bool {
		self.period > 2
			&& self.left >= 1
			&& self.right >= 1
			&& self.left.saturating_add(self.right) < PeriodType::MAX
	}

	fn set(&mut self, name: &str, value: String) -> Result<(), Error> {
		match name {
			"period" => match value.parse() {
				Err(_) => return Err(Error::ParameterParse(name.to_string(), value.to_string())),
				Ok(value) => self.period = value,
			},
			"left" => match value.parse() {
				Err(_) => return Err(Error::ParameterParse(name.to_string(), value.to_string())),
				Ok(value) => self.left = value,
			},
			"right" => match value.parse() {
				Err(_) => return Err(Error::ParameterParse(name.to_string(), value.to_string())),
				Ok(value) => self.right = value,
			},
			"source" => match value.parse() {
				Err(_) => return Err(Error::ParameterParse(name.to_string(), value.to_string())),
				Ok(value) => self.source = value,
			},

			_ => {
				return Err(Error::ParameterParse(name.to_string(), value));
			}
		};

		Ok(())
	}

	fn size(&self) -> (u8, u8) {
		(1, 1)
	}
}

impl Default for HullMovingAverage {
	fn default() -> Self {
		Self {
			period: 9,
			left: 3,
			right: 2,
			source: Source::Close,
		}
	}
}

#[derive(Debug, Clone)]
pub struct HullMovingAverageInstance {
	cfg: HullMovingAverage,
	hma: HMA,
}

impl IndicatorInstance for HullMovingAverageInstance {
	type Config = HullMovingAverage;

	fn config(&self) -> &Self::Config {
		&self.cfg
	}

	fn next<T: OHLCV>(&mut self, candle: &T) -> IndicatorResult {
		let value = self.hma.next(candle.source(self.cfg.source));
		IndicatorResult::new(&[value])
	}
}

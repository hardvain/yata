use crate::core::{ValueType, IndicatorResult, Action, Error, OHLCV};
use std::fmt;

pub trait StrategyConfig: Clone {
    /// Type of **State**
    type Instance: StrategyInstance<Config = Self>;

    /// Name of an indicator
    const NAME: &'static str;

    /// Returns a name of the indicator
    fn name(&self) -> &'static str {
        Self::NAME
    }

    /// Initializes the **State** based on current **Configuration**
    fn init<T: OHLCV>(self, initial_value: &T) -> Result<Self::Instance, Error>;
}


/// Base trait for implementing indicators **State**
pub trait StrategyInstance: Sized {
    /// Type of Indicator **Configuration**
    type Config: StrategyConfig; // <Instance = Self>;

    /// Returns a reference to the indicator **Configuration**
    fn config(&self) -> &Self::Config;

    /// Evaluates given candle and returns [`StrategyResult`](crate::core::StrategyResult)
    fn next<T: OHLCV>(&mut self, candle: &T) -> StrategyResult;

    /// Returns a name of the indicator
    fn name(&self) -> &'static str {
        Self::Config::NAME
    }

}


pub struct StrategyResult {
    indicator_result:IndicatorResult,
    signals: [Action; IndicatorResult::SIZE],
    length: u8,
}
impl StrategyResult {
    /// Returns a slice of signals of current indicator result
    #[must_use]
    pub fn signals(&self) -> &[Action] {
        let len = self.length as usize;
        &self.signals[..len]
    }

    /// Returns a slice of raw indicator values of current indicator result
    #[must_use]
    pub fn values(&self) -> &[ValueType] {
        self.indicator_result.values()
    }

    /// Returns count of signals
    #[must_use]
    pub const fn signals_length(&self) -> u8 {
        self.length
    }

    /// Returns count of raw values
    #[must_use]
    pub const fn values_length(&self) -> u8 {
        self.length
    }

    /// Returns a tuple of count of raw values and count of signals
    #[must_use]
    pub const fn size(&self) -> u8 {
        self.length
    }

    /// Returns a raw value at given index
    #[inline]
    #[must_use]
    pub fn value(&self, index: usize) -> ValueType {
        self.indicator_result.value(index)
    }

    /// Returns a signal at given index
    #[inline]
    #[must_use]
    pub fn signal(&self, index: usize) -> Action {
        debug_assert!(index < self.length.1 as usize);
        self.signals[index]
    }

    /// Creates a new instance of `StrategyResult` with provided *values* and *signals*
    #[inline]
    #[must_use]
    pub fn new(values_slice: &[ValueType], signals_slice: &[Action]) -> Self {
        let indicator_result = IndicatorResult::new(values_slice,);
        let mut signals = [Action::default(); Self::SIZE];

        let signals_length = Self::SIZE.min(signals_slice.len());
        signals[..signals_length].copy_from_slice(&signals_slice[..signals_length]);

        Self {
            indicator_result,
            signals,
            length:indicator_result.values_length(),
        }
    }
}

impl fmt::Debug for StrategyResult {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let values: Vec<String> = self
            .indicator_result.values()
            .iter()
            .take(self.length.0 as usize)
            .map(|&x| format!("{:>7.4}", x))
            .collect();
        let signals: Vec<String> = self
            .signals
            .iter()
            .take(self.length.1 as usize)
            .map(std::string::ToString::to_string)
            .collect();
        write!(
            f,
            "S: [{:}], V: [{:}]",
            signals.join(", "),
            values.join(", ")
        )
    }
}
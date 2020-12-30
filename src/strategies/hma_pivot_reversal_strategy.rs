use crate::core::{PeriodType, StrategyConfig, OHLCV, Error, StrategyInstance, IndicatorConfig, Source, StrategyResult, Action, IndicatorInstanceDyn};
use crate::methods::{HMA, ReversalSignal};
use crate::indicators::HullMovingAverage;
use crate::prelude::Method;

#[derive(Debug, Clone)]
struct HMAPivotReversalStrategy {
    pub period: PeriodType,
    pub left: PeriodType,
    pub right: PeriodType,
    pub source: Source,
}

impl StrategyConfig for HMAPivotReversalStrategy {
    type Instance = HMAPivotReversal;
    const NAME: &'static str = "";

    fn init<T: OHLCV>(self, candle: &T) -> Result<Self::Instance, Error> {
        let hull_moving_average = HullMovingAverage {
            period: self.period,
            left: self.left,
            right: self.right,
            source: self.source,
        };
        let src = candle.source(self.source);
        Ok(Self::Instance {
            hma_indicator: hull_moving_average,
            config: self,
            pivot: ReversalSignal::new(self.left.clone(), self.right.clone(), src)?,
        })
    }
}

struct HMAPivotReversal {
    hma_indicator: HullMovingAverage,
    config: HMAPivotReversalStrategy,
    pivot: ReversalSignal,
}

impl StrategyInstance for HMAPivotReversal {
    type Config = HMAPivotReversalStrategy;

    fn config(&self) -> &Self::Config {
        &self.config
    }

    fn next<T: OHLCV>(&mut self, candle: &T) -> StrategyResult {
        let indicator_result = self.hma_indicator.next(candle);
        let values = indicator_result.values();
        let value = values.get(0).unwrap();
        let signal:Action = self.pivot.next(*value);
        StrategyResult::new(indicator_result.values(),&[signal])
    }
}


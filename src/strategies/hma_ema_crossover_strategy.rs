use crate::core::{PeriodType, StrategyConfig, OHLCV, Error, StrategyInstance, IndicatorConfig, Source, StrategyResult, Action, IndicatorInstanceDyn};
use crate::methods::{HMA, ReversalSignal, EMA};
use crate::indicators::HullMovingAverage;
use crate::prelude::Method;

#[derive(Debug, Clone)]
struct HmaEmaCrossOver {
    pub period: PeriodType,
    pub left: PeriodType,
    pub right: PeriodType,
    pub source: Source,
}

impl StrategyConfig for HmaEmaCrossOver {
    type Instance = HmaEmaCrossOverInstance;
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
            ema_indicator: EMA::new(self.period, (*candle).clone())?,
            hma_indicator: hull_moving_average,
            config: self,
        })
    }
}

struct HmaEmaCrossOverInstance {
    hma_indicator: HullMovingAverage,
    ema_indicator: EMA,
    config: HmaEmaCrossOver,
}

impl StrategyInstance for HmaEmaCrossOverInstance {
    type Config = HmaEmaCrossOver;

    fn config(&self) -> &Self::Config {
        &self.config
    }

    fn next<T: OHLCV>(&mut self, candle: &T) -> StrategyResult {
        let indicator_result = self.hma_indicator.next(candle);
        let ema_result = self.ema_indicator.next((*candle).clone());
        let values = indicator_result.values();
        let value = values.get(0).unwrap();
        let signal = if ema_result > *value { Action::BUY_ALL } else { Action::SELL_ALL };
        StrategyResult::new(indicator_result.values(), &[signal])
    }
}


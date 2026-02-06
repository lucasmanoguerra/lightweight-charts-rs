use crate::chart::{Candle, LinePoint};
use time::OffsetDateTime;

pub fn compute_ema(candles: &[Candle], period: usize) -> Vec<LinePoint> {
    if period == 0 || candles.is_empty() {
        return Vec::new();
    }
    let k = 2.0 / (period as f64 + 1.0);
    let mut out = Vec::with_capacity(candles.len());
    let mut ema = candles[0].close;
    for candle in candles {
        ema = candle.close * k + ema * (1.0 - k);
        out.push(LinePoint {
            time: candle.time,
            value: ema,
        });
    }
    out
}

pub fn compute_ema_for_times(values: &[(OffsetDateTime, f64)], period: usize) -> Vec<LinePoint> {
    if period == 0 || values.is_empty() {
        return Vec::new();
    }
    let k = 2.0 / (period as f64 + 1.0);
    let mut out = Vec::with_capacity(values.len());
    let mut ema = values[0].1;
    for (time, value) in values {
        ema = *value * k + ema * (1.0 - k);
        out.push(LinePoint {
            time: *time,
            value: ema,
        });
    }
    out
}

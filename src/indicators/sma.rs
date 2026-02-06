use crate::chart::{Candle, LinePoint};
use time::OffsetDateTime;

pub fn compute_sma(candles: &[Candle], period: usize) -> Vec<LinePoint> {
    if period == 0 {
        return Vec::new();
    }
    let mut out = Vec::with_capacity(candles.len());
    let mut sum = 0.0;
    let mut window = Vec::with_capacity(period);
    for candle in candles {
        sum += candle.close;
        window.push(candle.close);
        if window.len() > period {
            if let Some(first) = window.first().copied() {
                sum -= first;
            }
            window.remove(0);
        }
        if window.len() == period {
            let value = sum / period as f64;
            out.push(LinePoint {
                time: candle.time,
                value,
            });
        }
    }
    out
}

pub fn compute_sma_for_times(values: &[(OffsetDateTime, f64)], period: usize) -> Vec<LinePoint> {
    if period == 0 {
        return Vec::new();
    }
    let mut out = Vec::with_capacity(values.len());
    let mut sum = 0.0;
    let mut window = Vec::with_capacity(period);
    for (time, value) in values {
        sum += value;
        window.push(*value);
        if window.len() > period {
            if let Some(first) = window.first().copied() {
                sum -= first;
            }
            window.remove(0);
        }
        if window.len() == period {
            out.push(LinePoint {
                time: *time,
                value: sum / period as f64,
            });
        }
    }
    out
}

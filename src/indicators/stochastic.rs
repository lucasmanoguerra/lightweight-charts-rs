use crate::chart::{Candle, LinePoint};
use std::collections::VecDeque;

pub struct StochasticSeries {
    pub k: Vec<LinePoint>,
    pub d: Vec<LinePoint>,
}

pub fn compute_stochastic(
    candles: &[Candle],
    k_period: usize,
    d_period: usize,
) -> StochasticSeries {
    let mut k_values: Vec<(time::OffsetDateTime, f64)> = Vec::new();
    if k_period == 0 {
        return StochasticSeries {
            k: Vec::new(),
            d: Vec::new(),
        };
    }

    for idx in 0..candles.len() {
        if idx + 1 < k_period {
            continue;
        }
        let window = &candles[idx + 1 - k_period..=idx];
        let high = window.iter().map(|c| c.high).fold(f64::MIN, f64::max);
        let low = window.iter().map(|c| c.low).fold(f64::MAX, f64::min);
        let close = candles[idx].close;
        let value = if (high - low).abs() <= f64::EPSILON {
            0.0
        } else {
            (close - low) / (high - low) * 100.0
        };
        k_values.push((candles[idx].time, value));
    }

    let k_line: Vec<LinePoint> = k_values
        .iter()
        .map(|(t, v)| LinePoint {
            time: *t,
            value: *v,
        })
        .collect();

    let mut d_line = Vec::new();
    if d_period > 0 {
        let mut window: VecDeque<f64> = VecDeque::with_capacity(d_period);
        let mut sum = 0.0;
        for (time, value) in &k_values {
            window.push_back(*value);
            sum += value;
            if window.len() > d_period {
                if let Some(first) = window.pop_front() {
                    sum -= first;
                }
            }
            if window.len() == d_period {
                d_line.push(LinePoint {
                    time: *time,
                    value: sum / d_period as f64,
                });
            }
        }
    }

    StochasticSeries {
        k: k_line,
        d: d_line,
    }
}

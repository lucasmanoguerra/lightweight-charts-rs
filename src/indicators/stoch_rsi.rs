use crate::chart::{Candle, LinePoint};
use crate::indicators::rsi::compute_rsi;

pub struct StochRsiSeries {
    pub k: Vec<LinePoint>,
    pub d: Vec<LinePoint>,
}

pub fn compute_stoch_rsi(
    candles: &[Candle],
    rsi_period: usize,
    k_period: usize,
    d_period: usize,
) -> StochRsiSeries {
    let rsi_points = compute_rsi(candles, rsi_period);
    if k_period == 0 || rsi_points.is_empty() {
        return StochRsiSeries { k: Vec::new(), d: Vec::new() };
    }

    let mut k_values: Vec<(time::OffsetDateTime, f64)> = Vec::new();
    for idx in 0..rsi_points.len() {
        if idx + 1 < k_period {
            continue;
        }
        let window = &rsi_points[idx + 1 - k_period..=idx];
        let high = window.iter().map(|p| p.value).fold(f64::MIN, f64::max);
        let low = window.iter().map(|p| p.value).fold(f64::MAX, f64::min);
        let value = if (high - low).abs() <= f64::EPSILON {
            0.0
        } else {
            (rsi_points[idx].value - low) / (high - low) * 100.0
        };
        k_values.push((rsi_points[idx].time, value));
    }

    let k_line: Vec<LinePoint> = k_values
        .iter()
        .map(|(t, v)| LinePoint { time: *t, value: *v })
        .collect();

    let mut d_line = Vec::new();
    if d_period > 0 {
        let mut window: Vec<f64> = Vec::with_capacity(d_period);
        for (time, value) in &k_values {
            window.push(*value);
            if window.len() > d_period {
                window.remove(0);
            }
            if window.len() == d_period {
                let avg = window.iter().sum::<f64>() / d_period as f64;
                d_line.push(LinePoint { time: *time, value: avg });
            }
        }
    }

    StochRsiSeries { k: k_line, d: d_line }
}

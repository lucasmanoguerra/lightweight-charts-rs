use crate::chart::{Candle, HistogramPoint, LinePoint};
use crate::indicators::ema::compute_ema_for_times;

pub struct MacdSeries {
    pub macd: Vec<LinePoint>,
    pub signal: Vec<LinePoint>,
    pub histogram: Vec<HistogramPoint>,
}

pub fn compute_macd(candles: &[Candle], fast: usize, slow: usize, signal: usize) -> MacdSeries {
    let mut times: Vec<(time::OffsetDateTime, f64)> =
        candles.iter().map(|c| (c.time, c.close)).collect();
    if times.is_empty() {
        return MacdSeries {
            macd: Vec::new(),
            signal: Vec::new(),
            histogram: Vec::new(),
        };
    }
    let fast_ema = compute_ema_for_times(&times, fast);
    let slow_ema = compute_ema_for_times(&times, slow);

    let count = fast_ema.len().min(slow_ema.len());
    let mut macd_vals: Vec<(time::OffsetDateTime, f64)> = Vec::with_capacity(count);
    for idx in 0..count {
        let time = fast_ema[idx].time;
        let value = fast_ema[idx].value - slow_ema[idx].value;
        macd_vals.push((time, value));
    }

    let macd_line: Vec<LinePoint> = macd_vals
        .iter()
        .map(|(t, v)| LinePoint {
            time: *t,
            value: *v,
        })
        .collect();
    let signal_line = compute_ema_for_times(&macd_vals, signal);

    let count = macd_line.len().min(signal_line.len());
    let mut histogram = Vec::with_capacity(count);
    for idx in 0..count {
        let time = macd_line[idx].time;
        let value = macd_line[idx].value - signal_line[idx].value;
        histogram.push(HistogramPoint {
            time,
            value,
            color: None,
        });
    }

    MacdSeries {
        macd: macd_line,
        signal: signal_line,
        histogram,
    }
}

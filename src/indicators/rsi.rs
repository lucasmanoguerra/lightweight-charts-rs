use crate::chart::{Candle, LinePoint};

pub fn compute_rsi(candles: &[Candle], period: usize) -> Vec<LinePoint> {
    if candles.len() <= period {
        return Vec::new();
    }

    let mut gains = 0.0;
    let mut losses = 0.0;
    for i in 1..=period {
        let delta = candles[i].close - candles[i - 1].close;
        if delta >= 0.0 {
            gains += delta;
        } else {
            losses += -delta;
        }
    }

    let mut avg_gain = gains / period as f64;
    let mut avg_loss = losses / period as f64;
    let mut points = Vec::with_capacity(candles.len().saturating_sub(period));

    points.push(LinePoint {
        time: candles[period].time,
        value: rsi_from_avgs(avg_gain, avg_loss),
    });

    for i in (period + 1)..candles.len() {
        let delta = candles[i].close - candles[i - 1].close;
        let gain = if delta > 0.0 { delta } else { 0.0 };
        let loss = if delta < 0.0 { -delta } else { 0.0 };
        avg_gain = (avg_gain * (period as f64 - 1.0) + gain) / period as f64;
        avg_loss = (avg_loss * (period as f64 - 1.0) + loss) / period as f64;
        points.push(LinePoint {
            time: candles[i].time,
            value: rsi_from_avgs(avg_gain, avg_loss),
        });
    }

    points
}

fn rsi_from_avgs(avg_gain: f64, avg_loss: f64) -> f64 {
    if avg_loss.abs() < 1e-9 {
        return 100.0;
    }
    let rs = avg_gain / avg_loss;
    100.0 - (100.0 / (1.0 + rs))
}

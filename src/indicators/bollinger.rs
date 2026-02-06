use crate::chart::{Candle, LinePoint};

pub struct BollingerBands {
    pub middle: Vec<LinePoint>,
    pub upper: Vec<LinePoint>,
    pub lower: Vec<LinePoint>,
}

pub fn compute_bollinger(candles: &[Candle], period: usize, mult: f64) -> BollingerBands {
    if period == 0 {
        return BollingerBands {
            middle: Vec::new(),
            upper: Vec::new(),
            lower: Vec::new(),
        };
    }
    let mut middle = Vec::with_capacity(candles.len());
    let mut upper = Vec::with_capacity(candles.len());
    let mut lower = Vec::with_capacity(candles.len());

    let mut window: Vec<f64> = Vec::with_capacity(period);
    for candle in candles {
        window.push(candle.close);
        if window.len() > period {
            window.remove(0);
        }
        if window.len() == period {
            let mean = window.iter().sum::<f64>() / period as f64;
            let variance = window
                .iter()
                .map(|v| (v - mean) * (v - mean))
                .sum::<f64>()
                / period as f64;
            let std = variance.sqrt();
            let up = mean + std * mult;
            let down = mean - std * mult;
            middle.push(LinePoint {
                time: candle.time,
                value: mean,
            });
            upper.push(LinePoint {
                time: candle.time,
                value: up,
            });
            lower.push(LinePoint {
                time: candle.time,
                value: down,
            });
        }
    }

    BollingerBands { middle, upper, lower }
}

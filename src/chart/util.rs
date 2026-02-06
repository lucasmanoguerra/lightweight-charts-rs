use cairo::Context;
use time::OffsetDateTime;

use super::types::{
    Candle, HistogramPoint, LinePoint, LineStyle, PriceScaleMode, ScaleMargins,
};

pub(crate) fn map_price_to_y(price: f64, min: f64, max: f64, top: f64, height: f64) -> f64 {
    let norm = (price - min) / (max - min);
    top + (1.0 - norm) * height
}

pub(crate) fn map_y_to_price(y: f64, min: f64, max: f64, top: f64, height: f64) -> f64 {
    let norm = ((top + height) - y) / height;
    min + norm * (max - min)
}

pub(crate) fn map_price_to_y_scaled(
    price: f64,
    min: f64,
    max: f64,
    top: f64,
    height: f64,
    margins: ScaleMargins,
    invert: bool,
    mode: PriceScaleMode,
    base: f64,
) -> f64 {
    let (scale_top, scale_height) = scale_area(top, height, margins);
    let t_price = transform_price(price, mode, base);
    let t_min = transform_price(min, mode, base);
    let t_max = transform_price(max, mode, base);
    let norm = if (t_max - t_min).abs() < f64::EPSILON {
        0.5
    } else {
        (t_price - t_min) / (t_max - t_min)
    };
    if invert {
        scale_top + norm * scale_height
    } else {
        scale_top + (1.0 - norm) * scale_height
    }
}

pub(crate) fn map_y_to_price_scaled(
    y: f64,
    min: f64,
    max: f64,
    top: f64,
    height: f64,
    margins: ScaleMargins,
    invert: bool,
    mode: PriceScaleMode,
    base: f64,
) -> f64 {
    let (scale_top, scale_height) = scale_area(top, height, margins);
    let t_min = transform_price(min, mode, base);
    let t_max = transform_price(max, mode, base);
    let norm = if invert {
        (y - scale_top) / scale_height
    } else {
        ((scale_top + scale_height) - y) / scale_height
    };
    let t_value = t_min + norm * (t_max - t_min);
    inverse_transform_price(t_value, mode, base)
}

pub(crate) fn scale_area(top: f64, height: f64, margins: ScaleMargins) -> (f64, f64) {
    let mut top_margin = margins.top.clamp(0.0, 0.49);
    let mut bottom_margin = margins.bottom.clamp(0.0, 0.49);
    if top_margin + bottom_margin >= 0.98 {
        let excess = (top_margin + bottom_margin) - 0.98;
        top_margin -= excess * 0.5;
        bottom_margin -= excess * 0.5;
    }
    let usable = (1.0 - top_margin - bottom_margin).max(0.02);
    let scale_top = top + top_margin * height;
    let scale_height = height * usable;
    (scale_top, scale_height)
}

pub(crate) fn transform_price(value: f64, mode: PriceScaleMode, base: f64) -> f64 {
    match mode {
        PriceScaleMode::Normal => value,
        PriceScaleMode::Logarithmic => {
            let safe = if value <= 0.0 { 1e-9 } else { value };
            safe.ln()
        }
        PriceScaleMode::Percentage => {
            if base.abs() < f64::EPSILON {
                0.0
            } else {
                (value / base - 1.0) * 100.0
            }
        }
        PriceScaleMode::IndexedTo100 => {
            if base.abs() < f64::EPSILON {
                0.0
            } else {
                (value / base) * 100.0
            }
        }
    }
}

pub(crate) fn inverse_transform_price(value: f64, mode: PriceScaleMode, base: f64) -> f64 {
    match mode {
        PriceScaleMode::Normal => value,
        PriceScaleMode::Logarithmic => value.exp(),
        PriceScaleMode::Percentage => base * (value / 100.0 + 1.0),
        PriceScaleMode::IndexedTo100 => base * (value / 100.0),
    }
}

pub(crate) fn map_time_to_x(time: f64, start: f64, end: f64, left: f64, width: f64) -> f64 {
    let norm = (time - start) / (end - start);
    left + norm * width
}

pub(crate) fn candle_time(time: OffsetDateTime) -> f64 {
    time.unix_timestamp() as f64
}

pub(crate) fn series_bar_width_times<I>(times: I, start: f64, end: f64, plot_width: f64) -> f64
where
    I: IntoIterator<Item = f64>,
{
    let mut sorted: Vec<f64> = times.into_iter().collect();
    if sorted.is_empty() {
        return 3.0;
    }
    sorted.sort_by(|a, b| a.partial_cmp(b).unwrap_or(std::cmp::Ordering::Equal));

    if sorted.len() == 1 {
        return (plot_width * 0.08).max(3.0);
    }

    let mut total_delta = 0.0;
    for pair in sorted.windows(2) {
        let delta = (pair[1] - pair[0]).abs();
        total_delta += delta;
    }

    let avg = total_delta / (sorted.len() - 1) as f64;
    let range = (end - start).max(1.0);
    let width = avg / range * plot_width * 0.7;
    width.clamp(2.0, plot_width * 0.5)
}

pub(crate) fn visible_candles(data: &[Candle], start: f64, end: f64) -> Vec<&Candle> {
    let mut visible: Vec<&Candle> = data
        .iter()
        .filter(|candle| {
            let time = candle_time(candle.time);
            time >= start && time <= end
        })
        .collect();

    if visible.is_empty() {
        visible = data.iter().collect();
    }

    visible
}

pub(crate) fn visible_line_points(data: &[LinePoint], start: f64, end: f64) -> Vec<&LinePoint> {
    let mut visible: Vec<&LinePoint> = data
        .iter()
        .filter(|point| {
            let time = candle_time(point.time);
            time >= start && time <= end
        })
        .collect();

    if visible.is_empty() {
        visible = data.iter().collect();
    }

    visible
}

pub(crate) fn visible_histogram_points(
    data: &[HistogramPoint],
    start: f64,
    end: f64,
) -> Vec<&HistogramPoint> {
    let mut visible: Vec<&HistogramPoint> = data
        .iter()
        .filter(|point| {
            let time = candle_time(point.time);
            time >= start && time <= end
        })
        .collect();

    if visible.is_empty() {
        visible = data.iter().collect();
    }

    visible
}

pub(crate) fn histogram_range(points: &[&HistogramPoint]) -> Option<(f64, f64)> {
    if points.is_empty() {
        return None;
    }

    let mut min = f64::INFINITY;
    let mut max = f64::NEG_INFINITY;
    for point in points {
        min = min.min(point.value);
        max = max.max(point.value);
    }

    if min > 0.0 {
        min = 0.0;
    }
    if max < 0.0 {
        max = 0.0;
    }
    if (max - min).abs() < f64::EPSILON {
        max = min + 1.0;
    } else {
        max += (max - min) * 0.05;
    }

    Some((min, max))
}

pub(crate) fn expand_range(min: f64, max: f64) -> (f64, f64) {
    if (max - min).abs() < f64::EPSILON {
        return (min - 1.0, max + 1.0);
    }
    let padding = (max - min) * 0.02;
    (min - padding, max + padding)
}

pub(crate) fn apply_line_style(cr: &Context, style: LineStyle, line_width: f64) {
    match style {
        LineStyle::Solid => cr.set_dash(&[], 0.0),
        LineStyle::Dotted => {
            let dot = line_width.max(1.0);
            cr.set_dash(&[dot, dot * 3.0], 0.0);
        }
        LineStyle::Dashed => {
            let dash = line_width.max(1.0) * 6.0;
            cr.set_dash(&[dash, dash * 0.5], 0.0);
        }
    }
}

pub(crate) fn nearest_by_time<T: super::data::HasTime>(
    data: &[T],
    target_time: f64,
) -> Option<&T> {
    if data.is_empty() {
        return None;
    }

    let mut best = &data[0];
    let mut best_delta = (candle_time(best.time()) - target_time).abs();
    for item in &data[1..] {
        let delta = (candle_time(item.time()) - target_time).abs();
        if delta < best_delta {
            best = item;
            best_delta = delta;
        }
    }

    Some(best)
}

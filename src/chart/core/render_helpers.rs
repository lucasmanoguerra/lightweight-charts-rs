use super::super::data::{Series, SeriesData, SeriesScale};
use super::super::layout::ChartLayout;
use super::super::options::PriceScaleOptions;
use super::super::ticks::{build_price_ticks, PriceTicks};
use super::super::types::{Candle, Color, PriceScale, TooltipPosition};
use super::super::util::{
    inverse_transform_price, map_price_to_y_scaled, map_y_to_price_scaled, scale_area,
    transform_price,
};
use super::super::options::ChartStyle;
use cairo::Context;

pub(super) fn primary_candles(primary: Option<usize>, series: &[Series]) -> Option<&[Candle]> {
    if let Some(id) = primary {
        if let Some(series) = series.get(id) {
            if let SeriesData::Candlestick { data } = &series.data {
                return Some(data);
            }
        }
    }

    for series in series {
        if let SeriesData::Candlestick { data } = &series.data {
            return Some(data);
        }
    }

    None
}

pub(super) fn primary_candle_side(
    primary: Option<usize>,
    series: &[Series],
) -> Option<PriceScale> {
    if let Some(id) = primary {
        if let Some(series) = series.get(id) {
            return Some(series.scale);
        }
    }
    None
}

pub(super) fn build_ticks_for_scale(
    scale: SeriesScale,
    plot_top: f64,
    plot_height: f64,
    options: &PriceScaleOptions,
) -> PriceTicks {
    let (_scale_top, scale_height) = scale_area(plot_top, plot_height, scale.margins);
    let t_min = transform_price(scale.min, scale.mode, scale.base);
    let t_max = transform_price(scale.max, scale.mode, scale.base);
    let (t_min, t_max) = if t_min <= t_max { (t_min, t_max) } else { (t_max, t_min) };
    let mut ticks = build_price_ticks(t_min, t_max, scale_height);
    if options.ensure_edge_tick_marks_visible {
        ensure_edge_ticks(&mut ticks, t_min, t_max);
    }
    ticks
}

pub(super) fn ensure_edge_ticks(ticks: &mut PriceTicks, min: f64, max: f64) {
    let eps = 1e-9;
    if ticks.ticks.is_empty() {
        ticks.ticks.push(min);
        ticks.ticks.push(max);
        return;
    }
    if (ticks.ticks[0] - min).abs() > eps {
        ticks.ticks.insert(0, min);
    }
    if let Some(last) = ticks.ticks.last() {
        if (*last - max).abs() > eps {
            ticks.ticks.push(max);
        }
    }
}

pub(super) fn aligned_price_ticks(
    primary_ticks: &PriceTicks,
    primary_scale: SeriesScale,
    secondary_scale: SeriesScale,
    layout: &ChartLayout,
) -> PriceTicks {
    let mut ticks = Vec::with_capacity(primary_ticks.ticks.len());
    for tick in &primary_ticks.ticks {
        let raw_primary = inverse_transform_price(*tick, primary_scale.mode, primary_scale.base);
        let y = map_price_to_y_scaled(
            raw_primary,
            primary_scale.min,
            primary_scale.max,
            layout.plot_top,
            layout.main_height,
            primary_scale.margins,
            primary_scale.invert,
            primary_scale.mode,
            primary_scale.base,
        );
        let raw_secondary = map_y_to_price_scaled(
            y,
            secondary_scale.min,
            secondary_scale.max,
            layout.plot_top,
            layout.main_height,
            secondary_scale.margins,
            secondary_scale.invert,
            secondary_scale.mode,
            secondary_scale.base,
        );
        let transformed_secondary =
            transform_price(raw_secondary, secondary_scale.mode, secondary_scale.base);
        ticks.push(transformed_secondary);
    }
    PriceTicks {
        ticks,
        precision: primary_ticks.precision,
    }
}

pub(super) fn primary_candle_scale(
    primary: Option<usize>,
    series: &[Series],
    left: Option<SeriesScale>,
    right: Option<SeriesScale>,
) -> Option<SeriesScale> {
    if let Some(id) = primary {
        if let Some(series) = series.get(id) {
            return match series.scale {
                PriceScale::Left => left,
                PriceScale::Right => right,
            };
        }
    }
    None
}

pub(super) fn tooltip_position(
    position: TooltipPosition,
    plot_left: f64,
    plot_right: f64,
    plot_top: f64,
    plot_bottom: f64,
    x: f64,
    y: f64,
    width: f64,
    height: f64,
) -> (f64, f64) {
    match position {
        TooltipPosition::TopLeft => (plot_left + 8.0, plot_top + 8.0),
        TooltipPosition::TopRight => (plot_right - width - 8.0, plot_top + 8.0),
        TooltipPosition::BottomLeft => (plot_left + 8.0, plot_bottom - height - 8.0),
        TooltipPosition::BottomRight => (plot_right - width - 8.0, plot_bottom - height - 8.0),
        TooltipPosition::Follow => {
            let mut box_x = x + 12.0;
            let mut box_y = y - height - 12.0;
            if box_x + width > plot_right {
                box_x = x - width - 12.0;
            }
            if box_y < plot_top {
                box_y = y + 12.0;
            }
            (box_x, box_y)
        }
        TooltipPosition::Auto => (plot_left + 8.0, plot_top + 8.0),
    }
}

pub(super) fn series_last_value(series: &Series, style: &ChartStyle) -> Option<(f64, Color)> {
    match &series.data {
        SeriesData::Candlestick { data } => {
            let candle = data.last()?;
            let up = candle.close >= candle.open;
            let color = if up { style.up } else { style.down };
            Some((candle.close, color))
        }
        SeriesData::Line { data } => {
            let point = data.last()?;
            Some((point.value, style.line))
        }
        SeriesData::Histogram { data } => {
            let point = data.last()?;
            let color = point.color.unwrap_or(style.histogram);
            Some((point.value, color))
        }
    }
}

pub(super) fn draw_rounded_rect(
    cr: &Context,
    x: f64,
    y: f64,
    width: f64,
    height: f64,
    radius: f64,
) {
    let r = radius.min(width * 0.5).min(height * 0.5).max(0.0);
    if r <= 0.0 {
        cr.rectangle(x, y, width, height);
        return;
    }
    let x0 = x;
    let y0 = y;
    let x1 = x + width;
    let y1 = y + height;

    cr.new_sub_path();
    cr.arc(x1 - r, y0 + r, r, -std::f64::consts::FRAC_PI_2, 0.0);
    cr.arc(x1 - r, y1 - r, r, 0.0, std::f64::consts::FRAC_PI_2);
    cr.arc(x0 + r, y1 - r, r, std::f64::consts::FRAC_PI_2, std::f64::consts::PI);
    cr.arc(x0 + r, y0 + r, r, std::f64::consts::PI, std::f64::consts::PI * 1.5);
    cr.close_path();
}

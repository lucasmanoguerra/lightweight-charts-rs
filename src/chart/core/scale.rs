use super::super::data::{SeriesData, SeriesScale};
use super::super::layout::ChartLayout;
use super::super::options::PriceScaleOptions;
use super::super::scales::{
    data_range_candles, data_range_line, merge_range, update_price_scale_state,
};
use super::super::types::{PriceFormat, PriceScale, PriceScaleMode};
use super::super::util::{
    candle_time, histogram_range, inverse_transform_price, transform_price, visible_candles,
    visible_histogram_points, visible_line_points,
};
use super::ChartCore;

impl ChartCore {
    pub(super) fn has_data(&self) -> bool {
        self.series.iter().any(|series| match &series.data {
            SeriesData::Candlestick { data } => !data.is_empty(),
            SeriesData::Line { data } => !data.is_empty(),
            SeriesData::Histogram { data } => !data.is_empty(),
        })
    }

    pub(super) fn scale_for_side(
        &mut self,
        side: PriceScale,
        start: f64,
        end: f64,
    ) -> Option<SeriesScale> {
        let (min, max) = self.data_range_for_side(side, start, end)?;
        let options = self.price_scale_options(side).clone();
        let base = self.base_value_for_side(side, start, end).unwrap_or(1.0);
        let state = match side {
            PriceScale::Left => &mut self.left_scale,
            PriceScale::Right => &mut self.right_scale,
        };
        update_price_scale_state(state, min, max, options.auto_scale, options.mode, base);
        Some(SeriesScale {
            min: state.view_min,
            max: state.view_max,
            mode: options.mode,
            base,
            invert: options.invert_scale,
            margins: options.scale_margins,
        })
    }

    pub(super) fn scale_for_rsi(&mut self, start: f64, end: f64) -> Option<SeriesScale> {
        if let Some(panel_id) = self.rsi_panel_id {
            if !self.panel_content_visible(panel_id) {
                return None;
            }
        }
        let panel = self.rsi_panel.as_mut()?;
        if panel.data.is_empty() {
            return None;
        }
        let base = 1.0;
        let range = data_range_line(&panel.data, start, end)?;
        let min = 0.0_f64.min(range.0);
        let max = 100.0_f64.max(range.1);
        update_price_scale_state(
            &mut panel.scale,
            min,
            max,
            panel.options.auto_scale,
            panel.options.mode,
            base,
        );
        Some(SeriesScale {
            min: panel.scale.view_min,
            max: panel.scale.view_max,
            mode: panel.options.mode,
            base,
            invert: panel.options.invert_scale,
            margins: panel.options.scale_margins,
        })
    }

    pub(super) fn data_range_for_side(
        &self,
        side: PriceScale,
        start: f64,
        end: f64,
    ) -> Option<(f64, f64)> {
        let mut range: Option<(f64, f64)> = None;

        for series in &self.series {
            if !self.panel_content_visible(series.panel_id) {
                continue;
            }
            if series.scale != side {
                continue;
            }

            match &series.data {
                SeriesData::Candlestick { data } => {
                    merge_range(&mut range, data_range_candles(data, start, end));
                }
                SeriesData::Line { data } => {
                    merge_range(&mut range, data_range_line(data, start, end));
                }
                SeriesData::Histogram { data } => {
                    merge_range(
                        &mut range,
                        histogram_range(&visible_histogram_points(data, start, end)),
                    );
                }
            }

            if series.options.markers_options.auto_scale {
                for marker in &series.markers {
                    let time = candle_time(marker.time);
                    if time < start || time > end {
                        continue;
                    }
                    if let Some(price) = marker.price {
                        merge_range(&mut range, Some((price, price)));
                    }
                }
            }
        }

        range
    }

    pub(super) fn price_scale_options(&self, side: PriceScale) -> &PriceScaleOptions {
        match side {
            PriceScale::Left => &self.options.left_price_scale,
            PriceScale::Right => &self.options.right_price_scale,
        }
    }

    pub(super) fn base_value_for_side(
        &self,
        side: PriceScale,
        start: f64,
        end: f64,
    ) -> Option<f64> {
        let mut best_time = f64::INFINITY;
        let mut best_value: Option<f64> = None;

        for series in &self.series {
            if !self.panel_content_visible(series.panel_id) {
                continue;
            }
            if series.scale != side {
                continue;
            }
            match &series.data {
                SeriesData::Candlestick { data } => {
                    for candle in visible_candles(data, start, end) {
                        let time = candle_time(candle.time);
                        if time < best_time {
                            best_time = time;
                            best_value = Some(candle.close);
                        }
                    }
                }
                SeriesData::Line { data } => {
                    for point in visible_line_points(data, start, end) {
                        let time = candle_time(point.time);
                        if time < best_time {
                            best_time = time;
                            best_value = Some(point.value);
                        }
                    }
                }
                SeriesData::Histogram { data } => {
                    for point in visible_histogram_points(data, start, end) {
                        let time = candle_time(point.time);
                        if time < best_time {
                            best_time = time;
                            best_value = Some(point.value);
                        }
                    }
                }
            }
        }

        best_value
    }

    pub(super) fn price_format_for_side(&self, side: PriceScale) -> PriceFormat {
        for series in &self.series {
            if !self.panel_content_visible(series.panel_id) {
                continue;
            }
            if series.scale == side {
                return series.options.price_format.clone();
            }
        }
        PriceFormat::default()
    }

    pub(super) fn pan_price_scale(&mut self, side: PriceScale, delta: f64) {
        let options = self.price_scale_options(side).clone();
        let base = self
            .base_value_for_side(side, self.time_scale.start, self.time_scale.end)
            .unwrap_or(1.0);
        let state = match side {
            PriceScale::Left => &mut self.left_scale,
            PriceScale::Right => &mut self.right_scale,
        };
        match options.mode {
            PriceScaleMode::Logarithmic => {
                let t_min = transform_price(state.view_min, options.mode, base);
                let t_max = transform_price(state.view_max, options.mode, base);
                let t_min = t_min + delta;
                let t_max = t_max + delta;
                state.view_min = inverse_transform_price(t_min, options.mode, base);
                state.view_max = inverse_transform_price(t_max, options.mode, base);
            }
            PriceScaleMode::Percentage => {
                let delta_raw = if base.abs() < f64::EPSILON {
                    0.0
                } else {
                    delta * base / 100.0
                };
                state.view_min += delta_raw;
                state.view_max += delta_raw;
            }
            PriceScaleMode::IndexedTo100 => {
                let delta_raw = if base.abs() < f64::EPSILON {
                    0.0
                } else {
                    delta * base / 100.0
                };
                state.view_min += delta_raw;
                state.view_max += delta_raw;
            }
            PriceScaleMode::Normal => {
                state.view_min += delta;
                state.view_max += delta;
            }
        }
        state.auto = false;
        match side {
            PriceScale::Left => self.options.left_price_scale.auto_scale = false,
            PriceScale::Right => self.options.right_price_scale.auto_scale = false,
        }
    }

    pub(super) fn zoom_price_scale(&mut self, side: PriceScale, factor: f64, anchor: f64) {
        let options = self.price_scale_options(side).clone();
        let base = self
            .base_value_for_side(side, self.time_scale.start, self.time_scale.end)
            .unwrap_or(1.0);
        let state = match side {
            PriceScale::Left => &mut self.left_scale,
            PriceScale::Right => &mut self.right_scale,
        };
        let t_min = transform_price(state.view_min, options.mode, base);
        let t_max = transform_price(state.view_max, options.mode, base);
        let range = (t_max - t_min).max(1e-9);
        let anchor_value = t_max - anchor * range;
        let new_range = (range * factor).max(1e-9);
        let t_max = anchor_value + anchor * new_range;
        let t_min = t_max - new_range;
        state.view_min = inverse_transform_price(t_min, options.mode, base);
        state.view_max = inverse_transform_price(t_max, options.mode, base);
        state.auto = false;
        match side {
            PriceScale::Left => self.options.left_price_scale.auto_scale = false,
            PriceScale::Right => self.options.right_price_scale.auto_scale = false,
        }
    }

    pub(super) fn pan_rsi_scale(&mut self, delta: f64) {
        let panel = match self.rsi_panel.as_mut() {
            Some(panel) => panel,
            None => return,
        };
        let options = panel.options.clone();
        let base = 1.0;
        let state = &mut panel.scale;
        match options.mode {
            PriceScaleMode::Logarithmic => {
                let t_min = transform_price(state.view_min, options.mode, base);
                let t_max = transform_price(state.view_max, options.mode, base);
                let t_min = t_min + delta;
                let t_max = t_max + delta;
                state.view_min = inverse_transform_price(t_min, options.mode, base);
                state.view_max = inverse_transform_price(t_max, options.mode, base);
            }
            PriceScaleMode::Percentage | PriceScaleMode::IndexedTo100 => {
                let delta_raw = delta * base / 100.0;
                state.view_min += delta_raw;
                state.view_max += delta_raw;
            }
            PriceScaleMode::Normal => {
                state.view_min += delta;
                state.view_max += delta;
            }
        }
        state.auto = false;
        panel.options.auto_scale = false;
    }

    pub(super) fn zoom_rsi_scale(&mut self, factor: f64, anchor: f64) {
        let panel = match self.rsi_panel.as_mut() {
            Some(panel) => panel,
            None => return,
        };
        let options = panel.options.clone();
        let base = 1.0;
        let state = &mut panel.scale;
        let t_min = transform_price(state.view_min, options.mode, base);
        let t_max = transform_price(state.view_max, options.mode, base);
        let range = (t_max - t_min).max(1e-9);
        let anchor_value = t_max - anchor * range;
        let new_range = (range * factor).max(1e-9);
        let t_max = anchor_value + anchor * new_range;
        let t_min = t_max - new_range;
        state.view_min = inverse_transform_price(t_min, options.mode, base);
        state.view_max = inverse_transform_price(t_max, options.mode, base);
        state.auto = false;
        panel.options.auto_scale = false;
    }

    pub(super) fn rsi_price_range(&self) -> f64 {
        let panel = match self.rsi_panel.as_ref() {
            Some(panel) => panel,
            None => return 1.0,
        };
        let options = &panel.options;
        let base = 1.0;
        let t_min = transform_price(panel.scale.view_min, options.mode, base);
        let t_max = transform_price(panel.scale.view_max, options.mode, base);
        (t_max - t_min).abs().max(1e-9)
    }

    pub(super) fn price_range_for_side(&self, side: PriceScale) -> f64 {
        let state = match side {
            PriceScale::Left => &self.left_scale,
            PriceScale::Right => &self.right_scale,
        };
        let options = self.price_scale_options(side);
        let base = self
            .base_value_for_side(side, self.time_scale.start, self.time_scale.end)
            .unwrap_or(1.0);
        let t_min = transform_price(state.view_min, options.mode, base);
        let t_max = transform_price(state.view_max, options.mode, base);
        (t_max - t_min).abs().max(1e-9)
    }

    pub(super) fn side_for_position(&self, x: f64, layout: &ChartLayout) -> PriceScale {
        if self.options.left_price_scale.visible && layout.in_left_axis(x) {
            return PriceScale::Left;
        }
        if self.options.right_price_scale.visible && layout.in_right_axis(x) {
            return PriceScale::Right;
        }

        if self.options.left_price_scale.visible && self.options.right_price_scale.visible {
            let mid = (layout.plot_left + layout.plot_right) * 0.5;
            if x < mid {
                PriceScale::Left
            } else {
                PriceScale::Right
            }
        } else if self.options.right_price_scale.visible {
            PriceScale::Right
        } else {
            PriceScale::Left
        }
    }
}

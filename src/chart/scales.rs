use super::data::{PriceScaleState, Series, SeriesData, SeriesKind};
use super::types::{Candle, LinePoint};
use super::util::{candle_time, expand_range};

#[derive(Clone, Copy, Debug)]
pub(crate) struct TimeScale {
    pub(crate) min: f64,
    pub(crate) max: f64,
    pub(crate) start: f64,
    pub(crate) end: f64,
    pub(crate) bar_spacing: f64,
    pub(crate) min_bar_spacing: f64,
    pub(crate) max_bar_spacing: f64,
    pub(crate) fix_left_edge: bool,
    pub(crate) fix_right_edge: bool,
    pub(crate) right_offset: f64,
    pub(crate) right_offset_pixels: f64,
    bar_time: f64,
}

impl Default for TimeScale {
    fn default() -> Self {
        Self {
            min: 0.0,
            max: 1.0,
            start: 0.0,
            end: 1.0,
            bar_spacing: 6.0,
            min_bar_spacing: 0.5,
            max_bar_spacing: 0.0,
            fix_left_edge: false,
            fix_right_edge: false,
            right_offset: 0.0,
            right_offset_pixels: 0.0,
            bar_time: 1.0,
        }
    }
}

impl TimeScale {
    pub(crate) fn recalculate(&mut self, series: &[Series]) {
        let mut min = f64::INFINITY;
        let mut max = f64::NEG_INFINITY;
        let mut has_data = false;
        let mut times: Vec<f64> = Vec::new();

        for item in series {
            match item.kind {
                SeriesKind::Candlestick => {
                    if let SeriesData::Candlestick { data } = &item.data {
                        for candle in data {
                            let time = candle_time(candle.time);
                            min = min.min(time);
                            max = max.max(time);
                            has_data = true;
                            times.push(time);
                        }
                    }
                }
                SeriesKind::Line => {
                    if let SeriesData::Line { data } = &item.data {
                        for point in data {
                            let time = candle_time(point.time);
                            min = min.min(time);
                            max = max.max(time);
                            has_data = true;
                            times.push(time);
                        }
                    }
                }
                SeriesKind::Histogram => {
                    if let SeriesData::Histogram { data } = &item.data {
                        for point in data {
                            let time = candle_time(point.time);
                            min = min.min(time);
                            max = max.max(time);
                            has_data = true;
                            times.push(time);
                        }
                    }
                }
            }
        }

        if !has_data {
            *self = Self::default();
            return;
        }

        if (max - min).abs() < f64::EPSILON {
            max += 1.0;
        }

        self.bar_time = average_bar_time(&mut times);
        self.min = min;
        self.max = max;
        self.start = min;
        self.end = max + self.effective_right_offset() * self.bar_time;
    }

    pub(crate) fn visible_range(&self) -> f64 {
        (self.end - self.start).max(1.0)
    }

    pub(crate) fn pan_by(&mut self, delta: f64) {
        let range = self.visible_range();
        let mut start = self.start + delta;
        let mut end = self.end + delta;
        let max_end = self.max + self.effective_right_offset() * self.bar_time();
        let (min_limit, max_limit) = self.pan_limits(range, max_end);

        if start < min_limit {
            start = min_limit;
            end = start + range;
        }

        if end > max_limit {
            end = max_limit;
            start = end - range;
        }

        self.start = start;
        self.end = end;
    }

    pub(crate) fn zoom_by(&mut self, factor: f64, anchor: f64) {
        let range = self.visible_range();
        let max_end = self.max + self.effective_right_offset() * self.bar_time();
        let max_range = (max_end - self.min).max(1.0);
        let min_range = (max_range / 200.0).max(1.0);

        let new_range = (range * factor).clamp(min_range, max_range);
        let anchor_time = self.start + anchor * range;
        let mut start = anchor_time - anchor * new_range;
        let mut end = start + new_range;

        let (min_limit, max_limit) = self.pan_limits(new_range, max_end);
        if start < min_limit {
            start = min_limit;
            end = start + new_range;
        }

        if end > max_limit {
            end = max_limit;
            start = end - new_range;
        }

        self.start = start;
        self.end = end;
    }

    pub(crate) fn bar_time(&self) -> f64 {
        self.bar_time.max(1.0)
    }

    pub(crate) fn set_right_offset(&mut self, offset: f64) {
        self.right_offset = offset.max(0.0);
        self.right_offset_pixels = 0.0;
    }

    pub(crate) fn set_right_offset_pixels(&mut self, pixels: f64) {
        self.right_offset_pixels = pixels.max(0.0);
    }

    pub(crate) fn set_bar_spacing(&mut self, spacing: f64) {
        let mut value = spacing.max(self.min_bar_spacing);
        if self.max_bar_spacing > 0.0 {
            value = value.min(self.max_bar_spacing);
        }
        self.bar_spacing = value;
    }

    pub(crate) fn set_min_bar_spacing(&mut self, value: f64) {
        self.min_bar_spacing = value.max(0.1);
        self.set_bar_spacing(self.bar_spacing);
    }

    pub(crate) fn set_max_bar_spacing(&mut self, value: f64) {
        self.max_bar_spacing = value.max(0.0);
        self.set_bar_spacing(self.bar_spacing);
    }

    pub(crate) fn set_fix_left_edge(&mut self, enabled: bool) {
        self.fix_left_edge = enabled;
    }

    pub(crate) fn set_fix_right_edge(&mut self, enabled: bool) {
        self.fix_right_edge = enabled;
    }

    pub(crate) fn pan_limits(&self, range: f64, max_end: f64) -> (f64, f64) {
        let extra = range * 0.25;
        let min_limit = if self.fix_left_edge {
            self.min
        } else {
            self.min - extra
        };
        let max_limit = if self.fix_right_edge {
            max_end
        } else {
            max_end + extra
        };
        (min_limit, max_limit)
    }

    pub(crate) fn effective_right_offset(&self) -> f64 {
        if self.right_offset_pixels > 0.0 {
            (self.right_offset_pixels / self.bar_spacing.max(1.0)).max(0.0)
        } else {
            self.right_offset
        }
    }
}

pub(crate) fn update_price_scale_state(
    state: &mut PriceScaleState,
    min: f64,
    max: f64,
    auto_scale: bool,
    mode: super::types::PriceScaleMode,
    base: f64,
) {
    let (auto_min, auto_max) = match mode {
        super::types::PriceScaleMode::Logarithmic => {
            let t_min = super::util::transform_price(min, mode, base);
            let t_max = super::util::transform_price(max, mode, base);
            let (t_auto_min, t_auto_max) = expand_range(t_min, t_max);
            (
                super::util::inverse_transform_price(t_auto_min, mode, base),
                super::util::inverse_transform_price(t_auto_max, mode, base),
            )
        }
        _ => expand_range(min, max),
    };

    state.data_min = min;
    state.data_max = max;
    if auto_scale {
        state.view_min = auto_min;
        state.view_max = auto_max;
        state.auto = true;
    } else {
        state.auto = false;
    }
}

fn average_bar_time(times: &mut Vec<f64>) -> f64 {
    if times.len() < 2 {
        return 1.0;
    }
    times.sort_by(|a, b| a.partial_cmp(b).unwrap_or(std::cmp::Ordering::Equal));
    let mut total = 0.0;
    let mut count = 0.0;
    for window in times.windows(2) {
        let delta = (window[1] - window[0]).abs();
        if delta > f64::EPSILON {
            total += delta;
            count += 1.0;
        }
    }
    if count == 0.0 {
        1.0
    } else {
        (total / count).max(1.0)
    }
}

pub(crate) fn data_range_candles(data: &[Candle], start: f64, end: f64) -> Option<(f64, f64)> {
    let mut min = f64::INFINITY;
    let mut max = f64::NEG_INFINITY;
    let mut has = false;

    for candle in data {
        let time = candle_time(candle.time);
        if time >= start && time <= end {
            min = min.min(candle.low);
            max = max.max(candle.high);
            has = true;
        }
    }

    if has {
        Some((min, max))
    } else {
        None
    }
}

pub(crate) fn data_range_line(data: &[LinePoint], start: f64, end: f64) -> Option<(f64, f64)> {
    let mut min = f64::INFINITY;
    let mut max = f64::NEG_INFINITY;
    let mut has = false;

    for point in data {
        let time = candle_time(point.time);
        if time >= start && time <= end {
            min = min.min(point.value);
            max = max.max(point.value);
            has = true;
        }
    }

    if has {
        Some((min, max))
    } else {
        None
    }
}

pub(crate) fn merge_range(current: &mut Option<(f64, f64)>, next: Option<(f64, f64)>) {
    if let Some((min, max)) = next {
        match current {
            Some((cmin, cmax)) => {
                *cmin = (*cmin).min(min);
                *cmax = (*cmax).max(max);
            }
            None => {
                *current = Some((min, max));
            }
        }
    }
}

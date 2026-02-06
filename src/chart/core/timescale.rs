use super::super::options::TimeScaleOptions;
use super::super::types::Color;
use super::ChartCore;

impl ChartCore {
    pub(crate) fn fit_content(&mut self) {
        self.time_scale.recalculate(&self.series);
        if self.last_plot_width > 0.0 {
            let bar_time = self.time_scale.bar_time();
            let total_bars = ((self.time_scale.max - self.time_scale.min) / bar_time).max(1.0);
            let spacing = (self.last_plot_width / total_bars).clamp(
                self.time_scale.min_bar_spacing,
                if self.time_scale.max_bar_spacing > 0.0 {
                    self.time_scale.max_bar_spacing
                } else {
                    40.0
                },
            );
            self.time_scale.set_bar_spacing(spacing);
        }
        self.apply_bar_spacing_with_anchor(1.0);
    }

    pub(crate) fn set_time_scale_right_offset(&mut self, offset: f64) {
        self.options.time_scale.right_offset = offset;
        self.options.time_scale.right_offset_pixels = 0.0;
        self.time_scale.set_right_offset(offset);
        self.fit_content();
    }

    pub(crate) fn set_time_scale_right_offset_pixels(&mut self, pixels: f64) {
        self.options.time_scale.right_offset_pixels = pixels;
        self.options.time_scale.right_offset = 0.0;
        self.time_scale.set_right_offset_pixels(pixels);
        self.fit_content();
    }

    pub(crate) fn set_time_scale_bar_spacing(&mut self, spacing: f64) {
        self.options.time_scale.bar_spacing = spacing;
        self.time_scale.set_bar_spacing(spacing);
        self.apply_bar_spacing();
    }

    pub(crate) fn set_time_scale_min_bar_spacing(&mut self, spacing: f64) {
        self.options.time_scale.min_bar_spacing = spacing;
        self.time_scale.set_min_bar_spacing(spacing);
        self.apply_bar_spacing();
    }

    pub(crate) fn set_time_scale_max_bar_spacing(&mut self, spacing: f64) {
        self.options.time_scale.max_bar_spacing = spacing;
        self.time_scale.set_max_bar_spacing(spacing);
        self.apply_bar_spacing();
    }

    pub(crate) fn set_time_scale_fix_left_edge(&mut self, enabled: bool) {
        self.options.time_scale.fix_left_edge = enabled;
        self.time_scale.set_fix_left_edge(enabled);
        self.apply_bar_spacing();
    }

    pub(crate) fn set_time_scale_fix_right_edge(&mut self, enabled: bool) {
        self.options.time_scale.fix_right_edge = enabled;
        self.time_scale.set_fix_right_edge(enabled);
        self.apply_bar_spacing();
    }

    pub(crate) fn set_time_scale_visible(&mut self, visible: bool) {
        self.options.time_scale.visible = visible;
    }

    pub(crate) fn set_time_scale_border(&mut self, visible: bool, color: Color) {
        self.options.time_scale.border_visible = visible;
        self.options.time_scale.border_color = color;
    }

    pub(crate) fn set_time_scale_ticks_visible(&mut self, visible: bool) {
        self.options.time_scale.ticks_visible = visible;
    }

    pub(crate) fn set_time_scale_time_visible(&mut self, visible: bool) {
        self.options.time_scale.time_visible = visible;
    }

    pub(crate) fn set_time_scale_seconds_visible(&mut self, visible: bool) {
        self.options.time_scale.seconds_visible = visible;
    }

    pub(crate) fn set_time_scale_tick_mark_format(&mut self, format: String) {
        self.options.time_scale.tick_mark_format = format;
    }

    pub(crate) fn set_time_scale_tick_mark_max_len(&mut self, len: usize) {
        self.options.time_scale.tick_mark_max_character_length = len;
    }

    pub(crate) fn set_time_scale_lock_visible_time_range_on_resize(&mut self, enabled: bool) {
        self.options.time_scale.lock_visible_time_range_on_resize = enabled;
    }

    pub(crate) fn set_time_scale_right_bar_stays_on_scroll(&mut self, enabled: bool) {
        self.options.time_scale.right_bar_stays_on_scroll = enabled;
    }

    pub(crate) fn set_time_scale_shift_visible_range_on_new_bar(&mut self, enabled: bool) {
        self.options.time_scale.shift_visible_range_on_new_bar = enabled;
    }

    pub(crate) fn set_time_scale_minimum_height(&mut self, height: f64) {
        self.options.time_scale.minimum_height = height.max(0.0);
    }

    pub(crate) fn set_time_scale_uniform_distribution(&mut self, enabled: bool) {
        self.options.time_scale.uniform_distribution = enabled;
    }

    pub(crate) fn apply_time_scale_options(&mut self, options: TimeScaleOptions) {
        self.options.time_scale = options.clone();
        self.time_scale.set_min_bar_spacing(options.min_bar_spacing);
        self.time_scale.set_max_bar_spacing(options.max_bar_spacing);
        self.time_scale.set_bar_spacing(options.bar_spacing);
        self.time_scale.set_fix_left_edge(options.fix_left_edge);
        self.time_scale.set_fix_right_edge(options.fix_right_edge);
        if options.right_offset_pixels > 0.0 {
            self.time_scale
                .set_right_offset_pixels(options.right_offset_pixels);
        } else {
            self.time_scale.set_right_offset(options.right_offset);
        }
        self.apply_bar_spacing();
    }

    pub(super) fn recalculate_time_scale_after_data_update(&mut self) {
        let prev_end = self.time_scale.end;
        let prev_range = self.time_scale.visible_range();
        let prev_max_end = self.time_scale.max
            + self.time_scale.effective_right_offset() * self.time_scale.bar_time();
        let was_at_right = (prev_end - prev_max_end).abs() <= self.time_scale.bar_time().max(1.0);

        self.time_scale.recalculate(&self.series);

        if self.last_plot_width <= 0.0 {
            return;
        }

        let max_end = self.time_scale.max
            + self.time_scale.effective_right_offset() * self.time_scale.bar_time();
        let range = prev_range.max(1.0);
        let mut end = if was_at_right
            && self.options.time_scale.right_bar_stays_on_scroll
            && self.options.time_scale.shift_visible_range_on_new_bar
        {
            max_end
        } else {
            prev_end
        };
        let mut start = end - range;
        let (min_limit, max_limit) = self.time_scale.pan_limits(range, max_end);
        if start < min_limit {
            start = min_limit;
            end = start + range;
        }
        if end > max_limit {
            end = max_limit;
            start = end - range;
        }
        self.time_scale.start = start;
        self.time_scale.end = end.max(start + 1.0);
    }

    pub(super) fn apply_bar_spacing(&mut self) {
        self.apply_bar_spacing_with_anchor(1.0);
    }

    pub(super) fn apply_bar_spacing_with_anchor(&mut self, anchor: f64) {
        if self.last_plot_width <= 0.0 {
            return;
        }
        let bar_spacing = self
            .time_scale
            .bar_spacing
            .max(self.time_scale.min_bar_spacing);
        let visible_bars = (self.last_plot_width / bar_spacing).max(1.0);
        let range = self.time_scale.bar_time() * visible_bars;
        let max_end = self.time_scale.max
            + self.time_scale.effective_right_offset() * self.time_scale.bar_time();
        let old_range = self.time_scale.visible_range();
        let anchor_time = self.time_scale.start + anchor.clamp(0.0, 1.0) * old_range;
        let mut start = anchor_time - anchor.clamp(0.0, 1.0) * range;
        let mut end = start + range;
        let (min_limit, max_limit) = self.time_scale.pan_limits(range, max_end);
        if start < min_limit {
            start = min_limit;
            end = start + range;
        }
        if end > max_limit {
            end = max_limit;
            start = end - range;
        }
        self.time_scale.start = start;
        self.time_scale.end = end.max(start + 1.0);
    }

    pub(super) fn zoom_time_by_factor(&mut self, factor: f64, anchor: f64) {
        let min_spacing = self.time_scale.min_bar_spacing.max(0.1);
        let max_spacing = if self.time_scale.max_bar_spacing > 0.0 {
            self.time_scale.max_bar_spacing
        } else {
            200.0
        };
        let new_spacing = (self.time_scale.bar_spacing / factor).clamp(min_spacing, max_spacing);
        self.time_scale.set_bar_spacing(new_spacing);
        self.apply_bar_spacing_with_anchor(anchor);
    }

    pub(super) fn clamp_time_scale_right_edge(&mut self) {
        if !self.options.time_scale.right_bar_stays_on_scroll {
            return;
        }
        let max_end = self.time_scale.max
            + self.time_scale.effective_right_offset() * self.time_scale.bar_time();
        let range = self.time_scale.visible_range();
        if self.time_scale.end > max_end {
            self.time_scale.end = max_end;
            self.time_scale.start = max_end - range;
        }
    }
}

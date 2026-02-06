use super::super::layout::ChartLayout;
use super::super::types::{PanResult, PanelResizeHandle, PriceScale};
use super::ChartCore;

impl ChartCore {
    pub(crate) fn set_crosshair(&mut self, x: f64, y: f64) {
        self.crosshair = Some((x, y));
    }

    pub(crate) fn clear_crosshair(&mut self) {
        self.crosshair = None;
    }

    pub(crate) fn pan_by_pixels(
        &mut self,
        dx: f64,
        dy: f64,
        width: f64,
        height: f64,
        x: f64,
        y: f64,
    ) -> PanResult {
        if self.tracking_mode_active {
            return PanResult {
                price_axis_zoomed: None,
                time_panned: false,
            };
        }
        let layout = ChartLayout::new(self, width, height);
        if layout.plot_width <= 0.0 || layout.plot_height <= 0.0 {
            return PanResult {
                price_axis_zoomed: None,
                time_panned: false,
            };
        }

        let mut time_panned = false;
        let in_rsi = layout.in_rsi_plot(y);
        let in_time_axis = layout.in_time_axis(y);
        if in_time_axis
            && self.options.handle_scale.axis_pressed_mouse_move_time
            && dx.abs() > f64::EPSILON
        {
            let anchor = ((x - layout.plot_left) / layout.plot_width).clamp(0.0, 1.0);
            let sensitivity = self.options.interaction_sensitivity.axis_drag_time;
            let factor = (1.0_f64 + sensitivity.max(0.0001)).powf(-dx);
            self.zoom_time_by_factor(factor, anchor);
            self.clamp_time_scale_right_edge();
            return PanResult {
                price_axis_zoomed: None,
                time_panned: false,
            };
        }
        if layout.in_axis(x)
            && self.options.handle_scale.axis_pressed_mouse_move_price
            && dy.abs() > f64::EPSILON
        {
            let side = self.side_for_position(x, &layout);
            let anchor = if in_rsi {
                ((y - layout.rsi_top) / layout.rsi_height.max(1.0)).clamp(0.0, 1.0)
            } else {
                ((y - layout.plot_top) / layout.main_height).clamp(0.0, 1.0)
            };
            let sensitivity = self.options.interaction_sensitivity.axis_drag_price;
            let factor = (1.0_f64 + sensitivity.max(0.0001)).powf(dy);
            if in_rsi {
                self.zoom_rsi_scale(factor, anchor);
                return PanResult {
                    price_axis_zoomed: None,
                    time_panned: false,
                };
            }
            self.zoom_price_scale(side, factor, anchor);
            return PanResult {
                price_axis_zoomed: Some(side),
                time_panned: false,
            };
        }

        if self.options.handle_scroll.pressed_mouse_move
            && dx.abs() > f64::EPSILON
            && !(in_time_axis && self.options.handle_scale.axis_pressed_mouse_move_time)
        {
            let range = self.time_scale.visible_range();
            let delta_time = -dx / layout.plot_width * range;
            self.time_scale.pan_by(delta_time);
            self.clamp_time_scale_right_edge();
            time_panned = true;
        }

        if dy.abs() > f64::EPSILON && self.options.handle_scroll.pressed_mouse_move {
            if self.options.handle_scroll.vert_touch_drag {
                if in_rsi {
                    let delta_price = dy / layout.rsi_height.max(1.0) * self.rsi_price_range();
                    self.pan_rsi_scale(delta_price);
                } else {
                    let side = self.side_for_position(x, &layout);
                    let delta_price = dy / layout.main_height * self.price_range_for_side(side);
                    self.pan_price_scale(side, delta_price);
                }
            }
        }

        PanResult {
            price_axis_zoomed: None,
            time_panned,
        }
    }

    pub(crate) fn pan_by_pixels_touch(
        &mut self,
        dx: f64,
        dy: f64,
        width: f64,
        height: f64,
        x: f64,
        y: f64,
    ) -> bool {
        if self.tracking_mode_active {
            return false;
        }
        let layout = ChartLayout::new(self, width, height);
        if layout.plot_width <= 0.0 || layout.plot_height <= 0.0 {
            return false;
        }

        let mut time_panned = false;
        if self.options.handle_scroll.horz_touch_drag && dx.abs() > f64::EPSILON {
            let range = self.time_scale.visible_range();
            let delta_time = -dx / layout.plot_width * range;
            self.time_scale.pan_by(delta_time);
            self.clamp_time_scale_right_edge();
            time_panned = true;
        }

        if self.options.handle_scroll.vert_touch_drag && dy.abs() > f64::EPSILON {
            if layout.in_rsi_plot(y) {
                let delta_price = dy / layout.rsi_height.max(1.0) * self.rsi_price_range();
                self.pan_rsi_scale(delta_price);
            } else {
                let side = self.side_for_position(x, &layout);
                let delta_price = dy / layout.main_height * self.price_range_for_side(side);
                self.pan_price_scale(side, delta_price);
            }
        }
        time_panned
    }

    pub(crate) fn zoom_by_delta(
        &mut self,
        delta: f64,
        x: f64,
        y: f64,
        width: f64,
        height: f64,
    ) -> Option<PriceScale> {
        if self.tracking_mode_active {
            return None;
        }
        let layout = ChartLayout::new(self, width, height);
        if layout.plot_width <= 0.0 || layout.plot_height <= 0.0 {
            return None;
        }
        self.last_plot_width = layout.plot_width;

        if !self.options.handle_scale.mouse_wheel {
            if !self.options.handle_scroll.mouse_wheel {
                return None;
            }
            let range = self.time_scale.visible_range();
            let wheel_pixels = delta * 40.0;
            let delta_time = -wheel_pixels / layout.plot_width * range;
            self.time_scale.pan_by(delta_time);
            self.clamp_time_scale_right_edge();
            return None;
        }

        let smooth: f64 = self.options.interaction_sensitivity.wheel_zoom.max(0.0001);
        let factor = (1.0_f64 + smooth).powf(delta).clamp(0.3, 3.0);

        if layout.in_time_axis(y) {
            let anchor = ((x - layout.plot_left) / layout.plot_width).clamp(0.0, 1.0);
            self.zoom_time_by_factor(factor, anchor);
            self.clamp_time_scale_right_edge();
            return None;
        }

        if layout.in_left_axis(x) {
            let in_rsi = layout.in_rsi_plot(y);
            let anchor = if in_rsi {
                ((y - layout.rsi_top) / layout.rsi_height.max(1.0)).clamp(0.0, 1.0)
            } else {
                ((y - layout.plot_top) / layout.main_height).clamp(0.0, 1.0)
            };
            if in_rsi {
                self.zoom_rsi_scale(factor, anchor);
                return None;
            }
            self.zoom_price_scale(PriceScale::Left, factor, anchor);
            return Some(PriceScale::Left);
        }

        if layout.in_right_axis(x) {
            let in_rsi = layout.in_rsi_plot(y);
            let anchor = if in_rsi {
                ((y - layout.rsi_top) / layout.rsi_height.max(1.0)).clamp(0.0, 1.0)
            } else {
                ((y - layout.plot_top) / layout.main_height).clamp(0.0, 1.0)
            };
            if in_rsi {
                self.zoom_rsi_scale(factor, anchor);
                return None;
            }
            self.zoom_price_scale(PriceScale::Right, factor, anchor);
            return Some(PriceScale::Right);
        }

        let anchor = ((x - layout.plot_left) / layout.plot_width).clamp(0.0, 1.0);
        self.zoom_time_by_factor(factor, anchor);
        self.clamp_time_scale_right_edge();
        None
    }

    pub(crate) fn zoom_by_delta_pinch(
        &mut self,
        delta: f64,
        x: f64,
        y: f64,
        width: f64,
        height: f64,
    ) {
        if self.tracking_mode_active {
            return;
        }
        let layout = ChartLayout::new(self, width, height);
        if layout.plot_width <= 0.0 || layout.plot_height <= 0.0 {
            return;
        }
        self.last_plot_width = layout.plot_width;

        if !self.options.handle_scale.pinch {
            return;
        }

        let smooth: f64 = self.options.interaction_sensitivity.pinch_zoom.max(0.0001);
        let factor = (1.0_f64 + smooth).powf(delta).clamp(0.3, 3.0);

        if layout.in_time_axis(y) {
            let anchor = ((x - layout.plot_left) / layout.plot_width).clamp(0.0, 1.0);
            self.zoom_time_by_factor(factor, anchor);
            self.clamp_time_scale_right_edge();
            return;
        }

        if layout.in_left_axis(x) {
            let in_rsi = layout.in_rsi_plot(y);
            let anchor = if in_rsi {
                ((y - layout.rsi_top) / layout.rsi_height.max(1.0)).clamp(0.0, 1.0)
            } else {
                ((y - layout.plot_top) / layout.main_height).clamp(0.0, 1.0)
            };
            if in_rsi {
                self.zoom_rsi_scale(factor, anchor);
                return;
            }
            self.zoom_price_scale(PriceScale::Left, factor, anchor);
            return;
        }

        if layout.in_right_axis(x) {
            let in_rsi = layout.in_rsi_plot(y);
            let anchor = if in_rsi {
                ((y - layout.rsi_top) / layout.rsi_height.max(1.0)).clamp(0.0, 1.0)
            } else {
                ((y - layout.plot_top) / layout.main_height).clamp(0.0, 1.0)
            };
            if in_rsi {
                self.zoom_rsi_scale(factor, anchor);
                return;
            }
            self.zoom_price_scale(PriceScale::Right, factor, anchor);
            return;
        }

        let anchor = ((x - layout.plot_left) / layout.plot_width).clamp(0.0, 1.0);
        self.zoom_time_by_factor(factor, anchor);
        self.clamp_time_scale_right_edge();
    }

    pub(crate) fn handle_double_click(&mut self, x: f64, y: f64, width: f64, height: f64) {
        let layout = ChartLayout::new(self, width, height);
        if layout.plot_width <= 0.0 || layout.plot_height <= 0.0 {
            return;
        }

        if layout.in_time_axis(y) && self.options.handle_scale.axis_double_click_reset_time {
            self.fit_content();
        }

        if layout.in_left_axis(x) && self.options.handle_scale.axis_double_click_reset_price {
            self.reset_autoscale(PriceScale::Left);
        }
        if layout.in_right_axis(x) && self.options.handle_scale.axis_double_click_reset_price {
            self.reset_autoscale(PriceScale::Right);
        }
    }

    pub(crate) fn panel_resize_handle_at(
        &self,
        y: f64,
        width: f64,
        height: f64,
    ) -> Option<PanelResizeHandle> {
        let layout = ChartLayout::new(self, width, height);
        let threshold = 4.0;
        for pair in layout.panels.windows(2) {
            let upper = pair[0];
            let lower = pair[1];
            if upper.group_id != lower.group_id {
                continue;
            }
            let boundary = upper.bottom;
            if (y - boundary).abs() <= threshold {
                return Some(PanelResizeHandle {
                    upper: upper.id,
                    lower: lower.id,
                });
            }
        }
        None
    }

    pub(crate) fn resize_panels_by_pixels(
        &mut self,
        handle: PanelResizeHandle,
        delta_y: f64,
        _width: f64,
        height: f64,
    ) {
        if delta_y.abs() <= f64::EPSILON {
            return;
        }
        let unit_height = match self.panel_unit_height(height) {
            Some(value) if value > 0.0 => value,
            _ => return,
        };

        let upper_index = self.panels.iter().position(|p| p.id == handle.upper);
        let lower_index = self.panels.iter().position(|p| p.id == handle.lower);
        let (upper_index, lower_index) = match (upper_index, lower_index) {
            (Some(u), Some(l)) => (u, l),
            _ => return,
        };
        if self.panels[upper_index].collapsed || self.panels[lower_index].collapsed {
            return;
        }

        let min_pixels = self.style.panel_toolbar_height.max(24.0);
        let min_weight = (min_pixels / unit_height).max(0.1);
        let mut delta_weight = delta_y / unit_height;

        let upper_weight = self.panels[upper_index].height_weight.max(0.1);
        let lower_weight = self.panels[lower_index].height_weight.max(0.1);

        let upper_new = upper_weight + delta_weight;
        let lower_new = lower_weight - delta_weight;

        if upper_new < min_weight {
            delta_weight = min_weight - upper_weight;
        }
        if lower_new < min_weight {
            delta_weight = lower_weight - min_weight;
        }

        if upper_index == lower_index {
            return;
        }
        if upper_index < lower_index {
            let (left, right) = self.panels.split_at_mut(lower_index);
            let upper = &mut left[upper_index];
            let lower = &mut right[0];
            upper.height_weight = (upper.height_weight + delta_weight).max(0.1);
            lower.height_weight = (lower.height_weight - delta_weight).max(0.1);
        } else {
            let (left, right) = self.panels.split_at_mut(upper_index);
            let lower = &mut left[lower_index];
            let upper = &mut right[0];
            upper.height_weight = (upper.height_weight + delta_weight).max(0.1);
            lower.height_weight = (lower.height_weight - delta_weight).max(0.1);
        }
    }

    fn panel_unit_height(&self, height: f64) -> Option<f64> {
        let time_axis_height = if self.options.time_scale.visible {
            self.style
                .axis_height
                .max(self.options.time_scale.minimum_height)
        } else {
            0.0
        };
        let collapsed_height = self.style.panel_toolbar_height.max(18.0);
        let collapsed_total =
            self.panels.iter().filter(|panel| panel.collapsed).count() as f64 * collapsed_height;
        let total_weight: f64 = self
            .panels
            .iter()
            .filter(|panel| !panel.collapsed)
            .map(|panel| panel.height_weight.max(0.1))
            .sum();
        if total_weight <= 0.0 {
            return None;
        }
        let time_axes_count = self.time_scales.len();
        let available_height = (height
            - self.style.padding * 2.0
            - time_axis_height * time_axes_count as f64
            - collapsed_total)
            .max(1.0);
        Some(available_height / total_weight)
    }
}

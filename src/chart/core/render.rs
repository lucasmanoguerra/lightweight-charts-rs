use cairo::Context;

use super::ChartCore;
use super::render_helpers::{
    aligned_price_ticks, build_ticks_for_scale, primary_candle_scale, primary_candles,
};
use super::super::data::SeriesData;
use super::super::layout::ChartLayout;
use super::super::ticks::build_time_ticks;
use super::super::types::{MarkerZOrder, PriceFormat, PriceScale};
use super::super::util::{
    candle_time, histogram_range, map_price_to_y, map_price_to_y_scaled, map_time_to_x,
    series_bar_width_times, visible_candles, visible_histogram_points, visible_line_points,
};

impl ChartCore {
    pub(crate) fn draw(&mut self, cr: &Context, width: f64, height: f64) {
        self.draw_background(cr, width, height);
        self.set_panel_controls(Vec::new());

        if !self.has_data() {
            return;
        }

        let layout = ChartLayout::new(self, width, height);
        if layout.plot_width <= 0.0 || layout.plot_height <= 0.0 {
            return;
        }
        let first_layout = self.last_plot_width <= f64::EPSILON;
        let width_changed = (layout.plot_width - self.last_plot_width).abs() > f64::EPSILON;
        self.last_plot_width = layout.plot_width;
        if first_layout {
            self.apply_bar_spacing();
        } else if width_changed && !self.options.time_scale.lock_visible_time_range_on_resize {
            self.apply_bar_spacing_with_anchor(1.0);
        }

        let start_time = self.time_scale.start;
        let end_time = self.time_scale.end;
        let time_ticks = build_time_ticks(
            start_time,
            end_time,
            layout.plot_width,
            self.options.time_scale.uniform_distribution,
        );

        let left_scale = self.scale_for_side(PriceScale::Left, start_time, end_time);
        let right_scale = self.scale_for_side(PriceScale::Right, start_time, end_time);
        let rsi_scale = self.scale_for_rsi(start_time, end_time);
        let primary_side = if right_scale.is_some() {
            PriceScale::Right
        } else {
            PriceScale::Left
        };

        self.draw_grid(
            cr,
            &layout,
            &time_ticks,
            left_scale,
            right_scale,
            rsi_scale,
            primary_side,
        );

        let mut top_marker_series: Vec<usize> = Vec::new();

        for (series_index, series) in self.series.iter().enumerate() {
            if !self.panel_content_visible(series.panel_id) {
                continue;
            }
            let scale = match series.scale {
                PriceScale::Left => left_scale,
                PriceScale::Right => right_scale,
            };
            let scale = match scale {
                Some(scale) => scale,
                None => continue,
            };

            match &series.data {
                SeriesData::Candlestick { data } => {
                    let visible = visible_candles(data, start_time, end_time);
                    if visible.is_empty() {
                        continue;
                    }

                    let z_order = series.options.markers_options.z_order;
                    if !series.markers.is_empty() && z_order == MarkerZOrder::Bottom {
                        self.draw_markers(
                            cr,
                            &series.markers,
                            data,
                            scale,
                            &layout,
                            start_time,
                            end_time,
                            |candle| (candle.high, candle.low, candle.close),
                            &series.options.markers_options,
                        );
                    }

                    let body_width = series_bar_width_times(
                        visible
                            .iter()
                            .map(|candle| candle_time(candle.time)),
                        start_time,
                        end_time,
                        layout.plot_width,
                    );

                    for candle in &visible {
                        let time = candle_time(candle.time);
                        let x_center = map_time_to_x(
                            time,
                            start_time,
                            end_time,
                            layout.plot_left,
                            layout.plot_width,
                        );
                        let high_y = map_price_to_y_scaled(
                            candle.high,
                            scale.min,
                            scale.max,
                            layout.plot_top,
                            layout.main_height,
                            scale.margins,
                            scale.invert,
                            scale.mode,
                            scale.base,
                        );
                        let low_y = map_price_to_y_scaled(
                            candle.low,
                            scale.min,
                            scale.max,
                            layout.plot_top,
                            layout.main_height,
                            scale.margins,
                            scale.invert,
                            scale.mode,
                            scale.base,
                        );
                        let open_y = map_price_to_y_scaled(
                            candle.open,
                            scale.min,
                            scale.max,
                            layout.plot_top,
                            layout.main_height,
                            scale.margins,
                            scale.invert,
                            scale.mode,
                            scale.base,
                        );
                        let close_y = map_price_to_y_scaled(
                            candle.close,
                            scale.min,
                            scale.max,
                            layout.plot_top,
                            layout.main_height,
                            scale.margins,
                            scale.invert,
                            scale.mode,
                            scale.base,
                        );

                        let up = candle.close >= candle.open;
                        let wick = if up { self.style.wick_up } else { self.style.wick_down };
                        let border = if up {
                            self.style.border_up
                        } else {
                            self.style.border_down
                        };

                        cr.set_source_rgb(wick.r, wick.g, wick.b);
                        cr.set_line_width(1.0);
                        cr.move_to(x_center, high_y);
                        cr.line_to(x_center, low_y);
                        let _ = cr.stroke();

                        let (top, bottom, color) = if up {
                            (close_y.min(open_y), close_y.max(open_y), self.style.up)
                        } else {
                            (open_y.min(close_y), open_y.max(close_y), self.style.down)
                        };
                        let body_height = (bottom - top).max(1.0);

                        cr.set_source_rgb(color.r, color.g, color.b);
                        cr.rectangle(x_center - body_width / 2.0, top, body_width, body_height);
                        let _ = cr.fill_preserve();

                        cr.set_source_rgb(border.r, border.g, border.b);
                        cr.set_line_width(1.0);
                        let _ = cr.stroke();
                    }

                    if !series.markers.is_empty() && z_order == MarkerZOrder::Normal {
                        self.draw_markers(
                            cr,
                            &series.markers,
                            data,
                            scale,
                            &layout,
                            start_time,
                            end_time,
                            |candle| (candle.high, candle.low, candle.close),
                            &series.options.markers_options,
                        );
                    }
                    if !series.markers.is_empty() && z_order == MarkerZOrder::Top {
                        top_marker_series.push(series_index);
                    }
                }
                SeriesData::Line { data } => {
                    let visible = visible_line_points(data, start_time, end_time);
                    if visible.is_empty() {
                        continue;
                    }

                    let z_order = series.options.markers_options.z_order;
                    if !series.markers.is_empty() && z_order == MarkerZOrder::Bottom {
                        self.draw_markers(
                            cr,
                            &series.markers,
                            data,
                            scale,
                            &layout,
                            start_time,
                            end_time,
                            |point| (point.value, point.value, point.value),
                            &series.options.markers_options,
                        );
                    }

                    cr.set_source_rgb(self.style.line.r, self.style.line.g, self.style.line.b);
                    cr.set_line_width(2.0);
                    let mut first = true;
                    for point in &visible {
                        let time = candle_time(point.time);
                        let x = map_time_to_x(
                            time,
                            start_time,
                            end_time,
                            layout.plot_left,
                            layout.plot_width,
                        );
                        let y = map_price_to_y_scaled(
                            point.value,
                            scale.min,
                            scale.max,
                            layout.plot_top,
                            layout.main_height,
                            scale.margins,
                            scale.invert,
                            scale.mode,
                            scale.base,
                        );
                        if first {
                            cr.move_to(x, y);
                            first = false;
                        } else {
                            cr.line_to(x, y);
                        }
                    }
                    let _ = cr.stroke();

                    if !series.markers.is_empty() && z_order == MarkerZOrder::Normal {
                        self.draw_markers(
                            cr,
                            &series.markers,
                            data,
                            scale,
                            &layout,
                            start_time,
                            end_time,
                            |point| (point.value, point.value, point.value),
                            &series.options.markers_options,
                        );
                    }

                    if !series.markers.is_empty() && z_order == MarkerZOrder::Top {
                        top_marker_series.push(series_index);
                    }
                }
                SeriesData::Histogram { data } => {
                    let visible = visible_histogram_points(data, start_time, end_time);
                    if visible.is_empty() {
                        continue;
                    }

                    let z_order = series.options.markers_options.z_order;
                    if !series.markers.is_empty() && z_order == MarkerZOrder::Bottom {
                        self.draw_markers(
                            cr,
                            &series.markers,
                            data,
                            scale,
                            &layout,
                            start_time,
                            end_time,
                            |point| (point.value, point.value, point.value),
                            &series.options.markers_options,
                        );
                    }

                    let bar_width = series_bar_width_times(
                        visible
                            .iter()
                            .map(|point| candle_time(point.time)),
                        start_time,
                        end_time,
                        layout.plot_width,
                    );

                    if layout.hist_height <= 0.0 {
                        continue;
                    }

                    let range = histogram_range(&visible);
                    let (hist_min, hist_max) = match range {
                        Some(range) => range,
                        None => continue,
                    };

                    for point in &visible {
                        let color = point.color.unwrap_or(self.style.histogram);
                        cr.set_source_rgb(color.r, color.g, color.b);
                        let time = candle_time(point.time);
                        let x_center = map_time_to_x(
                            time,
                            start_time,
                            end_time,
                            layout.plot_left,
                            layout.plot_width,
                        );
                        let y = map_price_to_y(
                            point.value,
                            hist_min,
                            hist_max,
                            layout.hist_top,
                            layout.hist_height,
                        );
                        let height = (layout.hist_bottom - y).max(1.0);
                        cr.rectangle(x_center - bar_width / 2.0, y, bar_width, height);
                        let _ = cr.fill();
                    }

                    if !series.markers.is_empty() && z_order == MarkerZOrder::Normal {
                        self.draw_markers(
                            cr,
                            &series.markers,
                            data,
                            scale,
                            &layout,
                            start_time,
                            end_time,
                            |point| (point.value, point.value, point.value),
                            &series.options.markers_options,
                        );
                    }

                    if !series.markers.is_empty() && z_order == MarkerZOrder::Top {
                        top_marker_series.push(series_index);
                    }
                }
            }
        }

        let rsi_visible = self
            .rsi_panel_id
            .map(|panel_id| self.panel_content_visible(panel_id))
            .unwrap_or(true);
        if rsi_visible {
            if let (Some(panel), Some(scale)) = (self.rsi_panel.as_ref(), rsi_scale) {
                if layout.rsi_height > 0.0 {
                    let visible = visible_line_points(&panel.data, start_time, end_time);
                    if !visible.is_empty() {
                        cr.set_source_rgb(panel.color.r, panel.color.g, panel.color.b);
                        cr.set_line_width(1.5);
                        let mut first = true;
                        for point in &visible {
                            let time = candle_time(point.time);
                            let x = map_time_to_x(
                                time,
                                start_time,
                                end_time,
                                layout.plot_left,
                                layout.plot_width,
                            );
                            let y = map_price_to_y_scaled(
                                point.value,
                                scale.min,
                                scale.max,
                                layout.rsi_top,
                                layout.rsi_height,
                                scale.margins,
                                scale.invert,
                                scale.mode,
                                scale.base,
                            );
                            if first {
                                cr.move_to(x, y);
                                first = false;
                            } else {
                                cr.line_to(x, y);
                            }
                        }
                        let _ = cr.stroke();
                    }

                    cr.set_source_rgba(
                        self.style.grid.r,
                        self.style.grid.g,
                        self.style.grid.b,
                        0.6,
                    );
                    cr.set_line_width(1.0);
                    cr.move_to(layout.plot_left, layout.rsi_top);
                    cr.line_to(layout.plot_right, layout.rsi_top);
                    let _ = cr.stroke();

                    cr.select_font_face("Sans", cairo::FontSlant::Normal, cairo::FontWeight::Normal);
                    cr.set_font_size(self.style.axis_font_size);
                    let title = if panel.title.is_empty() { "RSI" } else { &panel.title };
                    let title_y = layout.rsi_top + self.style.axis_font_size + 4.0;
                    cr.set_source_rgb(
                        self.style.axis_text.r,
                        self.style.axis_text.g,
                        self.style.axis_text.b,
                    );
                    cr.move_to(layout.plot_left + 6.0, title_y);
                    let _ = cr.show_text(title);

                    let ticks = build_ticks_for_scale(
                        scale,
                        layout.rsi_top,
                        layout.rsi_height,
                        &panel.options,
                    );
                    if panel.options.visible {
                        self.draw_price_axis_right(
                            cr,
                            layout.plot_left,
                            layout.plot_right,
                            layout.axis_right,
                            layout.rsi_top,
                            layout.rsi_bottom,
                            layout.rsi_height,
                            scale,
                            &ticks,
                            &panel.options,
                            &PriceFormat::default(),
                        );
                    }
                }
            }
        }

        self.draw_time_axis(
            cr,
            layout.plot_left,
            layout.plot_top,
            layout.plot_bottom,
            layout.plot_width,
            start_time,
            end_time,
            &time_ticks,
        );

        let mut left_ticks = left_scale.map(|scale| {
            build_ticks_for_scale(
                scale,
                layout.plot_top,
                layout.main_height,
                &self.options.left_price_scale,
            )
        });
        let mut right_ticks = right_scale.map(|scale| {
            build_ticks_for_scale(
                scale,
                layout.plot_top,
                layout.main_height,
                &self.options.right_price_scale,
            )
        });

        if let (Some(primary_scale), Some(secondary_scale)) = match primary_side {
            PriceScale::Left => (left_scale, right_scale),
            PriceScale::Right => (right_scale, left_scale),
        } {
            let (primary_ticks, secondary_ticks, secondary_options) = match primary_side {
                PriceScale::Left => (
                    left_ticks.as_ref(),
                    &mut right_ticks,
                    &self.options.right_price_scale,
                ),
                PriceScale::Right => (
                    right_ticks.as_ref(),
                    &mut left_ticks,
                    &self.options.left_price_scale,
                ),
            };
            if let (Some(primary_ticks), true) =
                (primary_ticks, secondary_options.align_labels)
            {
                *secondary_ticks = Some(aligned_price_ticks(
                    primary_ticks,
                    primary_scale,
                    secondary_scale,
                    &layout,
                ));
            }
        }

        if let (Some(scale), Some(ticks)) = (left_scale, left_ticks.as_ref()) {
            if self.options.left_price_scale.visible {
                let price_format = self.price_format_for_side(PriceScale::Left);
                self.draw_price_axis_left(
                    cr,
                    layout.axis_left,
                    layout.plot_left,
                    layout.plot_right,
                    layout.plot_top,
                    layout.main_bottom,
                    layout.main_height,
                    scale,
                    ticks,
                    &self.options.left_price_scale,
                    &price_format,
                );
            }
        }

        if let (Some(scale), Some(ticks)) = (right_scale, right_ticks.as_ref()) {
            if self.options.right_price_scale.visible {
                let price_format = self.price_format_for_side(PriceScale::Right);
                self.draw_price_axis_right(
                    cr,
                    layout.plot_left,
                    layout.plot_right,
                    layout.axis_right,
                    layout.plot_top,
                    layout.main_bottom,
                    layout.main_height,
                    scale,
                    ticks,
                    &self.options.right_price_scale,
                    &price_format,
                );
            }
        }

        self.draw_series_overlays(cr, &layout, left_scale, right_scale);
        self.draw_panel_controls(cr, &layout);
        self.draw_main_header(cr, &layout);

        if !top_marker_series.is_empty() {
            for series_index in top_marker_series {
                let series = match self.series.get(series_index) {
                    Some(series) => series,
                    None => continue,
                };
                let scale = match series.scale {
                    PriceScale::Left => left_scale,
                    PriceScale::Right => right_scale,
                };
                let scale = match scale {
                    Some(scale) => scale,
                    None => continue,
                };
                match &series.data {
                    SeriesData::Candlestick { data } => {
                        if series.markers.is_empty() {
                            continue;
                        }
                        self.draw_markers(
                            cr,
                            &series.markers,
                            data,
                            scale,
                            &layout,
                            start_time,
                            end_time,
                            |candle| (candle.high, candle.low, candle.close),
                            &series.options.markers_options,
                        );
                    }
                    SeriesData::Line { data } => {
                        if series.markers.is_empty() {
                            continue;
                        }
                        self.draw_markers(
                            cr,
                            &series.markers,
                            data,
                            scale,
                            &layout,
                            start_time,
                            end_time,
                            |point| (point.value, point.value, point.value),
                            &series.options.markers_options,
                        );
                    }
                    SeriesData::Histogram { data } => {
                        if series.markers.is_empty() {
                            continue;
                        }
                        self.draw_markers(
                            cr,
                            &series.markers,
                            data,
                            scale,
                            &layout,
                            start_time,
                            end_time,
                            |point| (point.value, point.value, point.value),
                            &series.options.markers_options,
                        );
                    }
                }
            }
        }

        cr.set_source_rgb(self.style.grid.r, self.style.grid.g, self.style.grid.b);
        cr.set_line_width(1.0);
        cr.rectangle(
            layout.plot_left,
            layout.plot_top,
            layout.plot_width,
            layout.plot_height,
        );
        let _ = cr.stroke();

        let primary_scale =
            primary_candle_scale(self.primary_candles, &self.series, left_scale, right_scale)
                .or_else(|| match primary_side {
                    PriceScale::Left => left_scale,
                    PriceScale::Right => right_scale,
                });

        self.draw_crosshair(
            cr,
            layout,
            start_time,
            end_time,
            &time_ticks,
            left_scale,
            right_scale,
            rsi_scale,
            primary_scale,
            primary_candles(self.primary_candles, &self.series),
        );
    }

    fn draw_background(&self, cr: &Context, width: f64, height: f64) {
        cr.set_source_rgb(
            self.style.background.r,
            self.style.background.g,
            self.style.background.b,
        );
        cr.rectangle(0.0, 0.0, width, height);
        let _ = cr.fill();
    }
}

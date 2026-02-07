use cairo::{Context, FontSlant, FontWeight};

use super::super::data::{SeriesData, SeriesScale};
use super::super::format::{
    format_price_with_format, format_series_tooltip, format_time_label, format_tooltip,
};
use super::super::layout::ChartLayout;
use super::super::ticks::TimeTicks;
use super::super::types::{
    Candle, CrosshairCenter, CrosshairMode, PanelId, PriceFormat, PriceScale, PriceScaleMode, Rect,
};
use super::super::util::{
    apply_line_style, candle_time, map_price_to_y_scaled, map_time_to_x, map_y_to_price_scaled,
    nearest_by_time, transform_price,
};
use super::render_helpers::{build_ticks_for_scale, primary_candle_side, tooltip_position};
use super::ChartCore;
use crate::icons::{draw_svg_icon, IconName};

impl ChartCore {
    pub(super) fn draw_crosshair(
        &self,
        cr: &Context,
        layout: ChartLayout,
        start_time: f64,
        end_time: f64,
        time_ticks: &TimeTicks,
        left_scale: Option<SeriesScale>,
        right_scale: Option<SeriesScale>,
        rsi_scale: Option<SeriesScale>,
        primary_scale: Option<SeriesScale>,
        primary_candles: Option<&[Candle]>,
    ) {
        self.set_tooltip_icon(None);
        let (x, y) = match self.crosshair {
            Some(point) => point,
            None => return,
        };

        if matches!(self.options.crosshair.mode, CrosshairMode::Hidden) {
            return;
        }

        if x < layout.plot_left
            || x > layout.plot_right
            || y < layout.plot_top
            || y > layout.plot_bottom
        {
            return;
        }

        let in_rsi = layout.in_rsi_plot(y);
        let in_hist = layout.in_histogram(y);

        let mut x = x;
        let mut y = y;
        let mut snapped_time: Option<f64> = None;
        let mut snapped_price: Option<f64> = None;

        let snap_to_ohlc = match self.options.crosshair.mode {
            CrosshairMode::MagnetOhlc => true,
            CrosshairMode::Magnet => false,
            CrosshairMode::Normal => self.options.crosshair.snap_to_ohlc,
            CrosshairMode::Hidden => false,
        };
        let snap_to_series = match self.options.crosshair.mode {
            CrosshairMode::Magnet => true,
            CrosshairMode::MagnetOhlc => false,
            CrosshairMode::Normal => self.options.crosshair.snap_to_series,
            CrosshairMode::Hidden => false,
        };

        if layout.in_main_plot(y) && (snap_to_ohlc || snap_to_series) {
            let side = self.side_for_position(x, &layout);
            let scale = match side {
                PriceScale::Left => left_scale,
                PriceScale::Right => right_scale,
            };
            if let Some(scale) = scale {
                let cursor_price = map_y_to_price_scaled(
                    y,
                    scale.min,
                    scale.max,
                    layout.plot_top,
                    layout.main_height,
                    scale.margins,
                    scale.invert,
                    scale.mode,
                    scale.base,
                );
                let target_time = start_time
                    + ((x - layout.plot_left) / layout.plot_width).clamp(0.0, 1.0)
                        * (end_time - start_time);
                let mut best_dist = f64::INFINITY;

                if snap_to_ohlc {
                    if let Some(candles) = primary_candles {
                        if let Some(candle) = nearest_by_time(candles, target_time) {
                            let candle_side =
                                primary_candle_side(self.primary_candles, &self.series);
                            if candle_side == Some(side) {
                                let values = [candle.open, candle.high, candle.low, candle.close];
                                for value in values {
                                    let dist = (cursor_price - value).abs();
                                    if dist < best_dist {
                                        best_dist = dist;
                                        snapped_time = Some(candle_time(candle.time));
                                        snapped_price = Some(value);
                                    }
                                }
                            }
                        }
                    }
                }

                if snap_to_series {
                    for series in &self.series {
                        if !self.panel_content_visible(series.panel_id) {
                            continue;
                        }
                        if series.scale != side {
                            continue;
                        }
                        if self
                            .options
                            .crosshair
                            .do_not_snap_to_hidden_series_indices
                            && match &series.data {
                                SeriesData::Candlestick { data } => data.is_empty(),
                                SeriesData::Line { data } => data.is_empty(),
                                SeriesData::Histogram { data } => data.is_empty(),
                            }
                        {
                            continue;
                        }
                        match &series.data {
                            SeriesData::Line { data } => {
                                if let Some(point) = nearest_by_time(data, target_time) {
                                    let dist = (cursor_price - point.value).abs();
                                    if dist < best_dist {
                                        best_dist = dist;
                                        snapped_time = Some(candle_time(point.time));
                                        snapped_price = Some(point.value);
                                    }
                                }
                            }
                            SeriesData::Histogram { data } => {
                                if let Some(point) = nearest_by_time(data, target_time) {
                                    let dist = (cursor_price - point.value).abs();
                                    if dist < best_dist {
                                        best_dist = dist;
                                        snapped_time = Some(candle_time(point.time));
                                        snapped_price = Some(point.value);
                                    }
                                }
                            }
                            _ => {}
                        }
                    }
                }

                if let Some(time) = snapped_time {
                    x = map_time_to_x(
                        time,
                        start_time,
                        end_time,
                        layout.plot_left,
                        layout.plot_width,
                    );
                }
                if let Some(price) = snapped_price {
                    y = map_price_to_y_scaled(
                        price,
                        scale.min,
                        scale.max,
                        layout.plot_top,
                        layout.main_height,
                        scale.margins,
                        scale.invert,
                        scale.mode,
                        scale.base,
                    );
                }
            }
        } else if in_rsi && snap_to_series {
            if let (Some(panel), Some(scale)) = (self.rsi_panel.as_ref(), rsi_scale) {
                let target_time = start_time
                    + ((x - layout.plot_left) / layout.plot_width).clamp(0.0, 1.0)
                        * (end_time - start_time);
                if let Some(point) = nearest_by_time(&panel.data, target_time) {
                    snapped_time = Some(candle_time(point.time));
                    snapped_price = Some(point.value);
                }
                if let Some(time) = snapped_time {
                    x = map_time_to_x(
                        time,
                        start_time,
                        end_time,
                        layout.plot_left,
                        layout.plot_width,
                    );
                }
                if let Some(price) = snapped_price {
                    y = map_price_to_y_scaled(
                        price,
                        scale.min,
                        scale.max,
                        layout.rsi_top,
                        layout.rsi_height,
                        scale.margins,
                        scale.invert,
                        scale.mode,
                        scale.base,
                    );
                }
            }
        }

        let line_width = self.options.crosshair.line_width.max(0.5);
        cr.set_source_rgba(
            self.style.crosshair.r,
            self.style.crosshair.g,
            self.style.crosshair.b,
            0.35,
        );
        cr.set_line_width(line_width);
        apply_line_style(cr, self.options.crosshair.line_style, line_width);

        if self.options.crosshair.show_vertical {
            cr.move_to(x, layout.plot_top);
            cr.line_to(x, layout.plot_bottom);
        }
        if self.options.crosshair.show_horizontal {
            cr.move_to(layout.plot_left, y);
            cr.line_to(layout.plot_right, y);
        }
        let _ = cr.stroke();
        cr.set_dash(&[], 0.0);

        cr.set_source_rgba(
            self.options.crosshair.center_color.r,
            self.options.crosshair.center_color.g,
            self.options.crosshair.center_color.b,
            0.9,
        );
        match self.options.crosshair.center {
            CrosshairCenter::Cross => {
                let size = self.options.crosshair.center_size.max(1.0) * 0.5;
                cr.set_line_width(1.0);
                cr.move_to(x - size, y);
                cr.line_to(x + size, y);
                cr.move_to(x, y - size);
                cr.line_to(x, y + size);
                let _ = cr.stroke();
            }
            CrosshairCenter::Dot => {
                let radius = self.options.crosshair.center_size.max(1.0) * 0.5;
                cr.arc(x, y, radius, 0.0, std::f64::consts::TAU);
                let _ = cr.fill();
            }
            CrosshairCenter::Circle => {
                cr.set_line_width(1.0);
                let radius = self.options.crosshair.center_size.max(1.0) * 0.5;
                cr.arc(x, y, radius, 0.0, std::f64::consts::TAU);
                let _ = cr.stroke();
            }
        }

        let mut tooltip_precision = 2;
        if layout.in_main_plot(y) {
            let side = self.side_for_position(x, &layout);
            let scale = match side {
                PriceScale::Left => left_scale,
                PriceScale::Right => right_scale,
            };
            if let Some(scale) = scale {
                let options = self.price_scale_options(side);
                if options.visible {
                    let price = map_y_to_price_scaled(
                        y,
                        scale.min,
                        scale.max,
                        layout.plot_top,
                        layout.main_height,
                        scale.margins,
                        scale.invert,
                        scale.mode,
                        scale.base,
                    );
                    let ticks =
                        build_ticks_for_scale(scale, layout.plot_top, layout.main_height, options);
                    tooltip_precision = ticks.precision;
                    let label_value = if matches!(
                        scale.mode,
                        PriceScaleMode::Percentage | PriceScaleMode::IndexedTo100
                    ) {
                        transform_price(price, scale.mode, scale.base)
                    } else {
                        price
                    };
                    let price_format = self.price_format_for_side(side);
                    let label = format_price_with_format(
                        label_value,
                        &price_format,
                        ticks.precision,
                        scale.mode,
                    );
                    let extents = match cr.text_extents(&label) {
                        Ok(extents) => extents,
                        Err(_) => return,
                    };
                    let box_width = extents.width() + 10.0;
                    let box_height = extents.height() + 6.0;
                    let box_x = match side {
                        PriceScale::Right => layout.axis_right - box_width - 4.0,
                        PriceScale::Left => layout.axis_left + 4.0,
                    };
                    let mut box_y = y - box_height / 2.0;
                    if box_y < layout.plot_top {
                        box_y = layout.plot_top;
                    }
                    if box_y + box_height > layout.main_bottom {
                        box_y = layout.main_bottom - box_height;
                    }

                    cr.set_source_rgba(0.1, 0.12, 0.14, 0.85);
                    cr.rectangle(box_x, box_y, box_width, box_height);
                    let _ = cr.fill();

                    cr.set_source_rgb(
                        options.text_color.r,
                        options.text_color.g,
                        options.text_color.b,
                    );
                    cr.move_to(box_x + 5.0, box_y + box_height - 3.0);
                    let _ = cr.show_text(&label);
                }
            }
        } else if in_rsi {
            if let (Some(scale), Some(panel)) = (rsi_scale, self.rsi_panel.as_ref()) {
                if panel.options.visible {
                    let price = map_y_to_price_scaled(
                        y,
                        scale.min,
                        scale.max,
                        layout.rsi_top,
                        layout.rsi_height,
                        scale.margins,
                        scale.invert,
                        scale.mode,
                        scale.base,
                    );
                    let ticks = build_ticks_for_scale(
                        scale,
                        layout.rsi_top,
                        layout.rsi_height,
                        &panel.options,
                    );
                    tooltip_precision = ticks.precision;
                    let label_value = if matches!(
                        scale.mode,
                        PriceScaleMode::Percentage | PriceScaleMode::IndexedTo100
                    ) {
                        transform_price(price, scale.mode, scale.base)
                    } else {
                        price
                    };
                    let label = format_price_with_format(
                        label_value,
                        &PriceFormat::default(),
                        ticks.precision,
                        scale.mode,
                    );
                    let extents = match cr.text_extents(&label) {
                        Ok(extents) => extents,
                        Err(_) => return,
                    };
                    let box_width = extents.width() + 10.0;
                    let box_height = extents.height() + 6.0;
                    let box_x = layout.axis_right - box_width - 4.0;
                    let mut box_y = y - box_height / 2.0;
                    if box_y < layout.rsi_top {
                        box_y = layout.rsi_top;
                    }
                    if box_y + box_height > layout.rsi_bottom {
                        box_y = layout.rsi_bottom - box_height;
                    }

                    cr.set_source_rgba(0.1, 0.12, 0.14, 0.85);
                    cr.rectangle(box_x, box_y, box_width, box_height);
                    let _ = cr.fill();

                    cr.set_source_rgb(
                        panel.options.text_color.r,
                        panel.options.text_color.g,
                        panel.options.text_color.b,
                    );
                    cr.move_to(box_x + 5.0, box_y + box_height - 3.0);
                    let _ = cr.show_text(&label);
                }
            }
        }

        let plot_width = layout.plot_width;
        let anchor = ((x - layout.plot_left) / plot_width).clamp(0.0, 1.0);
        let time = start_time + anchor * (end_time - start_time);
        let time_label = format_time_label(
            time,
            time_ticks.step,
            self.options.time_label_mode,
            &self.options.time_label_format,
            self.options.time_scale.time_visible,
            self.options.time_scale.seconds_visible,
            &self.options.time_scale.tick_mark_format,
            self.options.time_scale.tick_mark_max_character_length,
        );
        if !time_label.is_empty() && self.options.time_scale.visible {
            let extents = match cr.text_extents(&time_label) {
                Ok(extents) => extents,
                Err(_) => return,
            };
            let mut label_x = x - extents.width() / 2.0;
            let min_x = layout.plot_left;
            let max_x = layout.plot_right - extents.width();
            if label_x < min_x {
                label_x = min_x;
            }
            if label_x > max_x {
                label_x = max_x;
            }
            let label_y = layout.plot_bottom + 5.0 + extents.height();

            cr.set_source_rgba(0.1, 0.12, 0.14, 0.85);
            cr.rectangle(
                label_x - 4.0,
                layout.plot_bottom + 2.0,
                extents.width() + 8.0,
                extents.height() + 6.0,
            );
            let _ = cr.fill();

            cr.set_source_rgb(
                self.style.axis_text.r,
                self.style.axis_text.g,
                self.style.axis_text.b,
            );
            cr.move_to(label_x, label_y);
            let _ = cr.show_text(&time_label);
        }

        if self.options.tooltip.enabled {
            let left_ticks = left_scale.map(|scale| {
                build_ticks_for_scale(
                    scale,
                    layout.plot_top,
                    layout.main_height,
                    &self.options.left_price_scale,
                )
            });
            let right_ticks = right_scale.map(|scale| {
                build_ticks_for_scale(
                    scale,
                    layout.plot_top,
                    layout.main_height,
                    &self.options.right_price_scale,
                )
            });
            let rsi_ticks = if let (Some(scale), Some(panel)) = (rsi_scale, self.rsi_panel.as_ref())
            {
                Some(build_ticks_for_scale(
                    scale,
                    layout.rsi_top,
                    layout.rsi_height,
                    &panel.options,
                ))
            } else {
                None
            };

            let mut lines: Vec<String> = Vec::new();
            let panel_top = if in_rsi {
                layout.rsi_top
            } else if in_hist {
                layout.hist_top
            } else {
                layout.plot_top
            };
            let panel_bottom = if in_rsi {
                layout.rsi_bottom
            } else if in_hist {
                layout.hist_bottom
            } else {
                layout.main_bottom
            };

            if in_rsi
                && self
                    .rsi_panel_id
                    .map(|panel_id| self.panel_content_visible(panel_id))
                    .unwrap_or(true)
            {
                if let (Some(panel), Some(scale)) = (self.rsi_panel.as_ref(), rsi_scale) {
                    if let Some(point) = nearest_by_time(&panel.data, time) {
                        let precision = rsi_ticks.as_ref().map(|t| t.precision).unwrap_or(2);
                        let display_value = if matches!(
                            scale.mode,
                            PriceScaleMode::Percentage | PriceScaleMode::IndexedTo100
                        ) {
                            transform_price(point.value, scale.mode, scale.base)
                        } else {
                            point.value
                        };
                        let label = if panel.title.is_empty() {
                            "RSI"
                        } else {
                            &panel.title
                        };
                        lines.push(format_series_tooltip(
                            &self.options.tooltip_line_format,
                            label,
                            point.time,
                            display_value,
                            precision,
                            &PriceFormat::default(),
                            scale.mode,
                        ));
                    }
                }
            } else if in_hist {
                let mut hist_index = 1;
                for series in &self.series {
                    if !self.panel_content_visible(series.panel_id) {
                        continue;
                    }
                    if let SeriesData::Histogram { data } = &series.data {
                        if let Some(point) = nearest_by_time(data, time) {
                            let (precision, mode, base) = match series.scale {
                                PriceScale::Left => left_scale
                                    .map(|scale| (left_ticks.as_ref(), scale))
                                    .map(|(ticks, scale)| {
                                        (
                                            ticks.map(|t| t.precision).unwrap_or(2),
                                            scale.mode,
                                            scale.base,
                                        )
                                    })
                                    .unwrap_or((2, PriceScaleMode::Normal, 1.0)),
                                PriceScale::Right => right_scale
                                    .map(|scale| (right_ticks.as_ref(), scale))
                                    .map(|(ticks, scale)| {
                                        (
                                            ticks.map(|t| t.precision).unwrap_or(2),
                                            scale.mode,
                                            scale.base,
                                        )
                                    })
                                    .unwrap_or((2, PriceScaleMode::Normal, 1.0)),
                            };
                            let display_value = if matches!(
                                mode,
                                PriceScaleMode::Percentage | PriceScaleMode::IndexedTo100
                            ) {
                                transform_price(point.value, mode, base)
                            } else {
                                point.value
                            };
                            let label = format!("Histogram {}", hist_index);
                            lines.push(format_series_tooltip(
                                &self.options.tooltip_histogram_format,
                                &label,
                                point.time,
                                display_value,
                                precision,
                                &series.options.price_format,
                                mode,
                            ));
                            hist_index += 1;
                        }
                    }
                }
            } else {
                if let Some(candles) = primary_candles {
                    if let Some(candle) = nearest_by_time(candles, time) {
                        let (precision, mode, base, format) = if let Some(scale) = primary_scale {
                            let side = primary_candle_side(self.primary_candles, &self.series)
                                .unwrap_or(PriceScale::Right);
                            let ticks = build_ticks_for_scale(
                                scale,
                                layout.plot_top,
                                layout.main_height,
                                self.price_scale_options(side),
                            );
                            let format = self
                                .primary_candles
                                .and_then(|id| self.series.get(id))
                                .map(|series| series.options.price_format.clone())
                                .unwrap_or_default();
                            (ticks.precision, scale.mode, scale.base, format)
                        } else {
                            (
                                tooltip_precision,
                                PriceScaleMode::Normal,
                                1.0,
                                PriceFormat::default(),
                            )
                        };
                        let display_candle = if matches!(
                            mode,
                            PriceScaleMode::Percentage | PriceScaleMode::IndexedTo100
                        ) {
                            let mut display = candle.clone();
                            display.open = transform_price(candle.open, mode, base);
                            display.high = transform_price(candle.high, mode, base);
                            display.low = transform_price(candle.low, mode, base);
                            display.close = transform_price(candle.close, mode, base);
                            display
                        } else {
                            candle.clone()
                        };
                        lines.push(format_tooltip(
                            &self.options.tooltip.format,
                            &display_candle,
                            precision,
                            &format,
                            mode,
                        ));
                    }
                }

                let mut line_index = 1;
                let mut hist_index = 1;
                for series in &self.series {
                    if !self.panel_content_visible(series.panel_id) {
                        continue;
                    }
                    match &series.data {
                        SeriesData::Line { data } => {
                            if let Some(point) = nearest_by_time(data, time) {
                                let (precision, mode, base) = match series.scale {
                                    PriceScale::Left => left_scale
                                        .map(|scale| (left_ticks.as_ref(), scale))
                                        .map(|(ticks, scale)| {
                                            (
                                                ticks.map(|t| t.precision).unwrap_or(2),
                                                scale.mode,
                                                scale.base,
                                            )
                                        })
                                        .unwrap_or((2, PriceScaleMode::Normal, 1.0)),
                                    PriceScale::Right => right_scale
                                        .map(|scale| (right_ticks.as_ref(), scale))
                                        .map(|(ticks, scale)| {
                                            (
                                                ticks.map(|t| t.precision).unwrap_or(2),
                                                scale.mode,
                                                scale.base,
                                            )
                                        })
                                        .unwrap_or((2, PriceScaleMode::Normal, 1.0)),
                                };
                                let display_value = if matches!(
                                    mode,
                                    PriceScaleMode::Percentage | PriceScaleMode::IndexedTo100
                                ) {
                                    transform_price(point.value, mode, base)
                                } else {
                                    point.value
                                };
                                let label = format!("Line {}", line_index);
                                lines.push(format_series_tooltip(
                                    &self.options.tooltip_line_format,
                                    &label,
                                    point.time,
                                    display_value,
                                    precision,
                                    &series.options.price_format,
                                    mode,
                                ));
                                line_index += 1;
                            }
                        }
                        SeriesData::Histogram { data } => {
                            if let Some(point) = nearest_by_time(data, time) {
                                let (precision, mode, base) = match series.scale {
                                    PriceScale::Left => left_scale
                                        .map(|scale| (left_ticks.as_ref(), scale))
                                        .map(|(ticks, scale)| {
                                            (
                                                ticks.map(|t| t.precision).unwrap_or(2),
                                                scale.mode,
                                                scale.base,
                                            )
                                        })
                                        .unwrap_or((2, PriceScaleMode::Normal, 1.0)),
                                    PriceScale::Right => right_scale
                                        .map(|scale| (right_ticks.as_ref(), scale))
                                        .map(|(ticks, scale)| {
                                            (
                                                ticks.map(|t| t.precision).unwrap_or(2),
                                                scale.mode,
                                                scale.base,
                                            )
                                        })
                                        .unwrap_or((2, PriceScaleMode::Normal, 1.0)),
                                };
                                let display_value = if matches!(
                                    mode,
                                    PriceScaleMode::Percentage | PriceScaleMode::IndexedTo100
                                ) {
                                    transform_price(point.value, mode, base)
                                } else {
                                    point.value
                                };
                                let label = format!("Histogram {}", hist_index);
                                lines.push(format_series_tooltip(
                                    &self.options.tooltip_histogram_format,
                                    &label,
                                    point.time,
                                    display_value,
                                    precision,
                                    &series.options.price_format,
                                    mode,
                                ));
                                hist_index += 1;
                            }
                        }
                        _ => {}
                    }
                }
            }

            if !lines.is_empty() {
                cr.select_font_face("Sans", FontSlant::Normal, FontWeight::Normal);
                cr.set_font_size(self.style.axis_font_size);

                let mut max_width: f64 = 0.0;
                let mut total_height: f64 = 0.0;
                let mut heights: Vec<f64> = Vec::with_capacity(lines.len());
                for line in &lines {
                    let extents = match cr.text_extents(line) {
                        Ok(extents) => extents,
                        Err(_) => continue,
                    };
                    max_width = max_width.max(extents.width());
                    total_height += extents.height();
                    heights.push(extents.height());
                }

                let padding = 6.0;
                let spacing = 3.0;
                let line_count = heights.len();
                let box_width = max_width + padding * 2.0;
                let box_height =
                    total_height + padding * 2.0 + spacing * (line_count.saturating_sub(1)) as f64;

                let (mut box_x, mut box_y) = tooltip_position(
                    self.options.tooltip.position,
                    layout.plot_left,
                    layout.plot_right,
                    panel_top,
                    panel_bottom,
                    x,
                    y,
                    box_width,
                    box_height,
                );
                if box_x + box_width > layout.plot_right {
                    box_x = layout.plot_right - box_width;
                }
                if box_y + box_height > panel_bottom {
                    box_y = panel_bottom - box_height;
                }

                cr.set_source_rgba(
                    self.options.tooltip.background.r,
                    self.options.tooltip.background.g,
                    self.options.tooltip.background.b,
                    0.9,
                );
                cr.rectangle(box_x, box_y, box_width, box_height);
                let _ = cr.fill();

                cr.set_source_rgb(
                    self.options.tooltip.text.r,
                    self.options.tooltip.text.g,
                    self.options.tooltip.text.b,
                );

                let mut cursor_y = box_y + padding;
                for (line, height) in lines.iter().zip(heights.iter()) {
                    cursor_y += *height;
                    cr.move_to(box_x + padding, cursor_y);
                    let _ = cr.show_text(line);
                    cursor_y += spacing;
                }

                let panel_kind = layout
                    .panel_at(y)
                    .or_else(|| layout.panels.first().map(|panel| panel.id))
                    .unwrap_or(PanelId(1));
                let icon_padding = 4.0;
                let icon_size = 12.0;
                let icon_x = box_x + box_width - icon_size - icon_padding;
                let icon_y = box_y + icon_padding;
                draw_svg_icon(
                    cr,
                    IconName::Gear,
                    icon_x,
                    icon_y,
                    icon_size,
                    self.options.tooltip.text,
                );
                self.set_tooltip_icon(Some((
                    panel_kind,
                    Rect {
                        x: icon_x - 2.0,
                        y: icon_y - 2.0,
                        width: icon_size + 4.0,
                        height: icon_size + 4.0,
                    },
                )));
            }
        }
    }
}

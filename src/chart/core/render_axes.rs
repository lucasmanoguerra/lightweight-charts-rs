use cairo::{Context, FontSlant, FontWeight};

use super::ChartCore;
use super::render_helpers::build_ticks_for_scale;
use super::super::format::{format_price_with_format, format_time_label};
use super::super::layout::ChartLayout;
use super::super::options::PriceScaleOptions;
use super::super::ticks::{PriceTicks, TimeTicks};
use super::super::types::{PriceFormat, PriceScale, PriceScaleMode};
use super::super::util::{inverse_transform_price, map_price_to_y_scaled, map_time_to_x};
use super::super::data::SeriesScale;

impl ChartCore {
    pub(super) fn draw_grid(
        &self,
        cr: &Context,
        layout: &ChartLayout,
        time_ticks: &TimeTicks,
        left_scale: Option<SeriesScale>,
        right_scale: Option<SeriesScale>,
        rsi_scale: Option<SeriesScale>,
        primary_side: PriceScale,
    ) {
        if !self.options.show_grid {
            return;
        }

        cr.set_source_rgba(self.style.grid.r, self.style.grid.g, self.style.grid.b, 0.35);
        cr.set_line_width(1.0);

        for tick in &time_ticks.ticks {
            let x = map_time_to_x(
                *tick,
                self.time_scale.start,
                self.time_scale.end,
                layout.plot_left,
                layout.plot_width,
            );
            cr.move_to(x, layout.plot_top);
            cr.line_to(x, layout.plot_bottom);
        }
        let _ = cr.stroke();

        let primary_scale = match primary_side {
            PriceScale::Left => left_scale,
            PriceScale::Right => right_scale,
        };
        if let Some(scale) = primary_scale {
            let options = self.price_scale_options(primary_side);
            let ticks = build_ticks_for_scale(
                scale,
                layout.plot_top,
                layout.main_height,
                options,
            );
            cr.set_source_rgba(self.style.grid.r, self.style.grid.g, self.style.grid.b, 0.35);
            cr.set_line_width(1.0);
            for value in &ticks.ticks {
                let raw_value = if scale.mode == PriceScaleMode::Logarithmic {
                    inverse_transform_price(*value, scale.mode, scale.base)
                } else if matches!(
                    scale.mode,
                    PriceScaleMode::Percentage | PriceScaleMode::IndexedTo100
                ) {
                    inverse_transform_price(*value, scale.mode, scale.base)
                } else {
                    *value
                };
                let y = map_price_to_y_scaled(
                    raw_value,
                    scale.min,
                    scale.max,
                    layout.plot_top,
                    layout.main_height,
                    scale.margins,
                    scale.invert,
                    scale.mode,
                    scale.base,
                );
                cr.move_to(layout.plot_left, y);
                cr.line_to(layout.plot_right, y);
            }
            let _ = cr.stroke();
        }

        let rsi_visible = self
            .rsi_panel_id
            .map(|panel_id| self.panel_content_visible(panel_id))
            .unwrap_or(true);
        if rsi_visible {
            if let (Some(panel), Some(scale)) = (self.rsi_panel.as_ref(), rsi_scale) {
                let ticks = build_ticks_for_scale(
                    scale,
                    layout.rsi_top,
                    layout.rsi_height,
                    &panel.options,
                );
            cr.set_source_rgba(self.style.grid.r, self.style.grid.g, self.style.grid.b, 0.25);
            cr.set_line_width(1.0);
            for value in &ticks.ticks {
                let raw_value = if scale.mode == PriceScaleMode::Logarithmic {
                    inverse_transform_price(*value, scale.mode, scale.base)
                } else if matches!(
                    scale.mode,
                    PriceScaleMode::Percentage | PriceScaleMode::IndexedTo100
                ) {
                    inverse_transform_price(*value, scale.mode, scale.base)
                } else {
                    *value
                };
                let y = map_price_to_y_scaled(
                    raw_value,
                    scale.min,
                    scale.max,
                    layout.rsi_top,
                    layout.rsi_height,
                    scale.margins,
                    scale.invert,
                    scale.mode,
                    scale.base,
                );
                cr.move_to(layout.plot_left, y);
                cr.line_to(layout.plot_right, y);
            }
                let _ = cr.stroke();
            }
        }
    }

    pub(super) fn draw_time_axis(
        &self,
        cr: &Context,
        plot_left: f64,
        _plot_top: f64,
        plot_bottom: f64,
        plot_width: f64,
        start_time: f64,
        end_time: f64,
        ticks: &TimeTicks,
    ) {
        let axis_visible = self.options.time_scale.visible;
        let ticks_visible = self.options.time_scale.ticks_visible;

        if axis_visible && self.options.time_scale.border_visible {
            let color = self.options.time_scale.border_color;
            cr.set_source_rgb(color.r, color.g, color.b);
            cr.set_line_width(1.0);
            cr.move_to(plot_left, plot_bottom);
            cr.line_to(plot_left + plot_width, plot_bottom);
            let _ = cr.stroke();
        }

        cr.select_font_face("Sans", FontSlant::Normal, FontWeight::Normal);
        cr.set_font_size(self.style.axis_font_size);

        for tick in &ticks.ticks {
            let x = map_time_to_x(*tick, start_time, end_time, plot_left, plot_width);

            if axis_visible && ticks_visible {
                let color = self.options.time_scale.border_color;
                cr.set_source_rgb(color.r, color.g, color.b);
                cr.set_line_width(1.0);
                cr.move_to(x, plot_bottom);
                cr.line_to(x, plot_bottom + 5.0);
                let _ = cr.stroke();
            }

            let label = format_time_label(
                *tick,
                ticks.step,
                self.options.time_label_mode,
                &self.options.time_label_format,
                self.options.time_scale.time_visible,
                self.options.time_scale.seconds_visible,
                &self.options.time_scale.tick_mark_format,
                self.options.time_scale.tick_mark_max_character_length,
            );
            if label.is_empty() || !axis_visible {
                continue;
            }
            let extents = match cr.text_extents(&label) {
                Ok(extents) => extents,
                Err(_) => continue,
            };
            let mut text_x = x - extents.width() / 2.0;
            let max_x = plot_left + plot_width - extents.width();
            if text_x < plot_left {
                text_x = plot_left;
            }
            if text_x > max_x {
                text_x = max_x;
            }
            let text_y = plot_bottom + 5.0 + extents.height() + 2.0;

            cr.set_source_rgb(
                self.style.axis_text.r,
                self.style.axis_text.g,
                self.style.axis_text.b,
            );
            cr.move_to(text_x, text_y);
            let _ = cr.show_text(&label);
        }
    }

    pub(super) fn draw_price_axis_left(
        &self,
        cr: &Context,
        axis_left: f64,
        plot_left: f64,
        _plot_right: f64,
        plot_top: f64,
        plot_bottom: f64,
        plot_height: f64,
        scale: SeriesScale,
        ticks: &PriceTicks,
        options: &PriceScaleOptions,
        price_format: &PriceFormat,
    ) {
        if options.border_visible {
            let color = options.border_color;
            cr.set_source_rgb(color.r, color.g, color.b);
            cr.set_line_width(1.0);
            cr.move_to(plot_left, plot_top);
            cr.line_to(plot_left, plot_bottom);
            let _ = cr.stroke();
        }

        cr.select_font_face("Sans", FontSlant::Normal, FontWeight::Normal);
        cr.set_font_size(self.style.axis_font_size);

        for value in &ticks.ticks {
            let raw_value = if scale.mode == PriceScaleMode::Logarithmic {
                inverse_transform_price(*value, scale.mode, scale.base)
            } else if matches!(scale.mode, PriceScaleMode::Percentage | PriceScaleMode::IndexedTo100)
            {
                inverse_transform_price(*value, scale.mode, scale.base)
            } else {
                *value
            };
            let y = map_price_to_y_scaled(
                raw_value,
                scale.min,
                scale.max,
                plot_top,
                plot_height,
                scale.margins,
                scale.invert,
                scale.mode,
                scale.base,
            );

            let label_value = if matches!(scale.mode, PriceScaleMode::Logarithmic) {
                raw_value
            } else if matches!(scale.mode, PriceScaleMode::Percentage | PriceScaleMode::IndexedTo100)
            {
                *value
            } else {
                raw_value
            };
            let label = format_price_with_format(label_value, price_format, ticks.precision, scale.mode);
            let extents = match cr.text_extents(&label) {
                Ok(extents) => extents,
                Err(_) => continue,
            };

            let text_x = axis_left + 6.0;
            let text_y = y + extents.height() / 2.0;
            let axis_right = plot_left;
            if options.entire_text_only
                && (text_x + extents.width() > axis_right || text_x < axis_left)
            {
                continue;
            }

            cr.set_source_rgb(options.text_color.r, options.text_color.g, options.text_color.b);
            cr.move_to(text_x, text_y);
            let _ = cr.show_text(&label);

            if options.ticks_visible {
                let tick_len = 4.0;
                cr.set_source_rgb(options.border_color.r, options.border_color.g, options.border_color.b);
                cr.move_to(axis_right - tick_len, y);
                cr.line_to(axis_right, y);
                let _ = cr.stroke();
            }
        }
    }

    pub(super) fn draw_price_axis_right(
        &self,
        cr: &Context,
        _plot_left: f64,
        plot_right: f64,
        axis_right: f64,
        plot_top: f64,
        plot_bottom: f64,
        plot_height: f64,
        scale: SeriesScale,
        ticks: &PriceTicks,
        options: &PriceScaleOptions,
        price_format: &PriceFormat,
    ) {
        if options.border_visible {
            let color = options.border_color;
            cr.set_source_rgb(color.r, color.g, color.b);
            cr.set_line_width(1.0);
            cr.move_to(plot_right, plot_top);
            cr.line_to(plot_right, plot_bottom);
            let _ = cr.stroke();
        }

        cr.select_font_face("Sans", FontSlant::Normal, FontWeight::Normal);
        cr.set_font_size(self.style.axis_font_size);

        for value in &ticks.ticks {
            let raw_value = if scale.mode == PriceScaleMode::Logarithmic {
                inverse_transform_price(*value, scale.mode, scale.base)
            } else if matches!(scale.mode, PriceScaleMode::Percentage | PriceScaleMode::IndexedTo100)
            {
                inverse_transform_price(*value, scale.mode, scale.base)
            } else {
                *value
            };
            let y = map_price_to_y_scaled(
                raw_value,
                scale.min,
                scale.max,
                plot_top,
                plot_height,
                scale.margins,
                scale.invert,
                scale.mode,
                scale.base,
            );

            let label_value = if matches!(scale.mode, PriceScaleMode::Logarithmic) {
                raw_value
            } else if matches!(scale.mode, PriceScaleMode::Percentage | PriceScaleMode::IndexedTo100)
            {
                *value
            } else {
                raw_value
            };
            let label = format_price_with_format(label_value, price_format, ticks.precision, scale.mode);
            let extents = match cr.text_extents(&label) {
                Ok(extents) => extents,
                Err(_) => continue,
            };

            let text_x = axis_right - extents.width() - 6.0;
            let text_y = y + extents.height() / 2.0;
            let axis_left = plot_right;
            if options.entire_text_only
                && (text_x + extents.width() > axis_right || text_x < axis_left)
            {
                continue;
            }

            cr.set_source_rgb(options.text_color.r, options.text_color.g, options.text_color.b);
            cr.move_to(text_x, text_y);
            let _ = cr.show_text(&label);

            if options.ticks_visible {
                let tick_len = 4.0;
                cr.set_source_rgb(options.border_color.r, options.border_color.g, options.border_color.b);
                cr.move_to(axis_right, y);
                cr.line_to(axis_right + tick_len, y);
                let _ = cr.stroke();
            }
        }
    }
}

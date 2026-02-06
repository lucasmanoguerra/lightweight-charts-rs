use cairo::Context;

use super::super::data::{HasTime, SeriesScale};
use super::super::layout::ChartLayout;
use super::super::types::{Color, Marker, MarkerPosition, MarkerShape, SeriesMarkersOptions};
use super::super::util::{candle_time, map_price_to_y_scaled, map_time_to_x, nearest_by_time};
use super::render_helpers::draw_rounded_rect;
use super::ChartCore;
use crate::icons::draw_marker_svg_icon;

impl ChartCore {
    pub(super) fn draw_markers<T: HasTime>(
        &self,
        cr: &Context,
        markers: &[Marker],
        data: &[T],
        scale: SeriesScale,
        layout: &ChartLayout,
        start_time: f64,
        end_time: f64,
        data_values: impl Fn(&T) -> (f64, f64, f64),
        defaults: &SeriesMarkersOptions,
    ) {
        if markers.is_empty() || data.is_empty() {
            return;
        }

        let min_time = start_time.min(end_time);
        let max_time = start_time.max(end_time);

        for marker in markers {
            let time = candle_time(marker.time);
            if time < min_time || time > max_time {
                continue;
            }

            let x = map_time_to_x(
                time,
                start_time,
                end_time,
                layout.plot_left,
                layout.plot_width,
            );
            let mut y = layout.plot_top;

            let mut price_anchor = None;
            if let Some(price) = marker.price {
                price_anchor = Some(price);
            } else if let Some(point) = nearest_by_time(data, time) {
                let (high, low, close) = data_values(point);
                price_anchor = Some(match marker.position {
                    MarkerPosition::Above => high,
                    MarkerPosition::Below => low,
                    MarkerPosition::InBar => close,
                    MarkerPosition::AtPriceTop => high,
                    MarkerPosition::AtPriceBottom => low,
                    MarkerPosition::AtPriceMiddle => (high + low) * 0.5,
                });
            }

            if let Some(price) = price_anchor {
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

            let size = marker.size.max(4.0);
            let half = size * 0.5;

            cr.set_source_rgb(marker.color.r, marker.color.g, marker.color.b);

            match marker.shape {
                MarkerShape::ArrowUp => {
                    cr.move_to(x, y - half);
                    cr.line_to(x - half, y + half);
                    cr.line_to(x + half, y + half);
                    cr.close_path();
                    let _ = cr.fill();
                }
                MarkerShape::ArrowDown => {
                    cr.move_to(x, y + half);
                    cr.line_to(x - half, y - half);
                    cr.line_to(x + half, y - half);
                    cr.close_path();
                    let _ = cr.fill();
                }
                MarkerShape::Circle => {
                    cr.arc(x, y, half, 0.0, std::f64::consts::TAU);
                    let _ = cr.fill();
                }
                MarkerShape::Square => {
                    cr.rectangle(x - half, y - half, size, size);
                    let _ = cr.fill();
                }
                MarkerShape::Diamond => {
                    cr.move_to(x, y - half);
                    cr.line_to(x + half, y);
                    cr.line_to(x, y + half);
                    cr.line_to(x - half, y);
                    cr.close_path();
                    let _ = cr.fill();
                }
            }

            draw_marker_icon(
                cr,
                marker,
                defaults,
                x,
                y,
                self.style.axis_font_size,
                self.style.axis_text,
            );

            if let Some(text) = &marker.text {
                cr.select_font_face("Sans", cairo::FontSlant::Normal, cairo::FontWeight::Normal);
                let font_size = if marker.label_text_size > 0.0 {
                    marker.label_text_size
                } else {
                    self.style.axis_font_size
                };
                cr.set_font_size(font_size);
                let extents = match cr.text_extents(text) {
                    Ok(extents) => extents,
                    Err(_) => continue,
                };
                let padding = marker.label_padding.max(2.0);
                let label_width = extents.width() + padding * 2.0;
                let label_height = extents.height() + padding * 1.5;
                let radius = marker.label_radius.max(0.0);

                let mut label_x = x + half + padding + marker.label_offset_x;
                let mut label_y = y - label_height / 2.0 + marker.label_offset_y;

                if label_x + label_width > layout.plot_right {
                    label_x = x - half - padding - label_width;
                }
                if label_y < layout.plot_top {
                    label_y = layout.plot_top;
                }
                if label_y + label_height > layout.plot_bottom {
                    label_y = layout.plot_bottom - label_height;
                }

                let bg = marker.label_background.unwrap_or(Color::new(
                    marker.color.r,
                    marker.color.g,
                    marker.color.b,
                ));
                let alpha = marker.label_background_alpha.clamp(0.0, 1.0);
                cr.set_source_rgba(bg.r, bg.g, bg.b, alpha);
                draw_rounded_rect(cr, label_x, label_y, label_width, label_height, radius);
                let _ = cr.fill();

                if let Some(border) = marker.label_border_color {
                    let width = marker.label_border_width.max(0.0);
                    if width > 0.0 {
                        cr.set_line_width(width);
                        cr.set_source_rgb(border.r, border.g, border.b);
                        draw_rounded_rect(cr, label_x, label_y, label_width, label_height, radius);
                        let _ = cr.stroke();
                    }
                }

                let text_color = marker.label_text.unwrap_or(self.style.axis_text);
                cr.set_source_rgb(text_color.r, text_color.g, text_color.b);
                cr.move_to(label_x + padding, label_y + label_height - padding * 0.5);
                let _ = cr.show_text(text);
            }
        }
    }
}

fn draw_marker_icon(
    cr: &Context,
    marker: &Marker,
    defaults: &SeriesMarkersOptions,
    x: f64,
    y: f64,
    fallback_font_size: f64,
    fallback_color: Color,
) {
    let icon_text = marker
        .icon_text
        .as_ref()
        .or(defaults.default_icon_text.as_ref());
    let Some(icon_text) = icon_text else {
        return;
    };
    let padding = if marker.icon_padding > 0.0 {
        marker.icon_padding
    } else {
        defaults.default_icon_padding
    }
    .max(2.0);
    let icon_size = if marker.icon_font_size > 0.0 {
        marker.icon_font_size
    } else if defaults.default_icon_font_size > 0.0 {
        defaults.default_icon_font_size
    } else {
        fallback_font_size
    };
    let box_width = icon_size + padding * 2.0;
    let box_height = icon_size + padding * 2.0;
    let box_x = x - box_width / 2.0;
    let box_y = y - box_height / 2.0;

    let background = marker.icon_background.or(defaults.default_icon_background);
    if let Some(bg) = background {
        cr.set_source_rgba(bg.r, bg.g, bg.b, 0.85);
        draw_rounded_rect(cr, box_x, box_y, box_width, box_height, padding.min(6.0));
        let _ = cr.fill();
    }

    let border_color = marker
        .icon_border_color
        .or(defaults.default_icon_border_color);
    let border_width = if marker.icon_border_width > 0.0 {
        marker.icon_border_width
    } else {
        defaults.default_icon_border_width
    }
    .max(0.0);
    if let Some(border) = border_color {
        let width = border_width;
        if width > 0.0 {
            cr.set_line_width(width);
            cr.set_source_rgb(border.r, border.g, border.b);
            draw_rounded_rect(cr, box_x, box_y, box_width, box_height, padding.min(6.0));
            let _ = cr.stroke();
        }
    }

    let text_color = marker
        .icon_text_color
        .or(defaults.default_icon_text_color)
        .unwrap_or(fallback_color);
    let svg_drawn = draw_marker_svg_icon(
        cr,
        icon_text,
        box_x + padding,
        box_y + padding,
        icon_size,
        text_color,
    );
    if svg_drawn {
        return;
    }

    cr.select_font_face("Sans", cairo::FontSlant::Normal, cairo::FontWeight::Normal);
    cr.set_font_size(icon_size);
    let extents = match cr.text_extents(icon_text) {
        Ok(extents) => extents,
        Err(_) => return,
    };
    cr.set_source_rgb(text_color.r, text_color.g, text_color.b);
    cr.move_to(x - extents.width() / 2.0, y + extents.height() / 2.0);
    let _ = cr.show_text(icon_text);
}

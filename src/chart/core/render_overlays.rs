use cairo::{Context, FontSlant, FontWeight};

use super::super::data::SeriesScale;
use super::super::format::format_price_with_format;
use super::super::layout::ChartLayout;
use super::super::types::{
    Color, PanelControlAction, PanelControlHit, PanelId, PanelRole, PriceScale, PriceScaleMode,
    Rect,
};
use super::super::util::{apply_line_style, map_price_to_y_scaled, transform_price};
use super::render_helpers::{build_ticks_for_scale, draw_rounded_rect, series_last_value};
use super::ChartCore;
use crate::icons::{draw_svg_icon, IconName};
use std::collections::HashMap;

impl ChartCore {
    pub(super) fn draw_main_header(&self, cr: &Context, layout: &ChartLayout) {
        let symbol = self.options.main_symbol.trim();
        let timeframe = self.options.main_timeframe.trim();
        if symbol.is_empty() && timeframe.is_empty() {
            return;
        }

        let mut text = String::new();
        if !symbol.is_empty() {
            text.push_str(symbol);
        }
        if !timeframe.is_empty() {
            if !text.is_empty() {
                text.push_str(" · ");
            }
            text.push_str(timeframe);
        }

        cr.select_font_face("Sans", FontSlant::Normal, FontWeight::Bold);
        cr.set_font_size(self.style.axis_font_size + 1.0);
        cr.set_source_rgb(
            self.style.axis_text.r,
            self.style.axis_text.g,
            self.style.axis_text.b,
        );
        let extents = match cr.text_extents(&text) {
            Ok(extents) => extents,
            Err(_) => return,
        };
        let x = layout.plot_left + 6.0;
        let y = layout.plot_top + extents.height() + 4.0;
        cr.move_to(x, y);
        let _ = cr.show_text(&text);
    }

    pub(super) fn draw_series_overlays(
        &self,
        cr: &Context,
        layout: &ChartLayout,
        left_scale: Option<SeriesScale>,
        right_scale: Option<SeriesScale>,
    ) {
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

        for series in &self.series {
            let scale = match series.scale {
                PriceScale::Left => left_scale,
                PriceScale::Right => right_scale,
            };
            let scale = match scale {
                Some(scale) => scale,
                None => continue,
            };

            if !series.price_lines.is_empty() {
                let ticks = match series.scale {
                    PriceScale::Left => left_ticks.as_ref(),
                    PriceScale::Right => right_ticks.as_ref(),
                };
                let precision = ticks.map(|ticks| ticks.precision).unwrap_or(2);

                for price_line in &series.price_lines {
                    let options = &price_line.options;
                    let color = options.color;
                    let y = map_price_to_y_scaled(
                        options.price,
                        scale.min,
                        scale.max,
                        layout.plot_top,
                        layout.main_height,
                        scale.margins,
                        scale.invert,
                        scale.mode,
                        scale.base,
                    );

                    if options.line_visible {
                        cr.set_source_rgba(
                            color.r,
                            color.g,
                            color.b,
                            options.line_opacity.clamp(0.0, 1.0),
                        );
                        cr.set_line_width(options.line_width.max(0.5));
                        apply_line_style(cr, options.line_style, options.line_width);
                        cr.move_to(layout.plot_left, y);
                        cr.line_to(layout.plot_right, y);
                        let _ = cr.stroke();
                        cr.set_dash(&[], 0.0);
                    }

                    if options.axis_label_visible {
                        let label_value = if matches!(
                            scale.mode,
                            PriceScaleMode::Percentage | PriceScaleMode::IndexedTo100
                        ) {
                            transform_price(options.price, scale.mode, scale.base)
                        } else {
                            options.price
                        };
                        let price_label = format_price_with_format(
                            label_value,
                            &series.options.price_format,
                            precision,
                            scale.mode,
                        );
                        let text = match &options.title {
                            Some(title) if !title.is_empty() => format!("{title} {price_label}"),
                            _ => price_label,
                        };
                        let extents = match cr.text_extents(&text) {
                            Ok(extents) => extents,
                            Err(_) => continue,
                        };
                        let padding = options.axis_label_padding.max(2.0);
                        let box_width = extents.width() + padding * 2.0;
                        let box_height = extents.height() + padding * 1.5;
                        let mut box_y = y - box_height / 2.0;
                        if box_y < layout.plot_top {
                            box_y = layout.plot_top;
                        }
                        if box_y + box_height > layout.main_bottom {
                            box_y = layout.main_bottom - box_height;
                        }
                        let box_x = match series.scale {
                            PriceScale::Left => layout.axis_left + 4.0,
                            PriceScale::Right => layout.axis_right - box_width - 4.0,
                        };

                        let bg = options.axis_label_color.unwrap_or(color);
                        let text_color = options
                            .axis_label_text_color
                            .unwrap_or(Color::new(0.95, 0.96, 0.98));
                        cr.set_source_rgba(
                            bg.r,
                            bg.g,
                            bg.b,
                            options.axis_label_background_alpha.clamp(0.0, 1.0),
                        );
                        draw_rounded_rect(
                            cr,
                            box_x,
                            box_y,
                            box_width,
                            box_height,
                            options.axis_label_radius,
                        );
                        let _ = cr.fill();

                        if let Some(border) = options.axis_label_border_color {
                            let width = options.axis_label_border_width.max(0.0);
                            if width > 0.0 {
                                cr.set_line_width(width);
                                cr.set_source_rgb(border.r, border.g, border.b);
                                draw_rounded_rect(
                                    cr,
                                    box_x,
                                    box_y,
                                    box_width,
                                    box_height,
                                    options.axis_label_radius,
                                );
                                let _ = cr.stroke();
                            }
                        }

                        cr.set_source_rgb(text_color.r, text_color.g, text_color.b);
                        cr.move_to(box_x + padding, box_y + box_height - padding * 0.5);
                        let _ = cr.show_text(&text);
                    }
                }
            }

            let (value, color) = match series_last_value(series, &self.style) {
                Some(value) => value,
                None => continue,
            };

            let line_color = series.options.price_line_color.unwrap_or(color);
            if series.options.show_price_line {
                let y = map_price_to_y_scaled(
                    value,
                    scale.min,
                    scale.max,
                    layout.plot_top,
                    layout.main_height,
                    scale.margins,
                    scale.invert,
                    scale.mode,
                    scale.base,
                );
                let alpha = 0.35;
                cr.set_source_rgba(line_color.r, line_color.g, line_color.b, alpha);
                cr.set_line_width(series.options.price_line_width.max(0.5));
                apply_line_style(
                    cr,
                    series.options.price_line_style,
                    series.options.price_line_width,
                );
                cr.move_to(layout.plot_left, y);
                cr.line_to(layout.plot_right, y);
                let _ = cr.stroke();
                cr.set_dash(&[], 0.0);
            }

            if series.options.show_last_value {
                let label_color = series.options.last_value_background.unwrap_or(line_color);
                let text_color = series
                    .options
                    .last_value_text
                    .unwrap_or(Color::new(0.95, 0.96, 0.98));
                let ticks = match series.scale {
                    PriceScale::Left => left_ticks.as_ref(),
                    PriceScale::Right => right_ticks.as_ref(),
                };
                let precision = ticks.map(|ticks| ticks.precision).unwrap_or(2);
                let label_value = if matches!(
                    scale.mode,
                    PriceScaleMode::Percentage | PriceScaleMode::IndexedTo100
                ) {
                    transform_price(value, scale.mode, scale.base)
                } else {
                    value
                };
                let label = format_price_with_format(
                    label_value,
                    &series.options.price_format,
                    precision,
                    scale.mode,
                );
                let extents = match cr.text_extents(&label) {
                    Ok(extents) => extents,
                    Err(_) => continue,
                };
                let box_width = extents.width() + 10.0;
                let box_height = extents.height() + 6.0;
                let y = map_price_to_y_scaled(
                    value,
                    scale.min,
                    scale.max,
                    layout.plot_top,
                    layout.main_height,
                    scale.margins,
                    scale.invert,
                    scale.mode,
                    scale.base,
                );
                let mut box_y = y - box_height / 2.0;
                if box_y < layout.plot_top {
                    box_y = layout.plot_top;
                }
                if box_y + box_height > layout.main_bottom {
                    box_y = layout.main_bottom - box_height;
                }
                let box_x = match series.scale {
                    PriceScale::Left => layout.axis_left + 4.0,
                    PriceScale::Right => layout.axis_right - box_width - 4.0,
                };

                cr.set_source_rgba(label_color.r, label_color.g, label_color.b, 0.85);
                cr.rectangle(box_x, box_y, box_width, box_height);
                let _ = cr.fill();

                cr.set_source_rgb(text_color.r, text_color.g, text_color.b);
                cr.move_to(box_x + 5.0, box_y + box_height - 3.0);
                let _ = cr.show_text(&label);
            }
        }
    }

    pub(super) fn draw_panel_controls(&self, cr: &Context, layout: &ChartLayout) {
        let icon_size = self.style.panel_toolbar_icon_size.max(10.0);
        let padding = 4.0;
        let spacing = 6.0;
        let toolbar_width = icon_size + padding * 2.0;
        let mut hits: Vec<PanelControlHit> = Vec::new();

        let mut group_map: HashMap<super::super::types::TimeScaleId, Vec<PanelId>> = HashMap::new();
        for panel in &layout.panels {
            group_map
                .entry(panel.group_id)
                .or_insert_with(Vec::new)
                .push(panel.id);
        }

        for panel_layout in &layout.panels {
            let panel_state = match self.panels.iter().find(|panel| panel.id == panel_layout.id) {
                Some(panel) => panel,
                None => continue,
            };
            let group_panels = group_map.get(&panel_layout.group_id);
            let (has_above, has_below) = if let Some(panels) = group_panels {
                let index = panels
                    .iter()
                    .position(|id| *id == panel_layout.id)
                    .unwrap_or(0);
                let above = index > 0;
                let below = index + 1 < panels.len();
                (above, below)
            } else {
                (false, false)
            };

            let mut controls: Vec<(IconName, PanelControlAction)> = Vec::new();
            if matches!(panel_state.role, PanelRole::Main) {
                controls.push((IconName::AddDown, PanelControlAction::AddBelow));
            } else {
                if has_above {
                    controls.push((IconName::AddUp, PanelControlAction::AddAbove));
                }
                if has_below {
                    controls.push((IconName::AddDown, PanelControlAction::AddBelow));
                }
            }

            let visibility_icon = if panel_state.content_visible {
                IconName::Eye
            } else {
                IconName::EyeOff
            };
            controls.push((visibility_icon, PanelControlAction::ToggleVisible));

            let collapse_icon = if panel_state.collapsed {
                IconName::ChevronUp
            } else {
                IconName::ChevronDown
            };
            controls.push((collapse_icon, PanelControlAction::ToggleCollapsed));

            if panel_state.role != PanelRole::Main {
                controls.push((IconName::Trash, PanelControlAction::Remove));
            }

            if controls.is_empty() {
                continue;
            }

            let total_height = controls.len() as f64 * icon_size
                + (controls.len().saturating_sub(1)) as f64 * spacing;
            let start_y = (panel_layout.top + (panel_layout.height - total_height) / 2.0)
                .max(panel_layout.top + 4.0);
            let x =
                (panel_layout.axis_right - toolbar_width - 4.0).max(panel_layout.plot_right + 4.0);

            let bg_color = Color::new(0.1, 0.12, 0.16);
            cr.set_source_rgba(bg_color.r, bg_color.g, bg_color.b, 0.65);
            draw_rounded_rect(
                cr,
                x,
                start_y - padding,
                toolbar_width,
                total_height + padding * 2.0,
                6.0,
            );
            let _ = cr.fill();

            let mut cursor_y = start_y;
            for (icon, action) in controls {
                let icon_x = x + padding;
                let rect = Rect {
                    x,
                    y: cursor_y - padding,
                    width: toolbar_width,
                    height: icon_size + padding * 2.0,
                };
                draw_svg_icon(cr, icon, icon_x, cursor_y, icon_size, self.style.axis_text);
                hits.push(PanelControlHit {
                    panel: panel_layout.id,
                    action,
                    rect,
                });
                cursor_y += icon_size + spacing;
            }
        }

        self.set_panel_controls(hits);
    }
}

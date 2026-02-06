use crate::chart::{
    Color, CrosshairMode, LineStyle, PriceFormat, PriceScaleMode, PriceScaleOptions, ScaleMargins,
};
use crate::settings_ui::{PriceScaleSideControls, SeriesFormatControls};
use relm4::gtk::{gdk, prelude::*};

pub fn rgba_from_color(color: Color) -> gdk::RGBA {
    gdk::RGBA::new(color.r as f32, color.g as f32, color.b as f32, 1.0)
}

pub fn color_from_rgba(rgba: gdk::RGBA) -> Color {
    Color::new(rgba.red() as f64, rgba.green() as f64, rgba.blue() as f64)
}

pub fn line_style_from_combo(combo: &relm4::gtk::ComboBoxText) -> LineStyle {
    match combo.active() {
        Some(1) => LineStyle::Dotted,
        Some(2) => LineStyle::Dashed,
        _ => LineStyle::Solid,
    }
}

pub fn crosshair_mode_from_combo(combo: &relm4::gtk::ComboBoxText) -> CrosshairMode {
    match combo.active() {
        Some(1) => CrosshairMode::Magnet,
        Some(2) => CrosshairMode::MagnetOhlc,
        Some(3) => CrosshairMode::Hidden,
        _ => CrosshairMode::Normal,
    }
}

pub fn crosshair_mode_to_index(mode: CrosshairMode) -> u32 {
    match mode {
        CrosshairMode::Normal => 0,
        CrosshairMode::Magnet => 1,
        CrosshairMode::MagnetOhlc => 2,
        CrosshairMode::Hidden => 3,
    }
}

pub fn price_scale_mode_from_combo(combo: &relm4::gtk::ComboBoxText) -> PriceScaleMode {
    match combo.active() {
        Some(1) => PriceScaleMode::Logarithmic,
        Some(2) => PriceScaleMode::Percentage,
        Some(3) => PriceScaleMode::IndexedTo100,
        _ => PriceScaleMode::Normal,
    }
}

pub fn price_scale_mode_to_index(mode: PriceScaleMode) -> u32 {
    match mode {
        PriceScaleMode::Normal => 0,
        PriceScaleMode::Logarithmic => 1,
        PriceScaleMode::Percentage => 2,
        PriceScaleMode::IndexedTo100 => 3,
    }
}

pub fn price_format_from_controls(controls: &SeriesFormatControls) -> PriceFormat {
    let precision = controls.precision.value().round().max(0.0) as usize;
    match controls.format_combo.active() {
        Some(1) => PriceFormat::Percent { precision },
        Some(2) => PriceFormat::Volume { precision },
        _ => {
            let min_move = controls.min_move.value().max(0.0000001);
            PriceFormat::Price { precision, min_move }
        }
    }
}

pub fn price_scale_options_from_controls(
    controls: &PriceScaleSideControls,
) -> PriceScaleOptions {
    PriceScaleOptions {
        visible: controls.visible.state(),
        auto_scale: controls.auto_scale.state(),
        mode: price_scale_mode_from_combo(&controls.mode_combo),
        invert_scale: controls.invert.state(),
        align_labels: controls.align_labels.state(),
        scale_margins: ScaleMargins {
            top: controls.margin_top.value(),
            bottom: controls.margin_bottom.value(),
        },
        border_visible: controls.border_visible.state(),
        border_color: color_from_rgba(controls.border_color.rgba()),
        text_color: color_from_rgba(controls.text_color.rgba()),
        ticks_visible: controls.ticks_visible.state(),
        minimum_width: controls.min_width.value(),
        entire_text_only: controls.entire_text_only.state(),
        ensure_edge_tick_marks_visible: controls.ensure_edge_ticks.state(),
    }
}

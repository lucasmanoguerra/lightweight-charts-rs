use crate::chart::{
    Candle, CandlestickSeriesApi, ChartApi, ChartStyle, Color, CrosshairCenter, CrosshairOptions,
    HandleScaleOptions, HandleScrollOptions, HistogramPoint, HistogramSeriesApi,
    InteractionSensitivityOptions, KineticScrollOptions, LineSeriesApi, LineStyle, MarkerZOrder,
    PriceLineApi, PriceLineOptions, PriceScale, PriceScaleOptions, SeriesMarkersOptions,
    TimeLabelMode, TimeScaleOptions, TooltipOptions, TooltipPosition, TrackingModeOptions,
};
use crate::settings_ui::{
    PriceScaleSideControls, SeriesFormatControls, SeriesLastValueControls, SeriesMarkerControls,
    SeriesPriceLineControls, SeriesPriceLinesControls, SettingsControls,
};
use crate::ui::marker_icons::populate_marker_icon_combo;
use relm4::gtk;
use relm4::gtk::prelude::*;
use serde::{Deserialize, Serialize};
use std::fs;
use std::path::{Path, PathBuf};

use super::helpers::{
    color_from_rgba, crosshair_mode_from_combo, crosshair_mode_to_index, line_style_from_combo,
    price_format_from_controls, price_scale_mode_to_index, price_scale_options_from_controls,
    rgba_from_color,
};
use super::market_data::MarketStore;

#[derive(Clone)]
struct PriceLineEntry {
    api: PriceLineApi,
    options: PriceLineOptions,
}

#[derive(Debug, Serialize, Deserialize)]
struct ColorPreset {
    r: f64,
    g: f64,
    b: f64,
}

#[derive(Debug, Serialize, Deserialize)]
struct PriceLinePreset {
    price: f64,
    color: ColorPreset,
    line_width: f64,
    line_style: String,
    line_opacity: f64,
    line_visible: bool,
    axis_label_visible: bool,
    axis_label_color: Option<ColorPreset>,
    axis_label_text_color: Option<ColorPreset>,
    axis_label_background_alpha: f64,
    axis_label_padding: f64,
    axis_label_radius: f64,
    axis_label_border_color: Option<ColorPreset>,
    axis_label_border_width: f64,
    title: Option<String>,
}

#[derive(Debug, Serialize, Deserialize)]
struct SeriesPriceLineProfile {
    visible: bool,
    style_index: i32,
    width: f64,
    color: ColorPreset,
}

#[derive(Debug, Serialize, Deserialize)]
struct SeriesLastValueProfile {
    visible: bool,
    background: ColorPreset,
    text: ColorPreset,
}

#[derive(Debug, Serialize, Deserialize)]
struct SeriesFormatProfile {
    format_index: i32,
    precision: f64,
    min_move: f64,
}

#[derive(Debug, Serialize, Deserialize)]
struct ChartProfile {
    background: ColorPreset,
    grid_enabled: bool,
    grid_color: ColorPreset,
}

#[derive(Debug, Serialize, Deserialize)]
struct CandleProfile {
    up: ColorPreset,
    down: ColorPreset,
    border_up: ColorPreset,
    border_down: ColorPreset,
    wick_up: ColorPreset,
    wick_down: ColorPreset,
}

#[derive(Debug, Serialize, Deserialize)]
struct SeriesProfile {
    candles_scale_index: i32,
    line_scale_index: i32,
    hist_scale_index: i32,
    line_color: ColorPreset,
    hist_color: ColorPreset,
    hist_follow: bool,
    candles_price_line: SeriesPriceLineProfile,
    candles_last_value: SeriesLastValueProfile,
    line_price_line: SeriesPriceLineProfile,
    line_last_value: SeriesLastValueProfile,
    hist_price_line: SeriesPriceLineProfile,
    hist_last_value: SeriesLastValueProfile,
    candles_format: SeriesFormatProfile,
    line_format: SeriesFormatProfile,
    hist_format: SeriesFormatProfile,
}

#[derive(Debug, Serialize, Deserialize)]
struct PriceScaleSideProfile {
    visible: bool,
    auto_scale: bool,
    mode_index: i32,
    invert: bool,
    align_labels: bool,
    margin_top: f64,
    margin_bottom: f64,
    border_visible: bool,
    border_color: ColorPreset,
    text_color: ColorPreset,
    ticks_visible: bool,
    min_width: f64,
    entire_text_only: bool,
    ensure_edge_ticks: bool,
}

#[derive(Debug, Serialize, Deserialize)]
struct CrosshairProfile {
    mode_index: i32,
    vertical: bool,
    horizontal: bool,
    snap_ohlc: bool,
    snap_series: bool,
    line_style_index: i32,
    width: f64,
    center_index: i32,
    center_size: f64,
    color: ColorPreset,
    center_color: ColorPreset,
}

#[derive(Debug, Serialize, Deserialize)]
struct TimeScaleProfile {
    visible: bool,
    border_visible: bool,
    border_color: ColorPreset,
    ticks_visible: bool,
    time_visible: bool,
    seconds_visible: bool,
    tick_format: String,
    tick_max_len: f64,
    label_mode_index: i32,
    label_format: String,
    bar_spacing: f64,
    min_spacing: f64,
    max_spacing: f64,
    right_offset: f64,
    right_offset_px: f64,
    fix_left: bool,
    fix_right: bool,
    lock_visible_range: bool,
    right_bar_stays: bool,
    shift_on_new_bar: bool,
    uniform_distribution: bool,
    min_height: f64,
}

#[derive(Debug, Serialize, Deserialize)]
struct InteractionProfile {
    scroll_mouse_wheel: bool,
    scroll_pressed_move: bool,
    scroll_horz_touch: bool,
    scroll_vert_touch: bool,
    scale_mouse_wheel: bool,
    scale_pinch: bool,
    scale_axis_time: bool,
    scale_axis_price: bool,
    scale_reset_time: bool,
    scale_reset_price: bool,
    kinetic_mouse: bool,
    kinetic_touch: bool,
    tracking_mode: bool,
    axis_drag_time: f64,
    axis_drag_price: f64,
    wheel_zoom: f64,
    pinch_zoom: f64,
}

#[derive(Debug, Serialize, Deserialize)]
struct TooltipProfile {
    enabled: bool,
    position_index: i32,
    format: String,
    line_format: String,
    hist_format: String,
    background: ColorPreset,
    text: ColorPreset,
}

#[derive(Debug, Serialize, Deserialize)]
struct SettingsProfile {
    chart: ChartProfile,
    candles: CandleProfile,
    series: SeriesProfile,
    price_left: PriceScaleSideProfile,
    price_right: PriceScaleSideProfile,
    crosshair: CrosshairProfile,
    time_scale: TimeScaleProfile,
    interaction: InteractionProfile,
    tooltip: TooltipProfile,
}

fn preset_color(color: Color) -> ColorPreset {
    ColorPreset {
        r: color.r,
        g: color.g,
        b: color.b,
    }
}

fn color_from_preset(preset: ColorPreset) -> Color {
    Color::new(preset.r, preset.g, preset.b)
}

fn preset_from_options(options: &PriceLineOptions) -> PriceLinePreset {
    PriceLinePreset {
        price: options.price,
        color: preset_color(options.color),
        line_width: options.line_width,
        line_style: match options.line_style {
            LineStyle::Solid => "Solid".to_string(),
            LineStyle::Dotted => "Dotted".to_string(),
            LineStyle::Dashed => "Dashed".to_string(),
        },
        line_opacity: options.line_opacity,
        line_visible: options.line_visible,
        axis_label_visible: options.axis_label_visible,
        axis_label_color: options.axis_label_color.map(preset_color),
        axis_label_text_color: options.axis_label_text_color.map(preset_color),
        axis_label_background_alpha: options.axis_label_background_alpha,
        axis_label_padding: options.axis_label_padding,
        axis_label_radius: options.axis_label_radius,
        axis_label_border_color: options.axis_label_border_color.map(preset_color),
        axis_label_border_width: options.axis_label_border_width,
        title: options.title.clone(),
    }
}

fn options_from_preset(preset: PriceLinePreset) -> PriceLineOptions {
    let mut options = PriceLineOptions::default();
    options.price = preset.price;
    options.color = color_from_preset(preset.color);
    options.line_width = preset.line_width;
    options.line_style = match preset.line_style.as_str() {
        "Dotted" => LineStyle::Dotted,
        "Dashed" => LineStyle::Dashed,
        _ => LineStyle::Solid,
    };
    options.line_opacity = preset.line_opacity;
    options.line_visible = preset.line_visible;
    options.axis_label_visible = preset.axis_label_visible;
    options.axis_label_color = preset.axis_label_color.map(color_from_preset);
    options.axis_label_text_color = preset.axis_label_text_color.map(color_from_preset);
    options.axis_label_background_alpha = preset.axis_label_background_alpha;
    options.axis_label_padding = preset.axis_label_padding;
    options.axis_label_radius = preset.axis_label_radius;
    options.axis_label_border_color = preset.axis_label_border_color.map(color_from_preset);
    options.axis_label_border_width = preset.axis_label_border_width;
    options.title = preset.title;
    options
}

fn color_preset_from_button(button: &gtk::ColorButton) -> ColorPreset {
    let rgba = button.rgba();
    ColorPreset {
        r: rgba.red() as f64,
        g: rgba.green() as f64,
        b: rgba.blue() as f64,
    }
}

fn apply_color_preset(button: &gtk::ColorButton, preset: &ColorPreset) {
    button.set_rgba(&rgba_from_color(Color::new(preset.r, preset.g, preset.b)));
}

fn combo_index(combo: &gtk::ComboBoxText) -> i32 {
    combo.active().map(|v| v as i32).unwrap_or(-1)
}

fn set_combo_index(combo: &gtk::ComboBoxText, index: i32) {
    if index >= 0 {
        combo.set_active(Some(index as u32));
    }
}

fn series_price_line_profile(controls: &SeriesPriceLineControls) -> SeriesPriceLineProfile {
    SeriesPriceLineProfile {
        visible: controls.visible.state(),
        style_index: combo_index(&controls.style),
        width: controls.width.value(),
        color: color_preset_from_button(&controls.color),
    }
}

fn apply_series_price_line(controls: &SeriesPriceLineControls, profile: &SeriesPriceLineProfile) {
    controls.visible.set_state(profile.visible);
    set_combo_index(&controls.style, profile.style_index);
    controls.width.set_value(profile.width);
    apply_color_preset(&controls.color, &profile.color);
}

fn series_last_value_profile(controls: &SeriesLastValueControls) -> SeriesLastValueProfile {
    SeriesLastValueProfile {
        visible: controls.visible.state(),
        background: color_preset_from_button(&controls.background),
        text: color_preset_from_button(&controls.text),
    }
}

fn apply_series_last_value(controls: &SeriesLastValueControls, profile: &SeriesLastValueProfile) {
    controls.visible.set_state(profile.visible);
    apply_color_preset(&controls.background, &profile.background);
    apply_color_preset(&controls.text, &profile.text);
}

fn series_format_profile(controls: &SeriesFormatControls) -> SeriesFormatProfile {
    SeriesFormatProfile {
        format_index: combo_index(&controls.format_combo),
        precision: controls.precision.value(),
        min_move: controls.min_move.value(),
    }
}

fn apply_series_format(controls: &SeriesFormatControls, profile: &SeriesFormatProfile) {
    set_combo_index(&controls.format_combo, profile.format_index);
    controls.precision.set_value(profile.precision);
    controls.min_move.set_value(profile.min_move);
}

fn price_scale_profile(controls: &PriceScaleSideControls) -> PriceScaleSideProfile {
    PriceScaleSideProfile {
        visible: controls.visible.state(),
        auto_scale: controls.auto_scale.state(),
        mode_index: combo_index(&controls.mode_combo),
        invert: controls.invert.state(),
        align_labels: controls.align_labels.state(),
        margin_top: controls.margin_top.value(),
        margin_bottom: controls.margin_bottom.value(),
        border_visible: controls.border_visible.state(),
        border_color: color_preset_from_button(&controls.border_color),
        text_color: color_preset_from_button(&controls.text_color),
        ticks_visible: controls.ticks_visible.state(),
        min_width: controls.min_width.value(),
        entire_text_only: controls.entire_text_only.state(),
        ensure_edge_ticks: controls.ensure_edge_ticks.state(),
    }
}

fn apply_price_scale_profile(controls: &PriceScaleSideControls, profile: &PriceScaleSideProfile) {
    controls.visible.set_state(profile.visible);
    controls.auto_scale.set_state(profile.auto_scale);
    set_combo_index(&controls.mode_combo, profile.mode_index);
    controls.invert.set_state(profile.invert);
    controls.align_labels.set_state(profile.align_labels);
    controls.margin_top.set_value(profile.margin_top);
    controls.margin_bottom.set_value(profile.margin_bottom);
    controls.border_visible.set_state(profile.border_visible);
    apply_color_preset(&controls.border_color, &profile.border_color);
    apply_color_preset(&controls.text_color, &profile.text_color);
    controls.ticks_visible.set_state(profile.ticks_visible);
    controls.min_width.set_value(profile.min_width);
    controls.entire_text_only.set_state(profile.entire_text_only);
    controls.ensure_edge_ticks.set_state(profile.ensure_edge_ticks);
}

fn settings_profile_from_controls(controls: &SettingsControls) -> SettingsProfile {
    SettingsProfile {
        chart: ChartProfile {
            background: color_preset_from_button(&controls.chart.background_color),
            grid_enabled: controls.chart.grid_switch.state(),
            grid_color: color_preset_from_button(&controls.chart.grid_color),
        },
        candles: CandleProfile {
            up: color_preset_from_button(&controls.candles.up_color),
            down: color_preset_from_button(&controls.candles.down_color),
            border_up: color_preset_from_button(&controls.candles.border_up_color),
            border_down: color_preset_from_button(&controls.candles.border_down_color),
            wick_up: color_preset_from_button(&controls.candles.wick_up_color),
            wick_down: color_preset_from_button(&controls.candles.wick_down_color),
        },
        series: SeriesProfile {
            candles_scale_index: combo_index(&controls.series.candles_scale_combo),
            line_scale_index: combo_index(&controls.series.line_scale_combo),
            hist_scale_index: combo_index(&controls.series.hist_scale_combo),
            line_color: color_preset_from_button(&controls.series.line_color),
            hist_color: color_preset_from_button(&controls.series.hist_color),
            hist_follow: controls.series.hist_follow_candle_colors.state(),
            candles_price_line: series_price_line_profile(&controls.series.candles_price_line),
            candles_last_value: series_last_value_profile(&controls.series.candles_last_value),
            line_price_line: series_price_line_profile(&controls.series.line_price_line),
            line_last_value: series_last_value_profile(&controls.series.line_last_value),
            hist_price_line: series_price_line_profile(&controls.series.hist_price_line),
            hist_last_value: series_last_value_profile(&controls.series.hist_last_value),
            candles_format: series_format_profile(&controls.series.candles_format),
            line_format: series_format_profile(&controls.series.line_format),
            hist_format: series_format_profile(&controls.series.hist_format),
        },
        price_left: price_scale_profile(&controls.price_scale.left),
        price_right: price_scale_profile(&controls.price_scale.right),
        crosshair: CrosshairProfile {
            mode_index: combo_index(&controls.crosshair.mode_combo),
            vertical: controls.crosshair.vertical_switch.state(),
            horizontal: controls.crosshair.horizontal_switch.state(),
            snap_ohlc: controls.crosshair.snap_ohlc.state(),
            snap_series: controls.crosshair.snap_series.state(),
            line_style_index: combo_index(&controls.crosshair.line_style),
            width: controls.crosshair.width.value(),
            center_index: combo_index(&controls.crosshair.center_combo),
            center_size: controls.crosshair.center_size.value(),
            color: color_preset_from_button(&controls.crosshair.color),
            center_color: color_preset_from_button(&controls.crosshair.center_color),
        },
        time_scale: TimeScaleProfile {
            visible: controls.time.visible_switch.state(),
            border_visible: controls.time.border_visible.state(),
            border_color: color_preset_from_button(&controls.time.border_color),
            ticks_visible: controls.time.ticks_visible.state(),
            time_visible: controls.time.time_visible.state(),
            seconds_visible: controls.time.seconds_visible.state(),
            tick_format: controls.time.tick_format_entry.text().to_string(),
            tick_max_len: controls.time.tick_max_len.value(),
            label_mode_index: combo_index(&controls.time.label_mode_combo),
            label_format: controls.time.label_format_entry.text().to_string(),
            bar_spacing: controls.time.bar_spacing.value(),
            min_spacing: controls.time.min_spacing.value(),
            max_spacing: controls.time.max_spacing.value(),
            right_offset: controls.time.right_offset.value(),
            right_offset_px: controls.time.right_offset_px.value(),
            fix_left: controls.time.fix_left.state(),
            fix_right: controls.time.fix_right.state(),
            lock_visible_range: controls.time.lock_visible_range.state(),
            right_bar_stays: controls.time.right_bar_stays.state(),
            shift_on_new_bar: controls.time.shift_on_new_bar.state(),
            uniform_distribution: controls.time.uniform_distribution.state(),
            min_height: controls.time.min_height.value(),
        },
        interaction: InteractionProfile {
            scroll_mouse_wheel: controls.interaction.scroll_mouse_wheel.state(),
            scroll_pressed_move: controls.interaction.scroll_pressed_move.state(),
            scroll_horz_touch: controls.interaction.scroll_horz_touch.state(),
            scroll_vert_touch: controls.interaction.scroll_vert_touch.state(),
            scale_mouse_wheel: controls.interaction.mouse_wheel.state(),
            scale_pinch: controls.interaction.pinch.state(),
            scale_axis_time: controls.interaction.axis_move_time.state(),
            scale_axis_price: controls.interaction.axis_move_price.state(),
            scale_reset_time: controls.interaction.axis_reset_time.state(),
            scale_reset_price: controls.interaction.axis_reset_price.state(),
            kinetic_mouse: controls.interaction.kinetic_mouse.state(),
            kinetic_touch: controls.interaction.kinetic_touch.state(),
            tracking_mode: controls.interaction.tracking_mode.state(),
            axis_drag_time: controls.interaction.axis_drag_time.value(),
            axis_drag_price: controls.interaction.axis_drag_price.value(),
            wheel_zoom: controls.interaction.wheel_zoom.value(),
            pinch_zoom: controls.interaction.pinch_zoom.value(),
        },
        tooltip: TooltipProfile {
            enabled: controls.tooltip.enabled.state(),
            position_index: combo_index(&controls.tooltip.position),
            format: controls.tooltip.format.text().to_string(),
            line_format: controls.tooltip.line_format.text().to_string(),
            hist_format: controls.tooltip.hist_format.text().to_string(),
            background: color_preset_from_button(&controls.tooltip.background),
            text: color_preset_from_button(&controls.tooltip.text),
        },
    }
}

fn apply_settings_profile(controls: &SettingsControls, profile: &SettingsProfile) {
    apply_color_preset(&controls.chart.background_color, &profile.chart.background);
    controls.chart.grid_switch.set_state(profile.chart.grid_enabled);
    apply_color_preset(&controls.chart.grid_color, &profile.chart.grid_color);

    apply_color_preset(&controls.candles.up_color, &profile.candles.up);
    apply_color_preset(&controls.candles.down_color, &profile.candles.down);
    apply_color_preset(&controls.candles.border_up_color, &profile.candles.border_up);
    apply_color_preset(&controls.candles.border_down_color, &profile.candles.border_down);
    apply_color_preset(&controls.candles.wick_up_color, &profile.candles.wick_up);
    apply_color_preset(&controls.candles.wick_down_color, &profile.candles.wick_down);

    set_combo_index(&controls.series.candles_scale_combo, profile.series.candles_scale_index);
    set_combo_index(&controls.series.line_scale_combo, profile.series.line_scale_index);
    set_combo_index(&controls.series.hist_scale_combo, profile.series.hist_scale_index);
    apply_color_preset(&controls.series.line_color, &profile.series.line_color);
    apply_color_preset(&controls.series.hist_color, &profile.series.hist_color);
    controls
        .series
        .hist_follow_candle_colors
        .set_state(profile.series.hist_follow);

    apply_series_price_line(&controls.series.candles_price_line, &profile.series.candles_price_line);
    apply_series_last_value(
        &controls.series.candles_last_value,
        &profile.series.candles_last_value,
    );
    apply_series_price_line(&controls.series.line_price_line, &profile.series.line_price_line);
    apply_series_last_value(&controls.series.line_last_value, &profile.series.line_last_value);
    apply_series_price_line(&controls.series.hist_price_line, &profile.series.hist_price_line);
    apply_series_last_value(&controls.series.hist_last_value, &profile.series.hist_last_value);

    apply_series_format(&controls.series.candles_format, &profile.series.candles_format);
    apply_series_format(&controls.series.line_format, &profile.series.line_format);
    apply_series_format(&controls.series.hist_format, &profile.series.hist_format);

    apply_price_scale_profile(&controls.price_scale.left, &profile.price_left);
    apply_price_scale_profile(&controls.price_scale.right, &profile.price_right);

    set_combo_index(&controls.crosshair.mode_combo, profile.crosshair.mode_index);
    controls.crosshair.vertical_switch.set_state(profile.crosshair.vertical);
    controls.crosshair.horizontal_switch.set_state(profile.crosshair.horizontal);
    controls.crosshair.snap_ohlc.set_state(profile.crosshair.snap_ohlc);
    controls.crosshair.snap_series.set_state(profile.crosshair.snap_series);
    set_combo_index(&controls.crosshair.line_style, profile.crosshair.line_style_index);
    controls.crosshair.width.set_value(profile.crosshair.width);
    set_combo_index(&controls.crosshair.center_combo, profile.crosshair.center_index);
    controls.crosshair.center_size.set_value(profile.crosshair.center_size);
    apply_color_preset(&controls.crosshair.color, &profile.crosshair.color);
    apply_color_preset(&controls.crosshair.center_color, &profile.crosshair.center_color);

    controls.time.visible_switch.set_state(profile.time_scale.visible);
    controls.time.border_visible.set_state(profile.time_scale.border_visible);
    apply_color_preset(&controls.time.border_color, &profile.time_scale.border_color);
    controls.time.ticks_visible.set_state(profile.time_scale.ticks_visible);
    controls.time.time_visible.set_state(profile.time_scale.time_visible);
    controls.time.seconds_visible.set_state(profile.time_scale.seconds_visible);
    controls
        .time
        .tick_format_entry
        .set_text(&profile.time_scale.tick_format);
    controls.time.tick_max_len.set_value(profile.time_scale.tick_max_len);
    set_combo_index(&controls.time.label_mode_combo, profile.time_scale.label_mode_index);
    controls
        .time
        .label_format_entry
        .set_text(&profile.time_scale.label_format);
    controls.time.bar_spacing.set_value(profile.time_scale.bar_spacing);
    controls.time.min_spacing.set_value(profile.time_scale.min_spacing);
    controls.time.max_spacing.set_value(profile.time_scale.max_spacing);
    controls.time.right_offset.set_value(profile.time_scale.right_offset);
    controls.time.right_offset_px.set_value(profile.time_scale.right_offset_px);
    controls.time.fix_left.set_state(profile.time_scale.fix_left);
    controls.time.fix_right.set_state(profile.time_scale.fix_right);
    controls
        .time
        .lock_visible_range
        .set_state(profile.time_scale.lock_visible_range);
    controls.time.right_bar_stays.set_state(profile.time_scale.right_bar_stays);
    controls
        .time
        .shift_on_new_bar
        .set_state(profile.time_scale.shift_on_new_bar);
    controls
        .time
        .uniform_distribution
        .set_state(profile.time_scale.uniform_distribution);
    controls.time.min_height.set_value(profile.time_scale.min_height);

    controls
        .interaction
        .scroll_mouse_wheel
        .set_state(profile.interaction.scroll_mouse_wheel);
    controls
        .interaction
        .scroll_pressed_move
        .set_state(profile.interaction.scroll_pressed_move);
    controls
        .interaction
        .scroll_horz_touch
        .set_state(profile.interaction.scroll_horz_touch);
    controls
        .interaction
        .scroll_vert_touch
        .set_state(profile.interaction.scroll_vert_touch);
    controls
        .interaction
        .mouse_wheel
        .set_state(profile.interaction.scale_mouse_wheel);
    controls
        .interaction
        .pinch
        .set_state(profile.interaction.scale_pinch);
    controls
        .interaction
        .axis_move_time
        .set_state(profile.interaction.scale_axis_time);
    controls
        .interaction
        .axis_move_price
        .set_state(profile.interaction.scale_axis_price);
    controls
        .interaction
        .axis_reset_time
        .set_state(profile.interaction.scale_reset_time);
    controls
        .interaction
        .axis_reset_price
        .set_state(profile.interaction.scale_reset_price);
    controls
        .interaction
        .kinetic_mouse
        .set_state(profile.interaction.kinetic_mouse);
    controls
        .interaction
        .kinetic_touch
        .set_state(profile.interaction.kinetic_touch);
    controls
        .interaction
        .tracking_mode
        .set_state(profile.interaction.tracking_mode);
    controls
        .interaction
        .axis_drag_time
        .set_value(profile.interaction.axis_drag_time);
    controls
        .interaction
        .axis_drag_price
        .set_value(profile.interaction.axis_drag_price);
    controls
        .interaction
        .wheel_zoom
        .set_value(profile.interaction.wheel_zoom);
    controls
        .interaction
        .pinch_zoom
        .set_value(profile.interaction.pinch_zoom);

    controls.tooltip.enabled.set_state(profile.tooltip.enabled);
    set_combo_index(&controls.tooltip.position, profile.tooltip.position_index);
    controls.tooltip.format.set_text(&profile.tooltip.format);
    controls
        .tooltip
        .line_format
        .set_text(&profile.tooltip.line_format);
    controls
        .tooltip
        .hist_format
        .set_text(&profile.tooltip.hist_format);
    apply_color_preset(&controls.tooltip.background, &profile.tooltip.background);
    apply_color_preset(&controls.tooltip.text, &profile.tooltip.text);
}

fn profiles_folder(controls: &SettingsControls) -> PathBuf {
    let text = controls.profiles.folder_entry.text().to_string();
    if text.trim().is_empty() {
        PathBuf::from("profiles")
    } else {
        PathBuf::from(text)
    }
}

fn profile_path(folder: &Path, name: &str) -> PathBuf {
    folder.join(format!("{name}.toml"))
}

fn refresh_profiles_list(controls: &SettingsControls) {
    let folder = profiles_folder(controls);
    controls.profiles.profiles_combo.remove_all();
    let Ok(entries) = fs::read_dir(&folder) else { return };
    let mut names: Vec<String> = Vec::new();
    for entry in entries.flatten() {
        let path = entry.path();
        if path.extension().and_then(|s| s.to_str()) != Some("toml") {
            continue;
        }
        if let Some(stem) = path.file_stem().and_then(|s| s.to_str()) {
            names.push(stem.to_string());
        }
    }
    names.sort();
    for name in names {
        controls.profiles.profiles_combo.append_text(&name);
    }
    if controls.profiles.profiles_combo.active().is_none() {
        controls.profiles.profiles_combo.set_active(Some(0));
    }
}

fn marker_z_order_from_combo(combo: &gtk::ComboBoxText) -> MarkerZOrder {
    match combo.active() {
        Some(1) => MarkerZOrder::Top,
        Some(2) => MarkerZOrder::Bottom,
        _ => MarkerZOrder::Normal,
    }
}

fn marker_z_order_to_index(order: MarkerZOrder) -> u32 {
    match order {
        MarkerZOrder::Normal => 0,
        MarkerZOrder::Top => 1,
        MarkerZOrder::Bottom => 2,
    }
}

fn marker_default_icon_from_controls(controls: &SeriesMarkerControls) -> Option<String> {
    let custom = controls.icon_entry.text().to_string();
    if !custom.trim().is_empty() {
        return Some(custom);
    }
    let text = controls.icon_combo.active_text()?;
    let text = text.as_str();
    if text == "None" {
        None
    } else {
        Some(text.to_string())
    }
}

fn marker_icon_color_from_controls(controls: &SeriesMarkerControls) -> Option<Color> {
    Some(color_from_rgba(controls.icon_color.rgba()))
}

fn recolor_histogram_points(
    candles: &[Candle],
    volumes: &[HistogramPoint],
    up: Color,
    down: Color,
) -> Vec<HistogramPoint> {
    let count = candles.len().min(volumes.len());
    let mut points = Vec::with_capacity(count);
    for idx in 0..count {
        let candle = &candles[idx];
        let volume = &volumes[idx];
        let color = if candle.close >= candle.open { up } else { down };
        points.push(HistogramPoint {
            time: volume.time,
            value: volume.value,
            color: Some(color),
        });
    }
    points
}

fn strip_histogram_colors(volumes: &[HistogramPoint]) -> Vec<HistogramPoint> {
    volumes
        .iter()
        .map(|point| HistogramPoint {
            time: point.time,
            value: point.value,
            color: None,
        })
        .collect()
}

fn refresh_price_line_selector(combo: &gtk::ComboBoxText, lines: &[PriceLineEntry]) {
    combo.remove_all();
    for (idx, line) in lines.iter().enumerate() {
        let label = match &line.options.title {
            Some(title) if !title.is_empty() => format!("{}: {}", idx + 1, title),
            _ => format!("Line {}", idx + 1),
        };
        combo.append_text(&label);
    }
    if !lines.is_empty() {
        combo.set_active(Some(0));
    }
}

fn set_price_line_controls_from_options(
    controls: &SeriesPriceLinesControls,
    options: &PriceLineOptions,
) {
    controls.price.set_value(options.price);
    controls.line_visible.set_state(options.line_visible);
    controls.axis_label_visible.set_state(options.axis_label_visible);
    controls
        .style
        .set_active(Some(match options.line_style {
            LineStyle::Solid => 0,
            LineStyle::Dotted => 1,
            LineStyle::Dashed => 2,
        }));
    controls.width.set_value(options.line_width);
    controls.line_opacity.set_value(options.line_opacity);
    controls.color.set_rgba(&rgba_from_color(options.color));
    if let Some(color) = options.axis_label_color {
        controls.label_color.set_rgba(&rgba_from_color(color));
    }
    if let Some(color) = options.axis_label_text_color {
        controls.label_text_color.set_rgba(&rgba_from_color(color));
    }
    controls
        .label_alpha
        .set_value(options.axis_label_background_alpha);
    controls.label_padding.set_value(options.axis_label_padding);
    controls.label_radius.set_value(options.axis_label_radius);
    if let Some(color) = options.axis_label_border_color {
        controls.label_border_color.set_rgba(&rgba_from_color(color));
    }
    controls
        .label_border_width
        .set_value(options.axis_label_border_width);
    if let Some(title) = &options.title {
        controls.title.set_text(title);
    } else {
        controls.title.set_text("");
    }
}

fn price_line_options_from_controls(
    controls: &SeriesPriceLinesControls,
    current: &PriceLineOptions,
) -> PriceLineOptions {
    let mut options = current.clone();
    options.price = controls.price.value();
    options.line_visible = controls.line_visible.state();
    options.axis_label_visible = controls.axis_label_visible.state();
    options.line_style = line_style_from_combo(&controls.style);
    options.line_width = controls.width.value();
    options.line_opacity = controls.line_opacity.value();
    options.color = color_from_rgba(controls.color.rgba());
    options.axis_label_color = Some(color_from_rgba(controls.label_color.rgba()));
    options.axis_label_text_color = Some(color_from_rgba(controls.label_text_color.rgba()));
    options.axis_label_background_alpha = controls.label_alpha.value();
    options.axis_label_padding = controls.label_padding.value();
    options.axis_label_radius = controls.label_radius.value();
    options.axis_label_border_color = Some(color_from_rgba(controls.label_border_color.rgba()));
    options.axis_label_border_width = controls.label_border_width.value();
    let title_text = controls.title.text().to_string();
    options.title = if title_text.trim().is_empty() {
        None
    } else {
        Some(title_text)
    };
    options
}

fn set_price_lines_controls_sensitive(controls: &SeriesPriceLinesControls, enabled: bool) {
    controls.selector.set_sensitive(enabled);
    controls.remove_button.set_sensitive(enabled);
    controls.export_button.set_sensitive(enabled);
    controls.import_button.set_sensitive(true);
    controls.price.set_sensitive(enabled);
    controls.line_visible.set_sensitive(enabled);
    controls.axis_label_visible.set_sensitive(enabled);
    controls.style.set_sensitive(enabled);
    controls.width.set_sensitive(enabled);
    controls.line_opacity.set_sensitive(enabled);
    controls.color.set_sensitive(enabled);
    controls.label_color.set_sensitive(enabled);
    controls.label_text_color.set_sensitive(enabled);
    controls.label_alpha.set_sensitive(enabled);
    controls.label_padding.set_sensitive(enabled);
    controls.label_radius.set_sensitive(enabled);
    controls.label_border_color.set_sensitive(enabled);
    controls.label_border_width.set_sensitive(enabled);
    controls.title.set_sensitive(enabled);
}

pub fn wire_chart_draw(drawing_area: &gtk::DrawingArea, chart: ChartApi) {
    drawing_area.set_draw_func({
        let chart = chart.clone();
        move |_, cr, width, height| {
            chart.draw(cr, width as f64, height as f64);
        }
    });
}

fn wire_price_lines_controls(
    drawing_area: &gtk::DrawingArea,
    controls: &SeriesPriceLinesControls,
    create_line: std::rc::Rc<dyn Fn(PriceLineOptions) -> PriceLineApi>,
) {
    let lines = std::rc::Rc::new(std::cell::RefCell::new(Vec::<PriceLineEntry>::new()));
    controls.style.set_active(Some(0));
    set_price_lines_controls_sensitive(controls, false);

    controls.add_button.connect_clicked({
        let lines = lines.clone();
        let selector = controls.selector.clone();
        let controls = controls.clone();
        let drawing_area = drawing_area.clone();
        let create_line = create_line.clone();
        move |_| {
            let mut options = PriceLineOptions::default();
            let index = lines.borrow().len();
            options.title = Some(format!("Line {}", index + 1));
            let api = (create_line)(options.clone());
            lines.borrow_mut().push(PriceLineEntry { api, options: options.clone() });
            refresh_price_line_selector(&selector, &lines.borrow());
            selector.set_active(Some(index as u32));
            set_price_line_controls_from_options(&controls, &options);
            set_price_lines_controls_sensitive(&controls, true);
            drawing_area.queue_draw();
        }
    });

    controls.export_button.connect_clicked({
        let lines = lines.clone();
        let selector = controls.selector.clone();
        move |_| {
            let presets: Vec<PriceLinePreset> = lines
                .borrow()
                .iter()
                .map(|entry| preset_from_options(&entry.options))
                .collect();
            if let Ok(text) = serde_json::to_string_pretty(&presets) {
                selector.clipboard().set_text(&text);
            }
        }
    });

    controls.import_button.connect_clicked({
        let lines = lines.clone();
        let selector = controls.selector.clone();
        let controls = controls.clone();
        let drawing_area = drawing_area.clone();
        let create_line = create_line.clone();
        move |_| {
            let lines = lines.clone();
            let selector = selector.clone();
            let controls = controls.clone();
            let drawing_area = drawing_area.clone();
            let create_line = create_line.clone();
            let clipboard = selector.clipboard();
            clipboard.read_text_async(None::<&gtk::gio::Cancellable>, move |result| {
                let Ok(Some(text)) = result else {
                    return;
                };
                let Ok(presets) = serde_json::from_str::<Vec<PriceLinePreset>>(&text) else {
                    return;
                };

                let mut lines_mut = lines.borrow_mut();
                for entry in lines_mut.drain(..) {
                    entry.api.remove();
                }

                for preset in presets {
                    let options = options_from_preset(preset);
                    let api = (create_line)(options.clone());
                    lines_mut.push(PriceLineEntry { api, options });
                }

                refresh_price_line_selector(&selector, &lines_mut);
                if let Some(index) = selector.active() {
                    if let Some(entry) = lines_mut.get(index as usize) {
                        set_price_line_controls_from_options(&controls, &entry.options);
                        set_price_lines_controls_sensitive(&controls, true);
                    }
                } else {
                    set_price_lines_controls_sensitive(&controls, false);
                }
                drawing_area.queue_draw();
            });
        }
    });

    controls.remove_button.connect_clicked({
        let lines = lines.clone();
        let selector = controls.selector.clone();
        let controls = controls.clone();
        let drawing_area = drawing_area.clone();
        move |_| {
            let index = match selector.active() {
                Some(index) => index as usize,
                None => return,
            };
            let mut lines_mut = lines.borrow_mut();
            if index >= lines_mut.len() {
                return;
            }
            let entry = lines_mut.remove(index);
            entry.api.remove();
            refresh_price_line_selector(&selector, &lines_mut);
            if lines_mut.is_empty() {
                set_price_lines_controls_sensitive(&controls, false);
            } else {
                let new_index = index.min(lines_mut.len() - 1);
                selector.set_active(Some(new_index as u32));
                set_price_line_controls_from_options(&controls, &lines_mut[new_index].options);
                set_price_lines_controls_sensitive(&controls, true);
            }
            drawing_area.queue_draw();
        }
    });

    controls.selector.connect_changed({
        let lines = lines.clone();
        let controls = controls.clone();
        move |combo| {
            let index = match combo.active() {
                Some(index) => index as usize,
                None => return,
            };
            let lines_ref = lines.borrow();
            if let Some(entry) = lines_ref.get(index) {
                set_price_line_controls_from_options(&controls, &entry.options);
                set_price_lines_controls_sensitive(&controls, true);
            }
        }
    });

    let apply_current = std::rc::Rc::new({
        let lines = lines.clone();
        let controls = controls.clone();
        let drawing_area = drawing_area.clone();
        move || {
            let index = match controls.selector.active() {
                Some(index) => index as usize,
                None => return,
            };
            let mut lines_mut = lines.borrow_mut();
            if let Some(entry) = lines_mut.get_mut(index) {
                let options = price_line_options_from_controls(&controls, &entry.options);
                entry.options = options.clone();
                entry.api.apply_options(options);
                refresh_price_line_selector(&controls.selector, &lines_mut);
                controls.selector.set_active(Some(index as u32));
                drawing_area.queue_draw();
            }
        }
    });

    controls.price.connect_value_changed({
        let apply_current = apply_current.clone();
        move |_| apply_current()
    });
    controls.line_visible.connect_state_notify({
        let apply_current = apply_current.clone();
        move |_| apply_current()
    });
    controls.axis_label_visible.connect_state_notify({
        let apply_current = apply_current.clone();
        move |_| apply_current()
    });
    controls.style.connect_changed({
        let apply_current = apply_current.clone();
        move |_| apply_current()
    });
    controls.width.connect_value_changed({
        let apply_current = apply_current.clone();
        move |_| apply_current()
    });
    controls.line_opacity.connect_value_changed({
        let apply_current = apply_current.clone();
        move |_| apply_current()
    });
    controls.color.connect_color_set({
        let apply_current = apply_current.clone();
        move |_| apply_current()
    });
    controls.label_color.connect_color_set({
        let apply_current = apply_current.clone();
        move |_| apply_current()
    });
    controls.label_text_color.connect_color_set({
        let apply_current = apply_current.clone();
        move |_| apply_current()
    });
    controls.label_alpha.connect_value_changed({
        let apply_current = apply_current.clone();
        move |_| apply_current()
    });
    controls.label_padding.connect_value_changed({
        let apply_current = apply_current.clone();
        move |_| apply_current()
    });
    controls.label_radius.connect_value_changed({
        let apply_current = apply_current.clone();
        move |_| apply_current()
    });
    controls.label_border_color.connect_color_set({
        let apply_current = apply_current.clone();
        move |_| apply_current()
    });
    controls.label_border_width.connect_value_changed({
        let apply_current = apply_current.clone();
        move |_| apply_current()
    });
    controls.title.connect_changed({
        let apply_current = apply_current.clone();
        move |_| apply_current()
    });
}

pub fn wire_settings_panel(
    drawing_area: &gtk::DrawingArea,
    chart: ChartApi,
    candle_series: CandlestickSeriesApi,
    line_series: LineSeriesApi,
    hist_series: HistogramSeriesApi,
    settings: SettingsControls,
    store: std::rc::Rc<std::cell::RefCell<MarketStore>>,
    auto_scale_left_button: gtk::ToggleButton,
    auto_scale_right_button: gtk::ToggleButton,
) {
    let style = ChartStyle::default();
    let tooltip_defaults = TooltipOptions::default();
    let time_defaults = TimeScaleOptions::default();
    let crosshair_defaults = CrosshairOptions::default();
    let handle_defaults = HandleScaleOptions::default();
    let handle_scroll_defaults = HandleScrollOptions::default();
    let kinetic_defaults = KineticScrollOptions::default();
    let tracking_defaults = TrackingModeOptions::default();
    let sensitivity_defaults = InteractionSensitivityOptions::default();
    let left_price_defaults = PriceScaleOptions {
        visible: false,
        ..PriceScaleOptions::default()
    };
    let right_price_defaults = PriceScaleOptions::default();

    let chart_controls = settings.chart.clone();
    let candle_controls = settings.candles.clone();
    let series_controls = settings.series.clone();
    let price_scale_controls = settings.price_scale.clone();
    let crosshair_controls = settings.crosshair.clone();
    let time_controls = settings.time.clone();
    let interaction_controls = settings.interaction.clone();
    let tooltip_controls = settings.tooltip.clone();
    let profiles_controls = settings.profiles.clone();
    let store = store.clone();

    chart_controls
        .background_color
        .set_rgba(&rgba_from_color(style.background));
    chart_controls
        .grid_color
        .set_rgba(&rgba_from_color(style.grid));
    chart_controls.grid_switch.set_state(true);

    candle_controls.up_color.set_rgba(&rgba_from_color(style.up));
    candle_controls
        .down_color
        .set_rgba(&rgba_from_color(style.down));
    candle_controls
        .border_up_color
        .set_rgba(&rgba_from_color(style.border_up));
    candle_controls
        .border_down_color
        .set_rgba(&rgba_from_color(style.border_down));
    candle_controls
        .wick_up_color
        .set_rgba(&rgba_from_color(style.wick_up));
    candle_controls
        .wick_down_color
        .set_rgba(&rgba_from_color(style.wick_down));

    series_controls.candles_scale_combo.append_text("Right");
    series_controls.candles_scale_combo.append_text("Left");
    series_controls.candles_scale_combo.set_active(Some(0));

    series_controls.line_scale_combo.append_text("Left");
    series_controls.line_scale_combo.append_text("Right");
    series_controls.line_scale_combo.set_active(Some(0));

    series_controls.hist_scale_combo.append_text("Left");
    series_controls.hist_scale_combo.append_text("Right");
    series_controls.hist_scale_combo.set_active(Some(0));

    series_controls
        .line_color
        .set_rgba(&rgba_from_color(style.line));
    series_controls
        .hist_color
        .set_rgba(&rgba_from_color(style.histogram));
    series_controls
        .hist_follow_candle_colors
        .set_state(true);

    for combo in [
        &series_controls.candles_price_line.style,
        &series_controls.line_price_line.style,
        &series_controls.hist_price_line.style,
    ] {
        combo.append_text("Solid");
        combo.append_text("Dotted");
        combo.append_text("Dashed");
    }
    series_controls.candles_price_line.style.set_active(Some(0));
    series_controls.line_price_line.style.set_active(Some(2));
    series_controls.hist_price_line.style.set_active(Some(0));

    for spin in [
        &series_controls.candles_price_line.width,
        &series_controls.line_price_line.width,
        &series_controls.hist_price_line.width,
    ] {
        spin.set_range(0.5, 6.0);
        spin.set_increments(0.5, 1.0);
    }
    series_controls.candles_price_line.width.set_value(1.0);
    series_controls.line_price_line.width.set_value(1.5);
    series_controls.hist_price_line.width.set_value(1.0);

    series_controls.candles_price_line.visible.set_state(true);
    series_controls.candles_last_value.visible.set_state(true);
    series_controls
        .candles_price_line
        .color
        .set_rgba(&rgba_from_color(style.up));
    series_controls
        .candles_last_value
        .background
        .set_rgba(&rgba_from_color(style.up));
    series_controls
        .candles_last_value
        .text
        .set_rgba(&rgba_from_color(Color::new(0.95, 0.96, 0.98)));

    series_controls.line_price_line.visible.set_state(true);
    series_controls.line_last_value.visible.set_state(true);
    series_controls
        .line_price_line
        .color
        .set_rgba(&rgba_from_color(Color::new(0.33, 0.62, 0.98)));
    series_controls
        .line_last_value
        .background
        .set_rgba(&rgba_from_color(Color::new(0.18, 0.28, 0.4)));
    series_controls
        .line_last_value
        .text
        .set_rgba(&rgba_from_color(Color::new(0.95, 0.96, 0.98)));

    series_controls.hist_price_line.visible.set_state(true);
    series_controls.hist_last_value.visible.set_state(true);
    series_controls
        .hist_price_line
        .color
        .set_rgba(&rgba_from_color(style.histogram));
    series_controls
        .hist_last_value
        .background
        .set_rgba(&rgba_from_color(style.histogram));
    series_controls
        .hist_last_value
        .text
        .set_rgba(&rgba_from_color(Color::new(0.95, 0.96, 0.98)));

    for format in [
        &series_controls.candles_format,
        &series_controls.line_format,
        &series_controls.hist_format,
    ] {
        format.format_combo.append_text("Price");
        format.format_combo.append_text("Percent");
        format.format_combo.append_text("Volume");
        format.format_combo.set_active(Some(0));
        format.precision.set_range(0.0, 8.0);
        format.precision.set_increments(1.0, 1.0);
        format.precision.set_value(2.0);
        format.min_move.set_range(0.0001, 1000.0);
        format.min_move.set_increments(0.01, 0.1);
        format.min_move.set_value(0.01);
        format.min_move.set_sensitive(true);
    }

    let marker_defaults = SeriesMarkersOptions::default();
    for markers in [
        &series_controls.markers.candles,
        &series_controls.markers.line,
        &series_controls.markers.hist,
    ] {
        markers.auto_scale.set_state(marker_defaults.auto_scale);
        markers
            .z_order
            .set_active(Some(marker_z_order_to_index(marker_defaults.z_order)));
        markers.icon_category.set_active(Some(0));
        populate_marker_icon_combo(&markers.icon_combo, 0);
        let default_icon_color = marker_defaults
            .default_icon_text_color
            .unwrap_or(style.axis_text);
        markers
            .icon_color
            .set_rgba(&rgba_from_color(default_icon_color));
        markers.icon_entry.set_text("");
    }

    crosshair_controls.mode_combo.append_text("Normal");
    crosshair_controls.mode_combo.append_text("Magnet");
    crosshair_controls.mode_combo.append_text("Magnet OHLC");
    crosshair_controls.mode_combo.append_text("Hidden");
    crosshair_controls
        .mode_combo
        .set_active(Some(crosshair_mode_to_index(crosshair_defaults.mode)));

    crosshair_controls
        .vertical_switch
        .set_state(crosshair_defaults.show_vertical);
    crosshair_controls
        .horizontal_switch
        .set_state(crosshair_defaults.show_horizontal);
    crosshair_controls
        .snap_ohlc
        .set_state(crosshair_defaults.snap_to_ohlc);
    crosshair_controls
        .snap_series
        .set_state(crosshair_defaults.snap_to_series);

    crosshair_controls.line_style.append_text("Solid");
    crosshair_controls.line_style.append_text("Dotted");
    crosshair_controls.line_style.append_text("Dashed");
    crosshair_controls
        .line_style
        .set_active(Some(match crosshair_defaults.line_style {
            LineStyle::Solid => 0,
            LineStyle::Dotted => 1,
            LineStyle::Dashed => 2,
        }));
    crosshair_controls.width.set_range(0.5, 6.0);
    crosshair_controls.width.set_increments(0.5, 1.0);
    crosshair_controls
        .width
        .set_value(crosshair_defaults.line_width);

    crosshair_controls.center_combo.append_text("Cross");
    crosshair_controls.center_combo.append_text("Dot");
    crosshair_controls.center_combo.append_text("Circle");
    crosshair_controls
        .center_combo
        .set_active(Some(match crosshair_defaults.center {
            CrosshairCenter::Cross => 0,
            CrosshairCenter::Dot => 1,
            CrosshairCenter::Circle => 2,
        }));
    crosshair_controls.center_size.set_range(2.0, 16.0);
    crosshair_controls.center_size.set_increments(0.5, 1.0);
    crosshair_controls
        .center_size
        .set_value(crosshair_defaults.center_size);
    crosshair_controls
        .color
        .set_rgba(&rgba_from_color(style.crosshair));
    crosshair_controls
        .center_color
        .set_rgba(&rgba_from_color(crosshair_defaults.center_color));

    for combo in [
        &price_scale_controls.left.mode_combo,
        &price_scale_controls.right.mode_combo,
    ] {
        combo.append_text("Normal");
        combo.append_text("Logarithmic");
        combo.append_text("Percentage");
        combo.append_text("Indexed to 100");
    }

    let init_price_scale_controls =
        |controls: &PriceScaleSideControls, defaults: PriceScaleOptions| {
            controls.visible.set_state(defaults.visible);
            controls.auto_scale.set_state(defaults.auto_scale);
            controls
                .mode_combo
                .set_active(Some(price_scale_mode_to_index(defaults.mode)));
            controls.invert.set_state(defaults.invert_scale);
            controls.align_labels.set_state(defaults.align_labels);
            controls.margin_top.set_range(0.0, 0.5);
            controls.margin_top.set_increments(0.01, 0.05);
            controls.margin_top.set_value(defaults.scale_margins.top);
            controls.margin_bottom.set_range(0.0, 0.5);
            controls.margin_bottom.set_increments(0.01, 0.05);
            controls
                .margin_bottom
                .set_value(defaults.scale_margins.bottom);
            controls.border_visible.set_state(defaults.border_visible);
            controls
                .border_color
                .set_rgba(&rgba_from_color(defaults.border_color));
            controls
                .text_color
                .set_rgba(&rgba_from_color(defaults.text_color));
            controls.ticks_visible.set_state(defaults.ticks_visible);
            controls.min_width.set_range(0.0, 200.0);
            controls.min_width.set_increments(1.0, 10.0);
            controls.min_width.set_value(defaults.minimum_width);
            controls.entire_text_only.set_state(defaults.entire_text_only);
            controls
                .ensure_edge_ticks
                .set_state(defaults.ensure_edge_tick_marks_visible);
        };

    init_price_scale_controls(&price_scale_controls.left, left_price_defaults);
    init_price_scale_controls(&price_scale_controls.right, right_price_defaults);
    auto_scale_left_button.set_active(price_scale_controls.left.auto_scale.state());
    auto_scale_right_button.set_active(price_scale_controls.right.auto_scale.state());
    auto_scale_left_button.set_visible(price_scale_controls.left.visible.state());
    auto_scale_right_button.set_visible(price_scale_controls.right.visible.state());

    time_controls
        .visible_switch
        .set_state(time_defaults.visible);
    time_controls
        .border_visible
        .set_state(time_defaults.border_visible);
    time_controls
        .border_color
        .set_rgba(&rgba_from_color(time_defaults.border_color));
    time_controls
        .ticks_visible
        .set_state(time_defaults.ticks_visible);
    time_controls
        .time_visible
        .set_state(time_defaults.time_visible);
    time_controls
        .seconds_visible
        .set_state(time_defaults.seconds_visible);
    time_controls
        .tick_format_entry
        .set_text(&time_defaults.tick_mark_format);
    time_controls.tick_max_len.set_range(0.0, 20.0);
    time_controls.tick_max_len.set_increments(1.0, 5.0);
    time_controls
        .tick_max_len
        .set_value(time_defaults.tick_mark_max_character_length as f64);

    time_controls.label_mode_combo.append_text("Auto");
    time_controls.label_mode_combo.append_text("Time");
    time_controls.label_mode_combo.append_text("Date");
    time_controls.label_mode_combo.append_text("DateTime");
    time_controls.label_mode_combo.append_text("Custom");
    time_controls.label_mode_combo.set_active(Some(0));
    time_controls
        .label_format_entry
        .set_text("{YYYY}-{MM}-{DD} {HH}:{mm}");

    time_controls.bar_spacing.set_range(2.0, 40.0);
    time_controls.bar_spacing.set_increments(0.5, 1.0);
    time_controls.bar_spacing.set_value(time_defaults.bar_spacing);
    time_controls.min_spacing.set_range(0.5, 10.0);
    time_controls.min_spacing.set_increments(0.1, 0.5);
    time_controls
        .min_spacing
        .set_value(time_defaults.min_bar_spacing);
    time_controls.max_spacing.set_range(0.0, 200.0);
    time_controls.max_spacing.set_increments(0.5, 1.0);
    time_controls
        .max_spacing
        .set_value(time_defaults.max_bar_spacing);
    time_controls.right_offset.set_range(0.0, 200.0);
    time_controls.right_offset.set_increments(0.5, 1.0);
    time_controls
        .right_offset
        .set_value(time_defaults.right_offset);
    time_controls.right_offset_px.set_range(0.0, 200.0);
    time_controls.right_offset_px.set_increments(0.5, 1.0);
    time_controls
        .right_offset_px
        .set_value(time_defaults.right_offset_pixels);
    time_controls
        .fix_left
        .set_state(time_defaults.fix_left_edge);
    time_controls
        .fix_right
        .set_state(time_defaults.fix_right_edge);
    time_controls
        .lock_visible_range
        .set_state(time_defaults.lock_visible_time_range_on_resize);
    time_controls
        .right_bar_stays
        .set_state(time_defaults.right_bar_stays_on_scroll);
    time_controls
        .shift_on_new_bar
        .set_state(time_defaults.shift_visible_range_on_new_bar);
    time_controls
        .uniform_distribution
        .set_state(time_defaults.uniform_distribution);
    time_controls.min_height.set_range(0.0, 200.0);
    time_controls.min_height.set_increments(1.0, 10.0);
    time_controls
        .min_height
        .set_value(time_defaults.minimum_height);

    interaction_controls
        .scroll_mouse_wheel
        .set_state(handle_scroll_defaults.mouse_wheel);
    interaction_controls
        .scroll_pressed_move
        .set_state(handle_scroll_defaults.pressed_mouse_move);
    interaction_controls
        .scroll_horz_touch
        .set_state(handle_scroll_defaults.horz_touch_drag);
    interaction_controls
        .scroll_vert_touch
        .set_state(handle_scroll_defaults.vert_touch_drag);

    interaction_controls
        .mouse_wheel
        .set_state(handle_defaults.mouse_wheel);
    interaction_controls.pinch.set_state(handle_defaults.pinch);
    interaction_controls
        .axis_move_time
        .set_state(handle_defaults.axis_pressed_mouse_move_time);
    interaction_controls
        .axis_move_price
        .set_state(handle_defaults.axis_pressed_mouse_move_price);
    interaction_controls
        .axis_reset_time
        .set_state(handle_defaults.axis_double_click_reset_time);
    interaction_controls
        .axis_reset_price
        .set_state(handle_defaults.axis_double_click_reset_price);

    interaction_controls
        .kinetic_mouse
        .set_state(kinetic_defaults.mouse);
    interaction_controls
        .kinetic_touch
        .set_state(kinetic_defaults.touch);
    interaction_controls
        .tracking_mode
        .set_state(tracking_defaults.enabled);

    interaction_controls.axis_drag_time.set_range(0.0005, 0.02);
    interaction_controls.axis_drag_time.set_increments(0.0005, 0.001);
    interaction_controls
        .axis_drag_time
        .set_value(sensitivity_defaults.axis_drag_time);
    interaction_controls.axis_drag_price.set_range(0.0005, 0.02);
    interaction_controls
        .axis_drag_price
        .set_increments(0.0005, 0.001);
    interaction_controls
        .axis_drag_price
        .set_value(sensitivity_defaults.axis_drag_price);
    interaction_controls.wheel_zoom.set_range(0.01, 0.2);
    interaction_controls.wheel_zoom.set_increments(0.005, 0.01);
    interaction_controls
        .wheel_zoom
        .set_value(sensitivity_defaults.wheel_zoom);
    interaction_controls.pinch_zoom.set_range(0.01, 0.2);
    interaction_controls.pinch_zoom.set_increments(0.005, 0.01);
    interaction_controls
        .pinch_zoom
        .set_value(sensitivity_defaults.pinch_zoom);

    tooltip_controls
        .enabled
        .set_state(tooltip_defaults.enabled);
    tooltip_controls.position.append_text("Auto");
    tooltip_controls.position.append_text("TopLeft");
    tooltip_controls.position.append_text("TopRight");
    tooltip_controls.position.append_text("BottomLeft");
    tooltip_controls.position.append_text("BottomRight");
    tooltip_controls.position.append_text("Follow");
    tooltip_controls.position.set_active(Some(match tooltip_defaults.position {
        TooltipPosition::Auto => 0,
        TooltipPosition::TopLeft => 1,
        TooltipPosition::TopRight => 2,
        TooltipPosition::BottomLeft => 3,
        TooltipPosition::BottomRight => 4,
        TooltipPosition::Follow => 5,
    }));

    tooltip_controls
        .format
        .set_text(&tooltip_defaults.format);
    tooltip_controls
        .line_format
        .set_text("Line {series}: {value}");
    tooltip_controls
        .hist_format
        .set_text("Histogram {series}: {value}");
    tooltip_controls
        .background
        .set_rgba(&rgba_from_color(tooltip_defaults.background));
    tooltip_controls
        .text
        .set_rgba(&rgba_from_color(tooltip_defaults.text));

    if profiles_controls.folder_entry.text().is_empty() {
        profiles_controls.folder_entry.set_text("profiles");
    }
    refresh_profiles_list(&settings);

    profiles_controls.refresh.connect_clicked({
        let settings = settings.clone();
        move |_| refresh_profiles_list(&settings)
    });

    profiles_controls.select_folder.connect_clicked({
        let settings = settings.clone();
        move |_| {
            let parent = settings
                .profiles
                .folder_entry
                .root()
                .and_then(|root| root.downcast::<gtk::Window>().ok());
            let dialog = gtk::FileChooserNative::new(
                Some("Select profiles folder"),
                parent.as_ref(),
                gtk::FileChooserAction::SelectFolder,
                Some("Select"),
                Some("Cancel"),
            );
            dialog.connect_response({
                let settings = settings.clone();
                move |dialog, response| {
                    if response == gtk::ResponseType::Accept {
                        if let Some(file) = dialog.file() {
                            if let Some(path) = file.path() {
                                settings
                                    .profiles
                                    .folder_entry
                                    .set_text(path.to_string_lossy().as_ref());
                                refresh_profiles_list(&settings);
                            }
                        }
                    }
                    dialog.destroy();
                }
            });
            dialog.show();
        }
    });

    profiles_controls.open_file.connect_clicked({
        let settings = settings.clone();
        move |_| {
            let parent = settings
                .profiles
                .folder_entry
                .root()
                .and_then(|root| root.downcast::<gtk::Window>().ok());
            let dialog = gtk::FileChooserNative::new(
                Some("Open profile"),
                parent.as_ref(),
                gtk::FileChooserAction::Open,
                Some("Open"),
                Some("Cancel"),
            );
            dialog.connect_response({
                let settings = settings.clone();
                move |dialog, response| {
                    if response == gtk::ResponseType::Accept {
                        if let Some(file) = dialog.file() {
                            if let Some(path) = file.path() {
                                if let Some(parent) = path.parent() {
                                    settings
                                        .profiles
                                        .folder_entry
                                        .set_text(parent.to_string_lossy().as_ref());
                                }
                                if let Some(stem) = path.file_stem().and_then(|s| s.to_str()) {
                                    settings.profiles.profile_name.set_text(stem);
                                }
                                if let Ok(text) = fs::read_to_string(&path) {
                                    if let Ok(profile) = toml::from_str::<SettingsProfile>(&text) {
                                        apply_settings_profile(&settings, &profile);
                                    }
                                }
                                refresh_profiles_list(&settings);
                            }
                        }
                    }
                    dialog.destroy();
                }
            });
            dialog.show();
        }
    });

    profiles_controls.save.connect_clicked({
        let settings = settings.clone();
        move |_| {
            let name = settings
                .profiles
                .profile_name
                .text()
                .to_string()
                .trim()
                .to_string();
            if name.is_empty() {
                return;
            }
            let folder = profiles_folder(&settings);
            let _ = fs::create_dir_all(&folder);
            let path = profile_path(&folder, &name);
            let profile = settings_profile_from_controls(&settings);
            if let Ok(text) = toml::to_string_pretty(&profile) {
                if let Err(err) = fs::write(&path, text) {
                    eprintln!("Failed to save profile: {err}");
                }
            }
            refresh_profiles_list(&settings);
        }
    });

    profiles_controls.load.connect_clicked({
        let settings = settings.clone();
        move |_| {
            let mut name = settings
                .profiles
                .profile_name
                .text()
                .to_string()
                .trim()
                .to_string();
            if name.is_empty() {
                if let Some(selected) = settings.profiles.profiles_combo.active_text() {
                    name = selected.to_string();
                }
            }
            if name.is_empty() {
                return;
            }
            let folder = profiles_folder(&settings);
            let path = profile_path(&folder, &name);
            if let Ok(text) = fs::read_to_string(&path) {
                if let Ok(profile) = toml::from_str::<SettingsProfile>(&text) {
                    apply_settings_profile(&settings, &profile);
                }
            }
        }
    });

    let update_histogram_colors = {
        let hist_series = hist_series.clone();
        let candle_controls = candle_controls.clone();
        let series_controls = series_controls.clone();
        let store = store.clone();
        let drawing_area = drawing_area.clone();
        move || {
            let follow = series_controls.hist_follow_candle_colors.state();
            series_controls.hist_color.set_sensitive(!follow);
            if follow {
                let up = color_from_rgba(candle_controls.up_color.rgba());
                let down = color_from_rgba(candle_controls.down_color.rgba());
                let store_ref = store.borrow();
                let points =
                    recolor_histogram_points(&store_ref.candles, &store_ref.volumes, up, down);
                hist_series.set_data(points);
            } else {
                let store_ref = store.borrow();
                hist_series.set_data(strip_histogram_colors(&store_ref.volumes));
            }
            drawing_area.queue_draw();
        }
    };

    let update_candle_colors = {
        let chart = chart.clone();
        let drawing_area = drawing_area.clone();
        let candle_controls = candle_controls.clone();
        let update_histogram_colors = update_histogram_colors.clone();
        move || {
            let up = color_from_rgba(candle_controls.up_color.rgba());
            let down = color_from_rgba(candle_controls.down_color.rgba());
            chart.set_candle_colors(
                up,
                down,
                color_from_rgba(candle_controls.border_up_color.rgba()),
                color_from_rgba(candle_controls.border_down_color.rgba()),
                color_from_rgba(candle_controls.wick_up_color.rgba()),
                color_from_rgba(candle_controls.wick_down_color.rgba()),
            );
            update_histogram_colors();
            drawing_area.queue_draw();
        }
    };

    candle_controls.up_color.connect_color_set({
        let update_candle_colors = update_candle_colors.clone();
        move |_| update_candle_colors()
    });
    candle_controls.down_color.connect_color_set({
        let update_candle_colors = update_candle_colors.clone();
        move |_| update_candle_colors()
    });
    candle_controls.border_up_color.connect_color_set({
        let update_candle_colors = update_candle_colors.clone();
        move |_| update_candle_colors()
    });
    candle_controls.border_down_color.connect_color_set({
        let update_candle_colors = update_candle_colors.clone();
        move |_| update_candle_colors()
    });
    candle_controls.wick_up_color.connect_color_set({
        let update_candle_colors = update_candle_colors.clone();
        move |_| update_candle_colors()
    });
    candle_controls.wick_down_color.connect_color_set({
        let update_candle_colors = update_candle_colors.clone();
        move |_| update_candle_colors()
    });

    chart_controls.background_color.connect_color_set({
        let chart = chart.clone();
        let drawing_area = drawing_area.clone();
        let background_color = chart_controls.background_color.clone();
        move |_| {
            chart.set_background_color(color_from_rgba(background_color.rgba()));
            drawing_area.queue_draw();
        }
    });

    let update_grid = {
        let chart = chart.clone();
        let drawing_area = drawing_area.clone();
        let grid_color = chart_controls.grid_color.clone();
        let grid_switch = chart_controls.grid_switch.clone();
        move || {
            chart.set_grid(grid_switch.state(), color_from_rgba(grid_color.rgba()));
            drawing_area.queue_draw();
        }
    };
    chart_controls.grid_switch.connect_state_notify({
        let update_grid = update_grid.clone();
        move |_| update_grid()
    });
    chart_controls.grid_color.connect_color_set({
        let update_grid = update_grid.clone();
        move |_| update_grid()
    });

    series_controls.line_color.connect_color_set({
        let chart = chart.clone();
        let drawing_area = drawing_area.clone();
        let line_color = series_controls.line_color.clone();
        move |_| {
            chart.set_line_color(color_from_rgba(line_color.rgba()));
            drawing_area.queue_draw();
        }
    });

    series_controls.hist_color.connect_color_set({
        let chart = chart.clone();
        let drawing_area = drawing_area.clone();
        let hist_color = series_controls.hist_color.clone();
        move |_| {
            chart.set_histogram_color(color_from_rgba(hist_color.rgba()));
            drawing_area.queue_draw();
        }
    });
    series_controls
        .hist_follow_candle_colors
        .connect_state_notify({
            let update_histogram_colors = update_histogram_colors.clone();
            move |_| update_histogram_colors()
        });

    series_controls
        .candles_price_line
        .visible
        .connect_state_notify({
            let candle_series = candle_series.clone();
            move |switch: &gtk::Switch| {
                candle_series.set_price_line_visible(switch.state());
            }
        });
    series_controls.candles_price_line.style.connect_changed({
        let candle_series = candle_series.clone();
        move |combo: &gtk::ComboBoxText| {
            candle_series.set_price_line_style(line_style_from_combo(combo));
        }
    });
    series_controls
        .candles_price_line
        .width
        .connect_value_changed({
            let candle_series = candle_series.clone();
            move |spin: &gtk::SpinButton| {
                candle_series.set_price_line_width(spin.value());
            }
        });
    series_controls
        .candles_price_line
        .color
        .connect_color_set({
            let candle_series = candle_series.clone();
            let color = series_controls.candles_price_line.color.clone();
            move |_| {
                candle_series.set_price_line_color(color_from_rgba(color.rgba()));
            }
        });
    series_controls
        .candles_last_value
        .visible
        .connect_state_notify({
            let candle_series = candle_series.clone();
            move |switch: &gtk::Switch| {
                candle_series.set_last_value_visible(switch.state());
            }
        });
    series_controls
        .candles_last_value
        .background
        .connect_color_set({
            let candle_series = candle_series.clone();
            let color = series_controls.candles_last_value.background.clone();
            move |_| {
                candle_series.set_last_value_color(color_from_rgba(color.rgba()));
            }
        });
    series_controls
        .candles_last_value
        .text
        .connect_color_set({
            let candle_series = candle_series.clone();
            let color = series_controls.candles_last_value.text.clone();
            move |_| {
                candle_series.set_last_value_text_color(color_from_rgba(color.rgba()));
            }
        });

    series_controls
        .line_price_line
        .visible
        .connect_state_notify({
            let line_series = line_series.clone();
            move |switch: &gtk::Switch| {
                line_series.set_price_line_visible(switch.state());
            }
        });
    series_controls.line_price_line.style.connect_changed({
        let line_series = line_series.clone();
        move |combo: &gtk::ComboBoxText| {
            line_series.set_price_line_style(line_style_from_combo(combo));
        }
    });
    series_controls
        .line_price_line
        .width
        .connect_value_changed({
            let line_series = line_series.clone();
            move |spin: &gtk::SpinButton| {
                line_series.set_price_line_width(spin.value());
            }
        });
    series_controls
        .line_price_line
        .color
        .connect_color_set({
            let line_series = line_series.clone();
            let color = series_controls.line_price_line.color.clone();
            move |_| {
                line_series.set_price_line_color(color_from_rgba(color.rgba()));
            }
        });
    series_controls
        .line_last_value
        .visible
        .connect_state_notify({
            let line_series = line_series.clone();
            move |switch: &gtk::Switch| {
                line_series.set_last_value_visible(switch.state());
            }
        });
    series_controls
        .line_last_value
        .background
        .connect_color_set({
            let line_series = line_series.clone();
            let color = series_controls.line_last_value.background.clone();
            move |_| {
                line_series.set_last_value_color(color_from_rgba(color.rgba()));
            }
        });
    series_controls
        .line_last_value
        .text
        .connect_color_set({
            let line_series = line_series.clone();
            let color = series_controls.line_last_value.text.clone();
            move |_| {
                line_series.set_last_value_text_color(color_from_rgba(color.rgba()));
            }
        });

    series_controls
        .hist_price_line
        .visible
        .connect_state_notify({
            let hist_series = hist_series.clone();
            move |switch: &gtk::Switch| {
                hist_series.set_price_line_visible(switch.state());
            }
        });
    series_controls.hist_price_line.style.connect_changed({
        let hist_series = hist_series.clone();
        move |combo: &gtk::ComboBoxText| {
            hist_series.set_price_line_style(line_style_from_combo(combo));
        }
    });
    series_controls
        .hist_price_line
        .width
        .connect_value_changed({
            let hist_series = hist_series.clone();
            move |spin: &gtk::SpinButton| {
                hist_series.set_price_line_width(spin.value());
            }
        });
    series_controls
        .hist_price_line
        .color
        .connect_color_set({
            let hist_series = hist_series.clone();
            let color = series_controls.hist_price_line.color.clone();
            move |_| {
                hist_series.set_price_line_color(color_from_rgba(color.rgba()));
            }
        });
    series_controls
        .hist_last_value
        .visible
        .connect_state_notify({
            let hist_series = hist_series.clone();
            move |switch: &gtk::Switch| {
                hist_series.set_last_value_visible(switch.state());
            }
        });
    series_controls
        .hist_last_value
        .background
        .connect_color_set({
            let hist_series = hist_series.clone();
            let color = series_controls.hist_last_value.background.clone();
            move |_| {
                hist_series.set_last_value_color(color_from_rgba(color.rgba()));
            }
        });
    series_controls
        .hist_last_value
        .text
        .connect_color_set({
            let hist_series = hist_series.clone();
            let color = series_controls.hist_last_value.text.clone();
            move |_| {
                hist_series.set_last_value_text_color(color_from_rgba(color.rgba()));
            }
        });

    let update_left_scale = {
        let chart = chart.clone();
        let drawing_area = drawing_area.clone();
        let controls = price_scale_controls.left.clone();
        move || {
            chart.set_price_scale_options(
                PriceScale::Left,
                price_scale_options_from_controls(&controls),
            );
            drawing_area.queue_draw();
        }
    };

    let update_right_scale = {
        let chart = chart.clone();
        let drawing_area = drawing_area.clone();
        let controls = price_scale_controls.right.clone();
        move || {
            chart.set_price_scale_options(
                PriceScale::Right,
                price_scale_options_from_controls(&controls),
            );
            drawing_area.queue_draw();
        }
    };

    price_scale_controls.left.visible.connect_state_notify({
        let update_left_scale = update_left_scale.clone();
        let button = auto_scale_left_button.clone();
        move |switch: &gtk::Switch| {
            button.set_visible(switch.state());
            update_left_scale();
        }
    });
    price_scale_controls.left.auto_scale.connect_state_notify({
        let update_left_scale = update_left_scale.clone();
        let button = auto_scale_left_button.clone();
        move |switch: &gtk::Switch| {
            if button.is_active() != switch.state() {
                button.set_active(switch.state());
            }
            update_left_scale();
        }
    });
    auto_scale_left_button.connect_toggled({
        let switch = price_scale_controls.left.auto_scale.clone();
        move |button: &gtk::ToggleButton| {
            let state = button.is_active();
            if switch.state() != state {
                switch.set_state(state);
            }
        }
    });
    price_scale_controls.left.mode_combo.connect_changed({
        let update_left_scale = update_left_scale.clone();
        move |_| update_left_scale()
    });
    price_scale_controls.left.invert.connect_state_notify({
        let update_left_scale = update_left_scale.clone();
        move |_| update_left_scale()
    });
    price_scale_controls.left.align_labels.connect_state_notify({
        let update_left_scale = update_left_scale.clone();
        move |_| update_left_scale()
    });
    price_scale_controls.left.margin_top.connect_value_changed({
        let update_left_scale = update_left_scale.clone();
        move |_| update_left_scale()
    });
    price_scale_controls.left.margin_bottom.connect_value_changed({
        let update_left_scale = update_left_scale.clone();
        move |_| update_left_scale()
    });
    price_scale_controls.left.border_visible.connect_state_notify({
        let update_left_scale = update_left_scale.clone();
        move |_| update_left_scale()
    });
    price_scale_controls.left.border_color.connect_color_set({
        let update_left_scale = update_left_scale.clone();
        move |_| update_left_scale()
    });
    price_scale_controls.left.text_color.connect_color_set({
        let update_left_scale = update_left_scale.clone();
        move |_| update_left_scale()
    });
    price_scale_controls.left.ticks_visible.connect_state_notify({
        let update_left_scale = update_left_scale.clone();
        move |_| update_left_scale()
    });
    price_scale_controls.left.min_width.connect_value_changed({
        let update_left_scale = update_left_scale.clone();
        move |_| update_left_scale()
    });
    price_scale_controls.left.entire_text_only.connect_state_notify({
        let update_left_scale = update_left_scale.clone();
        move |_| update_left_scale()
    });
    price_scale_controls.left.ensure_edge_ticks.connect_state_notify({
        let update_left_scale = update_left_scale.clone();
        move |_| update_left_scale()
    });

    price_scale_controls.right.visible.connect_state_notify({
        let update_right_scale = update_right_scale.clone();
        let button = auto_scale_right_button.clone();
        move |switch: &gtk::Switch| {
            button.set_visible(switch.state());
            update_right_scale();
        }
    });
    price_scale_controls.right.auto_scale.connect_state_notify({
        let update_right_scale = update_right_scale.clone();
        let button = auto_scale_right_button.clone();
        move |switch: &gtk::Switch| {
            if button.is_active() != switch.state() {
                button.set_active(switch.state());
            }
            update_right_scale();
        }
    });
    auto_scale_right_button.connect_toggled({
        let switch = price_scale_controls.right.auto_scale.clone();
        move |button: &gtk::ToggleButton| {
            let state = button.is_active();
            if switch.state() != state {
                switch.set_state(state);
            }
        }
    });
    price_scale_controls.right.mode_combo.connect_changed({
        let update_right_scale = update_right_scale.clone();
        move |_| update_right_scale()
    });
    price_scale_controls.right.invert.connect_state_notify({
        let update_right_scale = update_right_scale.clone();
        move |_| update_right_scale()
    });
    price_scale_controls.right.align_labels.connect_state_notify({
        let update_right_scale = update_right_scale.clone();
        move |_| update_right_scale()
    });
    price_scale_controls.right.margin_top.connect_value_changed({
        let update_right_scale = update_right_scale.clone();
        move |_| update_right_scale()
    });
    price_scale_controls.right.margin_bottom.connect_value_changed({
        let update_right_scale = update_right_scale.clone();
        move |_| update_right_scale()
    });
    price_scale_controls.right.border_visible.connect_state_notify({
        let update_right_scale = update_right_scale.clone();
        move |_| update_right_scale()
    });
    price_scale_controls.right.border_color.connect_color_set({
        let update_right_scale = update_right_scale.clone();
        move |_| update_right_scale()
    });
    price_scale_controls.right.text_color.connect_color_set({
        let update_right_scale = update_right_scale.clone();
        move |_| update_right_scale()
    });
    price_scale_controls.right.ticks_visible.connect_state_notify({
        let update_right_scale = update_right_scale.clone();
        move |_| update_right_scale()
    });
    price_scale_controls.right.min_width.connect_value_changed({
        let update_right_scale = update_right_scale.clone();
        move |_| update_right_scale()
    });
    price_scale_controls.right.entire_text_only.connect_state_notify({
        let update_right_scale = update_right_scale.clone();
        move |_| update_right_scale()
    });
    price_scale_controls.right.ensure_edge_ticks.connect_state_notify({
        let update_right_scale = update_right_scale.clone();
        move |_| update_right_scale()
    });

    price_scale_controls.left.reset_button.connect_clicked({
        let chart = chart.clone();
        let drawing_area = drawing_area.clone();
        move |_| {
            chart.reset_autoscale(PriceScale::Left);
            drawing_area.queue_draw();
        }
    });
    price_scale_controls.right.reset_button.connect_clicked({
        let chart = chart.clone();
        let drawing_area = drawing_area.clone();
        move |_| {
            chart.reset_autoscale(PriceScale::Right);
            drawing_area.queue_draw();
        }
    });

    update_left_scale();
    update_right_scale();
    update_histogram_colors();

    crosshair_controls.mode_combo.connect_changed({
        let chart = chart.clone();
        let drawing_area = drawing_area.clone();
        move |combo: &gtk::ComboBoxText| {
            chart.set_crosshair_mode(crosshair_mode_from_combo(combo));
            drawing_area.queue_draw();
        }
    });

    let update_crosshair_visibility = {
        let chart = chart.clone();
        let drawing_area = drawing_area.clone();
        let vertical = crosshair_controls.vertical_switch.clone();
        let horizontal = crosshair_controls.horizontal_switch.clone();
        move || {
            chart.set_crosshair_visibility(vertical.state(), horizontal.state());
            drawing_area.queue_draw();
        }
    };
    crosshair_controls.vertical_switch.connect_state_notify({
        let update_crosshair_visibility = update_crosshair_visibility.clone();
        move |_| update_crosshair_visibility()
    });
    crosshair_controls
        .horizontal_switch
        .connect_state_notify({
            let update_crosshair_visibility = update_crosshair_visibility.clone();
            move |_| update_crosshair_visibility()
        });

    crosshair_controls.snap_ohlc.connect_state_notify({
        let chart = chart.clone();
        let drawing_area = drawing_area.clone();
        move |switch: &gtk::Switch| {
            chart.set_crosshair_snap_to_ohlc(switch.state());
            drawing_area.queue_draw();
        }
    });

    crosshair_controls.snap_series.connect_state_notify({
        let chart = chart.clone();
        let drawing_area = drawing_area.clone();
        move |switch: &gtk::Switch| {
            chart.set_crosshair_snap_to_series(switch.state());
            drawing_area.queue_draw();
        }
    });

    crosshair_controls.line_style.connect_changed({
        let chart = chart.clone();
        let drawing_area = drawing_area.clone();
        move |combo: &gtk::ComboBoxText| {
            chart.set_crosshair_line_style(line_style_from_combo(combo));
            drawing_area.queue_draw();
        }
    });

    crosshair_controls.width.connect_value_changed({
        let chart = chart.clone();
        let drawing_area = drawing_area.clone();
        move |spin: &gtk::SpinButton| {
            chart.set_crosshair_line_width(spin.value());
            drawing_area.queue_draw();
        }
    });

    crosshair_controls.center_combo.connect_changed({
        let chart = chart.clone();
        let drawing_area = drawing_area.clone();
        move |combo: &gtk::ComboBoxText| {
            let center = match combo.active() {
                Some(1) => CrosshairCenter::Dot,
                Some(2) => CrosshairCenter::Circle,
                _ => CrosshairCenter::Cross,
            };
            chart.set_crosshair_center(center);
            drawing_area.queue_draw();
        }
    });

    crosshair_controls.center_size.connect_value_changed({
        let chart = chart.clone();
        let drawing_area = drawing_area.clone();
        move |spin: &gtk::SpinButton| {
            chart.set_crosshair_center_size(spin.value());
            drawing_area.queue_draw();
        }
    });

    crosshair_controls.color.connect_color_set({
        let chart = chart.clone();
        let drawing_area = drawing_area.clone();
        let crosshair_color = crosshair_controls.color.clone();
        move |_| {
            chart.set_crosshair_color(color_from_rgba(crosshair_color.rgba()));
            drawing_area.queue_draw();
        }
    });

    crosshair_controls.center_color.connect_color_set({
        let chart = chart.clone();
        let drawing_area = drawing_area.clone();
        let crosshair_center_color = crosshair_controls.center_color.clone();
        move |_| {
            chart.set_crosshair_center_color(color_from_rgba(crosshair_center_color.rgba()));
            drawing_area.queue_draw();
        }
    });

    let update_time_scale = {
        let chart = chart.clone();
        let drawing_area = drawing_area.clone();
        let time_controls = time_controls.clone();
        move || {
            chart.set_time_scale_visible(time_controls.visible_switch.state());
            chart.set_time_scale_border(
                time_controls.border_visible.state(),
                color_from_rgba(time_controls.border_color.rgba()),
            );
            chart.set_time_scale_ticks_visible(time_controls.ticks_visible.state());
            chart.set_time_scale_time_visible(time_controls.time_visible.state());
            chart.set_time_scale_seconds_visible(time_controls.seconds_visible.state());
            chart.set_time_scale_tick_mark_format(time_controls.tick_format_entry.text().to_string());
            chart.set_time_scale_tick_mark_max_len(
                time_controls.tick_max_len.value().round() as usize,
            );
            let mode = match time_controls.label_mode_combo.active() {
                Some(1) => TimeLabelMode::Time,
                Some(2) => TimeLabelMode::Date,
                Some(3) => TimeLabelMode::DateTime,
                Some(4) => TimeLabelMode::Custom,
                _ => TimeLabelMode::Auto,
            };
            chart.set_time_label_mode(mode);
            chart.set_time_label_format(time_controls.label_format_entry.text().to_string());
            chart.set_time_scale_bar_spacing(time_controls.bar_spacing.value());
            chart.set_time_scale_min_bar_spacing(time_controls.min_spacing.value());
            chart.set_time_scale_max_bar_spacing(time_controls.max_spacing.value());
            chart.set_time_scale_right_offset(time_controls.right_offset.value());
            chart.set_time_scale_right_offset_pixels(time_controls.right_offset_px.value());
            chart.set_time_scale_fix_left_edge(time_controls.fix_left.state());
            chart.set_time_scale_fix_right_edge(time_controls.fix_right.state());
            chart.set_time_scale_lock_visible_time_range_on_resize(
                time_controls.lock_visible_range.state(),
            );
            chart.set_time_scale_right_bar_stays_on_scroll(time_controls.right_bar_stays.state());
            chart.set_time_scale_shift_visible_range_on_new_bar(
                time_controls.shift_on_new_bar.state(),
            );
            chart.set_time_scale_uniform_distribution(time_controls.uniform_distribution.state());
            chart.set_time_scale_minimum_height(time_controls.min_height.value());
            drawing_area.queue_draw();
        }
    };

    time_controls.visible_switch.connect_state_notify({
        let update_time_scale = update_time_scale.clone();
        move |_| update_time_scale()
    });
    time_controls.border_visible.connect_state_notify({
        let update_time_scale = update_time_scale.clone();
        move |_| update_time_scale()
    });
    time_controls.border_color.connect_color_set({
        let update_time_scale = update_time_scale.clone();
        move |_| update_time_scale()
    });
    time_controls.ticks_visible.connect_state_notify({
        let update_time_scale = update_time_scale.clone();
        move |_| update_time_scale()
    });
    time_controls.time_visible.connect_state_notify({
        let update_time_scale = update_time_scale.clone();
        move |_| update_time_scale()
    });
    time_controls.seconds_visible.connect_state_notify({
        let update_time_scale = update_time_scale.clone();
        move |_| update_time_scale()
    });
    time_controls.tick_format_entry.connect_changed({
        let update_time_scale = update_time_scale.clone();
        move |_| update_time_scale()
    });
    time_controls.tick_max_len.connect_value_changed({
        let update_time_scale = update_time_scale.clone();
        move |_| update_time_scale()
    });
    time_controls.label_mode_combo.connect_changed({
        let update_time_scale = update_time_scale.clone();
        move |_| update_time_scale()
    });
    time_controls.label_format_entry.connect_changed({
        let update_time_scale = update_time_scale.clone();
        move |_| update_time_scale()
    });
    time_controls.bar_spacing.connect_value_changed({
        let update_time_scale = update_time_scale.clone();
        move |_| update_time_scale()
    });
    time_controls.min_spacing.connect_value_changed({
        let update_time_scale = update_time_scale.clone();
        move |_| update_time_scale()
    });
    time_controls.max_spacing.connect_value_changed({
        let update_time_scale = update_time_scale.clone();
        move |_| update_time_scale()
    });
    time_controls.right_offset.connect_value_changed({
        let update_time_scale = update_time_scale.clone();
        move |_| update_time_scale()
    });
    time_controls.right_offset_px.connect_value_changed({
        let update_time_scale = update_time_scale.clone();
        move |_| update_time_scale()
    });
    time_controls.fix_left.connect_state_notify({
        let update_time_scale = update_time_scale.clone();
        move |_| update_time_scale()
    });
    time_controls.fix_right.connect_state_notify({
        let update_time_scale = update_time_scale.clone();
        move |_| update_time_scale()
    });
    time_controls.lock_visible_range.connect_state_notify({
        let update_time_scale = update_time_scale.clone();
        move |_| update_time_scale()
    });
    time_controls.right_bar_stays.connect_state_notify({
        let update_time_scale = update_time_scale.clone();
        move |_| update_time_scale()
    });
    time_controls.shift_on_new_bar.connect_state_notify({
        let update_time_scale = update_time_scale.clone();
        move |_| update_time_scale()
    });
    time_controls.uniform_distribution.connect_state_notify({
        let update_time_scale = update_time_scale.clone();
        move |_| update_time_scale()
    });
    time_controls.min_height.connect_value_changed({
        let update_time_scale = update_time_scale.clone();
        move |_| update_time_scale()
    });

    time_controls.fit_content.connect_clicked({
        let chart = chart.clone();
        let drawing_area = drawing_area.clone();
        move |_| {
            chart.fit_content();
            drawing_area.queue_draw();
        }
    });

    let update_handle_scroll = {
        let chart = chart.clone();
        let drawing_area = drawing_area.clone();
        let interaction_controls = interaction_controls.clone();
        move || {
            let options = HandleScrollOptions {
                mouse_wheel: interaction_controls.scroll_mouse_wheel.state(),
                pressed_mouse_move: interaction_controls.scroll_pressed_move.state(),
                horz_touch_drag: interaction_controls.scroll_horz_touch.state(),
                vert_touch_drag: interaction_controls.scroll_vert_touch.state(),
            };
            chart.apply_handle_scroll_options(options);
            drawing_area.queue_draw();
        }
    };

    interaction_controls.scroll_mouse_wheel.connect_state_notify({
        let update_handle_scroll = update_handle_scroll.clone();
        move |_| update_handle_scroll()
    });
    interaction_controls.scroll_pressed_move.connect_state_notify({
        let update_handle_scroll = update_handle_scroll.clone();
        move |_| update_handle_scroll()
    });
    interaction_controls.scroll_horz_touch.connect_state_notify({
        let update_handle_scroll = update_handle_scroll.clone();
        move |_| update_handle_scroll()
    });
    interaction_controls.scroll_vert_touch.connect_state_notify({
        let update_handle_scroll = update_handle_scroll.clone();
        move |_| update_handle_scroll()
    });

    update_handle_scroll();

    let update_kinetic_scroll = {
        let chart = chart.clone();
        let drawing_area = drawing_area.clone();
        let interaction_controls = interaction_controls.clone();
        move || {
            let options = KineticScrollOptions {
                mouse: interaction_controls.kinetic_mouse.state(),
                touch: interaction_controls.kinetic_touch.state(),
            };
            chart.apply_kinetic_scroll_options(options);
            drawing_area.queue_draw();
        }
    };

    interaction_controls.kinetic_mouse.connect_state_notify({
        let update_kinetic_scroll = update_kinetic_scroll.clone();
        move |_| update_kinetic_scroll()
    });
    interaction_controls.kinetic_touch.connect_state_notify({
        let update_kinetic_scroll = update_kinetic_scroll.clone();
        move |_| update_kinetic_scroll()
    });

    update_kinetic_scroll();

    let update_handle_scale = {
        let chart = chart.clone();
        let drawing_area = drawing_area.clone();
        let interaction_controls = interaction_controls.clone();
        move || {
            let options = HandleScaleOptions {
                mouse_wheel: interaction_controls.mouse_wheel.state(),
                pinch: interaction_controls.pinch.state(),
                axis_pressed_mouse_move_time: interaction_controls.axis_move_time.state(),
                axis_pressed_mouse_move_price: interaction_controls.axis_move_price.state(),
                axis_double_click_reset_time: interaction_controls.axis_reset_time.state(),
                axis_double_click_reset_price: interaction_controls.axis_reset_price.state(),
            };
            chart.apply_handle_scale_options(options);
            drawing_area.queue_draw();
        }
    };

    interaction_controls.mouse_wheel.connect_state_notify({
        let update_handle_scale = update_handle_scale.clone();
        move |_| update_handle_scale()
    });
    interaction_controls.pinch.connect_state_notify({
        let update_handle_scale = update_handle_scale.clone();
        move |_| update_handle_scale()
    });
    interaction_controls.axis_move_time.connect_state_notify({
        let update_handle_scale = update_handle_scale.clone();
        move |_| update_handle_scale()
    });
    interaction_controls.axis_move_price.connect_state_notify({
        let update_handle_scale = update_handle_scale.clone();
        move |_| update_handle_scale()
    });
    interaction_controls.axis_reset_time.connect_state_notify({
        let update_handle_scale = update_handle_scale.clone();
        move |_| update_handle_scale()
    });
    interaction_controls.axis_reset_price.connect_state_notify({
        let update_handle_scale = update_handle_scale.clone();
        move |_| update_handle_scale()
    });

    update_handle_scale();

    let update_tracking_mode = {
        let chart = chart.clone();
        let drawing_area = drawing_area.clone();
        let interaction_controls = interaction_controls.clone();
        move || {
            let options = TrackingModeOptions {
                enabled: interaction_controls.tracking_mode.state(),
                ..TrackingModeOptions::default()
            };
            chart.apply_tracking_mode_options(options);
            drawing_area.queue_draw();
        }
    };
    interaction_controls.tracking_mode.connect_state_notify({
        let update_tracking_mode = update_tracking_mode.clone();
        move |_| update_tracking_mode()
    });
    update_tracking_mode();

    let update_sensitivity = {
        let chart = chart.clone();
        let drawing_area = drawing_area.clone();
        let interaction_controls = interaction_controls.clone();
        move || {
            let options = InteractionSensitivityOptions {
                axis_drag_time: interaction_controls.axis_drag_time.value(),
                axis_drag_price: interaction_controls.axis_drag_price.value(),
                wheel_zoom: interaction_controls.wheel_zoom.value(),
                pinch_zoom: interaction_controls.pinch_zoom.value(),
            };
            chart.apply_interaction_sensitivity(options);
            drawing_area.queue_draw();
        }
    };
    interaction_controls.axis_drag_time.connect_value_changed({
        let update_sensitivity = update_sensitivity.clone();
        move |_| update_sensitivity()
    });
    interaction_controls.axis_drag_price.connect_value_changed({
        let update_sensitivity = update_sensitivity.clone();
        move |_| update_sensitivity()
    });
    interaction_controls.wheel_zoom.connect_value_changed({
        let update_sensitivity = update_sensitivity.clone();
        move |_| update_sensitivity()
    });
    interaction_controls.pinch_zoom.connect_value_changed({
        let update_sensitivity = update_sensitivity.clone();
        move |_| update_sensitivity()
    });
    update_sensitivity();

    tooltip_controls.enabled.connect_state_notify({
        let chart = chart.clone();
        let drawing_area = drawing_area.clone();
        move |switch: &gtk::Switch| {
            chart.set_tooltip_enabled(switch.state());
            drawing_area.queue_draw();
        }
    });

    tooltip_controls.position.connect_changed({
        let chart = chart.clone();
        let drawing_area = drawing_area.clone();
        move |combo: &gtk::ComboBoxText| {
            let position = match combo.active() {
                Some(1) => TooltipPosition::TopLeft,
                Some(2) => TooltipPosition::TopRight,
                Some(3) => TooltipPosition::BottomLeft,
                Some(4) => TooltipPosition::BottomRight,
                Some(5) => TooltipPosition::Follow,
                _ => TooltipPosition::Auto,
            };
            chart.set_tooltip_position(position);
            drawing_area.queue_draw();
        }
    });

    tooltip_controls.format.connect_changed({
        let chart = chart.clone();
        let drawing_area = drawing_area.clone();
        move |entry: &gtk::Entry| {
            chart.set_tooltip_format(entry.text().to_string());
            drawing_area.queue_draw();
        }
    });

    tooltip_controls.line_format.connect_changed({
        let chart = chart.clone();
        let drawing_area = drawing_area.clone();
        move |entry: &gtk::Entry| {
            chart.set_tooltip_line_format(entry.text().to_string());
            drawing_area.queue_draw();
        }
    });

    tooltip_controls.hist_format.connect_changed({
        let chart = chart.clone();
        let drawing_area = drawing_area.clone();
        move |entry: &gtk::Entry| {
            chart.set_tooltip_histogram_format(entry.text().to_string());
            drawing_area.queue_draw();
        }
    });

    let update_tooltip_colors = {
        let chart = chart.clone();
        let drawing_area = drawing_area.clone();
        let bg = tooltip_controls.background.clone();
        let text = tooltip_controls.text.clone();
        move || {
            chart.set_tooltip_colors(color_from_rgba(bg.rgba()), color_from_rgba(text.rgba()));
            drawing_area.queue_draw();
        }
    };

    tooltip_controls.background.connect_color_set({
        let update_tooltip_colors = update_tooltip_colors.clone();
        move |_| update_tooltip_colors()
    });
    tooltip_controls.text.connect_color_set({
        let update_tooltip_colors = update_tooltip_colors.clone();
        move |_| update_tooltip_colors()
    });

    series_controls.candles_scale_combo.connect_changed({
        let candle_series = candle_series.clone();
        move |combo: &gtk::ComboBoxText| {
            let scale = match combo.active() {
                Some(1) => PriceScale::Left,
                _ => PriceScale::Right,
            };
            candle_series.set_price_scale(scale);
        }
    });

    series_controls.line_scale_combo.connect_changed({
        let line_series = line_series.clone();
        move |combo: &gtk::ComboBoxText| {
            let scale = match combo.active() {
                Some(1) => PriceScale::Right,
                _ => PriceScale::Left,
            };
            line_series.set_price_scale(scale);
        }
    });

    series_controls.hist_scale_combo.connect_changed({
        let hist_series = hist_series.clone();
        move |combo: &gtk::ComboBoxText| {
            let scale = match combo.active() {
                Some(1) => PriceScale::Right,
                _ => PriceScale::Left,
            };
            hist_series.set_price_scale(scale);
        }
    });

    let update_candles_markers = {
        let candle_series = candle_series.clone();
        let controls = series_controls.markers.candles.clone();
        let drawing_area = drawing_area.clone();
        move || {
            let options = SeriesMarkersOptions {
                auto_scale: controls.auto_scale.state(),
                z_order: marker_z_order_from_combo(&controls.z_order),
                default_icon_text: marker_default_icon_from_controls(&controls),
                default_icon_text_color: marker_icon_color_from_controls(&controls),
                ..SeriesMarkersOptions::default()
            };
            candle_series.set_markers_options(options);
            drawing_area.queue_draw();
        }
    };
    series_controls
        .markers
        .candles
        .auto_scale
        .connect_state_notify({
            let update_candles_markers = update_candles_markers.clone();
            move |_| update_candles_markers()
        });
    series_controls
        .markers
        .candles
        .z_order
        .connect_changed({
            let update_candles_markers = update_candles_markers.clone();
            move |_| update_candles_markers()
        });
    series_controls
        .markers
        .candles
        .icon_category
        .connect_changed({
            let update_candles_markers = update_candles_markers.clone();
            let icon_combo = series_controls.markers.candles.icon_combo.clone();
            move |combo: &gtk::ComboBoxText| {
                let index = combo.active().unwrap_or(0) as usize;
                populate_marker_icon_combo(&icon_combo, index);
                update_candles_markers();
            }
        });
    series_controls
        .markers
        .candles
        .icon_combo
        .connect_changed({
            let update_candles_markers = update_candles_markers.clone();
            move |_| update_candles_markers()
        });
    series_controls
        .markers
        .candles
        .icon_color
        .connect_color_set({
            let update_candles_markers = update_candles_markers.clone();
            move |_| update_candles_markers()
        });
    series_controls
        .markers
        .candles
        .icon_entry
        .connect_changed({
            let update_candles_markers = update_candles_markers.clone();
            move |_| update_candles_markers()
        });

    let update_line_markers = {
        let line_series = line_series.clone();
        let controls = series_controls.markers.line.clone();
        let drawing_area = drawing_area.clone();
        move || {
            let options = SeriesMarkersOptions {
                auto_scale: controls.auto_scale.state(),
                z_order: marker_z_order_from_combo(&controls.z_order),
                default_icon_text: marker_default_icon_from_controls(&controls),
                default_icon_text_color: marker_icon_color_from_controls(&controls),
                ..SeriesMarkersOptions::default()
            };
            line_series.set_markers_options(options);
            drawing_area.queue_draw();
        }
    };
    series_controls
        .markers
        .line
        .auto_scale
        .connect_state_notify({
            let update_line_markers = update_line_markers.clone();
            move |_| update_line_markers()
        });
    series_controls.markers.line.z_order.connect_changed({
        let update_line_markers = update_line_markers.clone();
        move |_| update_line_markers()
    });
    series_controls
        .markers
        .line
        .icon_category
        .connect_changed({
            let update_line_markers = update_line_markers.clone();
            let icon_combo = series_controls.markers.line.icon_combo.clone();
            move |combo: &gtk::ComboBoxText| {
                let index = combo.active().unwrap_or(0) as usize;
                populate_marker_icon_combo(&icon_combo, index);
                update_line_markers();
            }
        });
    series_controls.markers.line.icon_combo.connect_changed({
        let update_line_markers = update_line_markers.clone();
        move |_| update_line_markers()
    });
    series_controls
        .markers
        .line
        .icon_color
        .connect_color_set({
            let update_line_markers = update_line_markers.clone();
            move |_| update_line_markers()
        });
    series_controls.markers.line.icon_entry.connect_changed({
        let update_line_markers = update_line_markers.clone();
        move |_| update_line_markers()
    });

    let update_hist_markers = {
        let hist_series = hist_series.clone();
        let controls = series_controls.markers.hist.clone();
        let drawing_area = drawing_area.clone();
        move || {
            let options = SeriesMarkersOptions {
                auto_scale: controls.auto_scale.state(),
                z_order: marker_z_order_from_combo(&controls.z_order),
                default_icon_text: marker_default_icon_from_controls(&controls),
                default_icon_text_color: marker_icon_color_from_controls(&controls),
                ..SeriesMarkersOptions::default()
            };
            hist_series.set_markers_options(options);
            drawing_area.queue_draw();
        }
    };
    series_controls
        .markers
        .hist
        .auto_scale
        .connect_state_notify({
            let update_hist_markers = update_hist_markers.clone();
            move |_| update_hist_markers()
        });
    series_controls.markers.hist.z_order.connect_changed({
        let update_hist_markers = update_hist_markers.clone();
        move |_| update_hist_markers()
    });
    series_controls
        .markers
        .hist
        .icon_category
        .connect_changed({
            let update_hist_markers = update_hist_markers.clone();
            let icon_combo = series_controls.markers.hist.icon_combo.clone();
            move |combo: &gtk::ComboBoxText| {
                let index = combo.active().unwrap_or(0) as usize;
                populate_marker_icon_combo(&icon_combo, index);
                update_hist_markers();
            }
        });
    series_controls.markers.hist.icon_combo.connect_changed({
        let update_hist_markers = update_hist_markers.clone();
        move |_| update_hist_markers()
    });
    series_controls
        .markers
        .hist
        .icon_color
        .connect_color_set({
            let update_hist_markers = update_hist_markers.clone();
            move |_| update_hist_markers()
        });
    series_controls.markers.hist.icon_entry.connect_changed({
        let update_hist_markers = update_hist_markers.clone();
        move |_| update_hist_markers()
    });

    let update_candles_format = {
        let candle_series = candle_series.clone();
        let controls = series_controls.candles_format.clone();
        move || {
            candle_series.set_price_format(price_format_from_controls(&controls));
        }
    };
    series_controls
        .candles_format
        .format_combo
        .connect_changed({
            let update_candles_format = update_candles_format.clone();
            let controls = series_controls.candles_format.clone();
            move |combo: &gtk::ComboBoxText| {
                controls.min_move.set_sensitive(combo.active() == Some(0));
                update_candles_format();
            }
        });
    series_controls
        .candles_format
        .precision
        .connect_value_changed({
            let update_candles_format = update_candles_format.clone();
            move |_| update_candles_format()
        });
    series_controls
        .candles_format
        .min_move
        .connect_value_changed({
            let update_candles_format = update_candles_format.clone();
            move |_| update_candles_format()
        });

    let update_line_format = {
        let line_series = line_series.clone();
        let controls = series_controls.line_format.clone();
        move || {
            line_series.set_price_format(price_format_from_controls(&controls));
        }
    };
    series_controls.line_format.format_combo.connect_changed({
        let update_line_format = update_line_format.clone();
        let controls = series_controls.line_format.clone();
        move |combo: &gtk::ComboBoxText| {
            controls.min_move.set_sensitive(combo.active() == Some(0));
            update_line_format();
        }
    });
    series_controls.line_format.precision.connect_value_changed({
        let update_line_format = update_line_format.clone();
        move |_| update_line_format()
    });
    series_controls.line_format.min_move.connect_value_changed({
        let update_line_format = update_line_format.clone();
        move |_| update_line_format()
    });

    let update_hist_format = {
        let hist_series = hist_series.clone();
        let controls = series_controls.hist_format.clone();
        move || {
            hist_series.set_price_format(price_format_from_controls(&controls));
        }
    };
    series_controls.hist_format.format_combo.connect_changed({
        let update_hist_format = update_hist_format.clone();
        let controls = series_controls.hist_format.clone();
        move |combo: &gtk::ComboBoxText| {
            controls.min_move.set_sensitive(combo.active() == Some(0));
            update_hist_format();
        }
    });
    series_controls
        .hist_format
        .precision
        .connect_value_changed({
            let update_hist_format = update_hist_format.clone();
            move |_| update_hist_format()
        });
    series_controls
        .hist_format
        .min_move
        .connect_value_changed({
            let update_hist_format = update_hist_format.clone();
            move |_| update_hist_format()
        });

    wire_price_lines_controls(
        drawing_area,
        &series_controls.candles_lines,
        std::rc::Rc::new({
            let candle_series = candle_series.clone();
            move |options| candle_series.create_price_line(options)
        }),
    );
    wire_price_lines_controls(
        drawing_area,
        &series_controls.line_lines,
        std::rc::Rc::new({
            let line_series = line_series.clone();
            move |options| line_series.create_price_line(options)
        }),
    );
    wire_price_lines_controls(
        drawing_area,
        &series_controls.hist_lines,
        std::rc::Rc::new({
            let hist_series = hist_series.clone();
            move |options| hist_series.create_price_line(options)
        }),
    );

    update_candles_format();
    update_line_format();
    update_hist_format();
    update_candles_markers();
    update_line_markers();
    update_hist_markers();
}

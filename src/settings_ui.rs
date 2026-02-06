mod helpers;
mod series;
mod time_scale;

use relm4::gtk;
use relm4::gtk::prelude::*;

use helpers::{page_box, row_with_label, row_with_two, section_label, separator};

pub use series::{
    SeriesControls, SeriesFormatControls, SeriesLastValueControls, SeriesMarkerControls,
    SeriesPriceLineControls, SeriesPriceLinesControls,
};
pub use time_scale::TimeScaleControls;

#[derive(Clone)]
pub struct ChartControls {
    pub background_color: gtk::ColorButton,
    pub grid_switch: gtk::Switch,
    pub grid_color: gtk::ColorButton,
}

#[derive(Clone)]
pub struct CandleControls {
    pub up_color: gtk::ColorButton,
    pub down_color: gtk::ColorButton,
    pub border_up_color: gtk::ColorButton,
    pub border_down_color: gtk::ColorButton,
    pub wick_up_color: gtk::ColorButton,
    pub wick_down_color: gtk::ColorButton,
}

#[derive(Clone)]
pub struct PriceScaleSideControls {
    pub visible: gtk::Switch,
    pub auto_scale: gtk::Switch,
    pub mode_combo: gtk::ComboBoxText,
    pub invert: gtk::Switch,
    pub align_labels: gtk::Switch,
    pub margin_top: gtk::SpinButton,
    pub margin_bottom: gtk::SpinButton,
    pub border_visible: gtk::Switch,
    pub border_color: gtk::ColorButton,
    pub text_color: gtk::ColorButton,
    pub ticks_visible: gtk::Switch,
    pub min_width: gtk::SpinButton,
    pub entire_text_only: gtk::Switch,
    pub ensure_edge_ticks: gtk::Switch,
    pub reset_button: gtk::Button,
}

#[derive(Clone)]
pub struct PriceScaleControls {
    pub left: PriceScaleSideControls,
    pub right: PriceScaleSideControls,
}

#[derive(Clone)]
pub struct CrosshairControls {
    pub mode_combo: gtk::ComboBoxText,
    pub vertical_switch: gtk::Switch,
    pub horizontal_switch: gtk::Switch,
    pub snap_ohlc: gtk::Switch,
    pub snap_series: gtk::Switch,
    pub line_style: gtk::ComboBoxText,
    pub width: gtk::SpinButton,
    pub center_combo: gtk::ComboBoxText,
    pub center_size: gtk::SpinButton,
    pub color: gtk::ColorButton,
    pub center_color: gtk::ColorButton,
}

#[derive(Clone)]
pub struct InteractionControls {
    pub scroll_mouse_wheel: gtk::Switch,
    pub scroll_pressed_move: gtk::Switch,
    pub scroll_horz_touch: gtk::Switch,
    pub scroll_vert_touch: gtk::Switch,
    pub mouse_wheel: gtk::Switch,
    pub pinch: gtk::Switch,
    pub axis_move_time: gtk::Switch,
    pub axis_move_price: gtk::Switch,
    pub axis_reset_time: gtk::Switch,
    pub axis_reset_price: gtk::Switch,
    pub kinetic_mouse: gtk::Switch,
    pub kinetic_touch: gtk::Switch,
    pub tracking_mode: gtk::Switch,
    pub axis_drag_time: gtk::SpinButton,
    pub axis_drag_price: gtk::SpinButton,
    pub wheel_zoom: gtk::SpinButton,
    pub pinch_zoom: gtk::SpinButton,
}

#[derive(Clone)]
pub struct TooltipControls {
    pub enabled: gtk::Switch,
    pub position: gtk::ComboBoxText,
    pub format: gtk::Entry,
    pub line_format: gtk::Entry,
    pub hist_format: gtk::Entry,
    pub background: gtk::ColorButton,
    pub text: gtk::ColorButton,
}

#[derive(Clone)]
pub struct ProfilesControls {
    pub folder_entry: gtk::Entry,
    pub select_folder: gtk::Button,
    pub profile_name: gtk::Entry,
    pub profiles_combo: gtk::ComboBoxText,
    pub refresh: gtk::Button,
    pub open_file: gtk::Button,
    pub load: gtk::Button,
    pub save: gtk::Button,
}

#[derive(Clone)]
pub struct SettingsControls {
    pub chart: ChartControls,
    pub candles: CandleControls,
    pub series: SeriesControls,
    pub price_scale: PriceScaleControls,
    pub crosshair: CrosshairControls,
    pub time: TimeScaleControls,
    pub interaction: InteractionControls,
    pub tooltip: TooltipControls,
    pub profiles: ProfilesControls,
}

pub fn build_settings(
    stack: &gtk::Stack,
    sidebar: &gtk::StackSidebar,
) -> SettingsControls {
    sidebar.set_stack(stack);

    let (chart_page, chart) = build_chart_page();
    stack.add_titled(&chart_page, Some("chart"), "Chart");

    let (candles_page, candles) = build_candles_page();
    stack.add_titled(&candles_page, Some("candles"), "Candles");

    let (series_page, series) = series::build_series_page();
    stack.add_titled(&series_page, Some("series"), "Series");

    let (price_scale_page, price_scale) = build_price_scale_page();
    stack.add_titled(&price_scale_page, Some("price_scale"), "Price Scales");

    let (crosshair_page, crosshair) = build_crosshair_page();
    stack.add_titled(&crosshair_page, Some("crosshair"), "Crosshair");

    let (time_page, time) = time_scale::build_time_scale_page();
    stack.add_titled(&time_page, Some("time"), "Time Scale");

    let (interaction_page, interaction) = build_interaction_page();
    stack.add_titled(&interaction_page, Some("interaction"), "Interaction");

    let (tooltip_page, tooltip) = build_tooltip_page();
    stack.add_titled(&tooltip_page, Some("tooltip"), "Tooltip");

    let (profiles_page, profiles) = build_profiles_page();
    stack.add_titled(&profiles_page, Some("profiles"), "Profiles");

    SettingsControls {
        chart,
        candles,
        series,
        price_scale,
        crosshair,
        time,
        interaction,
        tooltip,
        profiles,
    }
}

fn build_chart_page() -> (gtk::Box, ChartControls) {
    let page = page_box();

    let background_color = gtk::ColorButton::new();
    let grid_switch = gtk::Switch::new();
    let grid_color = gtk::ColorButton::new();

    page.append(&section_label("Chart"));
    page.append(&row_with_label("Background", &background_color));
    page.append(&row_with_two("Grid", &grid_switch, &grid_color));

    (
        page,
        ChartControls {
            background_color,
            grid_switch,
            grid_color,
        },
    )
}

fn build_candles_page() -> (gtk::Box, CandleControls) {
    let page = page_box();

    let up_color = gtk::ColorButton::new();
    let down_color = gtk::ColorButton::new();
    let border_up_color = gtk::ColorButton::new();
    let border_down_color = gtk::ColorButton::new();
    let wick_up_color = gtk::ColorButton::new();
    let wick_down_color = gtk::ColorButton::new();

    page.append(&section_label("Candles"));
    page.append(&row_with_label("Body Up", &up_color));
    page.append(&row_with_label("Body Down", &down_color));
    page.append(&row_with_label("Border Up", &border_up_color));
    page.append(&row_with_label("Border Down", &border_down_color));
    page.append(&row_with_label("Wick Up", &wick_up_color));
    page.append(&row_with_label("Wick Down", &wick_down_color));

    (
        page,
        CandleControls {
            up_color,
            down_color,
            border_up_color,
            border_down_color,
            wick_up_color,
            wick_down_color,
        },
    )
}

fn build_price_scale_page() -> (gtk::Box, PriceScaleControls) {
    let page = page_box();

    let left = build_price_scale_side_controls("Left");
    let right = build_price_scale_side_controls("Right");

    page.append(&section_label("Left Scale"));
    append_price_scale_side(&page, &left);
    page.append(&separator());
    page.append(&section_label("Right Scale"));
    append_price_scale_side(&page, &right);

    (page, PriceScaleControls { left, right })
}

fn build_crosshair_page() -> (gtk::Box, CrosshairControls) {
    let page = page_box();

    let mode_combo = gtk::ComboBoxText::new();
    let vertical_switch = gtk::Switch::new();
    let horizontal_switch = gtk::Switch::new();
    let snap_ohlc = gtk::Switch::new();
    let snap_series = gtk::Switch::new();
    let line_style = gtk::ComboBoxText::new();
    let width = gtk::SpinButton::new(None::<&gtk::Adjustment>, 0.0, 1);
    width.set_numeric(true);
    width.set_digits(1);
    let center_combo = gtk::ComboBoxText::new();
    let center_size = gtk::SpinButton::new(None::<&gtk::Adjustment>, 0.0, 1);
    center_size.set_numeric(true);
    center_size.set_digits(1);
    let color = gtk::ColorButton::new();
    let center_color = gtk::ColorButton::new();

    page.append(&section_label("Crosshair"));
    page.append(&row_with_label("Mode", &mode_combo));
    page.append(&row_with_label("Vertical", &vertical_switch));
    page.append(&row_with_label("Horizontal", &horizontal_switch));
    page.append(&row_with_label("Snap OHLC", &snap_ohlc));
    page.append(&row_with_label("Snap Series", &snap_series));
    page.append(&row_with_label("Style", &line_style));
    page.append(&row_with_label("Width", &width));
    page.append(&row_with_label("Center", &center_combo));
    page.append(&row_with_label("Center size", &center_size));
    page.append(&row_with_label("Color", &color));
    page.append(&row_with_label("Center color", &center_color));

    (
        page,
        CrosshairControls {
            mode_combo,
            vertical_switch,
            horizontal_switch,
            snap_ohlc,
            snap_series,
            line_style,
            width,
            center_combo,
            center_size,
            color,
            center_color,
        },
    )
}

fn build_interaction_page() -> (gtk::Box, InteractionControls) {
    let page = page_box();

    let scroll_mouse_wheel = gtk::Switch::new();
    let scroll_pressed_move = gtk::Switch::new();
    let scroll_horz_touch = gtk::Switch::new();
    let scroll_vert_touch = gtk::Switch::new();
    let mouse_wheel = gtk::Switch::new();
    let pinch = gtk::Switch::new();
    let axis_move_time = gtk::Switch::new();
    let axis_move_price = gtk::Switch::new();
    let axis_reset_time = gtk::Switch::new();
    let axis_reset_price = gtk::Switch::new();
    let kinetic_mouse = gtk::Switch::new();
    let kinetic_touch = gtk::Switch::new();
    let tracking_mode = gtk::Switch::new();
    let axis_drag_time = gtk::SpinButton::new(None::<&gtk::Adjustment>, 0.0, 4);
    axis_drag_time.set_numeric(true);
    axis_drag_time.set_digits(4);
    let axis_drag_price = gtk::SpinButton::new(None::<&gtk::Adjustment>, 0.0, 4);
    axis_drag_price.set_numeric(true);
    axis_drag_price.set_digits(4);
    let wheel_zoom = gtk::SpinButton::new(None::<&gtk::Adjustment>, 0.0, 3);
    wheel_zoom.set_numeric(true);
    wheel_zoom.set_digits(3);
    let pinch_zoom = gtk::SpinButton::new(None::<&gtk::Adjustment>, 0.0, 3);
    pinch_zoom.set_numeric(true);
    pinch_zoom.set_digits(3);

    page.append(&section_label("Handle Scroll"));
    page.append(&row_with_label("Mouse wheel", &scroll_mouse_wheel));
    page.append(&row_with_label("Pressed move", &scroll_pressed_move));
    page.append(&row_with_label("Horz touch", &scroll_horz_touch));
    page.append(&row_with_label("Vert touch", &scroll_vert_touch));

    page.append(&separator());
    page.append(&section_label("Handle Scale"));
    page.append(&row_with_label("Mouse wheel", &mouse_wheel));
    page.append(&row_with_label("Pinch", &pinch));
    page.append(&row_with_label("Axis drag time", &axis_move_time));
    page.append(&row_with_label("Axis drag price", &axis_move_price));
    page.append(&row_with_label("Double click time", &axis_reset_time));
    page.append(&row_with_label("Double click price", &axis_reset_price));

    page.append(&separator());
    page.append(&section_label("Kinetic Scroll"));
    page.append(&row_with_label("Mouse", &kinetic_mouse));
    page.append(&row_with_label("Touch", &kinetic_touch));

    page.append(&separator());
    page.append(&section_label("Tracking Mode"));
    page.append(&row_with_label("Enabled", &tracking_mode));

    page.append(&separator());
    page.append(&section_label("Sensitivity"));
    page.append(&row_with_label("Axis drag (time)", &axis_drag_time));
    page.append(&row_with_label("Axis drag (price)", &axis_drag_price));
    page.append(&row_with_label("Wheel zoom", &wheel_zoom));
    page.append(&row_with_label("Pinch zoom", &pinch_zoom));

    (
        page,
        InteractionControls {
            scroll_mouse_wheel,
            scroll_pressed_move,
            scroll_horz_touch,
            scroll_vert_touch,
            mouse_wheel,
            pinch,
            axis_move_time,
            axis_move_price,
            axis_reset_time,
            axis_reset_price,
            kinetic_mouse,
            kinetic_touch,
            tracking_mode,
            axis_drag_time,
            axis_drag_price,
            wheel_zoom,
            pinch_zoom,
        },
    )
}

fn build_tooltip_page() -> (gtk::Box, TooltipControls) {
    let page = page_box();

    let enabled = gtk::Switch::new();
    let position = gtk::ComboBoxText::new();
    let format = gtk::Entry::new();
    let line_format = gtk::Entry::new();
    let hist_format = gtk::Entry::new();
    let background = gtk::ColorButton::new();
    let text = gtk::ColorButton::new();

    page.append(&section_label("Tooltip"));
    page.append(&row_with_label("Enabled", &enabled));
    page.append(&row_with_label("Position", &position));
    page.append(&row_with_label("Format", &format));
    page.append(&row_with_label("Line fmt", &line_format));
    page.append(&row_with_label("Hist fmt", &hist_format));
    page.append(&row_with_label("BG", &background));
    page.append(&row_with_label("Text", &text));

    (
        page,
        TooltipControls {
            enabled,
            position,
            format,
            line_format,
            hist_format,
            background,
            text,
        },
    )
}

fn build_profiles_page() -> (gtk::Box, ProfilesControls) {
    let page = page_box();

    let folder_entry = gtk::Entry::new();
    folder_entry.set_hexpand(true);
    let select_folder = gtk::Button::with_label("Browse");
    let folder_row = gtk::Box::new(gtk::Orientation::Horizontal, 8);
    folder_row.append(&label_left("Folder"));
    folder_row.append(&folder_entry);
    folder_row.append(&select_folder);

    let profile_name = gtk::Entry::new();
    profile_name.set_hexpand(true);
    page.append(&section_label("Storage"));
    page.append(&folder_row);
    page.append(&row_with_label("Profile name", &profile_name));

    let profiles_combo = gtk::ComboBoxText::new();
    profiles_combo.set_hexpand(true);
    let refresh = gtk::Button::with_label("Refresh");
    let combo_row = gtk::Box::new(gtk::Orientation::Horizontal, 8);
    combo_row.append(&label_left("Profiles"));
    combo_row.append(&profiles_combo);
    combo_row.append(&refresh);

    let open_file = gtk::Button::with_label("Open file");
    let load = gtk::Button::with_label("Load");
    let save = gtk::Button::with_label("Save");
    let actions = gtk::Box::new(gtk::Orientation::Horizontal, 8);
    actions.append(&open_file);
    actions.append(&load);
    actions.append(&save);

    page.append(&separator());
    page.append(&section_label("Profiles"));
    page.append(&combo_row);
    page.append(&actions);

    (
        page,
        ProfilesControls {
            folder_entry,
            select_folder,
            profile_name,
            profiles_combo,
            refresh,
            open_file,
            load,
            save,
        },
    )
}

fn label_left(text: &str) -> gtk::Label {
    let label = gtk::Label::new(Some(text));
    label.set_halign(gtk::Align::Start);
    label
}

fn build_price_scale_side_controls(_title: &str) -> PriceScaleSideControls {
    let visible = gtk::Switch::new();
    let auto_scale = gtk::Switch::new();
    let mode_combo = gtk::ComboBoxText::new();
    let invert = gtk::Switch::new();
    let align_labels = gtk::Switch::new();
    let margin_top = gtk::SpinButton::new(None::<&gtk::Adjustment>, 0.0, 2);
    margin_top.set_numeric(true);
    margin_top.set_digits(2);
    let margin_bottom = gtk::SpinButton::new(None::<&gtk::Adjustment>, 0.0, 2);
    margin_bottom.set_numeric(true);
    margin_bottom.set_digits(2);
    let border_visible = gtk::Switch::new();
    let border_color = gtk::ColorButton::new();
    let text_color = gtk::ColorButton::new();
    let ticks_visible = gtk::Switch::new();
    let min_width = gtk::SpinButton::new(None::<&gtk::Adjustment>, 0.0, 1);
    min_width.set_numeric(true);
    min_width.set_digits(1);
    let entire_text_only = gtk::Switch::new();
    let ensure_edge_ticks = gtk::Switch::new();
    let reset_button = gtk::Button::with_label("Reset Autoscale");

    PriceScaleSideControls {
        visible,
        auto_scale,
        mode_combo,
        invert,
        align_labels,
        margin_top,
        margin_bottom,
        border_visible,
        border_color,
        text_color,
        ticks_visible,
        min_width,
        entire_text_only,
        ensure_edge_ticks,
        reset_button,
    }
}

fn append_price_scale_side(page: &gtk::Box, controls: &PriceScaleSideControls) {
    page.append(&row_with_label("Visible", &controls.visible));
    page.append(&row_with_label("Auto scale", &controls.auto_scale));
    page.append(&row_with_label("Mode", &controls.mode_combo));
    page.append(&row_with_label("Invert", &controls.invert));
    page.append(&row_with_label("Align labels", &controls.align_labels));
    page.append(&row_with_label("Margin top", &controls.margin_top));
    page.append(&row_with_label("Margin bottom", &controls.margin_bottom));
    page.append(&row_with_label("Border", &controls.border_visible));
    page.append(&row_with_label("Border color", &controls.border_color));
    page.append(&row_with_label("Text color", &controls.text_color));
    page.append(&row_with_label("Ticks visible", &controls.ticks_visible));
    page.append(&row_with_label("Min width", &controls.min_width));
    page.append(&row_with_label("Entire text", &controls.entire_text_only));
    page.append(&row_with_label("Edge ticks", &controls.ensure_edge_ticks));
    page.append(&row_with_label("Reset", &controls.reset_button));
}

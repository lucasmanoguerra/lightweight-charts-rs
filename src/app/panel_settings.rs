use crate::chart::{ChartApi, Color, PanelId, PanelRole};
use crate::settings_ui::SettingsControls;
use relm4::gtk;
use relm4::gtk::prelude::*;
use relm4::RelmWidgetExt;
use std::cell::Cell;
use std::rc::Rc;

#[derive(Clone)]
pub struct PanelSettingsUi {
    pub window: gtk::Window,
    pub title_label: gtk::Label,
    pub note_label: gtk::Label,
    pub color_row: gtk::Box,
    pub color_button: gtk::ColorButton,
    pub follow_row: gtk::Box,
    pub follow_switch: gtk::Switch,
    pub auto_scale_row: gtk::Box,
    pub auto_scale_switch: gtk::Switch,
    pub visible_row: gtk::Box,
    pub visible_switch: gtk::Switch,
    pub open_global: gtk::Button,
    pub current_panel: Rc<Cell<PanelId>>,
}

fn panel_row(label: &str, widget: &impl IsA<gtk::Widget>) -> gtk::Box {
    let row = gtk::Box::new(gtk::Orientation::Horizontal, 8);
    let label = gtk::Label::new(Some(label));
    label.set_halign(gtk::Align::Start);
    label.set_hexpand(true);
    row.append(&label);
    row.append(widget);
    row
}

pub fn build_panel_settings_ui(
    parent: &gtk::ApplicationWindow,
    settings_window: &gtk::Window,
    chart: ChartApi,
    settings: &SettingsControls,
    drawing_area: &gtk::DrawingArea,
) -> PanelSettingsUi {
    let window = gtk::Window::new();
    window.set_title(Some("Panel Settings"));
    window.set_default_width(320);
    window.set_default_height(220);
    window.set_modal(true);
    window.set_transient_for(Some(parent));

    let content = gtk::Box::new(gtk::Orientation::Vertical, 10);
    content.set_margin_all(12);

    let title_label = gtk::Label::new(Some("Panel"));
    title_label.set_halign(gtk::Align::Start);
    let note_label = gtk::Label::new(None);
    note_label.set_halign(gtk::Align::Start);
    note_label.set_wrap(true);

    let color_button = gtk::ColorButton::new();
    let color_row = panel_row("Color", &color_button);

    let follow_switch = gtk::Switch::new();
    let follow_row = panel_row("Follow candle colors", &follow_switch);

    let auto_scale_switch = gtk::Switch::new();
    let auto_scale_row = panel_row("Auto-scale", &auto_scale_switch);

    let visible_switch = gtk::Switch::new();
    let visible_row = panel_row("Price scale visible", &visible_switch);

    let open_global = gtk::Button::with_label("Open global settings");

    content.append(&title_label);
    content.append(&note_label);
    content.append(&color_row);
    content.append(&follow_row);
    content.append(&auto_scale_row);
    content.append(&visible_row);
    content.append(&open_global);

    window.set_child(Some(&content));

    let current_panel = Rc::new(Cell::new(PanelId(1)));

    open_global.connect_clicked({
        let settings_window = settings_window.clone();
        move |_| settings_window.present()
    });

    color_button.connect_color_set({
        let chart = chart.clone();
        let settings = settings.clone();
        let drawing_area = drawing_area.clone();
        let current_panel = current_panel.clone();
        move |button| {
            let panel_id = current_panel.get();
            let role = chart.panel_role(panel_id);
            if matches!(role, Some(PanelRole::Indicator)) {
                let rgba = button.rgba();
                chart.set_panel_line_color(
                    panel_id,
                    Color::new(rgba.red() as f64, rgba.green() as f64, rgba.blue() as f64),
                );
                drawing_area.queue_draw();
            } else if matches!(role, Some(PanelRole::Main)) {
                settings
                    .series
                    .hist_color
                    .set_rgba(&button.rgba());
            }
        }
    });

    follow_switch.connect_state_notify({
        let settings = settings.clone();
        let color_button = color_button.clone();
        move |switch| {
            settings
                .series
                .hist_follow_candle_colors
                .set_state(switch.state());
            color_button.set_sensitive(!switch.state());
        }
    });

    auto_scale_switch.connect_state_notify({
        let chart = chart.clone();
        let drawing_area = drawing_area.clone();
        let current_panel = current_panel.clone();
        move |switch| {
            let panel_id = current_panel.get();
            if matches!(chart.panel_role(panel_id), Some(PanelRole::Indicator)) {
                chart.set_panel_auto_scale(panel_id, switch.state());
                drawing_area.queue_draw();
            }
        }
    });

    visible_switch.connect_state_notify({
        let chart = chart.clone();
        let drawing_area = drawing_area.clone();
        let current_panel = current_panel.clone();
        move |switch| {
            let panel_id = current_panel.get();
            if matches!(chart.panel_role(panel_id), Some(PanelRole::Indicator)) {
                chart.set_panel_price_scale_visible(panel_id, switch.state());
                drawing_area.queue_draw();
            }
        }
    });

    PanelSettingsUi {
        window,
        title_label,
        note_label,
        color_row,
        color_button,
        follow_row,
        follow_switch,
        auto_scale_row,
        auto_scale_switch,
        visible_row,
        visible_switch,
        open_global,
        current_panel,
    }
}

pub fn configure_panel_settings(
    ui: &PanelSettingsUi,
    panel: PanelId,
    chart: &ChartApi,
    settings: &SettingsControls,
) {
    ui.current_panel.set(panel);
    let style = chart.style();
    let title = chart
        .panel_title(panel)
        .unwrap_or_else(|| "Panel".to_string());
    ui.title_label.set_text(&title);
    match chart.panel_role(panel) {
        Some(PanelRole::Indicator) => {
            ui.note_label.set_text("Indicator panel settings.");
            ui.color_row.set_visible(true);
            ui.follow_row.set_visible(false);
            ui.auto_scale_row.set_visible(true);
            ui.visible_row.set_visible(true);
            if let Some(color) = chart.panel_line_color(panel) {
                ui.color_button.set_rgba(&gtk::gdk::RGBA::new(
                    color.r as f32,
                    color.g as f32,
                    color.b as f32,
                    1.0,
                ));
            }
            if let Some(auto_scale) = chart.panel_auto_scale(panel) {
                ui.auto_scale_switch.set_state(auto_scale);
            }
            if let Some(visible) = chart.panel_price_scale_visible(panel) {
                ui.visible_switch.set_state(visible);
            }
        }
        _ => {
            ui.note_label
                .set_text("Main panel settings and histogram follow.");
            ui.color_row.set_visible(true);
            ui.follow_row.set_visible(true);
            ui.auto_scale_row.set_visible(false);
            ui.visible_row.set_visible(false);
            ui.color_button.set_rgba(&gtk::gdk::RGBA::new(
                style.histogram.r as f32,
                style.histogram.g as f32,
                style.histogram.b as f32,
                1.0,
            ));
            let follow = settings.series.hist_follow_candle_colors.state();
            ui.follow_switch.set_state(follow);
            ui.color_button.set_sensitive(!follow);
        }
    }
}

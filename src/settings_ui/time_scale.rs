use relm4::gtk;
use relm4::gtk::prelude::*;

use crate::settings_ui::helpers::{page_box, row_with_label, section_label, separator};

#[derive(Clone)]
pub struct TimeScaleControls {
    pub visible_switch: gtk::Switch,
    pub border_visible: gtk::Switch,
    pub border_color: gtk::ColorButton,
    pub ticks_visible: gtk::Switch,
    pub time_visible: gtk::Switch,
    pub seconds_visible: gtk::Switch,
    pub tick_format_entry: gtk::Entry,
    pub tick_max_len: gtk::SpinButton,
    pub label_mode_combo: gtk::ComboBoxText,
    pub label_format_entry: gtk::Entry,
    pub bar_spacing: gtk::SpinButton,
    pub min_spacing: gtk::SpinButton,
    pub max_spacing: gtk::SpinButton,
    pub right_offset: gtk::SpinButton,
    pub right_offset_px: gtk::SpinButton,
    pub fix_left: gtk::Switch,
    pub fix_right: gtk::Switch,
    pub lock_visible_range: gtk::Switch,
    pub right_bar_stays: gtk::Switch,
    pub shift_on_new_bar: gtk::Switch,
    pub uniform_distribution: gtk::Switch,
    pub min_height: gtk::SpinButton,
    pub fit_content: gtk::Button,
}

pub(crate) fn build_time_scale_page() -> (gtk::Box, TimeScaleControls) {
    let page = page_box();

    let visible_switch = gtk::Switch::new();
    let border_visible = gtk::Switch::new();
    let border_color = gtk::ColorButton::new();
    let ticks_visible = gtk::Switch::new();
    let time_visible = gtk::Switch::new();
    let seconds_visible = gtk::Switch::new();
    let tick_format_entry = gtk::Entry::new();
    let tick_max_len = gtk::SpinButton::new(None::<&gtk::Adjustment>, 1.0, 0);
    tick_max_len.set_numeric(true);
    let label_mode_combo = gtk::ComboBoxText::new();
    let label_format_entry = gtk::Entry::new();
    label_format_entry.set_hexpand(true);
    label_format_entry.set_placeholder_text(Some("{YYYY}-{MM}-{DD} {HH}:{mm}"));
    let bar_spacing = gtk::SpinButton::new(None::<&gtk::Adjustment>, 0.0, 1);
    bar_spacing.set_numeric(true);
    bar_spacing.set_digits(1);
    let min_spacing = gtk::SpinButton::new(None::<&gtk::Adjustment>, 0.0, 1);
    min_spacing.set_numeric(true);
    min_spacing.set_digits(1);
    let max_spacing = gtk::SpinButton::new(None::<&gtk::Adjustment>, 0.0, 1);
    max_spacing.set_numeric(true);
    max_spacing.set_digits(1);
    let right_offset = gtk::SpinButton::new(None::<&gtk::Adjustment>, 0.0, 1);
    right_offset.set_numeric(true);
    right_offset.set_digits(1);
    let right_offset_px = gtk::SpinButton::new(None::<&gtk::Adjustment>, 0.0, 1);
    right_offset_px.set_numeric(true);
    right_offset_px.set_digits(1);
    let fix_left = gtk::Switch::new();
    let fix_right = gtk::Switch::new();
    let lock_visible_range = gtk::Switch::new();
    let right_bar_stays = gtk::Switch::new();
    let shift_on_new_bar = gtk::Switch::new();
    let uniform_distribution = gtk::Switch::new();
    let min_height = gtk::SpinButton::new(None::<&gtk::Adjustment>, 0.0, 1);
    min_height.set_numeric(true);
    min_height.set_digits(1);
    let fit_content = gtk::Button::with_label("Fit Content");

    page.append(&section_label("Time Axis"));
    page.append(&row_with_label("Visible", &visible_switch));
    page.append(&row_with_label("Border", &border_visible));
    page.append(&row_with_label("Border color", &border_color));
    page.append(&row_with_label("Ticks visible", &ticks_visible));
    page.append(&row_with_label("Time visible", &time_visible));
    page.append(&row_with_label("Seconds visible", &seconds_visible));
    page.append(&row_with_label("Tick format", &tick_format_entry));
    page.append(&row_with_label("Tick max len", &tick_max_len));
    page.append(&row_with_label("Label mode", &label_mode_combo));
    page.append(&row_with_label("Label format", &label_format_entry));

    page.append(&separator());
    page.append(&section_label("Spacing"));
    page.append(&row_with_label("Bar spacing", &bar_spacing));
    page.append(&row_with_label("Min spacing", &min_spacing));
    page.append(&row_with_label("Max spacing", &max_spacing));
    page.append(&row_with_label("Right offset", &right_offset));
    page.append(&row_with_label("Right offset px", &right_offset_px));
    page.append(&row_with_label("Fix left", &fix_left));
    page.append(&row_with_label("Fix right", &fix_right));
    page.append(&row_with_label("Minimum height", &min_height));
    page.append(&row_with_label("Uniform ticks", &uniform_distribution));

    page.append(&separator());
    page.append(&section_label("Behavior"));
    page.append(&row_with_label("Lock range", &lock_visible_range));
    page.append(&row_with_label("Right bar stays", &right_bar_stays));
    page.append(&row_with_label("Shift on new", &shift_on_new_bar));
    page.append(&row_with_label("Fit content", &fit_content));

    (
        page,
        TimeScaleControls {
            visible_switch,
            border_visible,
            border_color,
            ticks_visible,
            time_visible,
            seconds_visible,
            tick_format_entry,
            tick_max_len,
            label_mode_combo,
            label_format_entry,
            bar_spacing,
            min_spacing,
            max_spacing,
            right_offset,
            right_offset_px,
            fix_left,
            fix_right,
            lock_visible_range,
            right_bar_stays,
            shift_on_new_bar,
            uniform_distribution,
            min_height,
            fit_content,
        },
    )
}

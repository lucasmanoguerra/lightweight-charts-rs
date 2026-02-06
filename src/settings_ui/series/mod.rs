use crate::ui::marker_icons::{marker_icon_categories, populate_marker_icon_combo};
use relm4::gtk;
use relm4::gtk::prelude::*;

use crate::settings_ui::helpers::{
    page_box, row_with_label, row_with_two, section_label, separator,
};

#[derive(Clone)]
pub struct SeriesPriceLineControls {
    pub visible: gtk::Switch,
    pub style: gtk::ComboBoxText,
    pub width: gtk::SpinButton,
    pub color: gtk::ColorButton,
}

#[derive(Clone)]
pub struct SeriesLastValueControls {
    pub visible: gtk::Switch,
    pub background: gtk::ColorButton,
    pub text: gtk::ColorButton,
}

#[derive(Clone)]
pub struct SeriesFormatControls {
    pub format_combo: gtk::ComboBoxText,
    pub precision: gtk::SpinButton,
    pub min_move: gtk::SpinButton,
}

#[derive(Clone)]
pub struct SeriesMarkerControls {
    pub auto_scale: gtk::Switch,
    pub z_order: gtk::ComboBoxText,
    pub icon_category: gtk::ComboBoxText,
    pub icon_combo: gtk::ComboBoxText,
    pub icon_color: gtk::ColorButton,
    pub icon_entry: gtk::Entry,
}

#[derive(Clone)]
pub struct SeriesMarkersControls {
    pub candles: SeriesMarkerControls,
    pub line: SeriesMarkerControls,
    pub hist: SeriesMarkerControls,
}

#[derive(Clone)]
pub struct SeriesPriceLinesControls {
    pub selector: gtk::ComboBoxText,
    pub add_button: gtk::Button,
    pub remove_button: gtk::Button,
    pub export_button: gtk::Button,
    pub import_button: gtk::Button,
    pub price: gtk::SpinButton,
    pub line_visible: gtk::Switch,
    pub axis_label_visible: gtk::Switch,
    pub style: gtk::ComboBoxText,
    pub width: gtk::SpinButton,
    pub line_opacity: gtk::SpinButton,
    pub color: gtk::ColorButton,
    pub label_color: gtk::ColorButton,
    pub label_text_color: gtk::ColorButton,
    pub label_alpha: gtk::SpinButton,
    pub label_padding: gtk::SpinButton,
    pub label_radius: gtk::SpinButton,
    pub label_border_color: gtk::ColorButton,
    pub label_border_width: gtk::SpinButton,
    pub title: gtk::Entry,
}

#[derive(Clone)]
pub struct SeriesControls {
    pub panel_selector: gtk::ComboBoxText,
    pub series_selector: gtk::ComboBoxText,
    pub candles_scale_combo: gtk::ComboBoxText,
    pub line_scale_combo: gtk::ComboBoxText,
    pub hist_scale_combo: gtk::ComboBoxText,
    pub line_color: gtk::ColorButton,
    pub hist_color: gtk::ColorButton,
    pub hist_follow_candle_colors: gtk::Switch,
    pub candles_price_line: SeriesPriceLineControls,
    pub candles_last_value: SeriesLastValueControls,
    pub line_price_line: SeriesPriceLineControls,
    pub line_last_value: SeriesLastValueControls,
    pub hist_price_line: SeriesPriceLineControls,
    pub hist_last_value: SeriesLastValueControls,
    pub candles_format: SeriesFormatControls,
    pub line_format: SeriesFormatControls,
    pub hist_format: SeriesFormatControls,
    pub markers: SeriesMarkersControls,
    pub candles_lines: SeriesPriceLinesControls,
    pub line_lines: SeriesPriceLinesControls,
    pub hist_lines: SeriesPriceLinesControls,
}

pub(crate) fn build_series_page() -> (gtk::Box, SeriesControls) {
    let page = page_box();

    let panel_selector = gtk::ComboBoxText::new();
    panel_selector.append_text("Main");
    panel_selector.set_active(Some(0));
    let series_selector = gtk::ComboBoxText::new();
    series_selector.append_text("Candles");
    series_selector.append_text("Line");
    series_selector.append_text("Histogram");
    series_selector.set_active(Some(0));

    let candles_scale_combo = gtk::ComboBoxText::new();
    let line_scale_combo = gtk::ComboBoxText::new();
    let hist_scale_combo = gtk::ComboBoxText::new();
    let line_color = gtk::ColorButton::new();
    let hist_color = gtk::ColorButton::new();
    let hist_follow_candle_colors = gtk::Switch::new();

    let candles_price_line = build_price_line_controls();
    let candles_last_value = build_last_value_controls();
    let line_price_line = build_price_line_controls();
    let line_last_value = build_last_value_controls();
    let hist_price_line = build_price_line_controls();
    let hist_last_value = build_last_value_controls();

    let candles_format = build_format_controls();
    let line_format = build_format_controls();
    let hist_format = build_format_controls();

    let markers = SeriesMarkersControls {
        candles: build_marker_controls(),
        line: build_marker_controls(),
        hist: build_marker_controls(),
    };

    let candles_lines = build_price_lines_controls();
    let line_lines = build_price_lines_controls();
    let hist_lines = build_price_lines_controls();

    page.append(&section_label("Series"));
    page.append(&row_with_label("Panel", &panel_selector));
    page.append(&row_with_label("Series", &series_selector));
    page.append(&row_with_label("Candles scale", &candles_scale_combo));
    page.append(&row_with_label("Line scale", &line_scale_combo));
    page.append(&row_with_label("Hist scale", &hist_scale_combo));
    page.append(&row_with_label("Line color", &line_color));
    page.append(&row_with_label("Hist color", &hist_color));
    page.append(&row_with_label(
        "Hist follow candles",
        &hist_follow_candle_colors,
    ));

    page.append(&separator());
    page.append(&section_label("Candles PriceLine"));
    append_price_line_controls(&page, &candles_price_line);
    page.append(&section_label("Candles Last Value"));
    append_last_value_controls(&page, &candles_last_value);
    page.append(&section_label("Candles Format"));
    append_format_controls(&page, &candles_format);

    page.append(&separator());
    page.append(&section_label("Line PriceLine"));
    append_price_line_controls(&page, &line_price_line);
    page.append(&section_label("Line Last Value"));
    append_last_value_controls(&page, &line_last_value);
    page.append(&section_label("Line Format"));
    append_format_controls(&page, &line_format);

    page.append(&separator());
    page.append(&section_label("Histogram PriceLine"));
    append_price_line_controls(&page, &hist_price_line);
    page.append(&section_label("Histogram Last Value"));
    append_last_value_controls(&page, &hist_last_value);
    page.append(&section_label("Histogram Format"));
    append_format_controls(&page, &hist_format);

    page.append(&separator());
    page.append(&section_label("Candles Markers"));
    append_marker_controls(&page, &markers.candles);
    page.append(&section_label("Line Markers"));
    append_marker_controls(&page, &markers.line);
    page.append(&section_label("Histogram Markers"));
    append_marker_controls(&page, &markers.hist);

    page.append(&separator());
    page.append(&section_label("Candles Price Lines"));
    append_price_lines_controls(&page, &candles_lines);
    page.append(&separator());
    page.append(&section_label("Line Price Lines"));
    append_price_lines_controls(&page, &line_lines);
    page.append(&separator());
    page.append(&section_label("Histogram Price Lines"));
    append_price_lines_controls(&page, &hist_lines);

    (
        page,
        SeriesControls {
            panel_selector,
            series_selector,
            candles_scale_combo,
            line_scale_combo,
            hist_scale_combo,
            line_color,
            hist_color,
            hist_follow_candle_colors,
            candles_price_line,
            candles_last_value,
            line_price_line,
            line_last_value,
            hist_price_line,
            hist_last_value,
            candles_format,
            line_format,
            hist_format,
            markers,
            candles_lines,
            line_lines,
            hist_lines,
        },
    )
}

fn build_price_line_controls() -> SeriesPriceLineControls {
    let width = gtk::SpinButton::new(None::<&gtk::Adjustment>, 0.0, 1);
    width.set_numeric(true);
    width.set_digits(1);
    SeriesPriceLineControls {
        visible: gtk::Switch::new(),
        style: gtk::ComboBoxText::new(),
        width,
        color: gtk::ColorButton::new(),
    }
}

fn append_price_line_controls(page: &gtk::Box, controls: &SeriesPriceLineControls) {
    page.append(&row_with_label("Visible", &controls.visible));
    page.append(&row_with_label("Style", &controls.style));
    page.append(&row_with_label("Width", &controls.width));
    page.append(&row_with_label("Color", &controls.color));
}

fn build_last_value_controls() -> SeriesLastValueControls {
    SeriesLastValueControls {
        visible: gtk::Switch::new(),
        background: gtk::ColorButton::new(),
        text: gtk::ColorButton::new(),
    }
}

fn append_last_value_controls(page: &gtk::Box, controls: &SeriesLastValueControls) {
    page.append(&row_with_label("Visible", &controls.visible));
    page.append(&row_with_label("Background", &controls.background));
    page.append(&row_with_label("Text", &controls.text));
}

fn build_format_controls() -> SeriesFormatControls {
    let precision = gtk::SpinButton::new(None::<&gtk::Adjustment>, 0.0, 0);
    precision.set_numeric(true);
    let min_move = gtk::SpinButton::new(None::<&gtk::Adjustment>, 0.0, 2);
    min_move.set_numeric(true);
    min_move.set_digits(2);
    SeriesFormatControls {
        format_combo: gtk::ComboBoxText::new(),
        precision,
        min_move,
    }
}

fn append_format_controls(page: &gtk::Box, controls: &SeriesFormatControls) {
    page.append(&row_with_label("Format", &controls.format_combo));
    page.append(&row_with_label("Precision", &controls.precision));
    page.append(&row_with_label("Min move", &controls.min_move));
}

fn build_marker_controls() -> SeriesMarkerControls {
    SeriesMarkerControls {
        auto_scale: gtk::Switch::new(),
        z_order: {
            let combo = gtk::ComboBoxText::new();
            combo.append_text("Normal");
            combo.append_text("Top");
            combo.append_text("Bottom");
            combo
        },
        icon_category: {
            let combo = gtk::ComboBoxText::new();
            for category in marker_icon_categories() {
                combo.append_text(category.name);
            }
            combo.set_active(Some(0));
            combo
        },
        icon_combo: {
            let combo = gtk::ComboBoxText::new();
            populate_marker_icon_combo(&combo, 0);
            combo
        },
        icon_color: gtk::ColorButton::new(),
        icon_entry: {
            let entry = gtk::Entry::new();
            entry.set_placeholder_text(Some("Custom glyph/emoji"));
            entry
        },
    }
}

fn append_marker_controls(page: &gtk::Box, controls: &SeriesMarkerControls) {
    page.append(&row_with_label("Auto scale", &controls.auto_scale));
    page.append(&row_with_label("Z order", &controls.z_order));
    page.append(&row_with_label("Icon category", &controls.icon_category));
    page.append(&row_with_two("Icon", &controls.icon_combo, &controls.icon_color));
    page.append(&row_with_label("Custom icon", &controls.icon_entry));
}

fn build_price_lines_controls() -> SeriesPriceLinesControls {
    let selector = gtk::ComboBoxText::new();
    let add_button = gtk::Button::with_label("Add");
    let remove_button = gtk::Button::with_label("Remove");
    let export_button = gtk::Button::with_label("Export");
    let import_button = gtk::Button::with_label("Import");
    let price_adjustment = gtk::Adjustment::new(
        0.0,
        -1_000_000_000.0,
        1_000_000_000.0,
        0.1,
        1.0,
        0.0,
    );
    let price = gtk::SpinButton::new(Some(&price_adjustment), 0.1, 2);
    price.set_numeric(true);
    price.set_digits(2);
    let line_visible = gtk::Switch::new();
    let axis_label_visible = gtk::Switch::new();
    let style = gtk::ComboBoxText::new();
    style.append_text("Solid");
    style.append_text("Dotted");
    style.append_text("Dashed");

    let width_adjustment = gtk::Adjustment::new(1.0, 0.1, 10.0, 0.1, 1.0, 0.0);
    let width = gtk::SpinButton::new(Some(&width_adjustment), 0.1, 1);
    width.set_numeric(true);
    width.set_digits(1);

    let opacity_adjustment = gtk::Adjustment::new(0.6, 0.0, 1.0, 0.05, 0.1, 0.0);
    let line_opacity = gtk::SpinButton::new(Some(&opacity_adjustment), 0.05, 2);
    line_opacity.set_numeric(true);
    line_opacity.set_digits(2);

    let color = gtk::ColorButton::new();
    let label_color = gtk::ColorButton::new();
    let label_text_color = gtk::ColorButton::new();

    let label_alpha_adjustment = gtk::Adjustment::new(0.85, 0.0, 1.0, 0.05, 0.1, 0.0);
    let label_alpha = gtk::SpinButton::new(Some(&label_alpha_adjustment), 0.05, 2);
    label_alpha.set_numeric(true);
    label_alpha.set_digits(2);

    let label_padding_adjustment = gtk::Adjustment::new(5.0, 0.0, 20.0, 0.5, 1.0, 0.0);
    let label_padding = gtk::SpinButton::new(Some(&label_padding_adjustment), 0.5, 1);
    label_padding.set_numeric(true);
    label_padding.set_digits(1);

    let label_radius_adjustment = gtk::Adjustment::new(3.0, 0.0, 20.0, 0.5, 1.0, 0.0);
    let label_radius = gtk::SpinButton::new(Some(&label_radius_adjustment), 0.5, 1);
    label_radius.set_numeric(true);
    label_radius.set_digits(1);

    let label_border_color = gtk::ColorButton::new();
    let label_border_width_adjustment = gtk::Adjustment::new(0.0, 0.0, 10.0, 0.5, 1.0, 0.0);
    let label_border_width = gtk::SpinButton::new(Some(&label_border_width_adjustment), 0.5, 1);
    label_border_width.set_numeric(true);
    label_border_width.set_digits(1);

    let title = gtk::Entry::new();
    title.set_hexpand(true);

    SeriesPriceLinesControls {
        selector,
        add_button,
        remove_button,
        export_button,
        import_button,
        price,
        line_visible,
        axis_label_visible,
        style,
        width,
        line_opacity,
        color,
        label_color,
        label_text_color,
        label_alpha,
        label_padding,
        label_radius,
        label_border_color,
        label_border_width,
        title,
    }
}

fn append_price_lines_controls(page: &gtk::Box, controls: &SeriesPriceLinesControls) {
    let selector_row = gtk::Box::new(gtk::Orientation::Horizontal, 6);
    selector_row.append(&controls.selector);
    selector_row.append(&controls.add_button);
    selector_row.append(&controls.remove_button);
    page.append(&row_with_label("Lines", &selector_row));
    let preset_row = gtk::Box::new(gtk::Orientation::Horizontal, 6);
    preset_row.append(&controls.export_button);
    preset_row.append(&controls.import_button);
    page.append(&row_with_label("Presets", &preset_row));
    page.append(&row_with_label("Price", &controls.price));
    page.append(&row_with_label("Visible", &controls.line_visible));
    page.append(&row_with_label("Axis label", &controls.axis_label_visible));
    page.append(&row_with_label("Style", &controls.style));
    page.append(&row_with_label("Width", &controls.width));
    page.append(&row_with_label("Opacity", &controls.line_opacity));
    page.append(&row_with_label("Color", &controls.color));
    page.append(&row_with_label("Label color", &controls.label_color));
    page.append(&row_with_label("Label text", &controls.label_text_color));
    page.append(&row_with_label("Label alpha", &controls.label_alpha));
    page.append(&row_with_label("Label padding", &controls.label_padding));
    page.append(&row_with_label("Label radius", &controls.label_radius));
    page.append(&row_with_label("Label border", &controls.label_border_color));
    page.append(&row_with_label("Border width", &controls.label_border_width));
    page.append(&row_with_label("Title", &controls.title));
}

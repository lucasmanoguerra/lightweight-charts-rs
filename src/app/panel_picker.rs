use relm4::gtk;
use relm4::gtk::prelude::*;
use relm4::RelmWidgetExt;

#[derive(Clone)]
pub struct PanelPickerUi {
    pub window: gtk::Window,
    pub symbol_combo: gtk::ComboBoxText,
    pub indicator_combo: gtk::ComboBoxText,
}

pub fn build_panel_picker(
    parent: &gtk::ApplicationWindow,
    symbols: &[String],
    on_create: impl Fn(String, String) + 'static,
) -> PanelPickerUi {
    let window = gtk::Window::new();
    window.set_title(Some("Add Panel"));
    window.set_default_width(300);
    window.set_default_height(160);
    window.set_modal(true);
    window.set_transient_for(Some(parent));

    let content = gtk::Box::new(gtk::Orientation::Vertical, 10);
    content.set_margin_all(12);

    let symbol_combo = gtk::ComboBoxText::new();
    for symbol in symbols {
        symbol_combo.append_text(symbol);
    }
    symbol_combo.set_active(Some(0));

    let indicator_combo = gtk::ComboBoxText::new();
    indicator_combo.append_text("Candles");
    indicator_combo.append_text("RSI");
    indicator_combo.append_text("Histogram");
    indicator_combo.set_active(Some(0));

    let symbol_row = gtk::Box::new(gtk::Orientation::Horizontal, 8);
    let symbol_label = gtk::Label::new(Some("Symbol"));
    symbol_label.set_width_chars(10);
    symbol_row.append(&symbol_label);
    symbol_row.append(&symbol_combo);

    let indicator_row = gtk::Box::new(gtk::Orientation::Horizontal, 8);
    let indicator_label = gtk::Label::new(Some("Indicator"));
    indicator_label.set_width_chars(10);
    indicator_row.append(&indicator_label);
    indicator_row.append(&indicator_combo);

    let buttons = gtk::Box::new(gtk::Orientation::Horizontal, 8);
    buttons.set_halign(gtk::Align::End);
    let cancel = gtk::Button::with_label("Cancel");
    let create = gtk::Button::with_label("Create");
    buttons.append(&cancel);
    buttons.append(&create);

    content.append(&symbol_row);
    content.append(&indicator_row);
    content.append(&buttons);
    window.set_child(Some(&content));

    cancel.connect_clicked({
        let window = window.clone();
        move |_| window.hide()
    });

    create.connect_clicked({
        let symbol_combo = symbol_combo.clone();
        let indicator_combo = indicator_combo.clone();
        let window = window.clone();
        move |_| {
            let symbol = symbol_combo
                .active_text()
                .map(|s| s.to_string())
                .unwrap_or_else(|| "BTCUSDT".to_string());
            let indicator = indicator_combo
                .active_text()
                .map(|s| s.to_string())
                .unwrap_or_else(|| "Candles".to_string());
            on_create(symbol, indicator);
            window.hide();
        }
    });

    PanelPickerUi {
        window,
        symbol_combo,
        indicator_combo,
    }
}

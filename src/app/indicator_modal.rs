use crate::chart::PanelId;
use relm4::gtk;
use relm4::gtk::prelude::*;
use relm4::RelmWidgetExt;
use std::cell::Cell;
use std::collections::HashSet;
use std::rc::Rc;

#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
pub enum IndicatorKind {
    Rsi,
    Macd,
    Stochastic,
    StochRsi,
    Bollinger,
    Ema,
    Sma,
}

impl IndicatorKind {
    pub fn label(self) -> &'static str {
        match self {
            IndicatorKind::Rsi => "RSI",
            IndicatorKind::Macd => "MACD",
            IndicatorKind::Stochastic => "Stochastic",
            IndicatorKind::StochRsi => "Stochastic RSI",
            IndicatorKind::Bollinger => "Bollinger Bands",
            IndicatorKind::Ema => "EMA",
            IndicatorKind::Sma => "SMA",
        }
    }

    pub fn all() -> &'static [IndicatorKind] {
        &[
            IndicatorKind::Rsi,
            IndicatorKind::Macd,
            IndicatorKind::Stochastic,
            IndicatorKind::StochRsi,
            IndicatorKind::Bollinger,
            IndicatorKind::Ema,
            IndicatorKind::Sma,
        ]
    }
}

#[derive(Clone)]
pub struct IndicatorModalUi {
    pub window: gtk::Window,
    pub list: gtk::ListBox,
    pub current_panel: Rc<Cell<PanelId>>,
}

pub fn build_indicator_modal(parent: &gtk::ApplicationWindow) -> IndicatorModalUi {
    let window = gtk::Window::new();
    window.set_title(Some("Indicators"));
    window.set_default_width(320);
    window.set_default_height(360);
    window.set_modal(true);
    window.set_transient_for(Some(parent));

    let content = gtk::Box::new(gtk::Orientation::Vertical, 8);
    content.set_margin_all(12);

    let list = gtk::ListBox::new();
    list.add_css_class("boxed-list");
    list.set_selection_mode(gtk::SelectionMode::None);
    content.append(&list);
    window.set_child(Some(&content));

    let current_panel = Rc::new(Cell::new(PanelId(1)));
    IndicatorModalUi {
        window,
        list,
        current_panel,
    }
}

pub fn configure_indicator_modal(
    ui: &IndicatorModalUi,
    panel_id: PanelId,
    active: &HashSet<IndicatorKind>,
    on_toggle: impl Fn(PanelId, IndicatorKind, bool) + 'static,
) {
    ui.current_panel.set(panel_id);
    while let Some(child) = ui.list.first_child() {
        ui.list.remove(&child);
    }

    let on_toggle = Rc::new(on_toggle);

    for indicator in IndicatorKind::all() {
        let row = gtk::Box::new(gtk::Orientation::Horizontal, 8);
        row.set_margin_top(6);
        row.set_margin_bottom(6);

        let label = gtk::Label::new(Some(indicator.label()));
        label.set_halign(gtk::Align::Start);
        label.set_hexpand(true);

        let is_active = active.contains(indicator);
        let state = Rc::new(Cell::new(is_active));
        let action_label = if is_active { "Remove" } else { "Add" };
        let action = gtk::Button::with_label(action_label);
        action.add_css_class("flat");

        action.connect_clicked({
            let on_toggle = on_toggle.clone();
            let indicator = *indicator;
            let action = action.clone();
            let state = state.clone();
            move |_| {
                let next = !state.get();
                on_toggle(panel_id, indicator, next);
                state.set(next);
                action.set_label(if next { "Remove" } else { "Add" });
            }
        });

        row.append(&label);
        row.append(&action);

        let list_row = gtk::ListBoxRow::new();
        list_row.set_child(Some(&row));
        list_row.set_selectable(false);
        list_row.set_activatable(false);
        ui.list.append(&list_row);
    }
}

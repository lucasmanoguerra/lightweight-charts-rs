use relm4::gtk;
use relm4::gtk::prelude::*;

pub struct MarkerIconCategory {
    pub name: &'static str,
    pub icons: &'static [&'static str],
}

const MARKER_ICON_CATEGORIES: &[MarkerIconCategory] = &[
    MarkerIconCategory {
        name: "Basic",
        icons: &["●", "○", "■", "□", "▲", "▼", "◆", "◇"],
    },
    MarkerIconCategory {
        name: "Signals",
        icons: &["✓", "✕", "★", "☆", "⚑", "⚐"],
    },
    MarkerIconCategory {
        name: "Alerts",
        icons: &["⚡", "⚠", "❗", "❓", "⛔"],
    },
    MarkerIconCategory {
        name: "Time",
        icons: &["⏰", "⏱", "⌛", "📅"],
    },
    MarkerIconCategory {
        name: "Arrows",
        icons: &["↑", "↓", "→", "←", "↗", "↘"],
    },
    MarkerIconCategory {
        name: "Finance",
        icons: &["💰", "💹", "₿", "$"],
    },
];

pub(crate) fn marker_icon_categories() -> &'static [MarkerIconCategory] {
    MARKER_ICON_CATEGORIES
}

pub(crate) fn populate_marker_icon_combo(combo: &gtk::ComboBoxText, category_index: usize) {
    combo.remove_all();
    combo.append_text("None");
    let categories = marker_icon_categories();
    let index = category_index.min(categories.len().saturating_sub(1));
    for icon in categories[index].icons {
        combo.append_text(icon);
    }
    combo.set_active(Some(0));
}

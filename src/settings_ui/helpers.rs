use relm4::gtk;
use relm4::gtk::prelude::*;
use relm4::RelmWidgetExt;

pub(crate) fn page_box() -> gtk::Box {
    let page = gtk::Box::new(gtk::Orientation::Vertical, 10);
    page.set_margin_all(12);
    page
}

pub(crate) fn section_label(text: &str) -> gtk::Label {
    let label = gtk::Label::new(Some(text));
    label.set_halign(gtk::Align::Start);
    label
}

pub(crate) fn row_with_label(label: &str, widget: &impl IsA<gtk::Widget>) -> gtk::Box {
    let row = gtk::Box::new(gtk::Orientation::Horizontal, 8);
    let label = gtk::Label::new(Some(label));
    label.set_width_chars(12);
    row.append(&label);
    row.append(widget);
    row
}

pub(crate) fn row_with_two(
    label: &str,
    first: &impl IsA<gtk::Widget>,
    second: &impl IsA<gtk::Widget>,
) -> gtk::Box {
    let row = gtk::Box::new(gtk::Orientation::Horizontal, 8);
    let label = gtk::Label::new(Some(label));
    label.set_width_chars(12);
    row.append(&label);
    row.append(first);
    row.append(second);
    row
}

pub(crate) fn row_with_spin(label: &str, digits: u32) -> gtk::SpinButton {
    let spin = gtk::SpinButton::new(None::<&gtk::Adjustment>, 0.0, digits as u32);
    spin.set_numeric(true);
    spin.set_digits(digits);
    let _row = row_with_label(label, &spin);
    spin
}

pub(crate) fn separator() -> gtk::Separator {
    let separator = gtk::Separator::new(gtk::Orientation::Horizontal);
    separator.set_margin_top(6);
    separator.set_margin_bottom(6);
    separator
}

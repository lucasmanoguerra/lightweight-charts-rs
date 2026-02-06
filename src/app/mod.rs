mod helpers;
mod interaction;
mod data_feed;
mod market_data;
mod indicator_modal;
mod panel_picker;
mod panel_settings;
mod settings_wiring;

use crate::chart::{
    Candle, create_chart, ChartStyle, Color, HistogramPoint, LinePoint, LineStyle, Marker,
    MarkerPosition, MarkerShape, PanelControlAction, PanelId, PanelRole, PriceLineOptions,
    PriceScale,
};
use data_feed::{spawn_kline_stream, DataEvent, LazyLoader};
use market_data::{load_market_data, MarketData, MarketStore};
use crate::settings_ui::build_settings;
use relm4::gtk;
use relm4::gtk::glib;
use relm4::gtk::prelude::*;
use relm4::prelude::*;

use interaction::install_interactions;
use indicator_modal::{build_indicator_modal, configure_indicator_modal, IndicatorKind};
use panel_picker::build_panel_picker;
use panel_settings::{build_panel_settings_ui, configure_panel_settings};
use settings_wiring::{wire_chart_draw, wire_settings_panel};
use std::collections::{HashMap, HashSet};
use std::time::Duration;

struct AppModel;

#[relm4::component]
impl SimpleComponent for AppModel {
    type Init = MarketData;
    type Input = ();
    type Output = ();

    view! {
        #[name = "drawing_area"]
        gtk::DrawingArea {
            set_hexpand: true,
            set_vexpand: true,
        },

        #[name = "auto_scale_left"]
        gtk::ToggleButton {
            set_label: "A",
            set_width_request: 18,
            set_height_request: 18,
            set_halign: gtk::Align::Start,
            set_valign: gtk::Align::End,
            set_margin_start: 6,
            set_margin_bottom: 28,
            set_focus_on_click: false,
            add_css_class: "flat",
            add_css_class: "autoscale-indicator",
            set_tooltip_text: Some("Auto-scale Left"),
        },

        #[name = "auto_scale_right"]
        gtk::ToggleButton {
            set_label: "A",
            set_width_request: 18,
            set_height_request: 18,
            set_halign: gtk::Align::End,
            set_valign: gtk::Align::End,
            set_margin_end: 6,
            set_margin_bottom: 28,
            set_focus_on_click: false,
            add_css_class: "flat",
            add_css_class: "autoscale-indicator",
            set_tooltip_text: Some("Auto-scale Right"),
        },

        #[name = "chart_overlay"]
        gtk::Overlay {
            set_child = Some(&drawing_area),
        },

        #[name = "header_bar"]
        gtk::HeaderBar {
            set_show_title_buttons: true,
            pack_start = &gtk::Label {
                set_label: "Lightweight Charts RS",
            },
        },

        #[name = "settings_stack"]
        gtk::Stack {
            set_hexpand: true,
            set_vexpand: true,
        },

        #[name = "settings_content"]
        gtk::Box {
            set_orientation: gtk::Orientation::Horizontal,
            set_spacing: 12,
            set_margin_all: 12,

            #[name = "settings_sidebar"]
            gtk::StackSidebar {
                set_vexpand: true,
                set_width_request: 160,
            },
            gtk::Separator {
                set_orientation: gtk::Orientation::Vertical,
            },
            gtk::ScrolledWindow {
                set_hexpand: true,
                set_vexpand: true,
                set_child = Some(&settings_stack),
            },
        },

        #[name = "settings_window"]
        gtk::Window {
            set_title: Some("Settings"),
            set_default_width: 640,
            set_default_height: 520,
            set_modal: true,
            set_hide_on_close: true,
            set_child = Some(&settings_content),
        },

        #[root]
        main_window = gtk::ApplicationWindow {
            set_title: Some("Lightweight Charts RS"),
            set_default_width: 960,
            set_default_height: 540,
            set_titlebar = Some(&header_bar),
            set_child = Some(&chart_overlay),
        },
    }

    fn init(
        init: Self::Init,
        _root: Self::Root,
        _sender: ComponentSender<Self>,
    ) -> ComponentParts<Self> {
        let chart = create_chart();
        let MarketData {
            candles,
            volumes,
            rsi,
            symbol,
            interval,
        } = init;
        let store = std::rc::Rc::new(std::cell::RefCell::new(MarketStore::new(
            candles.clone(),
            volumes.clone(),
            rsi.clone(),
            symbol.clone(),
            interval.clone(),
        )));
        let candle_series = chart.add_candlestick_series();
        candle_series.set_data(candles.clone());

        let line_series = chart.add_line_series();
        let line_points: Vec<LinePoint> = candles
            .iter()
            .map(|candle| LinePoint {
                time: candle.time,
                value: candle.close,
            })
            .collect();
        line_series.set_data(line_points);
        line_series.set_price_scale(PriceScale::Left);

        let hist_series = chart.add_histogram_series();
        let volumes_for_chart = volumes.clone();
        hist_series.set_data(volumes_for_chart);
        hist_series.set_price_scale(PriceScale::Left);

        candle_series.set_price_scale(PriceScale::Right);

        line_series.set_price_line_style(LineStyle::Dashed);
        line_series.set_price_line_width(1.5);
        line_series.set_price_line_color(Color::new(0.33, 0.62, 0.98));
        line_series.set_last_value_color(Color::new(0.18, 0.28, 0.4));
        line_series.set_last_value_text_color(Color::new(0.95, 0.96, 0.98));

        let mut price_line_opts = PriceLineOptions::default();
        if let Some(last) = candles.last() {
            price_line_opts.price = last.close;
        }
        price_line_opts.title = Some("Last".to_string());
        price_line_opts.color = Color::new(0.93, 0.76, 0.25);
        price_line_opts.line_style = LineStyle::Dotted;
        let _price_line = candle_series.create_price_line(price_line_opts);

        if candles.len() > 9 {
            candle_series.set_markers(vec![
                Marker {
                    time: candles[5].time,
                    position: MarkerPosition::Above,
                    price: None,
                    shape: MarkerShape::ArrowDown,
                    color: Color::new(0.92, 0.35, 0.32),
                    size: 10.0,
                    icon_text: None,
                    icon_text_color: None,
                    icon_font_size: 0.0,
                    icon_background: None,
                    icon_padding: 2.0,
                    icon_border_color: None,
                    icon_border_width: 0.0,
                    text: Some("S".to_string()),
                    label_background: Some(Color::new(0.18, 0.12, 0.14)),
                    label_text: Some(Color::new(0.95, 0.96, 0.98)),
                    label_padding: 4.0,
                    label_radius: 3.0,
                    label_background_alpha: 0.3,
                    label_border_color: None,
                    label_border_width: 0.0,
                    label_text_size: 0.0,
                    label_offset_x: 0.0,
                    label_offset_y: 0.0,
                },
                Marker {
                    time: candles[9].time,
                    position: MarkerPosition::Below,
                    price: None,
                    shape: MarkerShape::ArrowUp,
                    color: Color::new(0.25, 0.78, 0.54),
                    size: 10.0,
                    icon_text: None,
                    icon_text_color: None,
                    icon_font_size: 0.0,
                    icon_background: None,
                    icon_padding: 2.0,
                    icon_border_color: None,
                    icon_border_width: 0.0,
                    text: Some("B".to_string()),
                    label_background: Some(Color::new(0.12, 0.18, 0.14)),
                    label_text: Some(Color::new(0.95, 0.96, 0.98)),
                    label_padding: 4.0,
                    label_radius: 3.0,
                    label_background_alpha: 0.3,
                    label_border_color: None,
                    label_border_width: 0.0,
                    label_text_size: 0.0,
                    label_offset_x: 0.0,
                    label_offset_y: 0.0,
                },
            ]);
        }

        chart.set_main_header(symbol.clone(), interval.clone());
        chart.set_rsi_panel("RSI".to_string(), rsi.clone());

        let model = AppModel;
        let widgets = view_output!();
        widgets.chart_overlay.add_overlay(&widgets.auto_scale_left);
        widgets.chart_overlay.add_overlay(&widgets.auto_scale_right);
        let settings = build_settings(&widgets.settings_stack, &widgets.settings_sidebar);

        widgets
            .settings_window
            .set_transient_for(Some(&widgets.main_window));
        if let Some(app) = widgets.main_window.application() {
            widgets.settings_window.set_application(Some(&app));
        }

        let settings_button = gtk::Button::with_label("Settings");
        settings_button.connect_clicked({
            let window = widgets.settings_window.clone();
            move |_| {
                window.present();
            }
        });
        let mut symbols = vec![
            store.borrow().symbol.clone(),
            "ETHUSDT".to_string(),
            "BNBUSDT".to_string(),
            "SOLUSDT".to_string(),
        ];
        symbols.sort();
        symbols.dedup();
        let panel_picker = build_panel_picker(&widgets.main_window, &symbols, move |symbol, indicator| {
            eprintln!("Add panel requested: {symbol} / {indicator} (not implemented yet).");
        });

        let add_panel_button = gtk::Button::with_label("Add Panel");
        add_panel_button.set_tooltip_text(Some("Add a new panel"));
        add_panel_button.connect_clicked({
            let panel_picker = panel_picker.clone();
            move |_| panel_picker.window.present()
        });
        widgets.header_bar.pack_end(&add_panel_button);
        widgets.header_bar.pack_end(&settings_button);

        wire_chart_draw(&widgets.drawing_area, chart.clone());

        let panel_settings = build_panel_settings_ui(
            &widgets.main_window,
            &widgets.settings_window,
            chart.clone(),
            &settings,
            &widgets.drawing_area,
        );

        let indicator_modal = build_indicator_modal(&widgets.main_window);
        let indicator_state: std::rc::Rc<std::cell::RefCell<HashMap<PanelId, HashSet<IndicatorKind>>>> =
            std::rc::Rc::new(std::cell::RefCell::new(HashMap::new()));
        if chart.has_rsi_panel() {
            indicator_state
                .borrow_mut()
                .entry(PanelId(1))
                .or_insert_with(HashSet::new)
                .insert(IndicatorKind::Rsi);
        }

        let panel_menu = gtk::Popover::new();
        panel_menu.set_parent(&widgets.drawing_area);
        panel_menu.set_has_arrow(false);
        let menu_box = gtk::Box::new(gtk::Orientation::Vertical, 6);
        let menu_settings = gtk::Button::with_label("Panel settings");
        let menu_indicators = gtk::Button::with_label("Indicators");
        menu_box.append(&menu_settings);
        menu_box.append(&menu_indicators);
        panel_menu.set_child(Some(&menu_box));

        let panel_menu_state = std::rc::Rc::new(std::cell::Cell::new(PanelId(1)));
        menu_settings.connect_clicked({
            let panel_settings = panel_settings.clone();
            let chart = chart.clone();
            let settings = settings.clone();
            let panel_menu = panel_menu.clone();
            let panel_menu_state = panel_menu_state.clone();
            move |_| {
                let panel = panel_menu_state.get();
                configure_panel_settings(&panel_settings, panel, &chart, &settings);
                panel_settings.window.present();
                panel_menu.popdown();
            }
        });

        menu_indicators.connect_clicked({
            let indicator_modal = indicator_modal.clone();
            let indicator_state = indicator_state.clone();
            let chart = chart.clone();
            let store = store.clone();
            let panel_menu = panel_menu.clone();
            let panel_menu_state = panel_menu_state.clone();
            move |_| {
                let panel = panel_menu_state.get();
                let active = indicator_state
                    .borrow()
                    .get(&panel)
                    .cloned()
                    .unwrap_or_default();
                configure_indicator_modal(&indicator_modal, panel, &active, {
                    let indicator_state = indicator_state.clone();
                    let chart = chart.clone();
                    let store = store.clone();
                    move |panel_id, indicator, enabled| {
                        match indicator {
                            IndicatorKind::Rsi => {
                                if enabled {
                                    let data = store.borrow().rsi.clone();
                                    chart.set_rsi_panel("RSI".to_string(), data);
                                    indicator_state
                                        .borrow_mut()
                                        .entry(panel_id)
                                        .or_insert_with(HashSet::new)
                                        .insert(IndicatorKind::Rsi);
                                } else {
                                    chart.clear_rsi_panel();
                                    if let Some(set) = indicator_state.borrow_mut().get_mut(&panel_id) {
                                        set.remove(&IndicatorKind::Rsi);
                                    }
                                }
                            }
                            _ => {
                                eprintln!("Indicator {indicator:?} not wired yet.");
                            }
                        }
                    }
                });
                indicator_modal.window.present();
                panel_menu.popdown();
            }
        });

        let hist_follow = settings.series.hist_follow_candle_colors.clone();
        let (sender, receiver) = std::sync::mpsc::channel::<DataEvent>();
        let receiver = std::rc::Rc::new(std::cell::RefCell::new(receiver));
        let lazy_loader = std::rc::Rc::new(std::cell::RefCell::new(LazyLoader::new(500, 80)));

        glib::timeout_add_local(Duration::from_millis(50), {
            let store = store.clone();
            let candle_series = candle_series.clone();
            let line_series = line_series.clone();
            let hist_series = hist_series.clone();
            let chart = chart.clone();
            let drawing_area = widgets.drawing_area.clone();
            let lazy_loader = lazy_loader.clone();
            let hist_follow = hist_follow.clone();
            let receiver = receiver.clone();
            move || {
                let mut drained = false;
                while let Ok(event) = receiver.borrow_mut().try_recv() {
                    drained = true;
                    match event {
                        DataEvent::Prepend(batch) => {
                            let mut store_ref = store.borrow_mut();
                            let loaded_any = store_ref.prepend_batch(batch);
                            if loaded_any {
                                let candles = store_ref.candles.clone();
                                candle_series.set_data(candles.clone());
                                let line_points: Vec<LinePoint> = candles
                                    .iter()
                                    .map(|candle| LinePoint {
                                        time: candle.time,
                                        value: candle.close,
                                    })
                                    .collect();
                                line_series.set_data(line_points);
                                let volumes = histogram_points_for_chart(
                                    &store_ref.candles,
                                    &store_ref.volumes,
                                    hist_follow.state(),
                                    chart.style(),
                                );
                                hist_series.set_data(volumes);
                                chart.set_rsi_panel_data(store_ref.rsi.clone());
                            }
                            lazy_loader.borrow_mut().finish_success(loaded_any);
                            drawing_area.queue_draw();
                        }
                        DataEvent::Kline(event) => {
                            let style = chart.style();
                            let mut store_ref = store.borrow_mut();
                            let update = store_ref.apply_kline(event, &style);
                            candle_series.update(update.candle.clone());
                            line_series.update(LinePoint {
                                time: update.candle.time,
                                value: update.candle.close,
                            });
                            let mut volume = update.volume.clone();
                            if !hist_follow.state() {
                                volume.color = None;
                            }
                            hist_series.update(volume);
                            chart.set_rsi_panel_data(store_ref.rsi.clone());
                            drawing_area.queue_draw();
                        }
                        DataEvent::LoadFailed(err) => {
                            lazy_loader.borrow_mut().finish_failure();
                            eprintln!("Lazy load failed: {err}");
                        }
                    }
                }
                if drained {
                    drawing_area.queue_draw();
                }
                glib::ControlFlow::Continue
            }
        });

        glib::timeout_add_local(Duration::from_millis(350), {
            let store = store.clone();
            let chart = chart.clone();
            let sender = sender.clone();
            let lazy_loader = lazy_loader.clone();
            move || {
                let store_ref = store.borrow();
                lazy_loader.borrow_mut().maybe_request(&chart, &store_ref, &sender);
                glib::ControlFlow::Continue
            }
        });

        spawn_kline_stream(symbol.clone(), interval.clone(), sender.clone());

        let auto_scale_handler = {
            let left_button = widgets.auto_scale_left.clone();
            let right_button = widgets.auto_scale_right.clone();
            let left_switch = settings.price_scale.left.auto_scale.clone();
            let right_switch = settings.price_scale.right.auto_scale.clone();
            std::rc::Rc::new(move |side: PriceScale| match side {
                PriceScale::Left => {
                    if left_button.is_active() {
                        left_button.set_active(false);
                    }
                    if left_switch.state() {
                        left_switch.set_state(false);
                    }
                }
                PriceScale::Right => {
                    if right_button.is_active() {
                        right_button.set_active(false);
                    }
                    if right_switch.state() {
                        right_switch.set_state(false);
                    }
                }
            })
        };

        let panel_menu_handler = {
            let panel_menu = panel_menu.clone();
            let panel_menu_state = panel_menu_state.clone();
            std::rc::Rc::new(move |panel: PanelId, x: f64, y: f64| {
                panel_menu_state.set(panel);
                let rect = gtk::gdk::Rectangle::new(x as i32, y as i32, 1, 1);
                panel_menu.set_pointing_to(Some(&rect));
                panel_menu.popup();
            })
        };

        let panel_control_handler = {
            let indicator_modal = indicator_modal.clone();
            let indicator_state = indicator_state.clone();
            let chart = chart.clone();
            let store = store.clone();
            let drawing_area = widgets.drawing_area.clone();
            std::rc::Rc::new(move |panel: PanelId, action: PanelControlAction| {
                match action {
                    PanelControlAction::AddAbove | PanelControlAction::AddBelow => {
                        let active = indicator_state
                            .borrow()
                            .get(&panel)
                            .cloned()
                            .unwrap_or_default();
                        configure_indicator_modal(&indicator_modal, panel, &active, {
                            let indicator_state = indicator_state.clone();
                            let chart = chart.clone();
                            let store = store.clone();
                            move |panel_id, indicator, enabled| {
                                match indicator {
                                    IndicatorKind::Rsi => {
                                        if enabled {
                                            let data = store.borrow().rsi.clone();
                                            chart.set_rsi_panel("RSI".to_string(), data);
                                            indicator_state
                                                .borrow_mut()
                                                .entry(panel_id)
                                                .or_insert_with(HashSet::new)
                                                .insert(IndicatorKind::Rsi);
                                        } else {
                                            chart.clear_rsi_panel();
                                            if let Some(set) =
                                                indicator_state.borrow_mut().get_mut(&panel_id)
                                            {
                                                set.remove(&IndicatorKind::Rsi);
                                            }
                                        }
                                    }
                                    _ => {
                                        eprintln!("Indicator {indicator:?} not wired yet.");
                                    }
                                }
                            }
                        });
                        indicator_modal.window.present();
                    }
                    PanelControlAction::ToggleVisible => {
                        chart.toggle_panel_visibility(panel);
                        drawing_area.queue_draw();
                    }
                    PanelControlAction::ToggleCollapsed => {
                        chart.toggle_panel_collapsed(panel);
                        drawing_area.queue_draw();
                    }
                    PanelControlAction::Remove => {
                        if matches!(chart.panel_role(panel), Some(PanelRole::Indicator)) {
                            chart.clear_rsi_panel();
                            if let Some(set) = indicator_state.borrow_mut().get_mut(&panel) {
                                set.remove(&IndicatorKind::Rsi);
                            }
                        } else {
                            chart.remove_panel(panel);
                        }
                        drawing_area.queue_draw();
                    }
                }
            })
        };

        install_interactions(
            &widgets.drawing_area,
            chart.clone(),
            Some(auto_scale_handler),
            Some(panel_menu_handler),
            Some(panel_control_handler),
        );
        wire_settings_panel(
            &widgets.drawing_area,
            chart.clone(),
            candle_series.clone(),
            line_series.clone(),
            hist_series.clone(),
            settings,
            store.clone(),
            widgets.auto_scale_left.clone(),
            widgets.auto_scale_right.clone(),
        );

        ComponentParts { model, widgets }
    }

    fn update(&mut self, _msg: Self::Input, _sender: ComponentSender<Self>) {}
}

fn histogram_points_for_chart(
    candles: &[Candle],
    volumes: &[HistogramPoint],
    follow: bool,
    style: ChartStyle,
) -> Vec<HistogramPoint> {
    if !follow {
        return volumes
            .iter()
            .map(|point| HistogramPoint {
                time: point.time,
                value: point.value,
                color: None,
            })
            .collect();
    }

    let count = candles.len().min(volumes.len());
    let mut points = Vec::with_capacity(count);
    for idx in 0..count {
        let candle = &candles[idx];
        let volume = &volumes[idx];
        let color = if candle.close >= candle.open {
            style.up
        } else {
            style.down
        };
        points.push(HistogramPoint {
            time: volume.time,
            value: volume.value,
            color: Some(color),
        });
    }
    points
}

pub(crate) fn run() {
    let app = RelmApp::new("com.example.lightweight-charts-rs");
    install_css();
    let data = load_market_data();
    app.run::<AppModel>(data);
}

fn install_css() {
    let Some(display) = gtk::gdk::Display::default() else {
        return;
    };
    let provider = gtk::CssProvider::new();
    let _ = provider.load_from_data(
        ".autoscale-indicator { padding: 0; min-width: 18px; min-height: 18px; font-size: 10px; border-radius: 9px; }
.autoscale-indicator:checked { box-shadow: 0 0 6px rgba(130, 200, 255, 0.45); background: rgba(130, 200, 255, 0.12); }",
    );
    gtk::StyleContext::add_provider_for_display(
        &display,
        &provider,
        gtk::STYLE_PROVIDER_PRIORITY_APPLICATION,
    );
}

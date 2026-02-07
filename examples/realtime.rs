use lightweight_charts_rs::{create_chart, sample_candles, Candle, Color};
use relm4::gtk;
use relm4::gtk::prelude::*;
use relm4::gtk::glib;
use std::cell::RefCell;
use std::rc::Rc;
use std::time::Duration;

fn main() {
    let chart = create_chart();
    chart.set_main_header_str("BTC/USDT", "1m");
    chart.set_background_color(Color::new(0.07, 0.09, 0.12));
    chart.set_grid(true, Color::new(0.16, 0.19, 0.24));

    let series = chart.add_candlestick_series();
    let initial = sample_candles();
    series.set_data(initial.clone());

    let candles = Rc::new(RefCell::new(initial));

    let app = gtk::Application::new(
        Some("com.example.lightweight-charts-rs.realtime"),
        Default::default(),
    );
    app.connect_activate(move |app| {
        let window = gtk::ApplicationWindow::new(app);
        window.set_title(Some("Live Updates"));
        window.set_default_size(960, 540);

        let drawing_area = gtk::DrawingArea::new();
        drawing_area.set_hexpand(true);
        drawing_area.set_vexpand(true);

        let chart = chart.clone();
        drawing_area.set_draw_func(move |_, cr, width, height| {
            chart.draw(cr, width as f64, height as f64);
        });

        window.set_child(Some(&drawing_area));
        window.present();

        let series = series.clone();
        let candles = candles.clone();
        let drawing_area = drawing_area.clone();
        let mut direction = 1.0_f64;

        glib::timeout_add_local(Duration::from_millis(900), move || {
            let mut data = candles.borrow_mut();
            if let Some(last) = data.last().cloned() {
                let delta = if direction > 0.0 { 1.8 } else { -1.2 };
                direction = -direction;
                let open = last.close;
                let close = (open + delta).max(1.0);
                let high = open.max(close) + 1.0;
                let low = open.min(close) - 1.0;
                let next = Candle {
                    time: last.time + time::Duration::minutes(1),
                    open,
                    high,
                    low,
                    close,
                };
                data.push(next.clone());
                if data.len() > 120 {
                    data.remove(0);
                }
                series.update(next);
                drawing_area.queue_draw();
            }
            glib::ControlFlow::Continue
        });
    });
    app.run();
}

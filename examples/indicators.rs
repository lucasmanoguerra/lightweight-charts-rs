use lightweight_charts_rs::indicators::{compute_rsi, compute_sma};
use lightweight_charts_rs::{create_chart, sample_candles, Color};
use relm4::gtk;
use relm4::gtk::prelude::*;

fn main() {
    let chart = create_chart();
    chart.set_main_header_str("BTC/USD", "1D");
    chart.set_background_color(Color::new(0.07, 0.09, 0.12));
    chart.set_grid(true, Color::new(0.16, 0.19, 0.24));

    let candles = sample_candles();

    let candle_series = chart.add_candlestick_series();
    candle_series.set_data(candles.clone());

    let sma_points = compute_sma(&candles, 6);
    let sma_series = chart.add_line_series();
    sma_series.set_data(sma_points);

    let rsi_points = compute_rsi(&candles, 14);
    chart.set_rsi_panel_with_title("RSI", rsi_points);

    let app = gtk::Application::new(
        Some("com.example.lightweight-charts-rs.indicators"),
        Default::default(),
    );
    app.connect_activate(move |app| {
        let window = gtk::ApplicationWindow::new(app);
        window.set_title(Some("Indicators"));
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
    });
    app.run();
}

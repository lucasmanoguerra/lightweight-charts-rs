use lightweight_charts_rs::{create_chart, sample_candles, Color, LinePoint};
use relm4::gtk;
use relm4::gtk::prelude::*;

fn main() {
    let chart = create_chart();
    chart.set_main_header_str("EUR/USD", "1D");
    chart.set_background_color(Color::new(0.07, 0.09, 0.12));
    chart.set_grid(true, Color::new(0.16, 0.19, 0.24));
    chart.set_line_color(Color::new(0.33, 0.62, 0.98));

    let candles = sample_candles();
    let points: Vec<LinePoint> = candles
        .iter()
        .map(|candle| LinePoint {
            time: candle.time,
            value: candle.close,
        })
        .collect();

    let series = chart.add_line_series();
    series.set_data(points);

    let app = gtk::Application::new(
        Some("com.example.lightweight-charts-rs.line"),
        Default::default(),
    );
    app.connect_activate(move |app| {
        let window = gtk::ApplicationWindow::new(app);
        window.set_title(Some("Line Chart"));
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

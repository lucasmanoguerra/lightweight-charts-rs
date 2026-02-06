use lightweight_charts_rs::{create_chart, CandlestickSeriesApi, ChartStyle, Color};

fn main() {
    // Create a new chart with custom styling
    let mut chart = create_chart(
        "BTC/USD",
        ChartStyle {
            background_color: Color::from_rgb(24, 26, 27),
            grid_color: Color::from_rgb(44, 46, 47),
            text_color: Color::from_rgb(200, 200, 200),
            ..ChartStyle::default()
        },
    );

    // Sample candlestick data
    let candles = vec![
        // (timestamp, open, high, low, close)
        (1640995200, 47000.0, 47500.0, 46800.0, 47200.0),
        (1641081600, 47200.0, 47800.0, 47100.0, 47600.0),
        (1641168000, 47600.0, 48200.0, 47400.0, 47900.0),
        (1641254400, 47900.0, 48500.0, 47700.0, 48300.0),
        (1641340800, 48300.0, 48800.0, 48000.0, 48500.0),
        (1641427200, 48500.0, 49000.0, 48200.0, 48700.0),
        (1641513600, 48700.0, 49200.0, 48500.0, 49000.0),
        (1641600000, 49000.0, 49500.0, 48700.0, 49200.0),
    ];

    // Add candlestick series to the chart
    chart.add_candlestick_series(candles);

    // Display the chart
    chart.show();
}

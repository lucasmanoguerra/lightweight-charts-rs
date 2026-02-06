use lightweight_charts_rs::{
    create_chart,
    indicators::{RSI, SMA},
    CandlestickSeriesApi, ChartStyle, Color,
};

fn main() {
    // Create chart
    let mut chart = create_chart("BTC/USD", ChartStyle::default());

    // Sample price data
    let candles = vec![
        (1640995200, 47000.0, 47500.0, 46800.0, 47200.0),
        (1641081600, 47200.0, 47800.0, 47100.0, 47600.0),
        (1641168000, 47600.0, 48200.0, 47400.0, 47900.0),
        (1641254400, 47900.0, 48500.0, 47700.0, 48300.0),
        (1641340800, 48300.0, 48800.0, 48000.0, 48500.0),
        (1641427200, 48500.0, 49000.0, 48200.0, 48700.0),
        (1641513600, 48700.0, 49200.0, 48500.0, 49000.0),
        (1641600000, 49000.0, 49500.0, 48700.0, 49200.0),
        (1641686400, 49200.0, 49800.0, 49000.0, 49600.0),
        (1641772800, 49600.0, 50200.0, 49400.0, 50000.0),
        (1641859200, 50000.0, 50500.0, 49800.0, 50300.0),
        (1641945600, 50300.0, 50800.0, 50100.0, 50600.0),
        (1642032000, 50600.0, 51100.0, 50400.0, 50900.0),
        (1642118400, 50900.0, 51400.0, 50700.0, 51200.0),
        (1642204800, 51200.0, 51700.0, 51000.0, 51500.0),
    ];

    // Add candlestick series
    chart.add_candlestick_series(candles.clone());

    // Calculate and add SMA indicator
    let sma = SMA::new(14).calculate(&candles);
    chart.add_line_series(sma);

    // Calculate and add RSI indicator
    let rsi_values = RSI::new(14).calculate(&candles);
    chart.add_indicator_panel("RSI", rsi_values);

    // Display the chart
    chart.show();
}

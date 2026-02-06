use lightweight_charts_rs::{
    create_chart, CandlestickSeriesApi, ChartStyle, Color, WebSocketDataFeed,
};

fn main() {
    // Create chart
    let mut chart = create_chart("BTC/USDT", ChartStyle::default());

    // Add initial sample data
    let initial_candles = vec![
        (1640995200, 47000.0, 47500.0, 46800.0, 47200.0),
        (1641081600, 47200.0, 47800.0, 47100.0, 47600.0),
        (1641168000, 47600.0, 48200.0, 47400.0, 47900.0),
    ];

    chart.add_candlestick_series(initial_candles);

    // Connect to WebSocket for real-time data
    let ws_url = "wss://stream.binance.com:9443/ws/btcusdt@kline_1m";
    let data_feed = WebSocketDataFeed::new(ws_url);

    // Start real-time data streaming
    chart.start_realtime_data(data_feed);

    // Display the chart
    chart.show();
}

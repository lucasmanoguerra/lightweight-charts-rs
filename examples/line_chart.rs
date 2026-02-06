use lightweight_charts_rs::{create_chart, ChartStyle, Color, LinePoint, LineSeriesApi};

fn main() {
    // Create a chart with light theme
    let mut chart = create_chart(
        "EUR/USD",
        ChartStyle {
            background_color: Color::from_rgb(255, 255, 255),
            grid_color: Color::from_rgb(230, 230, 230),
            text_color: Color::from_rgb(50, 50, 50),
            ..ChartStyle::default()
        },
    );

    // Sample line chart data
    let line_data = vec![
        // (timestamp, value)
        (1640995200, 1.1350),
        (1641081600, 1.1375),
        (1641168000, 1.1340),
        (1641254400, 1.1385),
        (1641340800, 1.1360),
        (1641427200, 1.1395),
        (1641513600, 1.1410),
        (1641600000, 1.1370),
    ];

    // Add line series to the chart
    chart.add_line_series(line_data);

    // Display the chart
    chart.show();
}

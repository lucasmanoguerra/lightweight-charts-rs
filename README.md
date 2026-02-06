# Lightweight Charts RS

A lightweight, high-performance financial charting library written in Rust using Relm4 and GTK4. This library provides interactive financial charts with support for various technical indicators and real-time data visualization.

## Features

- 📊 **Interactive Charts**: Candlestick, line, and histogram chart types
- 📈 **Technical Indicators**: Built-in support for RSI, MACD, Bollinger Bands, SMA, EMA, Stochastic, and StochRSI
- 🔄 **Real-time Data**: WebSocket support for live market data
- 🎨 **Customizable**: Extensive styling and configuration options
- 📱 **Cross-platform**: Works on Linux, Windows, and macOS
- ⚡ **High Performance**: Built with Rust for optimal performance
- 🎯 **Lightweight**: Minimal dependencies and fast rendering

## Screenshots

*(Add screenshots here once the application is running)*

## Installation

### From Source

```bash
git clone https://github.com/lucasmanoguerra/lightweight-charts-rs.git
cd lightweight-charts-rs
cargo build --release
```

### As a Library

Add this to your `Cargo.toml`:

```toml
[dependencies]
lightweight-charts-rs = "0.1.0"
```

## Usage

### Basic Chart Example

```rust
use lightweight_charts_rs::{create_chart, CandlestickSeriesApi, ChartStyle};

fn main() {
    // Create a new chart
    let mut chart = create_chart("BTC/USD", ChartStyle::default());
    
    // Add candlestick data
    let candles = vec![
        // (timestamp, open, high, low, close)
        (1640995200, 47000.0, 47500.0, 46800.0, 47200.0),
        (1641081600, 47200.0, 47800.0, 47100.0, 47600.0),
        // ... more data
    ];
    
    chart.add_candlestick_series(candles);
    chart.show();
}
```

### Running the Application

```bash
# Run the main application
cargo run

# Run with example data
cargo run -- --sample-data
```

## Chart Types

- **Candlestick**: Traditional OHLC candlestick charts
- **Line**: Simple line charts for price data
- **Histogram**: Volume and indicator visualization

## Technical Indicators

- **RSI** (Relative Strength Index)
- **MACD** (Moving Average Convergence Divergence)
- **Bollinger Bands**
- **SMA** (Simple Moving Average)
- **EMA** (Exponential Moving Average)
- **Stochastic Oscillator**
- **StochRSI** (Stochastic RSI)

## Configuration

The library supports extensive customization through the `ChartStyle` and `PriceScaleOptions` structs:

```rust
use lightweight_charts_rs::{ChartStyle, PriceScaleOptions, Color};

let style = ChartStyle {
    background_color: Color::from_rgb(24, 26, 27),
    grid_color: Color::from_rgb(44, 46, 47),
    text_color: Color::from_rgb(200, 200, 200),
    // ... more options
};
```

## Development

### Prerequisites

- Rust 1.70+ (edition 2024)
- GTK4 development libraries
- Cairo development libraries

### Building

```bash
# Development build
cargo build

# Release build
cargo build --release

# Run tests
cargo test

# Run with debug output
RUST_LOG=debug cargo run
```

### Project Structure

```
src/
├── app/              # Main application logic
├── chart/            # Core charting functionality
├── indicators/       # Technical indicators
├── ui/               # UI components and styling
├── settings_ui/     # Settings panel implementation
└── main.rs          # Application entry point
```

## Contributing

We welcome contributions! Please see [CONTRIBUTING.md](CONTRIBUTING.md) for guidelines.

## License

This project is licensed under the MIT License - see the [LICENSE](LICENSE) file for details.

## Acknowledgments

- Inspired by [Lightweight Charts](https://github.com/tradingview/lightweight-charts) by TradingView
- Built with [Relm4](https://relm4.org/) for modern Rust GUI development
- Uses [GTK4](https://www.gtk.org/) for cross-platform native UI

## Roadmap

- [ ] More technical indicators
- [ ] Drawing tools support
- [ ] Chart export functionality
- [ ] Plugin system for custom indicators
- [ ] WebAssembly support
- [ ] Mobile app support

## Support

- 📖 [Documentation](https://docs.rs/lightweight-charts-rs)
- 🐛 [Bug Reports](https://github.com/lucasmanoguerra/lightweight-charts-rs/issues)
- 💬 [Discussions](https://github.com/lucasmanoguerra/lightweight-charts-rs/discussions)

---

**Made with ❤️ in Rust**
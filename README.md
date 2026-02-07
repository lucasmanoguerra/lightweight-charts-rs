# Lightweight Charts RS

A lightweight, high-performance financial charting library written in Rust using Relm4 and GTK4. This project is desktop-only (Linux, Windows, macOS). It does not target web or mobile platforms; the only network-facing pieces are optional REST and WebSocket data sources.

## What It Offers Today

- **Interactive charts**: Candlestick, line, and histogram series.
- **Indicators**: RSI, MACD, Bollinger Bands, SMA, EMA, Stochastic, StochRSI.
- **Multi-panel layout**: Main chart + stacked indicator panels.
- **Overlays**: Price lines and markers.
- **Crosshair and tooltips**: Configurable behavior and formatting.
- **Configurable styling**: Colors, grids, scales, and interaction options.
- **Desktop app**: GTK4 UI with settings panel and live data plumbing.
- **Data sources**: Optional REST and WebSocket feeds.

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

### Basic Chart Example (GTK4 DrawingArea)

```rust
use lightweight_charts_rs::{create_chart, sample_candles, Color};
use relm4::gtk;
use relm4::gtk::prelude::*;

fn main() {
    let chart = create_chart();
    chart.set_main_header_str("BTC/USD", "1D");
    chart.set_background_color(Color::new(0.07, 0.09, 0.12));
    chart.set_grid(true, Color::new(0.16, 0.19, 0.24));

    let series = chart.add_candlestick_series();
    series.set_data(sample_candles());

    let app = gtk::Application::new(
        Some("com.example.lightweight-charts-rs.basic"),
        Default::default(),
    );
    app.connect_activate(move |app| {
        let window = gtk::ApplicationWindow::new(app);
        window.set_title(Some("Basic Candlestick"));
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
```

### Running the Application

```bash
# Run the main application
cargo run
```

### Running Examples

```bash
cargo run --example basic_chart
cargo run --example line_chart
cargo run --example indicators
cargo run --example realtime
```

## Project Structure

```
src/
|-- app/           # Desktop application and wiring
|-- chart/         # Core charting engine
|-- indicators/    # Technical indicators
|-- ui/            # UI components and styling
|-- settings_ui/   # Settings panel
`-- main.rs        # Application entry point
```

## Desktop-Only Scope

- This is a GTK4 desktop project for Linux, Windows, and macOS.
- No web or mobile targets are planned.
- REST and WebSocket are used only for data ingestion.

## Roadmap (Condensed)

- **Series parity with Lightweight Charts v5.1**: Area, Bar, Baseline, Custom Series, plugins/primitives, conflation, localization, price line listing, crosshair improvements.
- **Drawing tools**: Trendline, Fibonacci, channels, long/short position tools, persistence.
- **UX and professionalism**: Themes, improved tooltips, layout/workspaces, watchlists, alerts.
- **Performance**: Conflation, culling, caching, profiling.
- **Rendering**: Pango/PangoCairo text layout integration.
- **Modular layout**: Separated UI sections with clear bounds and event routing.
- **Multi-chart**: Linked time scale and crosshair sync.

## Planning Docs (In `docs/`)

- `docs/lightweight-charts-v5.1-gap-plan.md`
- `docs/pango-plan.md`
- `docs/other-libs-plan.md`
- `docs/ux-professional-improvements.md`
- `docs/ux-pro-backlog.md`
- `docs/areas-modularization.md`
- `docs/modular-areas-technical-proposal.md`
- `docs/future-proposals.md`
- `docs/future-proposals-backlog.md`

## Contributing

We welcome contributions! Please see `CONTRIBUTING.md` for guidelines.

## License

This project is licensed under the MIT License - see `LICENSE` for details.

## Acknowledgments

- Inspired by [Lightweight Charts](https://github.com/tradingview/lightweight-charts) by TradingView
- Built with [Relm4](https://relm4.org/) for modern Rust GUI development
- Uses [GTK4](https://www.gtk.org/) for cross-platform native UI

---

**Made with care in Rust**

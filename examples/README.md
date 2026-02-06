# Examples

This directory contains example programs demonstrating how to use the lightweight-charts-rs library.

## Running Examples

Each example can be run individually using cargo:

```bash
# Basic candlestick chart
cargo run --example basic_chart

# Line chart example
cargo run --example line_chart

# Technical indicators
cargo run --example indicators

# Real-time data (requires WebSocket connection)
cargo run --example realtime
```

## Example Descriptions

### `basic_chart.rs`
Demonstrates creating a simple candlestick chart with custom styling and sample data.

### `line_chart.rs`
Shows how to create a line chart with a light theme and line data points.

### `indicators.rs`
Illustrates adding technical indicators (SMA and RSI) to a candlestick chart.

### `realtime.rs`
Example of connecting to a WebSocket data source for live market data updates.

## Building All Examples

```bash
cargo build --examples
```

## Running in Debug Mode

For more verbose output during development:

```bash
RUST_LOG=debug cargo run --example basic_chart
```
pub mod api;
pub mod core;
pub mod data;
pub mod format;
pub mod layout;
pub mod options;
pub mod scales;
pub mod ticks;
pub mod types;
pub mod util;

pub use api::{
    create_chart, CandlestickSeriesApi, ChartApi, HistogramSeriesApi, LineSeriesApi, PriceLineApi,
};
pub use options::{
    ChartStyle, CrosshairOptions, HandleScaleOptions, HandleScrollOptions,
    InteractionSensitivityOptions, KineticScrollOptions, PriceScaleOptions, TimeScaleOptions,
    TooltipOptions, TrackingModeExitMode, TrackingModeOptions,
};
pub use types::{
    Bar, BarConversionError, Candle, Color, CrosshairCenter, CrosshairMode, HistogramPoint,
    LinePoint, LineStyle, Marker, MarkerPosition, MarkerShape, MarkerZOrder, PanelControlAction,
    PanelId, PanelResizeHandle, PanelRole, PriceFormat, PriceLineOptions, PriceScale,
    PriceScaleMode, ScaleMargins, SeriesMarkersOptions, TimeLabelMode, TooltipPosition,
};

use time::OffsetDateTime;

pub fn sample_candles() -> Vec<Candle> {
    let start = OffsetDateTime::now_utc() - time::Duration::days(19);
    let mut price = 100.0;
    let mut candles = Vec::with_capacity(20);

    for i in 0..20 {
        let open = price;
        let high = open + 4.0 + (i as f64 * 0.15);
        let low = open - 3.0 - (i as f64 * 0.1);
        let close = if i % 2 == 0 { open + 2.5 } else { open - 1.8 };
        price = close + (i as f64 * 0.2);

        candles.push(Candle {
            time: start + time::Duration::days(i as i64),
            open,
            high,
            low,
            close,
        });
    }

    candles
}

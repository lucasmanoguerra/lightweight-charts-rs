use super::options::PriceScaleOptions;
use super::types::{
    Candle, Color, HistogramPoint, LinePoint, Marker, PanelId, PanelRole, PriceFormat,
    PriceLineOptions, PriceScale, SeriesMarkersOptions, TimeScaleId,
};
use time::OffsetDateTime;

#[derive(Clone, Copy, Debug)]
pub(crate) enum SeriesKind {
    Candlestick,
    Line,
    Histogram,
}

#[derive(Clone, Debug)]
pub(crate) enum SeriesData {
    Candlestick { data: Vec<Candle> },
    Line { data: Vec<LinePoint> },
    Histogram { data: Vec<HistogramPoint> },
}

#[derive(Clone, Debug)]
pub(crate) struct Series {
    pub(crate) kind: SeriesKind,
    pub(crate) scale: PriceScale,
    pub(crate) panel_id: PanelId,
    pub(crate) data: SeriesData,
    pub(crate) options: SeriesOptions,
    pub(crate) markers: Vec<Marker>,
    pub(crate) price_lines: Vec<PriceLine>,
    pub(crate) next_price_line_id: usize,
}

#[derive(Clone, Debug)]
pub(crate) struct PanelSeries {
    pub(crate) series_id: usize,
    pub(crate) kind: SeriesKind,
}

#[derive(Clone, Debug)]
pub(crate) struct Panel {
    pub(crate) id: PanelId,
    pub(crate) group_id: TimeScaleId,
    pub(crate) role: PanelRole,
    pub(crate) parent_id: Option<PanelId>,
    pub(crate) title: String,
    pub(crate) height_weight: f64,
    pub(crate) content_visible: bool,
    pub(crate) collapsed: bool,
    pub(crate) left_scale: PriceScaleState,
    pub(crate) right_scale: PriceScaleState,
    pub(crate) left_visible: bool,
    pub(crate) right_visible: bool,
    pub(crate) series: Vec<PanelSeries>,
    pub(crate) show_volume: bool,
}

#[derive(Clone, Debug)]
pub(crate) struct IndicatorPanel {
    pub(crate) title: String,
    pub(crate) data: Vec<LinePoint>,
    pub(crate) scale: PriceScaleState,
    pub(crate) options: PriceScaleOptions,
    pub(crate) color: Color,
}

#[derive(Clone, Debug)]
pub(crate) struct TimeScaleGroup {
    pub(crate) id: TimeScaleId,
    pub(crate) time_scale: super::scales::TimeScale,
    pub(crate) panels: Vec<PanelId>,
}

#[derive(Clone, Debug)]
pub(crate) struct SeriesOptions {
    pub(crate) show_price_line: bool,
    pub(crate) show_last_value: bool,
    pub(crate) price_line_color: Option<Color>,
    pub(crate) price_line_style: super::types::LineStyle,
    pub(crate) price_line_width: f64,
    pub(crate) last_value_background: Option<Color>,
    pub(crate) last_value_text: Option<Color>,
    pub(crate) price_format: PriceFormat,
    pub(crate) markers_options: SeriesMarkersOptions,
}

impl Default for SeriesOptions {
    fn default() -> Self {
        Self {
            show_price_line: true,
            show_last_value: true,
            price_line_color: None,
            price_line_style: super::types::LineStyle::Solid,
            price_line_width: 1.0,
            last_value_background: None,
            last_value_text: None,
            price_format: PriceFormat::default(),
            markers_options: SeriesMarkersOptions::default(),
        }
    }
}

#[derive(Clone, Debug)]
pub(crate) struct PriceLine {
    pub(crate) id: usize,
    pub(crate) options: PriceLineOptions,
}

#[derive(Clone, Copy, Debug)]
pub(crate) struct SeriesScale {
    pub(crate) min: f64,
    pub(crate) max: f64,
    pub(crate) mode: super::types::PriceScaleMode,
    pub(crate) base: f64,
    pub(crate) invert: bool,
    pub(crate) margins: super::types::ScaleMargins,
}

#[derive(Clone, Debug)]
pub(crate) struct PriceScaleState {
    pub(crate) data_min: f64,
    pub(crate) data_max: f64,
    pub(crate) view_min: f64,
    pub(crate) view_max: f64,
    pub(crate) auto: bool,
}

impl PriceScaleState {
    pub(crate) fn new() -> Self {
        Self {
            data_min: 0.0,
            data_max: 1.0,
            view_min: 0.0,
            view_max: 1.0,
            auto: true,
        }
    }

    pub(crate) fn update_data(&mut self, min: f64, max: f64, auto_range: (f64, f64)) {
        self.data_min = min;
        self.data_max = max;
        if self.auto {
            self.view_min = auto_range.0;
            self.view_max = auto_range.1;
        }
    }

    pub(crate) fn view_range(&self) -> f64 {
        (self.view_max - self.view_min).max(1.0)
    }

    pub(crate) fn pan(&mut self, delta: f64) {
        self.view_min += delta;
        self.view_max += delta;
        self.auto = false;
    }

    pub(crate) fn zoom(&mut self, factor: f64, anchor: f64) {
        let range = self.view_range();
        let price_at_anchor = self.view_max - anchor * range;
        let new_range = (range * factor).max(1e-9);
        self.view_max = price_at_anchor + anchor * new_range;
        self.view_min = self.view_max - new_range;
        self.auto = false;
    }
}

pub(crate) trait HasTime {
    fn time(&self) -> OffsetDateTime;
}

impl HasTime for Candle {
    fn time(&self) -> OffsetDateTime {
        self.time
    }
}

impl HasTime for LinePoint {
    fn time(&self) -> OffsetDateTime {
        self.time
    }
}

impl HasTime for HistogramPoint {
    fn time(&self) -> OffsetDateTime {
        self.time
    }
}

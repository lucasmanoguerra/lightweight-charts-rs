use rust_decimal::prelude::ToPrimitive;
use rust_decimal::Decimal;
use time::OffsetDateTime;

#[derive(Clone, Debug)]
pub struct Candle {
    pub time: OffsetDateTime,
    pub open: f64,
    pub high: f64,
    pub low: f64,
    pub close: f64,
}

#[derive(Clone, Debug)]
pub struct LinePoint {
    pub time: OffsetDateTime,
    pub value: f64,
}

#[derive(Clone, Debug)]
pub struct HistogramPoint {
    pub time: OffsetDateTime,
    pub value: f64,
    pub color: Option<Color>,
}

#[derive(Clone, Copy, Debug)]
pub struct PanResult {
    pub price_axis_zoomed: Option<PriceScale>,
    pub time_panned: bool,
}

#[derive(Clone, Copy, Debug)]
pub struct Rect {
    pub x: f64,
    pub y: f64,
    pub width: f64,
    pub height: f64,
}

impl Rect {
    pub fn contains(&self, x: f64, y: f64) -> bool {
        x >= self.x
            && x <= self.x + self.width
            && y >= self.y
            && y <= self.y + self.height
    }
}

#[derive(Clone, Debug)]
pub struct Bar {
    pub time: OffsetDateTime,
    pub open: Decimal,
    pub high: Decimal,
    pub low: Decimal,
    pub close: Decimal,
}

#[derive(Clone, Copy, Debug)]
pub struct Color {
    pub r: f64,
    pub g: f64,
    pub b: f64,
}

impl Color {
    pub const fn new(r: f64, g: f64, b: f64) -> Self {
        Self { r, g, b }
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum PriceScale {
    Left,
    Right,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum PriceScaleMode {
    Normal,
    Logarithmic,
    Percentage,
    IndexedTo100,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum TimeLabelMode {
    Auto,
    Time,
    Date,
    DateTime,
    Custom,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum TooltipPosition {
    Auto,
    TopLeft,
    TopRight,
    BottomLeft,
    BottomRight,
    Follow,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum LineStyle {
    Solid,
    Dotted,
    Dashed,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CrosshairMode {
    Normal,
    Magnet,
    MagnetOhlc,
    Hidden,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CrosshairCenter {
    Cross,
    Dot,
    Circle,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum MarkerPosition {
    Above,
    Below,
    InBar,
    AtPriceTop,
    AtPriceBottom,
    AtPriceMiddle,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum MarkerShape {
    ArrowUp,
    ArrowDown,
    Circle,
    Square,
    Diamond,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum MarkerZOrder {
    Top,
    Normal,
    Bottom,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub struct PanelId(pub usize);

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub struct TimeScaleId(pub usize);

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum PanelRole {
    Main,
    Indicator,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum PanelControlAction {
    AddAbove,
    AddBelow,
    ToggleVisible,
    ToggleCollapsed,
    Remove,
}

#[derive(Clone, Copy, Debug)]
pub struct PanelControlHit {
    pub panel: PanelId,
    pub action: PanelControlAction,
    pub rect: Rect,
}

#[derive(Clone, Copy, Debug)]
pub struct PanelResizeHandle {
    pub upper: PanelId,
    pub lower: PanelId,
}

#[derive(Clone, Debug)]
pub struct SeriesMarkersOptions {
    pub auto_scale: bool,
    pub z_order: MarkerZOrder,
    pub default_icon_text: Option<String>,
    pub default_icon_text_color: Option<Color>,
    pub default_icon_font_size: f64,
    pub default_icon_background: Option<Color>,
    pub default_icon_padding: f64,
    pub default_icon_border_color: Option<Color>,
    pub default_icon_border_width: f64,
}

impl Default for SeriesMarkersOptions {
    fn default() -> Self {
        Self {
            auto_scale: true,
            z_order: MarkerZOrder::Normal,
            default_icon_text: None,
            default_icon_text_color: None,
            default_icon_font_size: 0.0,
            default_icon_background: None,
            default_icon_padding: 2.0,
            default_icon_border_color: None,
            default_icon_border_width: 0.0,
        }
    }
}

#[derive(Clone, Copy, Debug)]
pub struct ScaleMargins {
    pub top: f64,
    pub bottom: f64,
}

impl Default for ScaleMargins {
    fn default() -> Self {
        Self {
            top: 0.1,
            bottom: 0.1,
        }
    }
}

#[derive(Clone, Debug)]
pub enum PriceFormat {
    Price { precision: usize, min_move: f64 },
    Percent { precision: usize },
    Volume { precision: usize },
}

impl Default for PriceFormat {
    fn default() -> Self {
        Self::Price {
            precision: 2,
            min_move: 0.01,
        }
    }
}

#[derive(Clone, Debug)]
pub struct PriceLineOptions {
    pub price: f64,
    pub color: Color,
    pub line_width: f64,
    pub line_style: LineStyle,
    pub line_opacity: f64,
    pub line_visible: bool,
    pub axis_label_visible: bool,
    pub axis_label_color: Option<Color>,
    pub axis_label_text_color: Option<Color>,
    pub axis_label_background_alpha: f64,
    pub axis_label_padding: f64,
    pub axis_label_radius: f64,
    pub axis_label_border_color: Option<Color>,
    pub axis_label_border_width: f64,
    pub title: Option<String>,
}

impl Default for PriceLineOptions {
    fn default() -> Self {
        Self {
            price: 0.0,
            color: Color::new(0.2, 0.5, 0.9),
            line_width: 1.0,
            line_style: LineStyle::Solid,
            line_opacity: 0.6,
            line_visible: true,
            axis_label_visible: true,
            axis_label_color: None,
            axis_label_text_color: None,
            axis_label_background_alpha: 0.85,
            axis_label_padding: 5.0,
            axis_label_radius: 3.0,
            axis_label_border_color: None,
            axis_label_border_width: 0.0,
            title: None,
        }
    }
}

#[derive(Clone, Debug)]
pub struct Marker {
    pub time: OffsetDateTime,
    pub position: MarkerPosition,
    pub price: Option<f64>,
    pub shape: MarkerShape,
    pub color: Color,
    pub size: f64,
    pub icon_text: Option<String>,
    pub icon_text_color: Option<Color>,
    pub icon_font_size: f64,
    pub icon_background: Option<Color>,
    pub icon_padding: f64,
    pub icon_border_color: Option<Color>,
    pub icon_border_width: f64,
    pub text: Option<String>,
    pub label_background: Option<Color>,
    pub label_text: Option<Color>,
    pub label_padding: f64,
    pub label_radius: f64,
    pub label_background_alpha: f64,
    pub label_border_color: Option<Color>,
    pub label_border_width: f64,
    pub label_text_size: f64,
    pub label_offset_x: f64,
    pub label_offset_y: f64,
}

#[derive(Debug)]
pub enum BarConversionError {
    NonFinite,
}

impl TryFrom<&Bar> for Candle {
    type Error = BarConversionError;

    fn try_from(bar: &Bar) -> Result<Self, Self::Error> {
        let open = bar.open.to_f64();
        let high = bar.high.to_f64();
        let low = bar.low.to_f64();
        let close = bar.close.to_f64();

        match (open, high, low, close) {
            (Some(open), Some(high), Some(low), Some(close)) => Ok(Candle {
                time: bar.time,
                open,
                high,
                low,
                close,
            }),
            _ => Err(BarConversionError::NonFinite),
        }
    }
}

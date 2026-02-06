use rust_decimal::prelude::ToPrimitive;
use rust_decimal::Decimal;
use time::OffsetDateTime;

/// Represents a single candlestick (OHLC) data point.
///
/// Candlesticks are the standard way to represent price movement
/// in financial charts, showing the open, high, low, and close prices
/// for a specific time period.
#[derive(Clone, Debug)]
pub struct Candle {
    /// The timestamp for this candlestick
    pub time: OffsetDateTime,
    /// The opening price
    pub open: f64,
    /// The highest price during the period
    pub high: f64,
    /// The lowest price during the period
    pub low: f64,
    /// The closing price
    pub close: f64,
}

/// Represents a single point in a line series.
///
/// Line points are used for line charts, indicators, and other
/// continuous data series where each point has a time and value.
#[derive(Clone, Debug)]
pub struct LinePoint {
    /// The timestamp for this point
    pub time: OffsetDateTime,
    /// The value at this point
    pub value: f64,
}

/// Represents a single bar in a histogram series.
///
/// Histogram points are used for volume bars, distribution charts,
/// and other vertical bar chart data where each bar can have
/// an optional custom color.
#[derive(Clone, Debug)]
pub struct HistogramPoint {
    /// The timestamp for this bar
    pub time: OffsetDateTime,
    /// The height/value of this bar
    pub value: f64,
    /// Optional custom color for this bar (uses series default if None)
    pub color: Option<Color>,
}

/// Result of a pan operation, indicating what was affected.
#[derive(Clone, Copy, Debug)]
pub struct PanResult {
    /// Which price scale was zoomed, if any
    pub price_axis_zoomed: Option<PriceScale>,
    /// Whether the time axis was panned
    pub time_panned: bool,
}

/// A rectangular area defined by position and dimensions.
#[derive(Clone, Copy, Debug)]
pub struct Rect {
    /// The x coordinate of the top-left corner
    pub x: f64,
    /// The y coordinate of the top-left corner
    pub y: f64,
    /// The width of the rectangle
    pub width: f64,
    /// The height of the rectangle
    pub height: f64,
}

impl Rect {
    pub fn contains(&self, x: f64, y: f64) -> bool {
        x >= self.x && x <= self.x + self.width && y >= self.y && y <= self.y + self.height
    }
}

/// Represents a single bar data point using Decimal precision.
///
/// Bars are similar to candlesticks but use Decimal for higher precision
/// financial calculations. They can be converted to Candle structs.
#[derive(Clone, Debug)]
pub struct Bar {
    /// The timestamp for this bar
    pub time: OffsetDateTime,
    /// The opening price (Decimal precision)
    pub open: Decimal,
    /// The highest price during the period (Decimal precision)
    pub high: Decimal,
    /// The lowest price during the period (Decimal precision)
    pub low: Decimal,
    /// The closing price (Decimal precision)
    pub close: Decimal,
}

/// Represents an RGB color with floating point components.
///
/// Color components are typically in the range 0.0 to 1.0,
/// where 0.0 is no intensity and 1.0 is full intensity.
#[derive(Clone, Copy, Debug)]
pub struct Color {
    /// Red component (0.0 to 1.0)
    pub r: f64,
    /// Green component (0.0 to 1.0)
    pub g: f64,
    /// Blue component (0.0 to 1.0)
    pub b: f64,
}

impl Color {
    /// Creates a new color from RGB components.
    ///
    /// # Arguments
    ///
    /// * `r` - Red component (0.0 to 1.0)
    /// * `g` - Green component (0.0 to 1.0)
    /// * `b` - Blue component (0.0 to 1.0)
    ///
    /// # Examples
    ///
    /// ```rust
    /// use lightweight_charts_rs::Color;
    ///
    /// let red = Color::new(1.0, 0.0, 0.0);
    /// let blue = Color::new(0.0, 0.0, 1.0);
    /// let gray = Color::new(0.5, 0.5, 0.5);
    /// ```
    pub const fn new(r: f64, g: f64, b: f64) -> Self {
        Self { r, g, b }
    }
}

/// Represents which price scale a series should use.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum PriceScale {
    /// Use the left price scale
    Left,
    /// Use the right price scale
    Right,
}

/// Represents the display mode for price scales.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum PriceScaleMode {
    /// Normal linear price scale
    Normal,
    /// Logarithmic price scale
    Logarithmic,
    /// Percentage change scale
    Percentage,
    /// Indexed to 100 scale
    IndexedTo100,
}

/// Represents how time labels are displayed on the time axis.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum TimeLabelMode {
    /// Automatically choose the best format
    Auto,
    /// Show only time
    Time,
    /// Show only date
    Date,
    /// Show both date and time
    DateTime,
    /// Use custom format string
    Custom,
}

/// Represents where tooltips are positioned on the chart.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum TooltipPosition {
    /// Automatically choose the best position
    Auto,
    /// Top left corner
    TopLeft,
    /// Top right corner
    TopRight,
    /// Bottom left corner
    BottomLeft,
    /// Bottom right corner
    BottomRight,
    /// Follow the mouse cursor
    Follow,
}

/// Represents the style of lines drawn on the chart.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum LineStyle {
    /// Solid line
    Solid,
    /// Dotted line
    Dotted,
    /// Dashed line
    Dashed,
}

/// Represents how the crosshair behaves on the chart.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CrosshairMode {
    /// Normal crosshair that follows the mouse
    Normal,
    /// Magnetic crosshair that snaps to nearest data point
    Magnet,
    /// Magnetic crosshair that snaps to OHLC data
    MagnetOhlc,
    /// Hidden crosshair
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

/// Unique identifier for a chart panel.
///
/// Panels are used to organize different series and indicators
/// on the same chart with separate price scales.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub struct PanelId(pub usize);

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub struct TimeScaleId(pub usize);

/// The role of a panel in the chart layout.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum PanelRole {
    /// The main price panel
    Main,
    /// An indicator panel (RSI, MACD, etc.)
    Indicator,
}

/// Actions that can be performed on panels via controls.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum PanelControlAction {
    /// Add a new panel above this one
    AddAbove,
    /// Add a new panel below this one
    AddBelow,
    /// Toggle the visibility of this panel
    ToggleVisible,
    /// Toggle the collapsed state of this panel
    ToggleCollapsed,
    /// Remove this panel from the chart
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

/// Defines the margins for price scales as percentages.
///
/// Margins control the padding above and below the data
/// range on price scales.
#[derive(Clone, Copy, Debug)]
pub struct ScaleMargins {
    /// Top margin as a percentage (0.0 to 1.0)
    pub top: f64,
    /// Bottom margin as a percentage (0.0 to 1.0)
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

/// Defines how prices are formatted and displayed on the chart.
#[derive(Clone, Debug)]
pub enum PriceFormat {
    /// Standard price format
    Price {
        /// Number of decimal places to display
        precision: usize,
        /// Minimum price movement increment
        min_move: f64,
    },
    /// Percentage format
    Percent {
        /// Number of decimal places to display
        precision: usize,
    },
    /// Volume format
    Volume {
        /// Number of decimal places to display
        precision: usize,
    },
}

impl Default for PriceFormat {
    fn default() -> Self {
        Self::Price {
            precision: 2,
            min_move: 0.01,
        }
    }
}

/// Configuration options for creating price lines on series.
///
/// Price lines are horizontal lines that mark specific price levels
/// with optional labels and styling.
#[derive(Clone, Debug)]
pub struct PriceLineOptions {
    /// The price level where the line should be drawn
    pub price: f64,
    /// The color of the line
    pub color: Color,
    /// The width of the line in pixels
    pub line_width: f64,
    /// The style of the line (solid, dotted, dashed)
    pub line_style: LineStyle,
    /// The opacity of the line (0.0 to 1.0)
    pub line_opacity: f64,
    /// Whether the line should be visible
    pub line_visible: bool,
    /// Whether the axis label should be visible
    pub axis_label_visible: bool,
    /// Optional background color for the axis label
    pub axis_label_color: Option<Color>,
    /// Optional text color for the axis label
    pub axis_label_text_color: Option<Color>,
    /// Background alpha for the axis label (0.0 to 1.0)
    pub axis_label_background_alpha: f64,
    /// Padding around the axis label in pixels
    pub axis_label_padding: f64,
    /// Corner radius for the axis label in pixels
    pub axis_label_radius: f64,
    /// Optional border color for the axis label
    pub axis_label_border_color: Option<Color>,
    /// Border width for the axis label in pixels
    pub axis_label_border_width: f64,
    /// Optional title text for the price line
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

/// A marker that can be placed on a series to highlight specific points.
///
/// Markers are used to draw attention to specific data points, events,
/// or price levels with customizable shapes, colors, and labels.
#[derive(Clone, Debug)]
pub struct Marker {
    /// The timestamp where the marker should be placed
    pub time: OffsetDateTime,
    /// The position relative to the price data
    pub position: MarkerPosition,
    /// Optional specific price for the marker (uses series value if None)
    pub price: Option<f64>,
    /// The shape of the marker
    pub shape: MarkerShape,
    /// The color of the marker
    pub color: Color,
    /// The size of the marker in pixels
    pub size: f64,
    /// Optional icon text to display inside the marker
    pub icon_text: Option<String>,
    /// Optional color for the icon text
    pub icon_text_color: Option<Color>,
    /// Font size for the icon text
    pub icon_font_size: f64,
    /// Optional background color for the icon
    pub icon_background: Option<Color>,
    /// Padding around the icon text
    pub icon_padding: f64,
    /// Optional border color for the icon
    pub icon_border_color: Option<Color>,
    /// Border width for the icon in pixels
    pub icon_border_width: f64,
    /// Optional label text to display near the marker
    pub text: Option<String>,
    /// Optional background color for the label
    pub label_background: Option<Color>,
    /// Optional text color for the label
    pub label_text: Option<Color>,
    /// Padding around the label text
    pub label_padding: f64,
    /// Corner radius for the label in pixels
    pub label_radius: f64,
    /// Background alpha for the label (0.0 to 1.0)
    pub label_background_alpha: f64,
    /// Optional border color for the label
    pub label_border_color: Option<Color>,
    /// Border width for the label in pixels
    pub label_border_width: f64,
    /// Text size for the label
    pub label_text_size: f64,
    /// Horizontal offset for the label in pixels
    pub label_offset_x: f64,
    /// Vertical offset for the label in pixels
    pub label_offset_y: f64,
}

/// Errors that can occur when converting Bar data to Candle data.
#[derive(Debug)]
pub enum BarConversionError {
    /// The conversion failed due to non-finite values (NaN or infinity)
    NonFinite,
}

impl TryFrom<&Bar> for Candle {
    type Error = BarConversionError;

    /// Converts a Bar to a Candle, converting Decimal values to f64.
    ///
    /// # Errors
    ///
    /// Returns `BarConversionError::NonFinite` if any of the bar values
    /// cannot be converted to finite f64 values.
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

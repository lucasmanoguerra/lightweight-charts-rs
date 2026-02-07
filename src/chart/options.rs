use super::types::{
    Color, CrosshairCenter, CrosshairMode, LineStyle, PriceScaleMode, ScaleMargins, TimeLabelMode,
    TooltipPosition,
};

/// Configuration options for chart tooltips.
///
/// Tooltips display information about data points when the user
/// hovers over the chart.
#[derive(Clone, Debug)]
pub struct TooltipOptions {
    /// Whether tooltips are enabled
    pub enabled: bool,
    /// Where tooltips are positioned on the chart
    pub position: TooltipPosition,
    /// Background color of tooltips
    pub background: Color,
    /// Text color of tooltips
    pub text: Color,
    /// Format string for tooltip content (supports placeholders like {time}, {open}, {high}, {low}, {close})
    pub format: String,
}

impl Default for TooltipOptions {
    fn default() -> Self {
        Self {
            enabled: true,
            position: TooltipPosition::Auto,
            background: Color::new(0.1, 0.12, 0.14),
            text: Color::new(0.9, 0.92, 0.95),
            format: "{time}  O:{open} H:{high} L:{low} C:{close}".to_string(),
        }
    }
}

#[derive(Clone, Copy, Debug)]
pub struct CrosshairOptions {
    pub mode: CrosshairMode,
    pub show_vertical: bool,
    pub show_horizontal: bool,
    pub line_style: LineStyle,
    pub line_width: f64,
    pub center: CrosshairCenter,
    pub center_size: f64,
    pub center_color: Color,
    pub snap_to_ohlc: bool,
    pub snap_to_series: bool,
    pub do_not_snap_to_hidden_series_indices: bool,
}

impl Default for CrosshairOptions {
    fn default() -> Self {
        let color = Color::new(0.85, 0.85, 0.88);
        Self {
            mode: CrosshairMode::Normal,
            show_vertical: true,
            show_horizontal: true,
            line_style: LineStyle::Solid,
            line_width: 1.0,
            center: CrosshairCenter::Cross,
            center_size: 5.0,
            center_color: color,
            snap_to_ohlc: true,
            snap_to_series: true,
            do_not_snap_to_hidden_series_indices: false,
        }
    }
}

#[derive(Clone, Debug)]
pub struct TimeScaleOptions {
    pub bar_spacing: f64,
    pub min_bar_spacing: f64,
    pub max_bar_spacing: f64,
    pub fix_left_edge: bool,
    pub fix_right_edge: bool,
    pub visible: bool,
    pub border_visible: bool,
    pub border_color: Color,
    pub ticks_visible: bool,
    pub time_visible: bool,
    pub seconds_visible: bool,
    pub tick_mark_format: String,
    pub tick_mark_max_character_length: usize,
    pub uniform_distribution: bool,
    pub minimum_height: f64,
    pub right_offset: f64,
    pub right_offset_pixels: f64,
    pub lock_visible_time_range_on_resize: bool,
    pub right_bar_stays_on_scroll: bool,
    pub shift_visible_range_on_new_bar: bool,
    pub allow_shift_visible_range_on_whitespace_replacement: bool,
}

impl Default for TimeScaleOptions {
    fn default() -> Self {
        Self {
            bar_spacing: 6.0,
            min_bar_spacing: 0.5,
            max_bar_spacing: 0.0,
            fix_left_edge: false,
            fix_right_edge: false,
            visible: true,
            border_visible: true,
            border_color: Color::new(0.16, 0.19, 0.24),
            ticks_visible: false,
            time_visible: false,
            seconds_visible: true,
            tick_mark_format: String::new(),
            tick_mark_max_character_length: 0,
            uniform_distribution: false,
            minimum_height: 0.0,
            right_offset: 0.0,
            right_offset_pixels: 0.0,
            lock_visible_time_range_on_resize: false,
            right_bar_stays_on_scroll: false,
            shift_visible_range_on_new_bar: true,
            allow_shift_visible_range_on_whitespace_replacement: false,
        }
    }
}

#[derive(Clone, Copy, Debug)]
pub struct HandleScaleOptions {
    pub mouse_wheel: bool,
    pub pinch: bool,
    pub axis_pressed_mouse_move_time: bool,
    pub axis_pressed_mouse_move_price: bool,
    pub axis_double_click_reset_time: bool,
    pub axis_double_click_reset_price: bool,
}

impl Default for HandleScaleOptions {
    fn default() -> Self {
        Self {
            mouse_wheel: true,
            pinch: true,
            axis_pressed_mouse_move_time: true,
            axis_pressed_mouse_move_price: true,
            axis_double_click_reset_time: true,
            axis_double_click_reset_price: true,
        }
    }
}

#[derive(Clone, Copy, Debug)]
pub struct HandleScrollOptions {
    pub mouse_wheel: bool,
    pub pressed_mouse_move: bool,
    pub horz_touch_drag: bool,
    pub vert_touch_drag: bool,
}

impl Default for HandleScrollOptions {
    fn default() -> Self {
        Self {
            mouse_wheel: true,
            pressed_mouse_move: true,
            horz_touch_drag: true,
            vert_touch_drag: true,
        }
    }
}

#[derive(Clone, Copy, Debug)]
pub struct KineticScrollOptions {
    pub touch: bool,
    pub mouse: bool,
}

impl Default for KineticScrollOptions {
    fn default() -> Self {
        Self {
            touch: true,
            mouse: false,
        }
    }
}

#[derive(Clone, Copy, Debug)]
pub enum TrackingModeExitMode {
    OnTouchEnd,
    OnNextTap,
}

#[derive(Clone, Copy, Debug)]
pub struct TrackingModeOptions {
    pub enabled: bool,
    pub exit_mode: TrackingModeExitMode,
}

impl Default for TrackingModeOptions {
    fn default() -> Self {
        Self {
            enabled: true,
            exit_mode: TrackingModeExitMode::OnNextTap,
        }
    }
}

/// Configuration options for interaction sensitivity.
///
/// These settings control how sensitive the chart is to various
/// user interactions like dragging and zooming.
#[derive(Clone, Copy, Debug)]
pub struct InteractionSensitivityOptions {
    /// Sensitivity for time axis dragging
    pub axis_drag_time: f64,
    /// Sensitivity for price axis dragging
    pub axis_drag_price: f64,
    /// Sensitivity for mouse wheel zooming
    pub wheel_zoom: f64,
    /// Sensitivity for pinch zooming
    pub pinch_zoom: f64,
}

impl Default for InteractionSensitivityOptions {
    fn default() -> Self {
        Self {
            axis_drag_time: 0.0025,
            axis_drag_price: 0.0025,
            wheel_zoom: 0.08,
            pinch_zoom: 0.08,
        }
    }
}

#[derive(Clone, Copy, Debug)]
pub struct PriceScaleOptions {
    pub visible: bool,
    pub auto_scale: bool,
    pub mode: PriceScaleMode,
    pub invert_scale: bool,
    pub align_labels: bool,
    pub scale_margins: ScaleMargins,
    pub border_visible: bool,
    pub border_color: Color,
    pub text_color: Color,
    pub ticks_visible: bool,
    pub minimum_width: f64,
    pub entire_text_only: bool,
    pub ensure_edge_tick_marks_visible: bool,
}

impl Default for PriceScaleOptions {
    fn default() -> Self {
        Self {
            visible: true,
            auto_scale: true,
            mode: PriceScaleMode::Normal,
            invert_scale: false,
            align_labels: true,
            scale_margins: ScaleMargins::default(),
            border_visible: true,
            border_color: Color::new(0.16, 0.19, 0.24),
            text_color: Color::new(0.78, 0.8, 0.83),
            ticks_visible: false,
            minimum_width: 0.0,
            entire_text_only: false,
            ensure_edge_tick_marks_visible: false,
        }
    }
}

/// Defines the visual style and colors used throughout the chart.
///
/// This structure contains all the color definitions and sizing
/// parameters that control the appearance of the chart.
#[derive(Clone, Copy, Debug)]
pub struct ChartStyle {
    /// Background color of the chart
    pub background: Color,
    /// Color of the grid lines
    pub grid: Color,
    /// Color for upward (bullish) candlesticks
    pub up: Color,
    /// Color for downward (bearish) candlesticks
    pub down: Color,
    /// Border color for upward candlesticks
    pub border_up: Color,
    /// Border color for downward candlesticks
    pub border_down: Color,
    /// Wick color for upward candlesticks
    pub wick_up: Color,
    /// Wick color for downward candlesticks
    pub wick_down: Color,
    /// Default color for line series
    pub line: Color,
    /// Default color for histogram series
    pub histogram: Color,
    /// Color for axis text and labels
    pub axis_text: Color,
    /// Color for the crosshair
    pub crosshair: Color,
    /// Padding around the chart in pixels
    pub padding: f64,
    /// Height of the time axis in pixels
    pub axis_height: f64,
    /// Font size for axis text in pixels
    pub axis_font_size: f64,
    /// Width of the price axis in pixels
    pub price_axis_width: f64,
    /// Height ratio for histogram panels (0.0 to 1.0)
    pub histogram_height_ratio: f64,
    /// Height ratio for RSI panels (0.0 to 1.0)
    pub rsi_height_ratio: f64,
    /// Height of panel toolbars in pixels
    pub panel_toolbar_height: f64,
    /// Size of panel toolbar icons in pixels
    pub panel_toolbar_icon_size: f64,
}

impl Default for ChartStyle {
    fn default() -> Self {
        let border = Color::new(0.31, 0.33, 0.36);
        let wick = Color::new(0.82, 0.84, 0.87);
        Self {
            background: Color::new(0.07, 0.09, 0.12),
            grid: Color::new(0.16, 0.19, 0.24),
            up: Color::new(0.25, 0.78, 0.54),
            down: Color::new(0.92, 0.35, 0.32),
            border_up: border,
            border_down: border,
            wick_up: wick,
            wick_down: wick,
            line: Color::new(0.33, 0.62, 0.98),
            histogram: Color::new(0.93, 0.76, 0.25),
            axis_text: Color::new(0.78, 0.8, 0.83),
            crosshair: Color::new(0.85, 0.85, 0.88),
            padding: 28.0,
            axis_height: 24.0,
            axis_font_size: 12.0,
            price_axis_width: 74.0,
            histogram_height_ratio: 0.2,
            rsi_height_ratio: 0.25,
            panel_toolbar_height: 28.0,
            panel_toolbar_icon_size: 12.0,
        }
    }
}

#[derive(Clone, Debug)]
pub(crate) struct ChartOptions {
    pub show_grid: bool,
    pub time_label_mode: TimeLabelMode,
    pub time_label_format: String,
    pub time_scale: TimeScaleOptions,
    pub left_price_scale: PriceScaleOptions,
    pub right_price_scale: PriceScaleOptions,
    pub handle_scroll: HandleScrollOptions,
    pub handle_scale: HandleScaleOptions,
    pub kinetic_scroll: KineticScrollOptions,
    pub tracking_mode: TrackingModeOptions,
    pub interaction_sensitivity: InteractionSensitivityOptions,
    pub tooltip: TooltipOptions,
    pub tooltip_line_format: String,
    pub tooltip_histogram_format: String,
    pub crosshair: CrosshairOptions,
    pub main_symbol: String,
    pub main_timeframe: String,
}

impl Default for ChartOptions {
    fn default() -> Self {
        Self {
            show_grid: true,
            time_label_mode: TimeLabelMode::Auto,
            time_label_format: String::new(),
            time_scale: TimeScaleOptions::default(),
            left_price_scale: PriceScaleOptions {
                visible: false,
                ..PriceScaleOptions::default()
            },
            right_price_scale: PriceScaleOptions::default(),
            handle_scroll: HandleScrollOptions::default(),
            handle_scale: HandleScaleOptions::default(),
            kinetic_scroll: KineticScrollOptions::default(),
            tracking_mode: TrackingModeOptions::default(),
            interaction_sensitivity: InteractionSensitivityOptions::default(),
            tooltip: TooltipOptions::default(),
            tooltip_line_format: "Line {series}: {value}".to_string(),
            tooltip_histogram_format: "Histogram {series}: {value}".to_string(),
            crosshair: CrosshairOptions::default(),
            main_symbol: String::new(),
            main_timeframe: String::new(),
        }
    }
}

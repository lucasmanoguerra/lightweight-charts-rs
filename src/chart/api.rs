use super::core::ChartCore;
use super::options::{
    ChartStyle, HandleScaleOptions, HandleScrollOptions, InteractionSensitivityOptions,
    KineticScrollOptions, PriceScaleOptions, TimeScaleOptions, TrackingModeOptions,
};
use super::types::{
    Bar, BarConversionError, Candle, Color, CrosshairCenter, CrosshairMode, HistogramPoint,
    LinePoint, LineStyle, Marker, PanResult, PanelControlAction, PanelId, PanelResizeHandle,
    PanelRole, PriceFormat, PriceLineOptions, PriceScale, PriceScaleMode, ScaleMargins,
    SeriesMarkersOptions, TimeLabelMode, TooltipPosition,
};
use std::cell::RefCell;
use std::rc::Rc;

#[derive(Clone, Debug)]
pub struct ChartApi {
    inner: Rc<RefCell<ChartCore>>,
}


#[derive(Clone, Debug)]
pub struct CandlestickSeriesApi {
    inner: Rc<RefCell<ChartCore>>,
    id: usize,
}

#[derive(Clone, Debug)]
pub struct LineSeriesApi {
    inner: Rc<RefCell<ChartCore>>,
    id: usize,
}

#[derive(Clone, Debug)]
pub struct HistogramSeriesApi {
    inner: Rc<RefCell<ChartCore>>,
    id: usize,
}

#[derive(Clone, Debug)]
pub struct PriceLineApi {
    inner: Rc<RefCell<ChartCore>>,
    series_id: usize,
    line_id: usize,
}

pub fn create_chart() -> ChartApi {
    ChartApi {
        inner: Rc::new(RefCell::new(ChartCore::new())),
    }
}

impl ChartApi {
    pub fn add_candlestick_series(&self) -> CandlestickSeriesApi {
        let id = self.inner.borrow_mut().add_candlestick_series();
        CandlestickSeriesApi {
            inner: self.inner.clone(),
            id,
        }
    }

    pub fn add_line_series(&self) -> LineSeriesApi {
        let id = self.inner.borrow_mut().add_line_series();
        LineSeriesApi {
            inner: self.inner.clone(),
            id,
        }
    }

    pub fn add_histogram_series(&self) -> HistogramSeriesApi {
        let id = self.inner.borrow_mut().add_histogram_series();
        HistogramSeriesApi {
            inner: self.inner.clone(),
            id,
        }
    }

    pub fn set_rsi_panel(&self, title: String, data: Vec<LinePoint>) {
        self.inner.borrow_mut().set_rsi_panel(title, data);
    }

    pub fn set_rsi_panel_data(&self, data: Vec<LinePoint>) {
        self.inner.borrow_mut().set_rsi_panel_data(data);
    }

    pub fn clear_rsi_panel(&self) {
        self.inner.borrow_mut().clear_rsi_panel();
    }

    pub fn has_rsi_panel(&self) -> bool {
        self.inner.borrow().has_rsi_panel()
    }

    pub fn rsi_color(&self) -> Option<Color> {
        self.inner.borrow().rsi_color()
    }

    pub fn set_rsi_color(&self, color: Color) {
        self.inner.borrow_mut().set_rsi_color(color);
    }

    pub fn set_rsi_auto_scale(&self, enabled: bool) {
        self.inner.borrow_mut().set_rsi_auto_scale(enabled);
    }

    pub fn set_rsi_price_scale_visible(&self, visible: bool) {
        self.inner.borrow_mut().set_rsi_price_scale_visible(visible);
    }

    pub fn rsi_auto_scale(&self) -> Option<bool> {
        self.inner.borrow().rsi_auto_scale()
    }

    pub fn rsi_price_scale_visible(&self) -> Option<bool> {
        self.inner.borrow().rsi_price_scale_visible()
    }

    pub fn draw(&self, cr: &cairo::Context, width: f64, height: f64) {
        self.inner.borrow_mut().draw(cr, width, height);
    }

    pub fn visible_time_range(&self) -> (f64, f64) {
        self.inner.borrow().visible_time_range()
    }

    pub fn style(&self) -> ChartStyle {
        self.inner.borrow().style()
    }

    pub fn tooltip_icon_at(&self, x: f64, y: f64) -> Option<PanelId> {
        self.inner.borrow().tooltip_icon_at(x, y)
    }

    pub fn panel_at(&self, x: f64, y: f64, width: f64, height: f64) -> Option<PanelId> {
        self.inner.borrow().panel_at(x, y, width, height)
    }

    pub fn panel_control_at(
        &self,
        x: f64,
        y: f64,
    ) -> Option<(PanelId, PanelControlAction)> {
        self.inner.borrow().panel_control_at(x, y)
    }

    pub fn panel_resize_handle_at(
        &self,
        y: f64,
        width: f64,
        height: f64,
    ) -> Option<PanelResizeHandle> {
        self.inner.borrow().panel_resize_handle_at(y, width, height)
    }

    pub fn resize_panels_by_pixels(
        &self,
        handle: PanelResizeHandle,
        delta_y: f64,
        width: f64,
        height: f64,
    ) {
        self.inner
            .borrow_mut()
            .resize_panels_by_pixels(handle, delta_y, width, height);
    }

    pub fn panel_role(&self, panel_id: PanelId) -> Option<PanelRole> {
        self.inner.borrow().panel_role(panel_id)
    }

    pub fn panel_title(&self, panel_id: PanelId) -> Option<String> {
        self.inner.borrow().panel_title(panel_id)
    }

    pub fn toggle_panel_visibility(&self, panel_id: PanelId) {
        self.inner.borrow_mut().toggle_panel_visibility(panel_id);
    }

    pub fn toggle_panel_collapsed(&self, panel_id: PanelId) {
        self.inner.borrow_mut().toggle_panel_collapsed(panel_id);
    }

    pub fn remove_panel(&self, panel_id: PanelId) {
        self.inner.borrow_mut().remove_panel(panel_id);
    }

    pub fn panel_line_color(&self, panel_id: PanelId) -> Option<Color> {
        self.inner.borrow().panel_line_color(panel_id)
    }

    pub fn set_panel_line_color(&self, panel_id: PanelId, color: Color) {
        self.inner.borrow_mut().set_panel_line_color(panel_id, color);
    }

    pub fn panel_auto_scale(&self, panel_id: PanelId) -> Option<bool> {
        self.inner.borrow().panel_auto_scale(panel_id)
    }

    pub fn set_panel_auto_scale(&self, panel_id: PanelId, enabled: bool) {
        self.inner.borrow_mut().set_panel_auto_scale(panel_id, enabled);
    }

    pub fn panel_price_scale_visible(&self, panel_id: PanelId) -> Option<bool> {
        self.inner.borrow().panel_price_scale_visible(panel_id)
    }

    pub fn set_panel_price_scale_visible(&self, panel_id: PanelId, visible: bool) {
        self.inner
            .borrow_mut()
            .set_panel_price_scale_visible(panel_id, visible);
    }

    pub fn pan_by_pixels(
        &self,
        dx: f64,
        dy: f64,
        width: f64,
        height: f64,
        x: f64,
        y: f64,
    ) -> PanResult {
        self.inner
            .borrow_mut()
            .pan_by_pixels(dx, dy, width, height, x, y)
    }

    pub fn pan_by_pixels_touch(
        &self,
        dx: f64,
        dy: f64,
        width: f64,
        height: f64,
        x: f64,
        y: f64,
    ) -> bool {
        self.inner
            .borrow_mut()
            .pan_by_pixels_touch(dx, dy, width, height, x, y)
    }

    pub fn zoom_by_delta(
        &self,
        delta: f64,
        x: f64,
        y: f64,
        width: f64,
        height: f64,
    ) -> Option<PriceScale> {
        self.inner
            .borrow_mut()
            .zoom_by_delta(delta, x, y, width, height)
    }

    pub fn zoom_by_delta_pinch(
        &self,
        delta: f64,
        x: f64,
        y: f64,
        width: f64,
        height: f64,
    ) {
        self.inner
            .borrow_mut()
            .zoom_by_delta_pinch(delta, x, y, width, height);
    }

    pub fn handle_double_click(&self, x: f64, y: f64, width: f64, height: f64) {
        self.inner
            .borrow_mut()
            .handle_double_click(x, y, width, height);
    }

    pub fn set_crosshair(&self, x: f64, y: f64) {
        self.inner.borrow_mut().set_crosshair(x, y);
    }

    pub fn clear_crosshair(&self) {
        self.inner.borrow_mut().clear_crosshair();
    }

    pub fn set_candle_colors(
        &self,
        up: Color,
        down: Color,
        border_up: Color,
        border_down: Color,
        wick_up: Color,
        wick_down: Color,
    ) {
        self.inner
            .borrow_mut()
            .set_candle_colors(up, down, border_up, border_down, wick_up, wick_down);
    }

    pub fn set_background_color(&self, color: Color) {
        self.inner.borrow_mut().set_background_color(color);
    }

    pub fn set_grid(&self, enabled: bool, color: Color) {
        self.inner.borrow_mut().set_grid(enabled, color);
    }

    pub fn set_line_color(&self, color: Color) {
        self.inner.borrow_mut().set_line_color(color);
    }

    pub fn set_histogram_color(&self, color: Color) {
        self.inner.borrow_mut().set_histogram_color(color);
    }

    pub fn set_axis_visibility(&self, left: bool, right: bool) {
        self.inner.borrow_mut().set_axis_visibility(left, right);
    }

    pub fn set_time_label_mode(&self, mode: TimeLabelMode) {
        self.inner.borrow_mut().set_time_label_mode(mode);
    }

    pub fn set_time_label_format(&self, format: String) {
        self.inner.borrow_mut().set_time_label_format(format);
    }

    pub fn set_tooltip_format(&self, format: String) {
        self.inner.borrow_mut().set_tooltip_format(format);
    }

    pub fn set_tooltip_colors(&self, background: Color, text: Color) {
        self.inner
            .borrow_mut()
            .set_tooltip_colors(background, text);
    }

    pub fn set_tooltip_position(&self, position: TooltipPosition) {
        self.inner.borrow_mut().set_tooltip_position(position);
    }

    pub fn set_tooltip_enabled(&self, enabled: bool) {
        self.inner.borrow_mut().set_tooltip_enabled(enabled);
    }

    pub fn set_tooltip_line_format(&self, format: String) {
        self.inner.borrow_mut().set_tooltip_line_format(format);
    }

    pub fn set_tooltip_histogram_format(&self, format: String) {
        self.inner
            .borrow_mut()
            .set_tooltip_histogram_format(format);
    }

    pub fn set_crosshair_visibility(&self, vertical: bool, horizontal: bool) {
        self.inner
            .borrow_mut()
            .set_crosshair_visibility(vertical, horizontal);
    }

    pub fn set_crosshair_line_style(&self, style: LineStyle) {
        self.inner.borrow_mut().set_crosshair_line_style(style);
    }

    pub fn set_crosshair_line_width(&self, width: f64) {
        self.inner.borrow_mut().set_crosshair_line_width(width);
    }

    pub fn set_crosshair_center(&self, center: CrosshairCenter) {
        self.inner.borrow_mut().set_crosshair_center(center);
    }

    pub fn set_crosshair_color(&self, color: Color) {
        self.inner.borrow_mut().set_crosshair_color(color);
    }

    pub fn set_crosshair_center_size(&self, size: f64) {
        self.inner.borrow_mut().set_crosshair_center_size(size);
    }

    pub fn set_crosshair_center_color(&self, color: Color) {
        self.inner.borrow_mut().set_crosshair_center_color(color);
    }

    pub fn reset_autoscale(&self, side: PriceScale) {
        self.inner.borrow_mut().reset_autoscale(side);
    }

    pub fn fit_content(&self) {
        self.inner.borrow_mut().fit_content();
    }

    pub fn set_time_scale_right_offset(&self, offset: f64) {
        self.inner.borrow_mut().set_time_scale_right_offset(offset);
    }

    pub fn set_time_scale_bar_spacing(&self, spacing: f64) {
        self.inner.borrow_mut().set_time_scale_bar_spacing(spacing);
    }

    pub fn set_crosshair_snap_to_ohlc(&self, enabled: bool) {
        self.inner.borrow_mut().set_crosshair_snap_to_ohlc(enabled);
    }

    pub fn set_crosshair_snap_to_series(&self, enabled: bool) {
        self.inner.borrow_mut().set_crosshair_snap_to_series(enabled);
    }

    pub fn set_time_scale_min_bar_spacing(&self, value: f64) {
        self.inner.borrow_mut().set_time_scale_min_bar_spacing(value);
    }

    pub fn set_time_scale_max_bar_spacing(&self, value: f64) {
        self.inner.borrow_mut().set_time_scale_max_bar_spacing(value);
    }

    pub fn set_time_scale_fix_left_edge(&self, enabled: bool) {
        self.inner.borrow_mut().set_time_scale_fix_left_edge(enabled);
    }

    pub fn set_time_scale_fix_right_edge(&self, enabled: bool) {
        self.inner.borrow_mut().set_time_scale_fix_right_edge(enabled);
    }

    pub fn set_time_scale_right_offset_pixels(&self, pixels: f64) {
        self.inner.borrow_mut().set_time_scale_right_offset_pixels(pixels);
    }

    pub fn set_time_scale_visible(&self, visible: bool) {
        self.inner.borrow_mut().set_time_scale_visible(visible);
    }

    pub fn set_time_scale_border(&self, visible: bool, color: Color) {
        self.inner.borrow_mut().set_time_scale_border(visible, color);
    }

    pub fn set_time_scale_ticks_visible(&self, visible: bool) {
        self.inner.borrow_mut().set_time_scale_ticks_visible(visible);
    }

    pub fn set_time_scale_time_visible(&self, visible: bool) {
        self.inner.borrow_mut().set_time_scale_time_visible(visible);
    }

    pub fn set_time_scale_seconds_visible(&self, visible: bool) {
        self.inner.borrow_mut().set_time_scale_seconds_visible(visible);
    }

    pub fn set_time_scale_tick_mark_format(&self, format: String) {
        self.inner.borrow_mut().set_time_scale_tick_mark_format(format);
    }

    pub fn set_time_scale_tick_mark_max_len(&self, len: usize) {
        self.inner.borrow_mut().set_time_scale_tick_mark_max_len(len);
    }

    pub fn set_time_scale_lock_visible_time_range_on_resize(&self, enabled: bool) {
        self.inner
            .borrow_mut()
            .set_time_scale_lock_visible_time_range_on_resize(enabled);
    }

    pub fn set_time_scale_right_bar_stays_on_scroll(&self, enabled: bool) {
        self.inner
            .borrow_mut()
            .set_time_scale_right_bar_stays_on_scroll(enabled);
    }

    pub fn set_time_scale_shift_visible_range_on_new_bar(&self, enabled: bool) {
        self.inner
            .borrow_mut()
            .set_time_scale_shift_visible_range_on_new_bar(enabled);
    }

    pub fn set_time_scale_minimum_height(&self, height: f64) {
        self.inner.borrow_mut().set_time_scale_minimum_height(height);
    }

    pub fn set_time_scale_uniform_distribution(&self, enabled: bool) {
        self.inner
            .borrow_mut()
            .set_time_scale_uniform_distribution(enabled);
    }

    pub fn apply_time_scale_options(&self, options: TimeScaleOptions) {
        self.inner.borrow_mut().apply_time_scale_options(options);
    }

    pub fn apply_handle_scale_options(&self, options: HandleScaleOptions) {
        self.inner.borrow_mut().set_handle_scale_options(options);
    }

    pub fn apply_handle_scroll_options(&self, options: HandleScrollOptions) {
        self.inner.borrow_mut().set_handle_scroll_options(options);
    }

    pub fn apply_kinetic_scroll_options(&self, options: KineticScrollOptions) {
        self.inner.borrow_mut().set_kinetic_scroll_options(options);
    }

    pub fn apply_tracking_mode_options(&self, options: TrackingModeOptions) {
        self.inner.borrow_mut().set_tracking_mode_options(options);
    }

    pub fn apply_interaction_sensitivity(&self, options: InteractionSensitivityOptions) {
        self.inner.borrow_mut().set_interaction_sensitivity(options);
    }

    pub fn kinetic_scroll_options(&self) -> KineticScrollOptions {
        self.inner.borrow().options.kinetic_scroll
    }

    pub fn tracking_mode_options(&self) -> TrackingModeOptions {
        self.inner.borrow().options.tracking_mode
    }

    pub fn set_tracking_mode_active(&self, active: bool) {
        self.inner.borrow_mut().set_tracking_mode_active(active);
    }

    pub fn tracking_mode_active(&self) -> bool {
        self.inner.borrow().tracking_mode_active()
    }

    pub fn interaction_sensitivity(&self) -> InteractionSensitivityOptions {
        self.inner.borrow().options.interaction_sensitivity
    }

    pub fn set_main_header(&self, symbol: String, timeframe: String) {
        self.inner.borrow_mut().set_main_header(symbol, timeframe);
    }

    pub fn set_price_scale_options(&self, side: PriceScale, options: PriceScaleOptions) {
        self.inner.borrow_mut().set_price_scale_options(side, options);
    }

    pub fn set_price_scale_mode(&self, side: PriceScale, mode: PriceScaleMode) {
        self.inner.borrow_mut().set_price_scale_mode(side, mode);
    }

    pub fn set_price_scale_auto_scale(&self, side: PriceScale, enabled: bool) {
        self.inner.borrow_mut().set_price_scale_auto_scale(side, enabled);
    }

    pub fn set_price_scale_visible(&self, side: PriceScale, visible: bool) {
        self.inner.borrow_mut().set_price_scale_visible(side, visible);
    }

    pub fn set_price_scale_margins(&self, side: PriceScale, margins: ScaleMargins) {
        self.inner.borrow_mut().set_price_scale_margins(side, margins);
    }

    pub fn set_price_scale_border(&self, side: PriceScale, visible: bool, color: Color) {
        self.inner
            .borrow_mut()
            .set_price_scale_border(side, visible, color);
    }

    pub fn set_price_scale_text_color(&self, side: PriceScale, color: Color) {
        self.inner.borrow_mut().set_price_scale_text_color(side, color);
    }

    pub fn set_price_scale_ticks_visible(&self, side: PriceScale, visible: bool) {
        self.inner.borrow_mut().set_price_scale_ticks_visible(side, visible);
    }

    pub fn set_price_scale_minimum_width(&self, side: PriceScale, width: f64) {
        self.inner.borrow_mut().set_price_scale_minimum_width(side, width);
    }

    pub fn set_price_scale_invert(&self, side: PriceScale, invert: bool) {
        self.inner.borrow_mut().set_price_scale_invert(side, invert);
    }

    pub fn set_price_scale_align_labels(&self, side: PriceScale, align: bool) {
        self.inner.borrow_mut().set_price_scale_align_labels(side, align);
    }

    pub fn set_price_scale_entire_text_only(&self, side: PriceScale, enabled: bool) {
        self.inner
            .borrow_mut()
            .set_price_scale_entire_text_only(side, enabled);
    }

    pub fn set_price_scale_ensure_edge_ticks(&self, side: PriceScale, enabled: bool) {
        self.inner
            .borrow_mut()
            .set_price_scale_ensure_edge_ticks(side, enabled);
    }

    pub fn set_crosshair_mode(&self, mode: CrosshairMode) {
        self.inner.borrow_mut().set_crosshair_mode(mode);
    }
}

impl CandlestickSeriesApi {
    pub fn set_data(&self, candles: Vec<Candle>) {
        self.inner.borrow_mut().set_candles(self.id, candles);
    }

    pub fn set_data_from_bars(&self, bars: Vec<Bar>) -> Result<(), BarConversionError> {
        let mut candles = Vec::with_capacity(bars.len());
        for bar in &bars {
            candles.push(Candle::try_from(bar)?);
        }
        self.set_data(candles);
        Ok(())
    }

    pub fn update(&self, candle: Candle) {
        self.inner.borrow_mut().update_candle(self.id, candle);
    }

    pub fn set_price_scale(&self, scale: PriceScale) {
        self.inner.borrow_mut().set_series_scale(self.id, scale);
    }

    pub fn set_price_line_visible(&self, visible: bool) {
        self.inner.borrow_mut().set_series_price_line(self.id, visible);
    }

    pub fn set_price_line_style(&self, style: LineStyle) {
        self.inner.borrow_mut().set_series_price_line_style(self.id, style);
    }

    pub fn set_price_line_width(&self, width: f64) {
        self.inner.borrow_mut().set_series_price_line_width(self.id, width);
    }

    pub fn set_price_line_color(&self, color: Color) {
        self.inner.borrow_mut().set_series_price_line_color(self.id, color);
    }

    pub fn set_last_value_visible(&self, visible: bool) {
        self.inner.borrow_mut().set_series_last_value(self.id, visible);
    }

    pub fn set_last_value_color(&self, color: Color) {
        self.inner.borrow_mut().set_series_last_value_color(self.id, color);
    }

    pub fn set_last_value_text_color(&self, color: Color) {
        self.inner
            .borrow_mut()
            .set_series_last_value_text_color(self.id, color);
    }

    pub fn set_markers(&self, markers: Vec<Marker>) {
        self.inner.borrow_mut().set_series_markers(self.id, markers);
    }

    pub fn set_markers_options(&self, options: SeriesMarkersOptions) {
        self.inner
            .borrow_mut()
            .set_series_markers_options(self.id, options);
    }

    pub fn set_price_format(&self, format: PriceFormat) {
        self.inner.borrow_mut().set_series_price_format(self.id, format);
    }

    pub fn create_price_line(&self, options: PriceLineOptions) -> PriceLineApi {
        let line_id = self
            .inner
            .borrow_mut()
            .create_price_line(self.id, options);
        PriceLineApi {
            inner: self.inner.clone(),
            series_id: self.id,
            line_id,
        }
    }
}

impl LineSeriesApi {
    pub fn set_data(&self, points: Vec<LinePoint>) {
        self.inner.borrow_mut().set_line_points(self.id, points);
    }

    pub fn update(&self, point: LinePoint) {
        self.inner.borrow_mut().update_line_point(self.id, point);
    }

    pub fn set_price_scale(&self, scale: PriceScale) {
        self.inner.borrow_mut().set_series_scale(self.id, scale);
    }

    pub fn set_price_line_visible(&self, visible: bool) {
        self.inner.borrow_mut().set_series_price_line(self.id, visible);
    }

    pub fn set_price_line_style(&self, style: LineStyle) {
        self.inner.borrow_mut().set_series_price_line_style(self.id, style);
    }

    pub fn set_price_line_width(&self, width: f64) {
        self.inner.borrow_mut().set_series_price_line_width(self.id, width);
    }

    pub fn set_price_line_color(&self, color: Color) {
        self.inner.borrow_mut().set_series_price_line_color(self.id, color);
    }

    pub fn set_markers(&self, markers: Vec<Marker>) {
        self.inner.borrow_mut().set_series_markers(self.id, markers);
    }

    pub fn set_markers_options(&self, options: SeriesMarkersOptions) {
        self.inner
            .borrow_mut()
            .set_series_markers_options(self.id, options);
    }

    pub fn set_price_format(&self, format: PriceFormat) {
        self.inner.borrow_mut().set_series_price_format(self.id, format);
    }

    pub fn set_last_value_visible(&self, visible: bool) {
        self.inner.borrow_mut().set_series_last_value(self.id, visible);
    }

    pub fn set_last_value_color(&self, color: Color) {
        self.inner.borrow_mut().set_series_last_value_color(self.id, color);
    }

    pub fn set_last_value_text_color(&self, color: Color) {
        self.inner
            .borrow_mut()
            .set_series_last_value_text_color(self.id, color);
    }

    pub fn create_price_line(&self, options: PriceLineOptions) -> PriceLineApi {
        let line_id = self
            .inner
            .borrow_mut()
            .create_price_line(self.id, options);
        PriceLineApi {
            inner: self.inner.clone(),
            series_id: self.id,
            line_id,
        }
    }
}

impl HistogramSeriesApi {
    pub fn set_data(&self, points: Vec<HistogramPoint>) {
        self.inner
            .borrow_mut()
            .set_histogram_points(self.id, points);
    }

    pub fn update(&self, point: HistogramPoint) {
        self.inner
            .borrow_mut()
            .update_histogram_point(self.id, point);
    }

    pub fn set_price_scale(&self, scale: PriceScale) {
        self.inner.borrow_mut().set_series_scale(self.id, scale);
    }

    pub fn set_price_line_visible(&self, visible: bool) {
        self.inner.borrow_mut().set_series_price_line(self.id, visible);
    }

    pub fn set_price_line_style(&self, style: LineStyle) {
        self.inner.borrow_mut().set_series_price_line_style(self.id, style);
    }

    pub fn set_price_line_width(&self, width: f64) {
        self.inner.borrow_mut().set_series_price_line_width(self.id, width);
    }

    pub fn set_price_line_color(&self, color: Color) {
        self.inner.borrow_mut().set_series_price_line_color(self.id, color);
    }

    pub fn set_markers(&self, markers: Vec<Marker>) {
        self.inner.borrow_mut().set_series_markers(self.id, markers);
    }

    pub fn set_markers_options(&self, options: SeriesMarkersOptions) {
        self.inner
            .borrow_mut()
            .set_series_markers_options(self.id, options);
    }

    pub fn set_price_format(&self, format: PriceFormat) {
        self.inner.borrow_mut().set_series_price_format(self.id, format);
    }

    pub fn set_last_value_visible(&self, visible: bool) {
        self.inner.borrow_mut().set_series_last_value(self.id, visible);
    }

    pub fn set_last_value_color(&self, color: Color) {
        self.inner.borrow_mut().set_series_last_value_color(self.id, color);
    }

    pub fn set_last_value_text_color(&self, color: Color) {
        self.inner
            .borrow_mut()
            .set_series_last_value_text_color(self.id, color);
    }

    pub fn create_price_line(&self, options: PriceLineOptions) -> PriceLineApi {
        let line_id = self
            .inner
            .borrow_mut()
            .create_price_line(self.id, options);
        PriceLineApi {
            inner: self.inner.clone(),
            series_id: self.id,
            line_id,
        }
    }
}

impl PriceLineApi {
    pub fn apply_options(&self, options: PriceLineOptions) {
        self.inner
            .borrow_mut()
            .update_price_line(self.series_id, self.line_id, options);
    }

    pub fn set_price(&self, price: f64) {
        self.inner
            .borrow_mut()
            .set_price_line_price(self.series_id, self.line_id, price);
    }

    pub fn remove(&self) {
        self.inner
            .borrow_mut()
            .remove_price_line(self.series_id, self.line_id);
    }
}

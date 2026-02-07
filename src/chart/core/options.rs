use super::super::layout::ChartLayout;
use super::super::options::{
    ChartStyle, HandleScaleOptions, HandleScrollOptions, InteractionSensitivityOptions,
    KineticScrollOptions, TrackingModeOptions,
};
use super::super::types::{
    Color, CrosshairCenter, CrosshairMode, LineStyle, PanelControlAction, PanelControlHit, PanelId,
    PanelRole, Rect, TimeLabelMode, TooltipPosition,
};
use super::ChartCore;

impl ChartCore {
    pub(crate) fn set_candle_colors(
        &mut self,
        up: Color,
        down: Color,
        border_up: Color,
        border_down: Color,
        wick_up: Color,
        wick_down: Color,
    ) {
        self.style.up = up;
        self.style.down = down;
        self.style.border_up = border_up;
        self.style.border_down = border_down;
        self.style.wick_up = wick_up;
        self.style.wick_down = wick_down;
    }

    pub(crate) fn set_background_color(&mut self, color: Color) {
        self.style.background = color;
    }

    pub(crate) fn set_grid(&mut self, enabled: bool, color: Color) {
        self.options.show_grid = enabled;
        self.style.grid = color;
    }

    pub(crate) fn set_line_color(&mut self, color: Color) {
        self.style.line = color;
    }

    pub(crate) fn set_histogram_color(&mut self, color: Color) {
        self.style.histogram = color;
    }

    pub(crate) fn set_axis_visibility(&mut self, left: bool, right: bool) {
        self.options.left_price_scale.visible = left;
        self.options.right_price_scale.visible = right;
    }

    pub(crate) fn set_time_label_mode(&mut self, mode: TimeLabelMode) {
        self.options.time_label_mode = mode;
    }

    pub(crate) fn set_time_label_format(&mut self, format: String) {
        self.options.time_label_format = format;
    }

    pub(crate) fn set_tooltip_format(&mut self, format: String) {
        self.options.tooltip.format = format;
    }

    pub(crate) fn set_tooltip_colors(&mut self, background: Color, text: Color) {
        self.options.tooltip.background = background;
        self.options.tooltip.text = text;
    }

    pub(crate) fn set_tooltip_position(&mut self, position: TooltipPosition) {
        self.options.tooltip.position = position;
    }

    pub(crate) fn set_tooltip_enabled(&mut self, enabled: bool) {
        self.options.tooltip.enabled = enabled;
    }

    pub(crate) fn set_tooltip_line_format(&mut self, format: String) {
        self.options.tooltip_line_format = format;
    }

    pub(crate) fn set_tooltip_histogram_format(&mut self, format: String) {
        self.options.tooltip_histogram_format = format;
    }

    pub(crate) fn set_kinetic_scroll_options(&mut self, options: KineticScrollOptions) {
        self.options.kinetic_scroll = options;
    }

    pub(crate) fn set_tracking_mode_options(&mut self, options: TrackingModeOptions) {
        self.options.tracking_mode = options;
    }

    pub(crate) fn set_interaction_sensitivity(&mut self, options: InteractionSensitivityOptions) {
        self.options.interaction_sensitivity = options;
    }

    pub(crate) fn set_tracking_mode_active(&mut self, active: bool) {
        self.tracking_mode_active = active;
    }

    pub(crate) fn tracking_mode_active(&self) -> bool {
        self.tracking_mode_active
    }

    pub(crate) fn set_main_header(&mut self, symbol: String, timeframe: String) {
        self.options.main_symbol = symbol;
        self.options.main_timeframe = timeframe;
    }

    pub(crate) fn set_crosshair_visibility(&mut self, vertical: bool, horizontal: bool) {
        self.options.crosshair.show_vertical = vertical;
        self.options.crosshair.show_horizontal = horizontal;
    }

    pub(crate) fn set_crosshair_line_style(&mut self, style: LineStyle) {
        self.options.crosshair.line_style = style;
    }

    pub(crate) fn set_crosshair_line_width(&mut self, width: f64) {
        self.options.crosshair.line_width = width.max(0.5);
    }

    pub(crate) fn set_crosshair_center(&mut self, center: CrosshairCenter) {
        self.options.crosshair.center = center;
    }

    pub(crate) fn set_crosshair_color(&mut self, color: Color) {
        self.style.crosshair = color;
    }

    pub(crate) fn set_crosshair_center_size(&mut self, size: f64) {
        self.options.crosshair.center_size = size.max(1.0);
    }

    pub(crate) fn set_crosshair_center_color(&mut self, color: Color) {
        self.options.crosshair.center_color = color;
    }

    pub(crate) fn set_crosshair_snap_to_ohlc(&mut self, enabled: bool) {
        self.options.crosshair.snap_to_ohlc = enabled;
    }

    pub(crate) fn set_crosshair_snap_to_series(&mut self, enabled: bool) {
        self.options.crosshair.snap_to_series = enabled;
    }

    pub(crate) fn set_crosshair_do_not_snap_to_hidden_series_indices(&mut self, enabled: bool) {
        self.options.crosshair.do_not_snap_to_hidden_series_indices = enabled;
    }

    pub(crate) fn set_crosshair_mode(&mut self, mode: CrosshairMode) {
        self.options.crosshair.mode = mode;
    }

    pub(crate) fn set_handle_scale_options(&mut self, options: HandleScaleOptions) {
        self.options.handle_scale = options;
    }

    pub(crate) fn set_handle_scroll_options(&mut self, options: HandleScrollOptions) {
        self.options.handle_scroll = options;
    }

    pub(crate) fn visible_time_range(&self) -> (f64, f64) {
        self.time_scales
            .first()
            .map(|group| (group.time_scale.start, group.time_scale.end))
            .unwrap_or((0.0, 0.0))
    }

    pub(crate) fn style(&self) -> ChartStyle {
        self.style
    }

    pub(crate) fn set_tooltip_icon(&self, panel: Option<(PanelId, Rect)>) {
        self.tooltip_icon.set(panel);
    }

    pub(crate) fn tooltip_icon_at(&self, x: f64, y: f64) -> Option<PanelId> {
        self.tooltip_icon.get().and_then(|(panel, rect)| {
            if rect.contains(x, y) {
                Some(panel)
            } else {
                None
            }
        })
    }

    pub(crate) fn set_panel_controls(&self, hits: Vec<PanelControlHit>) {
        *self.panel_controls.borrow_mut() = hits;
    }

    pub(crate) fn panel_control_at(&self, x: f64, y: f64) -> Option<(PanelId, PanelControlAction)> {
        for hit in self.panel_controls.borrow().iter() {
            if hit.rect.contains(x, y) {
                return Some((hit.panel, hit.action));
            }
        }
        None
    }

    pub(crate) fn panel_at(&self, x: f64, y: f64, width: f64, height: f64) -> Option<PanelId> {
        let layout = ChartLayout::new(self, width, height);
        if layout.panels.is_empty() {
            return None;
        }
        layout
            .panels
            .iter()
            .find(|panel| {
                y >= panel.top && y <= panel.bottom && x >= panel.plot_left && x <= panel.plot_right
            })
            .map(|panel| panel.id)
    }

    pub(crate) fn panel_role(&self, panel_id: PanelId) -> Option<PanelRole> {
        self.panels
            .iter()
            .find(|panel| panel.id == panel_id)
            .map(|panel| panel.role)
    }

    pub(crate) fn panel_title(&self, panel_id: PanelId) -> Option<String> {
        self.panels
            .iter()
            .find(|panel| panel.id == panel_id)
            .map(|panel| panel.title.clone())
    }
}

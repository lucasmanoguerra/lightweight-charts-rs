use super::super::data::{
    HasTime, IndicatorPanel, Panel, PanelSeries, PriceLine, PriceScaleState, Series, SeriesData,
    SeriesKind, SeriesOptions,
};
use super::super::types::{
    Candle, Color, HistogramPoint, LinePoint, LineStyle, Marker, PanelId, PanelRole, PriceFormat,
    PriceLineOptions, PriceScale, SeriesMarkersOptions, TimeScaleId,
};
use super::ChartCore;

impl ChartCore {
    pub(crate) fn add_candlestick_series(&mut self) -> usize {
        let id = self.series.len();
        let panel_id = self.main_panel_id();
        self.series.push(Series {
            kind: SeriesKind::Candlestick,
            scale: PriceScale::Right,
            panel_id,
            data: SeriesData::Candlestick { data: Vec::new() },
            options: SeriesOptions::default(),
            markers: Vec::new(),
            price_lines: Vec::new(),
            next_price_line_id: 0,
        });
        self.attach_series_to_panel(panel_id, id, SeriesKind::Candlestick);
        if self.primary_candles.is_none() {
            self.primary_candles = Some(id);
        }
        id
    }

    pub(crate) fn add_line_series(&mut self) -> usize {
        let id = self.series.len();
        let panel_id = self.main_panel_id();
        self.series.push(Series {
            kind: SeriesKind::Line,
            scale: PriceScale::Left,
            panel_id,
            data: SeriesData::Line { data: Vec::new() },
            options: SeriesOptions::default(),
            markers: Vec::new(),
            price_lines: Vec::new(),
            next_price_line_id: 0,
        });
        self.attach_series_to_panel(panel_id, id, SeriesKind::Line);
        id
    }

    pub(crate) fn add_histogram_series(&mut self) -> usize {
        let id = self.series.len();
        let panel_id = self.main_panel_id();
        self.series.push(Series {
            kind: SeriesKind::Histogram,
            scale: PriceScale::Left,
            panel_id,
            data: SeriesData::Histogram { data: Vec::new() },
            options: SeriesOptions::default(),
            markers: Vec::new(),
            price_lines: Vec::new(),
            next_price_line_id: 0,
        });
        self.attach_series_to_panel(panel_id, id, SeriesKind::Histogram);
        id
    }

    pub(crate) fn set_rsi_panel(&mut self, title: String, data: Vec<LinePoint>) {
        let options = super::super::options::PriceScaleOptions {
            visible: true,
            ticks_visible: true,
            ..super::super::options::PriceScaleOptions::default()
        };
        let panel = IndicatorPanel {
            title,
            data,
            scale: PriceScaleState::new(),
            options,
            color: self.style.line,
        };
        self.rsi_panel = Some(panel);
        let parent_id = self.main_panel_id();
        let panel_id = self.rsi_panel_id.unwrap_or_else(|| {
            let new_panel = self.add_indicator_panel("RSI".to_string(), 1.0, None, Some(parent_id));
            self.rsi_panel_id = Some(new_panel);
            new_panel
        });
        if let Some(panel) = self.panels.iter_mut().find(|panel| panel.id == panel_id) {
            panel.role = PanelRole::Indicator;
            panel.title = "RSI".to_string();
        }
    }

    pub(crate) fn clear_rsi_panel(&mut self) {
        self.rsi_panel = None;
        if let Some(panel_id) = self.rsi_panel_id.take() {
            self.remove_panel(panel_id);
        }
    }

    pub(crate) fn set_rsi_panel_data(&mut self, data: Vec<LinePoint>) {
        match self.rsi_panel.as_mut() {
            Some(panel) => panel.data = data,
            None => self.set_rsi_panel("RSI".to_string(), data),
        }
    }

    pub(crate) fn has_rsi_panel(&self) -> bool {
        self.rsi_panel.is_some()
    }

    pub(crate) fn rsi_color(&self) -> Option<Color> {
        self.rsi_panel.as_ref().map(|panel| panel.color)
    }

    pub(crate) fn set_rsi_color(&mut self, color: Color) {
        if let Some(panel) = self.rsi_panel.as_mut() {
            panel.color = color;
        }
    }

    pub(crate) fn set_rsi_auto_scale(&mut self, enabled: bool) {
        if let Some(panel) = self.rsi_panel.as_mut() {
            panel.options.auto_scale = enabled;
        }
    }

    pub(crate) fn set_rsi_price_scale_visible(&mut self, visible: bool) {
        if let Some(panel) = self.rsi_panel.as_mut() {
            panel.options.visible = visible;
        }
    }

    pub(crate) fn rsi_auto_scale(&self) -> Option<bool> {
        self.rsi_panel
            .as_ref()
            .map(|panel| panel.options.auto_scale)
    }

    pub(crate) fn rsi_price_scale_visible(&self) -> Option<bool> {
        self.rsi_panel.as_ref().map(|panel| panel.options.visible)
    }

    fn main_panel_id(&self) -> PanelId {
        self.panels
            .first()
            .map(|panel| panel.id)
            .unwrap_or(PanelId(1))
    }

    fn attach_series_to_panel(&mut self, panel_id: PanelId, series_id: usize, kind: SeriesKind) {
        if let Some(panel) = self.panels.iter_mut().find(|panel| panel.id == panel_id) {
            panel.series.push(PanelSeries { series_id, kind });
            if matches!(kind, SeriesKind::Histogram) {
                panel.show_volume = true;
            }
        }
    }

    pub(crate) fn add_indicator_panel(
        &mut self,
        title: String,
        height_weight: f64,
        group_id: Option<TimeScaleId>,
        parent_id: Option<PanelId>,
    ) -> PanelId {
        let id = PanelId(self.next_panel_id);
        self.next_panel_id += 1;
        let group_id = group_id.unwrap_or_else(|| self.default_time_scale_id());
        let panel = Panel {
            id,
            group_id,
            role: PanelRole::Indicator,
            parent_id,
            title,
            height_weight: height_weight.max(0.5),
            content_visible: true,
            collapsed: false,
            left_scale: PriceScaleState::new(),
            right_scale: PriceScaleState::new(),
            left_visible: false,
            right_visible: true,
            series: Vec::new(),
            show_volume: false,
        };
        self.panels.push(panel);
        if let Some(group) = self
            .time_scales
            .iter_mut()
            .find(|group| group.id == group_id)
        {
            group.panels.push(id);
        }
        id
    }

    pub(crate) fn remove_panel(&mut self, panel_id: PanelId) {
        let main_panel = self.main_panel_id();
        if panel_id == main_panel {
            return;
        }
        self.panels.retain(|panel| panel.id != panel_id);
        for group in &mut self.time_scales {
            group.panels.retain(|id| *id != panel_id);
        }
        self.series.retain(|series| series.panel_id != panel_id);
    }

    pub(crate) fn toggle_panel_visibility(&mut self, panel_id: PanelId) {
        if let Some(panel) = self.panels.iter_mut().find(|panel| panel.id == panel_id) {
            panel.content_visible = !panel.content_visible;
        }
    }

    pub(crate) fn toggle_panel_collapsed(&mut self, panel_id: PanelId) {
        if let Some(panel) = self.panels.iter_mut().find(|panel| panel.id == panel_id) {
            panel.collapsed = !panel.collapsed;
        }
    }

    pub(crate) fn panel_content_visible(&self, panel_id: PanelId) -> bool {
        self.panels
            .iter()
            .find(|panel| panel.id == panel_id)
            .map(|panel| panel.content_visible && !panel.collapsed)
            .unwrap_or(true)
    }

    pub(crate) fn panel_is_collapsed(&self, panel_id: PanelId) -> bool {
        self.panels
            .iter()
            .find(|panel| panel.id == panel_id)
            .map(|panel| panel.collapsed)
            .unwrap_or(false)
    }

    pub(crate) fn panel_is_visible(&self, panel_id: PanelId) -> bool {
        self.panels
            .iter()
            .find(|panel| panel.id == panel_id)
            .map(|panel| panel.content_visible)
            .unwrap_or(true)
    }

    pub(crate) fn set_indicator_panel_data(&mut self, panel_id: PanelId, data: Vec<LinePoint>) {
        let line_series_id = self.series.iter().position(|series| {
            series.panel_id == panel_id && matches!(series.kind, SeriesKind::Line)
        });
        let series_id = match line_series_id {
            Some(id) => id,
            None => {
                let id = self.series.len();
                self.series.push(Series {
                    kind: SeriesKind::Line,
                    scale: PriceScale::Right,
                    panel_id,
                    data: SeriesData::Line { data: Vec::new() },
                    options: SeriesOptions::default(),
                    markers: Vec::new(),
                    price_lines: Vec::new(),
                    next_price_line_id: 0,
                });
                self.attach_series_to_panel(panel_id, id, SeriesKind::Line);
                id
            }
        };
        if let Some(series) = self.series.get_mut(series_id) {
            series.data = SeriesData::Line { data };
        }
    }

    pub(crate) fn panel_line_color(&self, panel_id: PanelId) -> Option<Color> {
        self.series
            .iter()
            .find(|series| series.panel_id == panel_id && matches!(series.kind, SeriesKind::Line))
            .map(|series| series.options.price_line_color.unwrap_or(self.style.line))
    }

    pub(crate) fn set_panel_line_color(&mut self, panel_id: PanelId, color: Color) {
        if let Some(series) = self
            .series
            .iter_mut()
            .find(|series| series.panel_id == panel_id && matches!(series.kind, SeriesKind::Line))
        {
            series.options.price_line_color = Some(color);
        }
    }

    pub(crate) fn set_panel_auto_scale(&mut self, panel_id: PanelId, enabled: bool) {
        if let Some(panel) = self.panels.iter_mut().find(|panel| panel.id == panel_id) {
            panel.left_scale.auto = enabled;
            panel.right_scale.auto = enabled;
        }
    }

    pub(crate) fn set_panel_price_scale_visible(&mut self, panel_id: PanelId, visible: bool) {
        if let Some(panel) = self.panels.iter_mut().find(|panel| panel.id == panel_id) {
            panel.left_visible = visible;
            panel.right_visible = visible;
        }
    }

    pub(crate) fn panel_auto_scale(&self, panel_id: PanelId) -> Option<bool> {
        self.panels
            .iter()
            .find(|panel| panel.id == panel_id)
            .map(|panel| panel.right_scale.auto)
    }

    pub(crate) fn panel_price_scale_visible(&self, panel_id: PanelId) -> Option<bool> {
        self.panels
            .iter()
            .find(|panel| panel.id == panel_id)
            .map(|panel| panel.right_visible || panel.left_visible)
    }

    fn default_time_scale_id(&self) -> TimeScaleId {
        self.time_scales
            .first()
            .map(|group| group.id)
            .unwrap_or(TimeScaleId(1))
    }

    pub(crate) fn set_series_scale(&mut self, id: usize, scale: PriceScale) {
        if let Some(series) = self.series.get_mut(id) {
            series.scale = scale;
        }
    }

    pub(crate) fn set_series_price_line(&mut self, id: usize, visible: bool) {
        if let Some(series) = self.series.get_mut(id) {
            series.options.show_price_line = visible;
        }
    }

    pub(crate) fn set_series_last_value(&mut self, id: usize, visible: bool) {
        if let Some(series) = self.series.get_mut(id) {
            series.options.show_last_value = visible;
        }
    }

    pub(crate) fn set_series_price_line_color(&mut self, id: usize, color: Color) {
        if let Some(series) = self.series.get_mut(id) {
            series.options.price_line_color = Some(color);
        }
    }

    pub(crate) fn set_series_price_line_style(&mut self, id: usize, style: LineStyle) {
        if let Some(series) = self.series.get_mut(id) {
            series.options.price_line_style = style;
        }
    }

    pub(crate) fn set_series_price_line_width(&mut self, id: usize, width: f64) {
        if let Some(series) = self.series.get_mut(id) {
            series.options.price_line_width = width.max(0.5);
        }
    }

    pub(crate) fn set_series_last_value_color(&mut self, id: usize, color: Color) {
        if let Some(series) = self.series.get_mut(id) {
            series.options.last_value_background = Some(color);
        }
    }

    pub(crate) fn set_series_last_value_text_color(&mut self, id: usize, color: Color) {
        if let Some(series) = self.series.get_mut(id) {
            series.options.last_value_text = Some(color);
        }
    }

    pub(crate) fn set_series_markers(&mut self, id: usize, markers: Vec<Marker>) {
        if let Some(series) = self.series.get_mut(id) {
            series.markers = markers;
        }
    }

    pub(crate) fn set_series_markers_options(&mut self, id: usize, options: SeriesMarkersOptions) {
        if let Some(series) = self.series.get_mut(id) {
            series.options.markers_options = options;
        }
    }

    pub(crate) fn create_price_line(&mut self, id: usize, options: PriceLineOptions) -> usize {
        if let Some(series) = self.series.get_mut(id) {
            let line_id = series.next_price_line_id;
            series.next_price_line_id += 1;
            series.price_lines.push(PriceLine {
                id: line_id,
                options,
            });
            return line_id;
        }
        0
    }

    pub(crate) fn update_price_line(
        &mut self,
        id: usize,
        line_id: usize,
        options: PriceLineOptions,
    ) {
        if let Some(series) = self.series.get_mut(id) {
            if let Some(line) = series
                .price_lines
                .iter_mut()
                .find(|line| line.id == line_id)
            {
                line.options = options;
            }
        }
    }

    pub(crate) fn set_price_line_price(&mut self, id: usize, line_id: usize, price: f64) {
        if let Some(series) = self.series.get_mut(id) {
            if let Some(line) = series
                .price_lines
                .iter_mut()
                .find(|line| line.id == line_id)
            {
                line.options.price = price;
            }
        }
    }

    pub(crate) fn remove_price_line(&mut self, id: usize, line_id: usize) {
        if let Some(series) = self.series.get_mut(id) {
            series.price_lines.retain(|line| line.id != line_id);
        }
    }

    pub(crate) fn set_candles(&mut self, id: usize, mut candles: Vec<Candle>) {
        candles.sort_by(|a, b| a.time.cmp(&b.time));
        if let Some(series) = self.series.get_mut(id) {
            series.data = SeriesData::Candlestick { data: candles };
        }
        self.recalculate_time_scale_after_data_update();
    }

    pub(crate) fn set_line_points(&mut self, id: usize, mut points: Vec<LinePoint>) {
        points.sort_by(|a, b| a.time.cmp(&b.time));
        if let Some(series) = self.series.get_mut(id) {
            series.data = SeriesData::Line { data: points };
        }
        self.recalculate_time_scale_after_data_update();
    }

    pub(crate) fn set_histogram_points(&mut self, id: usize, mut points: Vec<HistogramPoint>) {
        points.sort_by(|a, b| a.time.cmp(&b.time));
        if let Some(series) = self.series.get_mut(id) {
            series.data = SeriesData::Histogram { data: points };
        }
        self.recalculate_time_scale_after_data_update();
    }

    pub(crate) fn update_candle(&mut self, id: usize, candle: Candle) {
        if let Some(series) = self.series.get_mut(id) {
            if let SeriesData::Candlestick { data } = &mut series.data {
                update_sorted_by_time(data, candle);
            }
        }
        self.recalculate_time_scale_after_data_update();
    }

    pub(crate) fn update_line_point(&mut self, id: usize, point: LinePoint) {
        if let Some(series) = self.series.get_mut(id) {
            if let SeriesData::Line { data } = &mut series.data {
                update_sorted_by_time(data, point);
            }
        }
        self.recalculate_time_scale_after_data_update();
    }

    pub(crate) fn update_histogram_point(&mut self, id: usize, point: HistogramPoint) {
        if let Some(series) = self.series.get_mut(id) {
            if let SeriesData::Histogram { data } = &mut series.data {
                update_sorted_by_time(data, point);
            }
        }
        self.recalculate_time_scale_after_data_update();
    }

    pub(crate) fn set_series_price_format(&mut self, id: usize, format: PriceFormat) {
        if let Some(series) = self.series.get_mut(id) {
            series.options.price_format = format;
        }
    }
}

fn update_sorted_by_time<T: HasTime>(data: &mut Vec<T>, item: T) {
    match data.last_mut() {
        Some(last) if last.time() == item.time() => {
            *last = item;
        }
        Some(last) if last.time() < item.time() => {
            data.push(item);
        }
        _ => {
            data.push(item);
            data.sort_by(|a, b| a.time().cmp(&b.time()));
        }
    }
}

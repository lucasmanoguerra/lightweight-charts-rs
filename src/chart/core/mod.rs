mod interaction;
mod options;
mod pricescale;
mod render;
mod render_axes;
mod render_crosshair;
mod render_helpers;
mod render_markers;
mod render_overlays;
mod scale;
mod series;
mod timescale;

use super::data::{IndicatorPanel, Panel, PriceScaleState, Series, TimeScaleGroup};
use super::options::{ChartOptions, ChartStyle};
use super::types::{PanelControlHit, PanelId, PanelRole, Rect, TimeScaleId};
use std::cell::{Cell, RefCell};
use std::rc::Rc;

#[derive(Clone, Debug)]
pub(crate) struct ChartCore {
    pub(crate) panels: Vec<Panel>,
    pub(crate) series: Vec<Series>,
    pub(crate) style: ChartStyle,
    pub(crate) options: ChartOptions,
    time_scale: super::scales::TimeScale,
    time_scales: Vec<TimeScaleGroup>,
    crosshair: Option<(f64, f64)>,
    primary_candles: Option<usize>,
    left_scale: PriceScaleState,
    right_scale: PriceScaleState,
    last_plot_width: f64,
    tracking_mode_active: bool,
    rsi_panel: Option<IndicatorPanel>,
    tooltip_icon: Cell<Option<(PanelId, Rect)>>,
    panel_controls: Rc<RefCell<Vec<PanelControlHit>>>,
    next_panel_id: usize,
    next_time_scale_id: usize,
    rsi_panel_id: Option<PanelId>,
}

impl ChartCore {
    pub(crate) fn new() -> Self {
        let main_panel_id = PanelId(1);
        let main_time_scale_id = TimeScaleId(1);
        let time_scales = vec![TimeScaleGroup {
            id: main_time_scale_id,
            time_scale: super::scales::TimeScale::default(),
            panels: vec![main_panel_id],
        }];
        let panels = vec![Panel {
            id: main_panel_id,
            group_id: main_time_scale_id,
            role: PanelRole::Main,
            parent_id: None,
            title: "Main".to_string(),
            height_weight: 3.0,
            content_visible: true,
            collapsed: false,
            left_scale: PriceScaleState::new(),
            right_scale: PriceScaleState::new(),
            left_visible: true,
            right_visible: true,
            series: Vec::new(),
            show_volume: true,
        }];
        Self {
            panels,
            series: Vec::new(),
            style: ChartStyle::default(),
            options: ChartOptions::default(),
            time_scale: super::scales::TimeScale::default(),
            time_scales,
            crosshair: None,
            primary_candles: None,
            left_scale: PriceScaleState::new(),
            right_scale: PriceScaleState::new(),
            last_plot_width: 0.0,
            tracking_mode_active: false,
            rsi_panel: None,
            tooltip_icon: Cell::new(None),
            panel_controls: Rc::new(RefCell::new(Vec::new())),
            next_panel_id: 2,
            next_time_scale_id: 2,
            rsi_panel_id: None,
        }
    }
}

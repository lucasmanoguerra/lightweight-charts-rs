use super::core::ChartCore;
use super::types::{PanelId, TimeScaleId};

#[derive(Clone, Copy)]
pub(crate) struct PanelLayout {
    pub(crate) id: PanelId,
    pub(crate) group_id: TimeScaleId,
    pub(crate) role: super::types::PanelRole,
    pub(crate) content_visible: bool,
    pub(crate) collapsed: bool,
    pub(crate) top: f64,
    pub(crate) bottom: f64,
    pub(crate) height: f64,
    pub(crate) plot_left: f64,
    pub(crate) plot_right: f64,
    pub(crate) plot_width: f64,
    pub(crate) main_top: f64,
    pub(crate) main_bottom: f64,
    pub(crate) main_height: f64,
    pub(crate) hist_top: f64,
    pub(crate) hist_bottom: f64,
    pub(crate) hist_height: f64,
    pub(crate) axis_left: f64,
    pub(crate) axis_right: f64,
}

#[derive(Clone, Copy)]
pub(crate) struct TimeAxisLayout {
    pub(crate) group_id: TimeScaleId,
    pub(crate) top: f64,
    pub(crate) bottom: f64,
    pub(crate) height: f64,
    pub(crate) plot_left: f64,
    pub(crate) plot_right: f64,
    pub(crate) plot_width: f64,
    pub(crate) axis_left: f64,
    pub(crate) axis_right: f64,
}

#[derive(Clone)]
pub(crate) struct ChartLayout {
    pub(crate) panels: Vec<PanelLayout>,
    pub(crate) time_axes: Vec<TimeAxisLayout>,
    pub(crate) plot_left: f64,
    pub(crate) plot_right: f64,
    pub(crate) plot_top: f64,
    pub(crate) plot_bottom: f64,
    pub(crate) plot_width: f64,
    pub(crate) plot_height: f64,
    pub(crate) main_bottom: f64,
    pub(crate) main_height: f64,
    pub(crate) hist_top: f64,
    pub(crate) hist_bottom: f64,
    pub(crate) hist_height: f64,
    pub(crate) rsi_top: f64,
    pub(crate) rsi_bottom: f64,
    pub(crate) rsi_height: f64,
    pub(crate) axis_left: f64,
    pub(crate) axis_right: f64,
    pub(crate) width: f64,
    pub(crate) height: f64,
    pub(crate) padding: f64,
}

impl ChartLayout {
    pub(crate) fn new(chart: &ChartCore, width: f64, height: f64) -> Self {
        let padding = chart.style.padding;
        let time_axis_height = if chart.options.time_scale.visible {
            chart
                .style
                .axis_height
                .max(chart.options.time_scale.minimum_height)
        } else {
            0.0
        };

        let mut group_order: Vec<TimeScaleId> = Vec::new();
        for panel in &chart.panels {
            if !group_order.contains(&panel.group_id) {
                group_order.push(panel.group_id);
            }
        }

        let collapsed_height = chart.style.panel_toolbar_height.max(18.0);
        let collapsed_total: f64 = chart
            .panels
            .iter()
            .filter(|panel| panel.collapsed)
            .count() as f64
            * collapsed_height;
        let total_weight: f64 = chart
            .panels
            .iter()
            .filter(|panel| !panel.collapsed)
            .map(|panel| panel.height_weight.max(0.1))
            .sum();

        let time_axes_count = group_order.len();
        let available_height = (height
            - padding * 2.0
            - time_axis_height * time_axes_count as f64
            - collapsed_total)
            .max(1.0);
        let unit_height = if total_weight > 0.0 {
            available_height / total_weight
        } else {
            0.0
        };

        let mut panels = Vec::new();
        let mut time_axes = Vec::new();
        let mut cursor_y = padding;
        for group_id in group_order {
            let group_panels: Vec<_> = chart
                .panels
                .iter()
                .filter(|panel| panel.group_id == group_id)
                .collect();
            let left_axis_width = if group_panels.iter().any(|panel| panel.left_visible) {
                chart
                    .style
                    .price_axis_width
                    .max(chart.options.left_price_scale.minimum_width)
            } else {
                0.0
            };
            let right_axis_width = if group_panels.iter().any(|panel| panel.right_visible) {
                chart
                    .style
                    .price_axis_width
                    .max(chart.options.right_price_scale.minimum_width)
            } else {
                0.0
            };
            let plot_left = padding + left_axis_width;
            let plot_right = (width - padding - right_axis_width).max(plot_left + 1.0);
            let plot_width = plot_right - plot_left;
            let axis_left = padding;
            let axis_right = (width - padding).max(plot_right + 1.0);

            for panel in group_panels {
                let height = if panel.collapsed {
                    collapsed_height
                } else {
                    (panel.height_weight.max(0.1) * unit_height).max(1.0)
                };
                let top = cursor_y;
                let bottom = (cursor_y + height).max(top + 1.0);
                let hist_ratio = chart.style.histogram_height_ratio.min(0.2);
                let hist_height = if panel.show_volume && !panel.collapsed {
                    (height * hist_ratio).max(1.0)
                } else {
                    0.0
                };
                let hist_height = hist_height.min(height.max(1.0));
                let main_bottom = (bottom - hist_height).max(top + 1.0);
                let main_height = (main_bottom - top).max(1.0);
                let hist_top = main_bottom;
                let hist_bottom = bottom;
                let hist_height = (hist_bottom - hist_top).max(0.0);
                panels.push(PanelLayout {
                    id: panel.id,
                    group_id: panel.group_id,
                    role: panel.role,
                    content_visible: panel.content_visible,
                    collapsed: panel.collapsed,
                    top,
                    bottom,
                    height,
                    plot_left,
                    plot_right,
                    plot_width,
                    main_top: top,
                    main_bottom,
                    main_height,
                    hist_top,
                    hist_bottom,
                    hist_height,
                    axis_left,
                    axis_right,
                });
                cursor_y = bottom;
            }

            if time_axis_height > 0.0 {
                let top = cursor_y;
                let bottom = (top + time_axis_height).max(top + 1.0);
                time_axes.push(TimeAxisLayout {
                    group_id,
                    top,
                    bottom,
                    height: time_axis_height,
                    plot_left,
                    plot_right,
                    plot_width,
                    axis_left,
                    axis_right,
                });
                cursor_y = bottom;
            }
        }

        let main_panel = panels
            .iter()
            .find(|panel| matches!(panel.role, super::types::PanelRole::Main))
            .or_else(|| panels.first());
        let (plot_left, plot_right, plot_top, plot_bottom, plot_width, plot_height, main_bottom, main_height, hist_top, hist_bottom, hist_height, axis_left, axis_right) =
            if let Some(panel) = main_panel {
                (
                    panel.plot_left,
                    panel.plot_right,
                    panel.top,
                    panel.bottom,
                    panel.plot_width,
                    panel.height,
                    panel.main_bottom,
                    panel.main_height,
                    panel.hist_top,
                    panel.hist_bottom,
                    panel.hist_height,
                    panel.axis_left,
                    panel.axis_right,
                )
            } else {
                (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0)
            };
        let (rsi_top, rsi_bottom, rsi_height) = panels
            .iter()
            .find(|panel| {
                matches!(panel.role, super::types::PanelRole::Indicator)
                    && panel.content_visible
                    && !panel.collapsed
            })
            .map(|panel| (panel.top, panel.bottom, panel.height))
            .unwrap_or((0.0, 0.0, 0.0));

        Self {
            panels,
            time_axes,
            plot_left,
            plot_right,
            plot_top,
            plot_bottom,
            plot_width,
            plot_height,
            main_bottom,
            main_height,
            hist_top,
            hist_bottom,
            hist_height,
            rsi_top,
            rsi_bottom,
            rsi_height,
            axis_left,
            axis_right,
            width,
            height,
            padding,
        }
    }

    pub(crate) fn panel_at(&self, y: f64) -> Option<PanelId> {
        self.panels
            .iter()
            .find(|panel| y >= panel.top && y <= panel.bottom)
            .map(|panel| panel.id)
    }

    pub(crate) fn panel_layout(&self, panel_id: PanelId) -> Option<&PanelLayout> {
        self.panels.iter().find(|panel| panel.id == panel_id)
    }

    pub(crate) fn time_axis_at(&self, y: f64) -> Option<TimeScaleId> {
        self.time_axes
            .iter()
            .find(|axis| y >= axis.top && y <= axis.bottom)
            .map(|axis| axis.group_id)
    }

    pub(crate) fn time_axis_layout(&self, group_id: TimeScaleId) -> Option<&TimeAxisLayout> {
        self.time_axes.iter().find(|axis| axis.group_id == group_id)
    }

    pub(crate) fn panel_axis_side(&self, panel_id: PanelId, x: f64) -> Option<bool> {
        let panel = self.panel_layout(panel_id)?;
        if x < panel.plot_left {
            return Some(true);
        }
        if x > panel.plot_right {
            return Some(false);
        }
        None
    }

    pub(crate) fn group_axis_side(&self, group_id: TimeScaleId, x: f64) -> Option<bool> {
        let axis = self.time_axis_layout(group_id)?;
        if x < axis.plot_left {
            return Some(true);
        }
        if x > axis.plot_right {
            return Some(false);
        }
        None
    }

    pub(crate) fn in_time_axis(&self, y: f64) -> bool {
        y > self.plot_bottom
    }

    pub(crate) fn in_histogram(&self, y: f64) -> bool {
        self.hist_height > 0.0 && y >= self.hist_top && y <= self.hist_bottom
    }

    pub(crate) fn in_rsi_plot(&self, y: f64) -> bool {
        self.rsi_height > 0.0 && y >= self.rsi_top && y <= self.rsi_bottom
    }

    pub(crate) fn in_main_plot(&self, y: f64) -> bool {
        y >= self.plot_top && y <= self.main_bottom
    }

    pub(crate) fn in_left_axis(&self, x: f64) -> bool {
        x < self.plot_left
    }

    pub(crate) fn in_right_axis(&self, x: f64) -> bool {
        x > self.plot_right
    }

    pub(crate) fn in_axis(&self, x: f64) -> bool {
        self.in_left_axis(x) || self.in_right_axis(x)
    }
}

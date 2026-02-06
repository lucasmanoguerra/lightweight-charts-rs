use super::super::options::PriceScaleOptions;
use super::super::types::{Color, PriceScale, PriceScaleMode, ScaleMargins};
use super::ChartCore;

impl ChartCore {
    pub(crate) fn reset_autoscale(&mut self, side: PriceScale) {
        let state = match side {
            PriceScale::Left => &mut self.left_scale,
            PriceScale::Right => &mut self.right_scale,
        };
        state.auto = true;
    }

    pub(crate) fn set_price_scale_options(&mut self, side: PriceScale, options: PriceScaleOptions) {
        match side {
            PriceScale::Left => self.options.left_price_scale = options,
            PriceScale::Right => self.options.right_price_scale = options,
        }
    }

    pub(crate) fn set_price_scale_mode(&mut self, side: PriceScale, mode: PriceScaleMode) {
        match side {
            PriceScale::Left => self.options.left_price_scale.mode = mode,
            PriceScale::Right => self.options.right_price_scale.mode = mode,
        }
    }

    pub(crate) fn set_price_scale_auto_scale(&mut self, side: PriceScale, enabled: bool) {
        match side {
            PriceScale::Left => self.options.left_price_scale.auto_scale = enabled,
            PriceScale::Right => self.options.right_price_scale.auto_scale = enabled,
        }
    }

    pub(crate) fn set_price_scale_visible(&mut self, side: PriceScale, visible: bool) {
        match side {
            PriceScale::Left => self.options.left_price_scale.visible = visible,
            PriceScale::Right => self.options.right_price_scale.visible = visible,
        }
    }

    pub(crate) fn set_price_scale_margins(&mut self, side: PriceScale, margins: ScaleMargins) {
        match side {
            PriceScale::Left => self.options.left_price_scale.scale_margins = margins,
            PriceScale::Right => self.options.right_price_scale.scale_margins = margins,
        }
    }

    pub(crate) fn set_price_scale_border(&mut self, side: PriceScale, visible: bool, color: Color) {
        match side {
            PriceScale::Left => {
                self.options.left_price_scale.border_visible = visible;
                self.options.left_price_scale.border_color = color;
            }
            PriceScale::Right => {
                self.options.right_price_scale.border_visible = visible;
                self.options.right_price_scale.border_color = color;
            }
        }
    }

    pub(crate) fn set_price_scale_text_color(&mut self, side: PriceScale, color: Color) {
        match side {
            PriceScale::Left => self.options.left_price_scale.text_color = color,
            PriceScale::Right => self.options.right_price_scale.text_color = color,
        }
    }

    pub(crate) fn set_price_scale_ticks_visible(&mut self, side: PriceScale, visible: bool) {
        match side {
            PriceScale::Left => self.options.left_price_scale.ticks_visible = visible,
            PriceScale::Right => self.options.right_price_scale.ticks_visible = visible,
        }
    }

    pub(crate) fn set_price_scale_minimum_width(&mut self, side: PriceScale, width: f64) {
        match side {
            PriceScale::Left => self.options.left_price_scale.minimum_width = width.max(0.0),
            PriceScale::Right => self.options.right_price_scale.minimum_width = width.max(0.0),
        }
    }

    pub(crate) fn set_price_scale_invert(&mut self, side: PriceScale, invert: bool) {
        match side {
            PriceScale::Left => self.options.left_price_scale.invert_scale = invert,
            PriceScale::Right => self.options.right_price_scale.invert_scale = invert,
        }
    }

    pub(crate) fn set_price_scale_align_labels(&mut self, side: PriceScale, align: bool) {
        match side {
            PriceScale::Left => self.options.left_price_scale.align_labels = align,
            PriceScale::Right => self.options.right_price_scale.align_labels = align,
        }
    }

    pub(crate) fn set_price_scale_entire_text_only(&mut self, side: PriceScale, enabled: bool) {
        match side {
            PriceScale::Left => self.options.left_price_scale.entire_text_only = enabled,
            PriceScale::Right => self.options.right_price_scale.entire_text_only = enabled,
        }
    }

    pub(crate) fn set_price_scale_ensure_edge_ticks(&mut self, side: PriceScale, enabled: bool) {
        match side {
            PriceScale::Left => {
                self.options.left_price_scale.ensure_edge_tick_marks_visible = enabled
            }
            PriceScale::Right => {
                self.options
                    .right_price_scale
                    .ensure_edge_tick_marks_visible = enabled
            }
        }
    }
}

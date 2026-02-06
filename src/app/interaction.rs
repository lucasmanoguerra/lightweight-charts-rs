use crate::chart::{
    ChartApi, PanelControlAction, PanelId, PanelResizeHandle, PriceScale, TrackingModeExitMode,
};
use relm4::gtk::{self, gdk, glib};
use relm4::gtk::prelude::*;
use std::cell::{Cell, RefCell};
use std::rc::Rc;

fn is_touch_device(device: Option<gdk::Device>) -> bool {
    device
        .map(|device| {
            matches!(
                device.source(),
                gdk::InputSource::Touchscreen | gdk::InputSource::Touchpad
            )
        })
        .unwrap_or(false)
}

struct KineticState {
    velocity_x: f64,
    last_time: i64,
    source: Option<glib::SourceId>,
}

impl KineticState {
    fn new() -> Self {
        Self {
            velocity_x: 0.0,
            last_time: 0,
            source: None,
        }
    }
}

fn stop_kinetic(state: &Rc<RefCell<KineticState>>) {
    if let Some(source) = state.borrow_mut().source.take() {
        source.remove();
    }
}

pub fn install_interactions(
    drawing_area: &gtk::DrawingArea,
    chart: ChartApi,
    on_price_axis_zoom: Option<Rc<dyn Fn(PriceScale)>>,
    on_panel_menu: Option<Rc<dyn Fn(PanelId, f64, f64)>>,
    on_panel_control: Option<Rc<dyn Fn(PanelId, PanelControlAction)>>,
) {
    let last_pointer_x = Rc::new(Cell::new(0.0));
    let last_pointer_y = Rc::new(Cell::new(0.0));
    let kinetic_state = Rc::new(RefCell::new(KineticState::new()));
    let touch_kinetic_state = Rc::new(RefCell::new(KineticState::new()));
    let motion = gtk::EventControllerMotion::new();
    motion.connect_motion({
        let last_pointer_x = last_pointer_x.clone();
        let last_pointer_y = last_pointer_y.clone();
        let chart = chart.clone();
        let drawing_area = drawing_area.clone();
        move |_, x, y| {
            last_pointer_x.set(x);
            last_pointer_y.set(y);
            chart.set_crosshair(x, y);
            drawing_area.queue_draw();
        }
    });
    motion.connect_leave({
        let chart = chart.clone();
        let drawing_area = drawing_area.clone();
        move |_| {
            chart.clear_crosshair();
            drawing_area.queue_draw();
        }
    });
    drawing_area.add_controller(motion);

    let scroll = gtk::EventControllerScroll::new(gtk::EventControllerScrollFlags::VERTICAL);
    scroll.connect_scroll({
        let chart = chart.clone();
        let drawing_area = drawing_area.clone();
        let last_pointer_x = last_pointer_x.clone();
        let last_pointer_y = last_pointer_y.clone();
        let on_price_axis_zoom = on_price_axis_zoom.clone();
        let kinetic_state = kinetic_state.clone();
        move |_, _dx, dy| {
            stop_kinetic(&kinetic_state);
            let width = drawing_area.width() as f64;
            let height = drawing_area.height() as f64;
            let zoomed_side =
                chart.zoom_by_delta(dy, last_pointer_x.get(), last_pointer_y.get(), width, height);
            if let (Some(side), Some(handler)) = (zoomed_side, on_price_axis_zoom.as_ref()) {
                handler(side);
            }
            drawing_area.queue_draw();
            glib::Propagation::Stop
        }
    });
    drawing_area.add_controller(scroll);

    let click = gtk::GestureClick::new();
    click.connect_pressed({
        let chart = chart.clone();
        let drawing_area = drawing_area.clone();
        let on_panel_control = on_panel_control.clone();
        move |_, n_press, x, y| {
            if n_press == 1 {
                if let Some((panel, action)) = chart.panel_control_at(x, y) {
                    if let Some(handler) = on_panel_control.as_ref() {
                        handler(panel, action);
                    }
                    drawing_area.queue_draw();
                    return;
                }
            }
            if chart.tracking_mode_active() {
                let tracking = chart.tracking_mode_options();
                if matches!(tracking.exit_mode, TrackingModeExitMode::OnNextTap) {
                    chart.set_tracking_mode_active(false);
                    drawing_area.queue_draw();
                    return;
                }
            }
            if n_press == 2 {
                let width = drawing_area.width() as f64;
                let height = drawing_area.height() as f64;
                chart.handle_double_click(x, y, width, height);
                drawing_area.queue_draw();
            }
        }
    });
    drawing_area.add_controller(click);

    let right_click = gtk::GestureClick::new();
    right_click.set_button(3);
    right_click.connect_pressed({
        let chart = chart.clone();
        let drawing_area = drawing_area.clone();
        let on_panel_menu = on_panel_menu.clone();
        move |_, _, x, y| {
            let width = drawing_area.width() as f64;
            let height = drawing_area.height() as f64;
            let panel = chart
                .tooltip_icon_at(x, y)
                .or_else(|| chart.panel_at(x, y, width, height));
            if let (Some(panel), Some(handler)) = (panel, on_panel_menu.as_ref()) {
                handler(panel, x, y);
            }
        }
    });
    drawing_area.add_controller(right_click);

    let long_press = gtk::GestureLongPress::new();
    long_press.set_touch_only(true);
    long_press.connect_pressed({
        let chart = chart.clone();
        let drawing_area = drawing_area.clone();
        move |_, _, _| {
            let tracking = chart.tracking_mode_options();
            if tracking.enabled {
                chart.set_tracking_mode_active(true);
                drawing_area.queue_draw();
            }
        }
    });
    long_press.connect_end({
        let chart = chart.clone();
        let drawing_area = drawing_area.clone();
        move |_, _| {
            let tracking = chart.tracking_mode_options();
            if chart.tracking_mode_active()
                && matches!(tracking.exit_mode, TrackingModeExitMode::OnTouchEnd)
            {
                chart.set_tracking_mode_active(false);
                drawing_area.queue_draw();
            }
        }
    });
    drawing_area.add_controller(long_press);

    let drag = gtk::GestureDrag::new();
    let last_drag_x = Rc::new(Cell::new(0.0));
    let last_drag_y = Rc::new(Cell::new(0.0));
    let resize_handle: Rc<Cell<Option<PanelResizeHandle>>> = Rc::new(Cell::new(None));
    drag.connect_drag_begin({
        let last_drag_x = last_drag_x.clone();
        let last_drag_y = last_drag_y.clone();
        let resize_handle = resize_handle.clone();
        let chart = chart.clone();
        let drawing_area = drawing_area.clone();
        let kinetic_state = kinetic_state.clone();
        move |_, start_x, start_y| {
            last_drag_x.set(0.0);
            last_drag_y.set(0.0);
            stop_kinetic(&kinetic_state);
            let mut state = kinetic_state.borrow_mut();
            state.velocity_x = 0.0;
            state.last_time = glib::monotonic_time();
            let width = drawing_area.width() as f64;
            let height = drawing_area.height() as f64;
            let handle = chart.panel_resize_handle_at(start_y, width, height);
            resize_handle.set(handle);
            let _ = start_x;
        }
    });
    drag.connect_drag_update({
        let last_drag_x = last_drag_x.clone();
        let last_drag_y = last_drag_y.clone();
        let resize_handle = resize_handle.clone();
        let chart = chart.clone();
        let drawing_area = drawing_area.clone();
        let last_pointer_x = last_pointer_x.clone();
        let last_pointer_y = last_pointer_y.clone();
        let on_price_axis_zoom = on_price_axis_zoom.clone();
        let kinetic_state = kinetic_state.clone();
        move |gesture, offset_x, offset_y| {
            if is_touch_device(gesture.device()) {
                return;
            }
            let delta_x = offset_x - last_drag_x.get();
            let delta_y = offset_y - last_drag_y.get();
            last_drag_x.set(offset_x);
            last_drag_y.set(offset_y);
            let width = drawing_area.width() as f64;
            let height = drawing_area.height() as f64;
            if let Some(handle) = resize_handle.get() {
                chart.resize_panels_by_pixels(handle, delta_y, width, height);
                drawing_area.queue_draw();
                return;
            }
            let result = chart.pan_by_pixels(
                delta_x,
                delta_y,
                width,
                height,
                last_pointer_x.get(),
                last_pointer_y.get(),
            );
            if let (Some(side), Some(handler)) =
                (result.price_axis_zoomed, on_price_axis_zoom.as_ref())
            {
                handler(side);
            }
            if result.time_panned {
                let now = glib::monotonic_time();
                let mut state = kinetic_state.borrow_mut();
                if state.last_time > 0 {
                    let dt = (now - state.last_time).max(1) as f64 / 1_000_000.0;
                    let instant = delta_x / dt;
                    state.velocity_x = state.velocity_x * 0.8 + instant * 0.2;
                }
                state.last_time = now;
            }
            drawing_area.queue_draw();
        }
    });
    drag.connect_drag_end({
        let chart = chart.clone();
        let drawing_area = drawing_area.clone();
        let last_pointer_x = last_pointer_x.clone();
        let last_pointer_y = last_pointer_y.clone();
        let kinetic_state = kinetic_state.clone();
        let resize_handle = resize_handle.clone();
        move |gesture, _, _| {
            if is_touch_device(gesture.device()) {
                return;
            }
            resize_handle.set(None);
            let options = chart.kinetic_scroll_options();
            if !options.mouse || chart.tracking_mode_active() {
                return;
            }
            let velocity = kinetic_state.borrow().velocity_x;
            let min_velocity = 40.0;
            if velocity.abs() < min_velocity {
                return;
            }
            stop_kinetic(&kinetic_state);
            let mut state = kinetic_state.borrow_mut();
            let mut v = velocity;
            let decay = 0.92;
            let frame = 1.0 / 60.0;
            let source = glib::timeout_add_local(std::time::Duration::from_millis(16), {
                let chart = chart.clone();
                let drawing_area = drawing_area.clone();
                let last_pointer_x = last_pointer_x.clone();
                let last_pointer_y = last_pointer_y.clone();
                move || {
                    v *= decay;
                    if v.abs() < min_velocity {
                        return glib::ControlFlow::Break;
                    }
                    let dx = v * frame;
                    let width = drawing_area.width() as f64;
                    let height = drawing_area.height() as f64;
                    chart.pan_by_pixels(dx, 0.0, width, height, last_pointer_x.get(), last_pointer_y.get());
                    drawing_area.queue_draw();
                    glib::ControlFlow::Continue
                }
            });
            state.source = Some(source);
        }
    });
    drawing_area.add_controller(drag);

    let touch_drag = gtk::GestureDrag::new();
    touch_drag.set_touch_only(true);
    let last_touch_x = Rc::new(Cell::new(0.0));
    let last_touch_y = Rc::new(Cell::new(0.0));
    touch_drag.connect_drag_begin({
        let last_touch_x = last_touch_x.clone();
        let last_touch_y = last_touch_y.clone();
        let touch_kinetic_state = touch_kinetic_state.clone();
        move |gesture, _, _| {
            last_touch_x.set(0.0);
            last_touch_y.set(0.0);
            if let Some((x, y)) = gesture.point(None) {
                last_touch_x.set(x);
                last_touch_y.set(y);
            }
            stop_kinetic(&touch_kinetic_state);
            let mut state = touch_kinetic_state.borrow_mut();
            state.velocity_x = 0.0;
            state.last_time = glib::monotonic_time();
        }
    });
    touch_drag.connect_drag_update({
        let last_touch_x = last_touch_x.clone();
        let last_touch_y = last_touch_y.clone();
        let chart = chart.clone();
        let drawing_area = drawing_area.clone();
        let last_pointer_x = last_pointer_x.clone();
        let last_pointer_y = last_pointer_y.clone();
        let touch_kinetic_state = touch_kinetic_state.clone();
        move |gesture, offset_x, offset_y| {
            let width = drawing_area.width() as f64;
            let height = drawing_area.height() as f64;
            if let Some((x, y)) = gesture.point(None) {
                last_pointer_x.set(x);
                last_pointer_y.set(y);
            }
            let delta_x = offset_x - last_touch_x.get();
            let delta_y = offset_y - last_touch_y.get();
            last_touch_x.set(offset_x);
            last_touch_y.set(offset_y);
            let time_panned = chart.pan_by_pixels_touch(
                delta_x,
                delta_y,
                width,
                height,
                last_pointer_x.get(),
                last_pointer_y.get(),
            );
            if time_panned {
                let now = glib::monotonic_time();
                let mut state = touch_kinetic_state.borrow_mut();
                if state.last_time > 0 {
                    let dt = (now - state.last_time).max(1) as f64 / 1_000_000.0;
                    let instant = delta_x / dt;
                    state.velocity_x = state.velocity_x * 0.8 + instant * 0.2;
                }
                state.last_time = now;
            }
            drawing_area.queue_draw();
        }
    });
    touch_drag.connect_drag_end({
        let chart = chart.clone();
        let drawing_area = drawing_area.clone();
        let last_pointer_x = last_pointer_x.clone();
        let last_pointer_y = last_pointer_y.clone();
        let touch_kinetic_state = touch_kinetic_state.clone();
        move |_, _, _| {
            let options = chart.kinetic_scroll_options();
            if !options.touch || chart.tracking_mode_active() {
                return;
            }
            let velocity = touch_kinetic_state.borrow().velocity_x;
            let min_velocity = 40.0;
            if velocity.abs() < min_velocity {
                return;
            }
            stop_kinetic(&touch_kinetic_state);
            let mut state = touch_kinetic_state.borrow_mut();
            let mut v = velocity;
            let decay = 0.92;
            let frame = 1.0 / 60.0;
            let source = glib::timeout_add_local(std::time::Duration::from_millis(16), {
                let chart = chart.clone();
                let drawing_area = drawing_area.clone();
                let last_pointer_x = last_pointer_x.clone();
                let last_pointer_y = last_pointer_y.clone();
                move || {
                    v *= decay;
                    if v.abs() < min_velocity {
                        return glib::ControlFlow::Break;
                    }
                    let dx = v * frame;
                    let width = drawing_area.width() as f64;
                    let height = drawing_area.height() as f64;
                    chart.pan_by_pixels(dx, 0.0, width, height, last_pointer_x.get(), last_pointer_y.get());
                    drawing_area.queue_draw();
                    glib::ControlFlow::Continue
                }
            });
            state.source = Some(source);
        }
    });
    drawing_area.add_controller(touch_drag);

    let pinch = gtk::GestureZoom::new();
    let last_pinch_scale = Rc::new(Cell::new(1.0));
    pinch.connect_begin({
        let last_pinch_scale = last_pinch_scale.clone();
        move |_, _| {
            last_pinch_scale.set(1.0);
        }
    });
    pinch.connect_scale_changed({
        let chart = chart.clone();
        let drawing_area = drawing_area.clone();
        let last_pointer_x = last_pointer_x.clone();
        let last_pointer_y = last_pointer_y.clone();
        let last_pinch_scale = last_pinch_scale.clone();
        move |gesture, scale| {
            if scale <= 0.0 {
                return;
            }
            let last = last_pinch_scale.get();
            let relative = if last > 0.0 { scale / last } else { 1.0 };
            last_pinch_scale.set(scale);

            let factor = (1.0 / relative).clamp(0.3, 3.0);
            let sensitivity = chart.interaction_sensitivity().pinch_zoom.max(0.0001);
            let denom = (1.0_f64 + sensitivity).ln();
            let delta = if denom.abs() < 1e-9 {
                0.0
            } else {
                factor.ln() / denom
            };

            let (x, y) = gesture
                .bounding_box_center()
                .unwrap_or((last_pointer_x.get(), last_pointer_y.get()));
            let width = drawing_area.width() as f64;
            let height = drawing_area.height() as f64;
            chart.zoom_by_delta_pinch(delta, x, y, width, height);
            drawing_area.queue_draw();
        }
    });
    drawing_area.add_controller(pinch);
}

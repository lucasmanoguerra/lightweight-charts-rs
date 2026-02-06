use cairo::{Context, Format, ImageSurface};
use resvg::{tiny_skia, usvg};
use std::sync::OnceLock;

use crate::chart::Color;

#[derive(Clone, Copy, Debug)]
pub enum IconName {
    Gear,
    Plus,
    Eye,
    EyeOff,
    Trash,
    ChevronUp,
    ChevronDown,
    AddUp,
    AddDown,
}

fn svg_source(name: IconName) -> &'static str {
    match name {
        IconName::Gear => include_str!("../../assets/icons/gear.svg"),
        IconName::Plus => include_str!("../../assets/icons/plus.svg"),
        IconName::Eye => include_str!("../../assets/icons/eye.svg"),
        IconName::EyeOff => include_str!("../../assets/icons/eye-off.svg"),
        IconName::Trash => include_str!("../../assets/icons/trash.svg"),
        IconName::ChevronUp => include_str!("../../assets/icons/chevron-up.svg"),
        IconName::ChevronDown => include_str!("../../assets/icons/chevron-down.svg"),
        IconName::AddUp => include_str!("../../assets/icons/add-up.svg"),
        IconName::AddDown => include_str!("../../assets/icons/add-down.svg"),
    }
}

fn marker_svg_source(text: &str) -> Option<&'static str> {
    match text.trim() {
        // Basic shapes
        "●" | "circle" | "Circle" => Some(include_str!("../../assets/icons/markers/circle.svg")),
        "○" | "circle-outline" | "Circle Outline" => {
            Some(include_str!("../../assets/icons/markers/circle-outline.svg"))
        }
        "■" | "square" | "Square" => Some(include_str!("../../assets/icons/markers/square.svg")),
        "□" | "square-outline" | "Square Outline" => {
            Some(include_str!("../../assets/icons/markers/square-outline.svg"))
        }
        "▲" | "triangle-up" | "Triangle Up" => {
            Some(include_str!("../../assets/icons/markers/triangle-up.svg"))
        }
        "▼" | "triangle-down" | "Triangle Down" => {
            Some(include_str!("../../assets/icons/markers/triangle-down.svg"))
        }
        "◆" | "diamond" | "Diamond" => Some(include_str!("../../assets/icons/markers/diamond.svg")),
        "◇" | "diamond-outline" | "Diamond Outline" => {
            Some(include_str!("../../assets/icons/markers/diamond-outline.svg"))
        }
        // Signals
        "✓" | "check" | "Check" => Some(include_str!("../../assets/icons/markers/check.svg")),
        "✕" | "x" | "X" => Some(include_str!("../../assets/icons/markers/x.svg")),
        "★" | "☆" | "star" | "Star" => Some(include_str!("../../assets/icons/markers/star.svg")),
        "⚑" | "⚐" | "flag" | "Flag" => Some(include_str!("../../assets/icons/markers/flag.svg")),
        // Alerts (mapped to warning icon)
        "⚡" | "⚠" | "❗" | "❓" | "⛔" | "warning" | "Warning" => {
            Some(include_str!("../../assets/icons/markers/warning.svg"))
        }
        // Time
        "⏰" | "⏱" | "⌛" | "clock" | "Clock" => {
            Some(include_str!("../../assets/icons/markers/clock.svg"))
        }
        "📅" | "calendar" | "Calendar" => {
            Some(include_str!("../../assets/icons/markers/calendar.svg"))
        }
        // Arrows
        "↑" | "arrow-up" | "Arrow Up" => {
            Some(include_str!("../../assets/icons/markers/arrow-up.svg"))
        }
        "↓" | "arrow-down" | "Arrow Down" => {
            Some(include_str!("../../assets/icons/markers/arrow-down.svg"))
        }
        "→" | "arrow-right" | "Arrow Right" | "↗" => {
            Some(include_str!("../../assets/icons/markers/arrow-right.svg"))
        }
        "←" | "arrow-left" | "Arrow Left" | "↘" => {
            Some(include_str!("../../assets/icons/markers/arrow-left.svg"))
        }
        // Finance
        "💰" | "💹" | "₿" | "$" | "money" | "Money" => {
            Some(include_str!("../../assets/icons/markers/money.svg"))
        }
        _ => None,
    }
}

fn fontdb() -> &'static usvg::fontdb::Database {
    static FONT_DB: OnceLock<usvg::fontdb::Database> = OnceLock::new();
    FONT_DB.get_or_init(|| {
        let mut db = usvg::fontdb::Database::new();
        db.load_system_fonts();
        db
    })
}

fn draw_svg_source(
    cr: &Context,
    svg_source: &str,
    x: f64,
    y: f64,
    size: f64,
    color: Color,
) {
    if size <= 1.0 {
        return;
    }

    let color_hex = format!(
        "#{:02x}{:02x}{:02x}",
        (color.r.clamp(0.0, 1.0) * 255.0) as u8,
        (color.g.clamp(0.0, 1.0) * 255.0) as u8,
        (color.b.clamp(0.0, 1.0) * 255.0) as u8
    );

    let svg = svg_source.replace("currentColor", &color_hex);
    let options = usvg::Options::default();
    let tree = match usvg::Tree::from_str(&svg, &options, fontdb()) {
        Ok(tree) => tree,
        Err(_) => return,
    };

    let size_px = size.ceil().max(1.0) as u32;
    let mut pixmap = match tiny_skia::Pixmap::new(size_px, size_px) {
        Some(pixmap) => pixmap,
        None => return,
    };

    let size = tree.size();
    let scale_x = size_px as f32 / size.width();
    let scale_y = size_px as f32 / size.height();
    let scale = scale_x.min(scale_y);
    let transform = tiny_skia::Transform::from_scale(scale, scale);
    let mut pixmap_mut = pixmap.as_mut();
    resvg::render(&tree, transform, &mut pixmap_mut);

    let mut surface = match ImageSurface::create(Format::ARgb32, size_px as i32, size_px as i32) {
        Ok(surface) => surface,
        Err(_) => return,
    };

    let stride = surface.stride() as usize;
    if let Ok(mut data) = surface.data() {
        let src = pixmap.data();
        let row_bytes = (size_px * 4) as usize;
        for row in 0..size_px as usize {
            let dst_range = row * stride..row * stride + row_bytes;
            let src_range = row * row_bytes..row * row_bytes + row_bytes;
            data[dst_range].copy_from_slice(&src[src_range]);
        }
    }

    cr.save().ok();
    cr.set_source_surface(&surface, x, y).ok();
    let _ = cr.paint();
    let _ = cr.restore();
}

pub fn draw_svg_icon(
    cr: &Context,
    icon: IconName,
    x: f64,
    y: f64,
    size: f64,
    color: Color,
) {
    draw_svg_source(cr, svg_source(icon), x, y, size, color);
}

pub fn draw_marker_svg_icon(
    cr: &Context,
    icon_text: &str,
    x: f64,
    y: f64,
    size: f64,
    color: Color,
) -> bool {
    let svg = match marker_svg_source(icon_text) {
        Some(svg) => svg,
        None => return false,
    };
    draw_svg_source(cr, svg, x, y, size, color);
    true
}

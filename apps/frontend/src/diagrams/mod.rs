pub mod architecture;
pub mod pipeline;

/// Straight edge between two points, drawn on the SVG layer under the nodes.
#[derive(Clone, Copy)]
pub struct Edge {
    pub from: (f64, f64),
    pub to: (f64, f64),
}

/// Anchor point on a node rect, mirroring ng-diagram's four named ports.
#[derive(Clone, Copy, PartialEq, Eq)]
pub enum Port {
    Left,
    Right,
    Top,
    Bottom,
}

pub fn port_point(x: f64, y: f64, w: f64, h: f64, port: Port) -> (f64, f64) {
    match port {
        Port::Left => (x, y + h / 2.0),
        Port::Right => (x + w, y + h / 2.0),
        Port::Top => (x + w / 2.0, y),
        Port::Bottom => (x + w / 2.0, y + h),
    }
}

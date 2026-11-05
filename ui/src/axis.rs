use amane::{Align, Column, Fill, Justify, Monitor, Padding, Rectangle, Row, Size, Widget};
use model::Edge;

use crate::card;

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub enum Axis {
    Horizontal,
    Vertical,
}

impl Axis {
    pub fn of(edge: Edge) -> Self {
        match edge {
            Edge::Top | Edge::Bottom => Self::Horizontal,
            Edge::Left | Edge::Right => Self::Vertical,
        }
    }

    pub fn is_horizontal(&self) -> bool {
        matches!(self, Self::Horizontal)
    }

    pub fn is_vertical(&self) -> bool {
        matches!(self, Self::Vertical)
    }

    pub fn main<T>(&self, h: T, v: T) -> T {
        match self {
            Self::Horizontal => h,
            Self::Vertical => v,
        }
    }

    pub fn cross<T>(&self, h: T, v: T) -> T {
        match self {
            Self::Horizontal => v,
            Self::Vertical => h,
        }
    }

    pub fn size<T>(&self, main: T, cross: T) -> (T, T) {
        match self {
            Self::Horizontal => (main, cross),
            Self::Vertical => (cross, main),
        }
    }

    /// Reorders (main, cross) into (horizontal, vertical).
    /// Used for alignment pairs and coordinate points.
    pub fn to_hv<T>(&self, main: T, cross: T) -> (T, T) {
        match self {
            Self::Horizontal => (main, cross),
            Self::Vertical => (cross, main),
        }
    }

    pub fn point<T>(&self, main: T, cross: T) -> (T, T) {
        self.to_hv(main, cross)
    }

    pub fn monitor_main(&self, monitor: &Monitor) -> f32 {
        self.main(monitor.width as f32, monitor.height as f32)
    }

    pub fn monitor_cross(&self, monitor: &Monitor) -> f32 {
        self.cross(monitor.width as f32, monitor.height as f32)
    }

    pub fn main_padding(&self, padding: Padding) -> f32 {
        let (a, b) = self.main((padding.left, padding.right), (padding.top, padding.bottom));
        a + b
    }

    pub fn cross_padding(&self, padding: Padding) -> f32 {
        let (a, b) = self.cross((padding.left, padding.right), (padding.top, padding.bottom));
        a + b
    }
}

pub enum Flow {
    Row(Row),
    Column(Column),
}

impl Flow {
    pub fn new(axis: Axis, children: Vec<Box<dyn Widget>>) -> Self {
        match axis {
            Axis::Horizontal => Self::Row(Row::new(children)),
            Axis::Vertical => Self::Column(Column::new(children)),
        }
    }

    pub fn gap(self, gap: f32) -> Self {
        match self {
            Self::Row(r) => Self::Row(r.gap(gap)),
            Self::Column(c) => Self::Column(c.gap(gap)),
        }
    }

    pub fn align(self, align: impl Into<Align>) -> Self {
        match self {
            Self::Row(r) => Self::Row(r.align(align)),
            Self::Column(c) => Self::Column(c.align(align)),
        }
    }

    pub fn justify(self, justify: impl Into<Justify>) -> Self {
        match self {
            Self::Row(r) => Self::Row(r.justify(justify)),
            Self::Column(c) => Self::Column(c.justify(justify)),
        }
    }

    pub fn cross_size(self, size: impl Into<Size>) -> Self {
        match self {
            Self::Row(r) => Self::Row(r.height(size)),
            Self::Column(c) => Self::Column(c.width(size)),
        }
    }

    pub fn main_size(self, size: impl Into<Size>) -> Self {
        match self {
            Self::Row(r) => Self::Row(r.width(size)),
            Self::Column(c) => Self::Column(c.height(size)),
        }
    }

    pub fn measured_main(&self) -> Size {
        match self {
            Self::Row(r) => Widget::width(r),
            Self::Column(c) => Widget::height(c),
        }
    }

    pub fn attach_to(self, rect: Rectangle) -> Rectangle {
        match self {
            Self::Row(r) => rect.child(r),
            Self::Column(c) => rect.child(c),
        }
    }

    pub fn into_card(
        self,
        padding: Padding,
        radius: f32,
        background: Option<impl Into<Fill>>,
    ) -> Rectangle {
        match self {
            Self::Row(r) => card::view(r, padding, radius, background),
            Self::Column(c) => card::view(c, padding, radius, background),
        }
    }

    pub fn into_widget(self) -> Box<dyn Widget> {
        match self {
            Self::Row(r) => Box::new(r),
            Self::Column(c) => Box::new(c),
        }
    }
}

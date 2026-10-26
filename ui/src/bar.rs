use amane::{Column, Horizontal, LayerWindow, Monitor, Row, Vertical, Widget, WindowSize};
use model::{MainAxisSize, bar::Bar};

use crate::section::section_view;

fn resolve_layer(layer: model::Layer) -> amane::Layer {
    match layer {
        model::Layer::Top => amane::Layer::Top,
        model::Layer::Background => amane::Layer::Background,
        model::Layer::Bottom => amane::Layer::Bottom,
        model::Layer::Overlay => amane::Layer::Overlay,
    }
}

fn resolve_anchors(bar: &Bar) -> (Vertical, Horizontal) {
    match bar.edge {
        model::Edge::Top => (Vertical::Top, Horizontal::Middle),
        model::Edge::Right => (Vertical::Middle, Horizontal::Right),
        model::Edge::Bottom => (Vertical::Bottom, Horizontal::Middle),
        model::Edge::Left => (Vertical::Middle, Horizontal::Left),
    }
}

fn main_axis_size(
    size: MainAxisSize,
    is_horizontal: bool,
    monitor: &Monitor,
    inner: &dyn Widget,
) -> WindowSize {
    match size {
        MainAxisSize::Auto => {
            let content = if is_horizontal {
                inner.width()
            } else {
                inner.height()
            };
            match content {
                amane::Size::Fixed(px) => WindowSize::Fixed(px),
                amane::Size::Parent => WindowSize::Full,
            }
        }
        MainAxisSize::Percent(pct) => {
            let span = if is_horizontal {
                monitor.width
            } else {
                monitor.height
            };
            WindowSize::Fixed(span as f32 * pct as f32 / 100.0)
        }
        MainAxisSize::Pixels(px) => {
            let cap = if is_horizontal {
                monitor.width
            } else {
                monitor.height
            };
            WindowSize::Fixed((px as f32).min(cap as f32))
        }
    }
}

fn resolve_margin(margin: model::Insets<u32>) -> amane::Margin {
    amane::Margin {
        top: margin.top as i32,
        right: margin.right as i32,
        bottom: margin.bottom as i32,
        left: margin.left as i32,
    }
}

pub fn bar_view(bar: &Bar, monitor: &Monitor) -> LayerWindow {
    let (vertical, horizontal) = resolve_anchors(bar);

    let font = bar.font.clone().unwrap_or_default();

    let is_bar_horizontal = Bar::is_horizontal(bar.edge);

    let margin = resolve_margin(bar.margin);

    let sections = bar
        .sections
        .iter()
        .map(|sec| section_view(sec, is_bar_horizontal, font.clone()))
        .collect();

    let cross = WindowSize::Fixed(bar.cross_axis_size as f32);

    let zone = if bar.exclusive_zone {
        amane::Zone::Reserve
    } else {
        amane::Zone::default()
    };

    if is_bar_horizontal {
        let row = Row::new(sections);

        let main = main_axis_size(bar.main_axis_size, is_bar_horizontal, monitor, &row);

        LayerWindow::new()
            .width(main)
            .height(cross)
            .layer(resolve_layer(bar.layer))
            .space(zone)
            .visible(bar.enabled)
            .margin(margin)
            .anchor_horizontal(horizontal)
            .anchor_vertical(vertical)
            .child(row)
    } else {
        let column = Column::new(sections);
        let main = main_axis_size(bar.main_axis_size, is_bar_horizontal, monitor, &column);

        LayerWindow::new()
            .width(cross)
            .height(main)
            .layer(resolve_layer(bar.layer))
            .space(zone)
            .visible(bar.enabled)
            .margin(margin)
            .anchor_horizontal(horizontal)
            .anchor_vertical(vertical)
            .child(column)
    }
}

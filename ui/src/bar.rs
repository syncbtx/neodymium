use amane::{
    Align, Color, Column, Horizontal, LayerWindow, Monitor, Padding, Rectangle, Row, Size,
    Vertical, Widget, WindowSize,
};
use model::{AttachmentMode, MainAxisSize, bar::Bar};

use crate::{color::resolve_color, section};

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

fn resolve_justify(justify: model::Justify) -> amane::Justify {
    match justify {
        model::Justify::Start => amane::Justify::Start,
        model::Justify::Center => amane::Justify::Center,
        model::Justify::End => amane::Justify::End,
        model::Justify::SpaceBetween => amane::Justify::SpaceBetween,
        model::Justify::SpaceAround => amane::Justify::SpaceAround,
        model::Justify::SpaceEvenly => amane::Justify::SpaceEvenly,
    }
}

struct BoundingInsets {
    top: f32,
    bottom: f32,
    left: f32,
    right: f32,
}

fn resolve_bounding_insets(mode: AttachmentMode, edge: model::Edge) -> BoundingInsets {
    match mode {
        AttachmentMode::Floating { margin } => {
            let m = margin as f32;
            match edge {
                model::Edge::Top | model::Edge::Bottom => BoundingInsets {
                    top: m,
                    bottom: m,
                    left: 0.0,
                    right: 0.0,
                },

                model::Edge::Left | model::Edge::Right => BoundingInsets {
                    top: 0.0,
                    bottom: 0.0,
                    left: m,
                    right: m,
                },
            }
        }
        AttachmentMode::Attached { .. } => BoundingInsets {
            top: 0.0,
            bottom: 0.0,
            left: 0.0,
            right: 0.0,
        },
    }
}

pub fn view(bar: &Bar, monitor: &Monitor) -> LayerWindow {
    let (vertical, horizontal) = resolve_anchors(bar);
    let is_bar_horizontal = Bar::is_horizontal(bar.edge);

    let bar_fill = if bar.islands {
        Color::TRANSPARENT
    } else {
        resolve_color(bar.background)
    };

    let sections = bar
        .sections
        .iter()
        .map(|sec| section::view(sec, bar))
        .collect();

    let zone = if bar.exclusive_zone {
        amane::Zone::Reserve
    } else {
        amane::Zone::default()
    };

    let insets = resolve_bounding_insets(bar.attachment_mode, bar.edge);
    let justify = resolve_justify(bar.sections_justify);

    let bar_padding = Padding {
        top: bar.padding.top as f32,
        right: bar.padding.right as f32,
        bottom: bar.padding.bottom as f32,
        left: bar.padding.left as f32,
    };

    if is_bar_horizontal {
        let bar_cross = bar.section_cross_axis_size as f32 + bar_padding.top + bar_padding.bottom;
        let window_cross = bar_cross + insets.top + insets.bottom;

        let mut row = Row::new(sections)
            .gap(12.0)
            .align(Align::Center)
            .justify(justify)
            .height(Size::Fixed(bar.section_cross_axis_size as f32));

        let (window_main, bar_main) = match bar.main_axis_size {
            MainAxisSize::Auto => {
                let measured = match Widget::width(&row) {
                    Size::Fixed(w) => w,
                    Size::Parent => monitor.width as f32,
                };
                let b_main = (measured + bar_padding.left + bar_padding.right)
                    .min((monitor.width as f32 - insets.left - insets.right).max(0.0));
                let w_main = b_main + insets.left + insets.right;
                (w_main, b_main)
            }
            MainAxisSize::Percent(pct) => {
                let w_main = (monitor.width as f32 * pct as f32 / 100.0).min(monitor.width as f32);
                let b_main = (w_main - insets.left - insets.right).max(0.0);
                row = row.width(Size::Parent);
                (w_main, b_main)
            }
            MainAxisSize::Pixels(px) => {
                let b_main =
                    (px as f32).min((monitor.width as f32 - insets.left - insets.right).max(0.0));
                let w_main = b_main + insets.left + insets.right;
                row = row.width(Size::Parent);
                (w_main, b_main)
            }
        };

        let bar_rect = Rectangle::new()
            .width(Size::Fixed(bar_main))
            .height(Size::Fixed(bar_cross))
            .padding(bar_padding)
            .align_child(Align::Center, Align::Center)
            .fill(bar_fill)
            .radius(bar.border_radius as f32)
            .child(row);

        let outer_envelope = Rectangle::new()
            .width(Size::Fixed(window_main))
            .height(Size::Fixed(window_cross))
            .padding(Padding {
                top: insets.top,
                bottom: insets.bottom,
                left: insets.left,
                right: insets.right,
            })
            .align_child(Align::Center, Align::Center)
            .fill(Color::TRANSPARENT)
            .child(bar_rect);

        LayerWindow::new()
            .width(WindowSize::Fixed(window_main))
            .height(WindowSize::Fixed(window_cross))
            .layer(resolve_layer(bar.layer))
            .space(zone)
            .visible(bar.enabled)
            .anchor_horizontal(horizontal)
            .anchor_vertical(vertical)
            .child(outer_envelope)
    } else {
        let bar_cross = bar.section_cross_axis_size as f32 + bar_padding.left + bar_padding.right;
        let window_cross = bar_cross + insets.left + insets.right;

        let mut column = Column::new(sections)
            .gap(12.0)
            .align(Align::Center)
            .justify(justify)
            .width(Size::Fixed(bar.section_cross_axis_size as f32));

        let (window_main, bar_main) = match bar.main_axis_size {
            MainAxisSize::Auto => {
                let measured = match Widget::height(&column) {
                    Size::Fixed(h) => h,
                    Size::Parent => monitor.height as f32,
                };
                let b_main = (measured + bar_padding.top + bar_padding.bottom)
                    .min((monitor.height as f32 - insets.top - insets.bottom).max(0.0));
                let w_main = b_main + insets.top + insets.bottom;
                (w_main, b_main)
            }
            MainAxisSize::Percent(pct) => {
                let w_main =
                    (monitor.height as f32 * pct as f32 / 100.0).min(monitor.height as f32);
                let b_main = (w_main - insets.top - insets.bottom).max(0.0);
                column = column.height(Size::Parent);
                (w_main, b_main)
            }
            MainAxisSize::Pixels(px) => {
                let b_main =
                    (px as f32).min((monitor.height as f32 - insets.top - insets.bottom).max(0.0));
                let w_main = b_main + insets.top + insets.bottom;
                column = column.height(Size::Parent);
                (w_main, b_main)
            }
        };

        let bar_rect = Rectangle::new()
            .width(Size::Fixed(bar_cross))
            .height(Size::Fixed(bar_main))
            .padding(bar_padding)
            .align_child(Align::Center, Align::Center)
            .fill(bar_fill)
            .radius(bar.border_radius as f32)
            .child(column);

        let outer_envelope = Rectangle::new()
            .width(Size::Fixed(window_cross))
            .height(Size::Fixed(window_main))
            .padding(Padding {
                top: insets.top,
                bottom: insets.bottom,
                left: insets.left,
                right: insets.right,
            })
            .align_child(Align::Center, Align::Center)
            .fill(Color::TRANSPARENT)
            .child(bar_rect);

        LayerWindow::new()
            .width(WindowSize::Fixed(window_cross))
            .height(WindowSize::Fixed(window_main))
            .layer(resolve_layer(bar.layer))
            .space(zone)
            .visible(bar.enabled)
            .anchor_horizontal(horizontal)
            .anchor_vertical(vertical)
            .child(outer_envelope)
    }
}

use amane::{
    Align, Color, Column, Horizontal, InputArea, LayerWindow, Monitor, Padding, Rectangle, Row,
    Size, Vertical, Widget, WindowSize,
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
    let h_anchor = |anchor: model::Anchor| -> Horizontal {
        match anchor {
            model::Anchor::Start => Horizontal::Left,
            model::Anchor::Center => Horizontal::Middle,
            model::Anchor::End => Horizontal::Right,
        }
    };

    let v_anchor = |anchor: model::Anchor| -> Vertical {
        match anchor {
            model::Anchor::Start => Vertical::Top,
            model::Anchor::Center => Vertical::Middle,
            model::Anchor::End => Vertical::Bottom,
        }
    };

    match bar.edge {
        model::Edge::Top => (Vertical::Top, h_anchor(bar.anchor)),
        model::Edge::Right => (v_anchor(bar.anchor), Horizontal::Right),
        model::Edge::Bottom => (Vertical::Bottom, h_anchor(bar.anchor)),
        model::Edge::Left => (v_anchor(bar.anchor), Horizontal::Left),
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

fn resolve_main_align(anchor: model::Anchor) -> Align {
    match anchor {
        model::Anchor::Start => Align::Start,
        model::Anchor::Center => Align::Center,
        model::Anchor::End => Align::End,
    }
}

fn main_axis_offset(anchor: model::Anchor, span: f32, len: f32) -> f32 {
    match anchor {
        model::Anchor::Start => 0.0,
        model::Anchor::Center => ((span - len) / 2.0).max(0.0),
        model::Anchor::End => (span - len).max(0.0),
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

fn trim_padding(edge: model::Edge, padding: Padding) -> Padding {
    let mut padding = padding;
    match edge {
        model::Edge::Top => padding.bottom = 0.0,
        model::Edge::Right => padding.left = 0.0,
        model::Edge::Bottom => padding.top = 0.0,
        model::Edge::Left => padding.right = 0.0,
    }
    padding
}

pub fn view(bar: &Bar, monitor: &Monitor) -> LayerWindow {
    let (vertical, horizontal) = resolve_anchors(bar);
    let is_bar_horizontal = Bar::is_horizontal(bar.edge);
    let is_attached = matches!(bar.attachment_mode, AttachmentMode::Attached { .. });

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

    // let bar_padding = if bar.islands {
    //     if is_bar_horizontal {
    //         Padding {
    //             top: 0.0,
    //             right: bar.padding.right as f32,
    //             bottom: 0.0,
    //             left: bar.padding.left as f32,
    //         }
    //     } else {
    //         Padding {
    //             top: bar.padding.top as f32,
    //             right: 0.0,
    //             bottom: bar.padding.bottom as f32,
    //             left: 0.0,
    //         }
    //     }
    // } else {
    //     Padding {
    //         top: bar.padding.top as f32,
    //         right: bar.padding.right as f32,
    //         bottom: bar.padding.bottom as f32,
    //         left: bar.padding.left as f32,
    //     }
    // };

    let padding = Padding {
        top: bar.padding.top as f32,
        right: bar.padding.right as f32,
        bottom: bar.padding.bottom as f32,
        left: bar.padding.left as f32,
    };

    let bar_padding = if is_attached && bar.islands {
        trim_padding(bar.edge, padding)
    } else {
        padding
    };

    if is_bar_horizontal {
        let bar_cross = bar.section_cross_axis_size as f32 + bar_padding.top + bar_padding.bottom;
        let window_cross = bar_cross + insets.top + insets.bottom;

        let mut row = Row::new(sections)
            .gap(bar.spacing as f32)
            .align(Align::Center)
            .justify(justify)
            .height(Size::Fixed(bar.section_cross_axis_size as f32));

        let (window_main_axis, bar_main_axis, bar_main_px) = match bar.main_axis_size {
            MainAxisSize::Auto => {
                let measured = match Widget::width(&row) {
                    Size::Fixed(w) => w,
                    Size::Parent => monitor.width as f32,
                };
                let b_main = (measured + bar_padding.left + bar_padding.right)
                    .min((monitor.width as f32 - insets.left - insets.right).max(0.0));
                let w_main = b_main + insets.left + insets.right;
                (WindowSize::Fixed(w_main), Size::Fixed(b_main), Some(b_main))
            }
            MainAxisSize::Percent(pct) if pct >= 100 => {
                row = row.width(Size::Parent);
                (WindowSize::Full, Size::Parent, None)
            }
            MainAxisSize::Percent(pct) => {
                let b_main = (monitor.width as f32 * pct as f32 / 100.0)
                    .min((monitor.width as f32 - insets.left - insets.right).max(0.0));
                let w_main = (b_main + insets.left + insets.right).min(monitor.width as f32);
                row = row.width(Size::Parent);
                (WindowSize::Fixed(w_main), Size::Fixed(b_main), Some(b_main))
            }
            MainAxisSize::Pixels(px) => {
                let b_main =
                    (px as f32).min((monitor.width as f32 - insets.left - insets.right).max(0.0));
                let w_main = b_main + insets.left + insets.right;
                row = row.width(Size::Parent);
                (WindowSize::Fixed(w_main), Size::Fixed(b_main), Some(b_main))
            }
        };

        // A corner-anchored surface (Start/End) has no deducible exclusive edge, so the
        // compositor drops the zone. Make the window span the whole edge instead, which
        // anchors three edges, and position the bar inside it.
        let spanning = bar.exclusive_zone && bar_main_px.is_some();
        let window_main_axis = if spanning {
            WindowSize::Full
        } else {
            window_main_axis
        };

        let envelope_width = match window_main_axis {
            WindowSize::Full => Size::Parent,
            WindowSize::Fixed(w) => Size::Fixed(w),
        };

        let main_align = if spanning {
            resolve_main_align(bar.anchor)
        } else {
            Align::Center
        };

        let bar_rect = Rectangle::new()
            .width(bar_main_axis)
            .height(Size::Fixed(bar_cross))
            .padding(bar_padding)
            .align_child(Align::Center, Align::Center)
            .fill(bar_fill)
            .radius(bar.border_radius as f32)
            .child(row);

        let outer_envelope = Rectangle::new()
            .width(envelope_width)
            .height(Size::Fixed(window_cross))
            .padding(Padding {
                top: insets.top,
                bottom: insets.bottom,
                left: insets.left,
                right: insets.right,
            })
            .align_child(main_align, Align::Center)
            .fill(Color::TRANSPARENT)
            .child(bar_rect);

        // Only the bar itself takes the mouse; the transparent rest of the window doesn't.
        let input_area = match (spanning, bar_main_px) {
            (true, Some(len)) => Some(InputArea {
                x: main_axis_offset(bar.anchor, monitor.width as f32, len) as i32,
                y: insets.top as i32,
                width: len as i32,
                height: bar_cross as i32,
            }),
            _ => None,
        };

        let window = LayerWindow::new()
            .width(window_main_axis)
            .height(WindowSize::Fixed(window_cross))
            .layer(resolve_layer(bar.layer))
            .space(zone)
            .visible(bar.enabled)
            .anchor_horizontal(horizontal)
            .anchor_vertical(vertical);

        let window = match input_area {
            Some(area) => window.input_region(vec![area]),
            None => window,
        };

        window.child(outer_envelope)
    } else {
        let bar_cross = bar.section_cross_axis_size as f32 + bar_padding.left + bar_padding.right;
        let window_cross = bar_cross + insets.left + insets.right;

        let mut column = Column::new(sections)
            .gap(bar.spacing as f32)
            .align(Align::Center)
            .justify(justify)
            .width(Size::Fixed(bar.section_cross_axis_size as f32));

        let (window_main_axis, bar_main_axis, bar_main_px) = match bar.main_axis_size {
            MainAxisSize::Auto => {
                let measured = match Widget::height(&column) {
                    Size::Fixed(h) => h,
                    Size::Parent => monitor.height as f32,
                };
                let b_main = (measured + bar_padding.top + bar_padding.bottom)
                    .min((monitor.height as f32 - insets.top - insets.bottom).max(0.0));
                let w_main = b_main + insets.top + insets.bottom;
                (WindowSize::Fixed(w_main), Size::Fixed(b_main), Some(b_main))
            }
            MainAxisSize::Percent(pct) if pct >= 100 => {
                column = column.height(Size::Parent);
                (WindowSize::Full, Size::Parent, None)
            }
            MainAxisSize::Percent(pct) => {
                let b_main = (monitor.height as f32 * pct as f32 / 100.0)
                    .min((monitor.height as f32 - insets.top - insets.bottom).max(0.0));
                let w_main = (b_main + insets.top + insets.bottom).min(monitor.height as f32);
                column = column.height(Size::Parent);
                (WindowSize::Fixed(w_main), Size::Fixed(b_main), Some(b_main))
            }
            MainAxisSize::Pixels(px) => {
                let b_main =
                    (px as f32).min((monitor.height as f32 - insets.top - insets.bottom).max(0.0));
                let w_main = b_main + insets.top + insets.bottom;
                column = column.height(Size::Parent);
                (WindowSize::Fixed(w_main), Size::Fixed(b_main), Some(b_main))
            }
        };

        let spanning = bar.exclusive_zone && bar_main_px.is_some();
        let window_main_axis = if spanning {
            WindowSize::Full
        } else {
            window_main_axis
        };

        let envelope_height = match window_main_axis {
            WindowSize::Full => Size::Parent,
            WindowSize::Fixed(h) => Size::Fixed(h),
        };

        let main_align = if spanning {
            resolve_main_align(bar.anchor)
        } else {
            Align::Center
        };

        let bar_rect = Rectangle::new()
            .width(Size::Fixed(bar_cross))
            .height(bar_main_axis)
            .padding(bar_padding)
            .align_child(Align::Center, Align::Center)
            .fill(bar_fill)
            .radius(bar.border_radius as f32)
            .child(column);

        let outer_envelope = Rectangle::new()
            .width(Size::Fixed(window_cross))
            .height(envelope_height)
            .padding(Padding {
                top: insets.top,
                bottom: insets.bottom,
                left: insets.left,
                right: insets.right,
            })
            .align_child(Align::Center, main_align)
            .fill(Color::TRANSPARENT)
            .child(bar_rect);

        let input_area = match (spanning, bar_main_px) {
            (true, Some(len)) => Some(InputArea {
                x: insets.left as i32,
                y: main_axis_offset(bar.anchor, monitor.height as f32, len) as i32,
                width: bar_cross as i32,
                height: len as i32,
            }),
            _ => None,
        };

        let window = LayerWindow::new()
            .width(WindowSize::Fixed(window_cross))
            .height(window_main_axis)
            .layer(resolve_layer(bar.layer))
            .space(zone)
            .visible(bar.enabled)
            .anchor_horizontal(horizontal)
            .anchor_vertical(vertical);

        let window = match input_area {
            Some(area) => window.input_region(vec![area]),
            None => window,
        };

        window.child(outer_envelope)
    }
}

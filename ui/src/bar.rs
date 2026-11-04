use amane::{
    Align, Color, Horizontal, InputArea, LayerWindow, Monitor, Padding, Rectangle, Service, Size,
    Vertical, WindowSize,
};
use model::{AttachmentMode, MainAxisSize, bar::Bar};

use crate::{
    axis::{Axis, Flow},
    section,
    services::Theme,
};

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

impl BoundingInsets {
    fn main(&self, axis: Axis) -> f32 {
        match axis {
            Axis::Horizontal => self.left + self.right,
            Axis::Vertical => self.top + self.bottom,
        }
    }

    fn cross(&self, axis: Axis) -> f32 {
        match axis {
            Axis::Horizontal => self.top + self.bottom,
            Axis::Vertical => self.left + self.right,
        }
    }

    fn cross_start(&self, axis: Axis) -> f32 {
        match axis {
            Axis::Horizontal => self.top,
            Axis::Vertical => self.left,
        }
    }
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

fn trim_inner_padding(edge: model::Edge, padding: Padding) -> Padding {
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
    let axis = Axis::of(bar.edge);
    let (vertical, horizontal) = resolve_anchors(bar);
    let is_attached = matches!(bar.attachment_mode, AttachmentMode::Attached { .. });

    let theme = Theme::read();

    let opacity = theme.opacity();

    let palette = theme.active();

    let bar_fill = if bar.islands {
        Color::TRANSPARENT
    } else {
        let eff = ((bar.opacity.0 as u16 * opacity as u16) / 255) as u8;
        crate::color::resolve_color_with_alpha(palette.surface, eff)
    };

    let sections = bar
        .sections
        .iter()
        .map(|sec| section::view(sec, bar, palette, opacity))
        .collect();

    let zone = if bar.exclusive_zone {
        amane::Zone::Reserve
    } else {
        amane::Zone::default()
    };

    let insets = resolve_bounding_insets(bar.attachment_mode, bar.edge);
    let justify = resolve_justify(bar.sections_justify);

    let padding = Padding {
        top: bar.padding.top as f32,
        right: bar.padding.right as f32,
        bottom: bar.padding.bottom as f32,
        left: bar.padding.left as f32,
    };

    let bar_padding = if is_attached && bar.islands {
        trim_inner_padding(bar.edge, padding)
    } else {
        padding
    };

    let bar_cross = bar.section_cross_axis_size as f32 + axis.cross_padding(bar_padding);
    let window_cross = bar_cross + insets.cross(axis);

    let mut container = Flow::new(axis, sections)
        .gap(bar.spacing as f32)
        .align(Align::Center)
        .justify(justify)
        .cross_size(Size::Fixed(bar.section_cross_axis_size as f32));

    let monitor_main = axis.monitor_main(monitor);
    let insets_main = insets.main(axis);
    let bar_padding_main = axis.main_padding(bar_padding);

    let (window_main_axis, bar_main_axis, bar_main_px) = match bar.main_axis_size {
        MainAxisSize::Auto => {
            let measured = match container.measured_main() {
                Size::Fixed(w) => w,
                Size::Parent => monitor_main,
            };
            let b_main = (measured + bar_padding_main).min((monitor_main - insets_main).max(0.0));
            let w_main = b_main + insets_main;
            (WindowSize::Fixed(w_main), Size::Fixed(b_main), Some(b_main))
        }
        MainAxisSize::Percent(pct) if pct >= 100 => {
            container = container.main_size(Size::Parent);
            (WindowSize::Full, Size::Parent, None)
        }
        MainAxisSize::Percent(pct) => {
            let b_main =
                (monitor_main * pct as f32 / 100.0).min((monitor_main - insets_main).max(0.0));
            let w_main = (b_main + insets_main).min(monitor_main);
            container = container.main_size(Size::Parent);
            (WindowSize::Fixed(w_main), Size::Fixed(b_main), Some(b_main))
        }
        MainAxisSize::Pixels(px) => {
            let b_main = (px as f32).min((monitor_main - insets_main).max(0.0));
            let w_main = b_main + insets_main;
            container = container.main_size(Size::Parent);
            (WindowSize::Fixed(w_main), Size::Fixed(b_main), Some(b_main))
        }
    };

    let spanning = bar.exclusive_zone && bar_main_px.is_some();
    let window_main_axis = if spanning {
        WindowSize::Full
    } else {
        window_main_axis
    };

    let envelope_main = match window_main_axis {
        WindowSize::Full => Size::Parent,
        WindowSize::Fixed(w) => Size::Fixed(w),
    };

    let main_align = if spanning {
        resolve_main_align(bar.anchor)
    } else {
        Align::Center
    };

    let (bar_w, bar_h) = axis.size(bar_main_axis, Size::Fixed(bar_cross));
    let bar_rect = Rectangle::new()
        .width(bar_w)
        .height(bar_h)
        .padding(bar_padding)
        .align_child(Align::Center, Align::Center)
        .fill(bar_fill)
        .radius(bar.border_radius as f32);
    let bar_rect = container.attach_to(bar_rect);

    let (env_w, env_h) = axis.size(envelope_main, Size::Fixed(window_cross));
    let (env_align_h, env_align_v) = axis.point(main_align, Align::Center);
    let outer_envelope = Rectangle::new()
        .width(env_w)
        .height(env_h)
        .padding(Padding {
            top: insets.top,
            bottom: insets.bottom,
            left: insets.left,
            right: insets.right,
        })
        .align_child(env_align_h, env_align_v)
        .fill(Color::TRANSPARENT)
        .child(bar_rect);

    let input_area = match (spanning, bar_main_px) {
        (true, Some(len)) => {
            let (x, y) = axis.point(
                main_axis_offset(bar.anchor, monitor_main, len) as i32,
                insets.cross_start(axis) as i32,
            );
            let (w, h) = axis.size(len as i32, bar_cross as i32);
            Some(InputArea {
                x,
                y,
                width: w,
                height: h,
            })
        }
        _ => None,
    };

    let (win_w, win_h) = axis.size(window_main_axis, WindowSize::Fixed(window_cross));
    let window = LayerWindow::new()
        .width(win_w)
        .height(win_h)
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

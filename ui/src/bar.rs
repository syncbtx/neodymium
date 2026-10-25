use std::sync::OnceLock;

use amane::{
    Center, Color, Column, End, Full, Horizontal, Layer, LayerWindow, Margin, Monitor, Padding,
    Parent, Rectangle, Row, Service, Size, Start, Text, Vertical, Widget, WindowSize, Workspace,
    Workspaces, Zone,
};
use configuration::{
    settings::bars::{AttachmentMode, BarLayer, MajorAxisSize, ScreenCornerClaimer},
    shared::Edge,
};
use model::bar::{Bar, Bars, module::BarModule, section::BarSection};

use crate::color::hex_to_color;

static BARS: OnceLock<Bars> = OnceLock::new();

pub fn init(bars: Bars) {
    BARS.set(bars).ok();
}

// The compositor lays surfaces out in creation order, each one inside what
// the earlier ones left free. So whoever is created first owns the corners:
// the claiming bars must come first.
pub fn views() -> Vec<fn(&Monitor) -> LayerWindow> {
    let Some(bars) = BARS.get() else {
        return Vec::new();
    };

    let order = match bars.screen_corner_claimer {
        ScreenCornerClaimer::HorizontalBars => [Edge::Top, Edge::Bottom, Edge::Left, Edge::Right],
        ScreenCornerClaimer::VerticalBars => [Edge::Left, Edge::Right, Edge::Top, Edge::Bottom],
    };

    order
        .into_iter()
        .filter(|edge| bar_for(bars, *edge).enabled)
        .map(view_for)
        .collect()
}

fn view_for(edge: Edge) -> fn(&Monitor) -> LayerWindow {
    match edge {
        Edge::Top => top,
        Edge::Right => right,
        Edge::Bottom => bottom,
        Edge::Left => left,
    }
}

fn top(monitor: &Monitor) -> LayerWindow {
    render(Edge::Top, monitor)
}

fn right(monitor: &Monitor) -> LayerWindow {
    render(Edge::Right, monitor)
}

fn bottom(monitor: &Monitor) -> LayerWindow {
    render(Edge::Bottom, monitor)
}

fn left(monitor: &Monitor) -> LayerWindow {
    render(Edge::Left, monitor)
}

fn hidden() -> LayerWindow {
    LayerWindow::new().width(0.0).height(0.0).visible(false)
}

fn bar_for(bars: &Bars, edge: Edge) -> &Bar {
    match edge {
        Edge::Top => &bars.top,
        Edge::Right => &bars.right,
        Edge::Bottom => &bars.bottom,
        Edge::Left => &bars.left,
    }
}

fn render(edge: Edge, monitor: &Monitor) -> LayerWindow {
    let Some(bars) = BARS.get() else {
        return hidden();
    };
    let bar = bar_for(bars, edge);
    if !bar.enabled || bar.autohide {
        return hidden();
    }

    let is_horizontal = Bar::is_horizontal(edge);
    let m = &bar.margin;

    // Floating: the minor-axis margin applies equally to both sides (outer
    // edge side, and the inner side amane adds to the reserved zone, i.e. the
    // gap to windows). Only the edge-side value is read from the model, since
    // the model may not fill the opposite side. Attached: flush, both zero.
    // Major-axis margins apply in both modes.
    let floating = matches!(bar.attachment_mode, AttachmentMode::Floating);
    let minor_margin = match edge {
        Edge::Top => m.top,
        Edge::Bottom => m.bottom,
        Edge::Left => m.left,
        Edge::Right => m.right,
    } as i32;
    // The zone must not depend on amane's version: released amane reserves
    // only the window thickness (niri adds the edge margin), newer amane also
    // adds the margin facing the windows. So keep the facing margin at 0 and
    // put the gap inside the window instead: the window is `gap` thicker than
    // the bar, with the bar drawn at the screen-edge side, so the reserved
    // zone is edge margin + bar + gap either way.
    let floating = matches!(bar.attachment_mode, AttachmentMode::Floating);
    let minor_margin = match edge {
        Edge::Top => m.top,
        Edge::Bottom => m.bottom,
        Edge::Left => m.left,
        Edge::Right => m.right,
    } as i32;
    let (edge_margin, inner_margin, gap) = if floating {
        (minor_margin, 0, minor_margin)
    } else {
        (0, 0, 0)
    };

    let (start_margin, end_margin) = if is_horizontal {
        (m.left as i32, m.right as i32)
    } else {
        (m.top as i32, m.bottom as i32)
    };
    let (start, end) = (start_margin, end_margin);

    // Full on the major axis makes amane anchor both ends (three edges), so
    // the compositor applies the zone and both major-axis margins, and the
    // window spans whatever the earlier (corner-claiming) bars left free.
    let (margin, vertical, horizontal) = match edge {
        Edge::Top => (
            Margin {
                top: edge_margin,
                right: end,
                bottom: inner_margin,
                left: start,
            },
            Vertical::Top,
            Horizontal::Middle,
        ),
        Edge::Bottom => (
            Margin {
                top: inner_margin,
                right: end,
                bottom: edge_margin,
                left: start,
            },
            Vertical::Bottom,
            Horizontal::Middle,
        ),
        Edge::Left => (
            Margin {
                top: start,
                right: inner_margin,
                bottom: end,
                left: edge_margin,
            },
            Vertical::Middle,
            Horizontal::Left,
        ),
        Edge::Right => (
            Margin {
                top: start,
                right: edge_margin,
                bottom: end,
                left: inner_margin,
            },
            Vertical::Middle,
            Horizontal::Right,
        ),
    };

    let thickness = bar.minor_axis_size.get() as f32;
    let window_thickness = thickness + gap as f32;
    let (width, height): (WindowSize, WindowSize) = if is_horizontal {
        (Full.into(), window_thickness.into())
    } else {
        (window_thickness.into(), Full.into())
    };

    let zone = if bar.exclusive_zone {
        Zone::Reserve
    } else {
        Zone::Respect
    };

    let bg = hex_to_color(bar.background);
    // config opacity is 0..=255, amane's Rectangle::opacity is 0.0..=1.0 and
    // not clamped when the layer is composited, so 255.0 blows colors out to white
    let opacity = (bar.opacity.0 as f32 / 255.0).clamp(0.0, 1.0);
    let radius = bar.border_radius as f32;
    let pad_h = bar.padding.left as f32;
    let pad_v = bar.padding.top as f32;

    let sections: Vec<Box<dyn Widget>> = bar
        .sections
        .iter()
        .map(|s| {
            Box::new(section_widget(s, bar.section_border_radius, is_horizontal)) as Box<dyn Widget>
        })
        .collect();

    let window = LayerWindow::new()
        .width(width)
        .height(height)
        .layer(resolve_layer(bar))
        .anchor_vertical(vertical)
        .anchor_horizontal(horizontal)
        .margin(margin)
        .space(zone)
        .namespace("neodymium");

    let (bar_w, bar_h) = if is_horizontal {
        (
            resolve_major_axis_size(&bar.major_axis_size, monitor, is_horizontal),
            Size::Fixed(thickness),
        )
    } else {
        (
            Size::Fixed(thickness),
            resolve_major_axis_size(&bar.major_axis_size, monitor, is_horizontal),
        )
    };

    let mut bar_rect = Rectangle::new()
        .width(bar_w)
        .height(bar_h)
        .fill(bg)
        .opacity(opacity)
        .radius(radius)
        .padding(Padding {
            top: pad_v,
            right: pad_h,
            bottom: pad_v,
            left: pad_h,
        })
        .align_child(Center, Center);

    if bar.border.width > 0 {
        bar_rect = bar_rect.border(bar.border.width as f32, hex_to_color(bar.border.color));
    }

    if !sections.is_empty() {
        if is_horizontal {
            bar_rect = bar_rect.child(Row::new(sections).gap(0.0));
        } else {
            bar_rect = bar_rect.child(Column::new(sections).gap(0.0));
        }
    }

    let root = Rectangle::new()
        .width(Parent)
        .height(Parent)
        .fill(Color::TRANSPARENT)
        .child(bar_rect);
    let root = match edge {
        Edge::Top => root.align_child(Center, Start),
        Edge::Bottom => root.align_child(Center, End),
        Edge::Left => root.align_child(Start, Center),
        Edge::Right => root.align_child(End, Center),
    };

    window.child(root)
}

fn resolve_layer(bar: &Bar) -> Layer {
    match bar.layer {
        BarLayer::Top => Layer::Top,
        BarLayer::Bottom => Layer::Bottom,
        BarLayer::Overlay => Layer::Overlay,
    }
}

fn resolve_major_axis_size(size: &MajorAxisSize, monitor: &Monitor, is_horizontal: bool) -> Size {
    match size {
        MajorAxisSize::Auto => Parent,
        MajorAxisSize::Pixels(px) => (*px as f32).into(),
        MajorAxisSize::Percent(pct) => {
            let dim = if is_horizontal {
                monitor.width
            } else {
                monitor.height
            };
            let px = (dim as f32 * (*pct as f32 / 100.0)).round();
            px.into()
        }
    }
}

fn section_widget(section: &BarSection, fallback_radius: u32, is_horizontal: bool) -> Rectangle {
    let bg = section
        .background
        .map(hex_to_color)
        .unwrap_or(Color::TRANSPARENT);

    let radius = section.border_radius.unwrap_or(fallback_radius) as f32;
    let pad_h = section.major_axis_padding as f32;
    let pad_v = section.minor_axis_padding as f32;

    let modules: Vec<Box<dyn Widget>> = section
        .modules
        .iter()
        .map(|m| build_module(m, is_horizontal))
        .collect();

    let mut rect = if is_horizontal {
        let row = Row::new(modules).gap(section.spacing as f32);
        let width = match Widget::width(&row) {
            Size::Fixed(w) => Size::Fixed(w + pad_h * 2.0),
            Size::Parent => Size::Parent,
        };
        Rectangle::new()
            .width(width)
            .height(Parent)
            .fill(bg)
            .radius(radius)
            .align_child(Center, Center)
            .padding(Padding {
                top: pad_v,
                right: pad_h,
                bottom: pad_v,
                left: pad_h,
            })
            .child(row)
    } else {
        let col = Column::new(modules).gap(section.spacing as f32);
        let height = match Widget::height(&col) {
            Size::Fixed(h) => Size::Fixed(h + pad_v * 2.0),
            Size::Parent => Size::Parent,
        };
        Rectangle::new()
            .width(Parent)
            .height(height)
            .fill(bg)
            .radius(radius)
            .align_child(Center, Center)
            .padding(Padding {
                top: pad_v,
                right: pad_h,
                bottom: pad_v,
                left: pad_h,
            })
            .child(col)
    };

    if section.border.width > 0 {
        rect = rect.border(
            section.border.width as f32,
            hex_to_color(section.border.color),
        );
    }

    rect
}

fn build_module(module: &BarModule, is_horizontal: bool) -> Box<dyn Widget> {
    match module {
        BarModule::Spacer(gap) => {
            let gap = *gap as f32;
            let (w, h) = if is_horizontal {
                (Size::Fixed(gap), Size::Parent)
            } else {
                (Size::Parent, Size::Fixed(gap))
            };
            Box::new(Rectangle::new().width(w).height(h).fill(Color::TRANSPARENT))
        }
        BarModule::Workspaces(_) => workspaces_widget(is_horizontal),
        BarModule::WindowTitle(_) => Box::new(
            Rectangle::new()
                .width(0.0)
                .height(0.0)
                .fill(Color::TRANSPARENT),
        ),
    }
}

fn workspaces_widget(is_horizontal: bool) -> Box<dyn Widget> {
    let workspaces = Workspaces::read();

    let buttons: Vec<Box<dyn Widget>> = workspaces
        .list()
        .iter()
        .map(|w| Box::new(workspace_button(w, is_horizontal)) as Box<dyn Widget>)
        .collect();

    if is_horizontal {
        Box::new(Row::new(buttons).gap(4.0))
    } else {
        Box::new(Column::new(buttons).gap(4.0))
    }
}

fn workspace_button(workspace: &Workspace, is_horizontal: bool) -> Rectangle {
    let fill = if workspace.focused() {
        Color::rgba(0x89, 0xb4, 0xfa, 255)
    } else if workspace.urgent() {
        Color::rgba(0xf3, 0x8b, 0xa8, 255)
    } else {
        Color::rgba(0x31, 0x32, 0x44, 255)
    };

    let id = workspace.id();
    let (w, h) = if is_horizontal {
        (Size::Fixed(30.0), Size::Parent)
    } else {
        (Size::Parent, Size::Fixed(30.0))
    };

    Rectangle::new()
        .width(w)
        .height(h)
        .fill(fill)
        .radius(6.0)
        .align_child(Center, Center)
        .on_click(move |_| Workspaces::focus(id))
        .child(Text::new(workspace.index().to_string()).color(Color::WHITE))
}

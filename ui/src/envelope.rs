use amane::{Align, Color, Fill, Padding, Rectangle, Widget};

pub fn view(
    inner: impl Widget + 'static,
    padding: Padding,
    radius: f32,
    background: Option<impl Into<Fill>>,
) -> Rectangle {
    let content_width = inner.width();
    let content_height = inner.height();

    let width = match content_width {
        amane::Size::Parent => amane::Size::Parent,
        amane::Size::Fixed(px) => amane::Size::Fixed(px + padding.left + padding.right),
    };

    let height = match content_height {
        amane::Size::Parent => amane::Size::Parent,
        amane::Size::Fixed(px) => amane::Size::Fixed(px + padding.top + padding.bottom),
    };

    let fill: Fill = match background {
        Some(fill) => fill.into(),
        None => Fill::Color(Color::TRANSPARENT),
    };

    Rectangle::new()
        .width(width)
        .height(height)
        .padding(padding)
        .radius(radius)
        .align_child(Align::Center, Align::Center)
        .fill(fill)
        .child(inner)
}

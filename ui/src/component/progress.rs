use amane::{Align, Color, Rectangle, Widget};

pub enum ProgressStyle {
    Bar,
    Ring,
    WaveyRing,
}

pub fn view(
    is_horizontal: bool,
    ratio: f32, // expected 0.0..=1.0
    style: ProgressStyle,
    length: f32,
    thickness: f32,
    radius: f32,
    track_color: Color,
    fill_color: Color,
) -> Box<dyn Widget> {
    let ratio = ratio.clamp(0.0, 1.0);

    let widget = match style {
        ProgressStyle::Bar => {
            let (total_w, total_h, inner_w, inner_h, align_h, align_v) = if is_horizontal {
                (
                    length,
                    thickness,
                    length * ratio,
                    thickness,
                    Align::Start,
                    Align::Center,
                )
            } else {
                (
                    thickness,
                    length,
                    thickness,
                    length * ratio,
                    Align::Center,
                    Align::End,
                )
            };

            Rectangle::new()
                .width(total_w)
                .height(total_h)
                .radius(radius)
                .fill(track_color)
                .align_child(align_h, align_v)
                .child(
                    Rectangle::new()
                        .width(inner_w)
                        .height(inner_h)
                        .radius(radius)
                        .fill(fill_color),
                )
        }
        ProgressStyle::Ring => todo!("Ring style not yet implemented"),
        ProgressStyle::WaveyRing => todo!("WaveyRing style not yet implemented"),
    };

    Box::new(widget) as Box<dyn Widget>
}

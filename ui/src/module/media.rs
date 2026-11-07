use amane::{Center, Media, Rectangle, Service, Text, Widget};
use hhmmss::Hhmmss;

use crate::{Flow, color::resolve_color, module::ModuleViewArgs};

pub fn view(module_view_args: &ModuleViewArgs) -> Box<dyn Widget> {
    let ModuleViewArgs {
        font,
        font_size,
        colors,
        thickness,
        palette,
        axis,
        radius,
        padding,
        ..
    } = *module_view_args;

    let media = Media::read();

    let text = match media.active() {
        Some(player) => {
            let info = if player.playing() {
                format!(
                    "{} / {}",
                    player.position().hhmmss(),
                    player.length().hhmmss()
                )
            } else {
                "Nothing is playing".to_string()
            };
            Text::new(&info)
                .font(&font.path_or_name)
                .size(font_size)
                .weight(font.weight.value())
                .color(resolve_color(colors.foreground))
        }
        None => Text::new("Nothing is playing")
            .font(&font.path_or_name)
            .size(font_size)
            .weight(font.weight.value())
            .color(resolve_color(colors.foreground)),
    };

    let width = text.width();
    let surface = resolve_color(palette.surface_container);
    Box::new(
        Flow::new(
            axis,
            vec![Box::new(
                Rectangle::new()
                    .width(width)
                    .height(thickness)
                    .child(text)
                    .fill(surface)
                    .align_child(Center, Center),
            ) as Box<dyn Widget>],
        )
        .into_envelope(padding, radius, Some(surface)),
    )
}

use amane::{Center, Rectangle, Service, Text, Widget};

use crate::{
    color::resolve_color,
    constants::{BLUETOOTH_OFF_ICON, BLUETOOTH_ON_ICON},
    module::ModuleViewArgs,
};

pub fn view(module_view_args: &ModuleViewArgs) -> Box<dyn Widget> {
    let ModuleViewArgs {
        thickness,
        palette,
        font,
        font_size,
        font_weight,
        ..
    } = *module_view_args;

    let bluetooth = amane::Bluetooth::read();
    let on = bluetooth.powered();

    let (icon, color) = if on {
        (BLUETOOTH_ON_ICON, resolve_color(palette.primary))
    } else {
        (BLUETOOTH_OFF_ICON, resolve_color(palette.on_surface))
    };

    Box::new(
        Rectangle::new()
            .width(thickness)
            .height(thickness)
            .align_child(Center, Center)
            .child(
                Text::new(icon)
                    .font(&font.icon_path_or_name)
                    .color(color)
                    .size(font_size)
                    .weight(font_weight)
                    .tight(),
            )
            .on_click(|_| {}),
    ) as Box<dyn Widget>
}

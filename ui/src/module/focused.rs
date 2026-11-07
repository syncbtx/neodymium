use amane::{Rectangle, Service, Widget};

use crate::{color::resolve_color, module::ModuleViewArgs, service};

pub fn view(module_view_args: &ModuleViewArgs) -> Box<dyn Widget> {
    let ModuleViewArgs {
        thickness, colors, ..
    } = *module_view_args;

    let focused = service::FocusedWindow::read();

    let inner = match focused.icon.clone() {
        Some(icon_path) => Rectangle::new()
            .width(thickness)
            .height(thickness)
            .fill(amane::Image::cover(icon_path)),
        None => Rectangle::new()
            .width(0.0)
            .height(0.0)
            .fill(resolve_color(colors.module_foreground)),
    };

    Box::new(inner) as Box<dyn Widget>
}

use amane::{Arc, Canvas, Cap, Shape, Widget, shapes};

use crate::{color::resolve_color, module::ModuleViewArgs};

pub fn view(module_view_args: &ModuleViewArgs) -> Box<dyn Widget> {
    let ModuleViewArgs {
        thickness, palette, ..
    } = *module_view_args;

    let value = 1.0f32;

    let size = thickness;

    let thickness = 1.5f32;

    let middle = size / 2.0;

    let radius = middle - thickness / 2.0;

    let sweep = 360.0 * value.clamp(0.0, 1.0);

    let secondary = resolve_color(palette.secondary);

    Box::new(Canvas::new().width(size).height(size).shapes(shapes![
        Arc::new()
            .center(middle, middle)
            .radius(radius)
            .stroke(thickness, secondary),
        Arc::new()
            .center(middle, middle)
            .radius(radius)
            .sweep(sweep)
            .stroke(thickness, secondary)
            .cap(Cap::Round),
    ])) as Box<dyn Widget>
}

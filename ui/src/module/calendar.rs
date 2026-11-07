use amane::{Center, Color, Rectangle, Service, Text, Widget};
use model::bar::module::Module;

use crate::{Flow, color::resolve_color, module::ModuleViewArgs, service};

pub fn view(module_view_args: &ModuleViewArgs) -> Box<dyn Widget> {
    let ModuleViewArgs {
        axis,
        module,
        radius,
        font_size,
        palette,
        thickness,
        padding,
        ..
    } = *module_view_args;

    let Module::Calendar { format_24_hrs, .. } = module else {
        return Box::new(Rectangle::new().width(0.0).height(0.0));
    };

    let time = service::Clock::read().time();

    let primary = resolve_color(palette.primary);
    let secondary = resolve_color(palette.secondary);
    let tertiary = resolve_color(palette.tertiary);
    let surface = resolve_color(palette.surface_container_low);

    let weight = 700;
    let main = axis.main(font_size, thickness);
    let cross = axis.cross(font_size, thickness);

    let cell = |width: f32, label: String, size: f32, color: Color| -> Box<dyn Widget> {
        Box::new(
            Rectangle::new()
                .width(width)
                .height(cross)
                .radius(radius)
                .align_child(Center, Center)
                .child(
                    Text::new(label)
                        .size(size)
                        .weight(weight)
                        .color(color)
                        .tight(),
                ),
        )
    };

    let hour = if format_24_hrs {
        time.hour
    } else {
        match time.hour % 12 {
            0 => 12,
            h => h,
        }
    };

    let mut cells = vec![cell(main, format!("{:02}", hour), font_size, primary)];

    if axis.is_horizontal() {
        cells.push(cell(main / 2.0, ":".into(), font_size, tertiary));
    }

    cells.push(cell(main, format!("{:02}", time.min), font_size, secondary));

    let digits = Flow::new(axis, cells)
        .align(Center)
        .justify(Center)
        .gap(0.0)
        .into_widget();

    if format_24_hrs {
        return digits;
    }

    let period = if time.am { "AM" } else { "PM" };

    let period_width = if axis.is_horizontal() {
        font_size * 1.3
    } else {
        main
    };

    let period_gap = if axis.is_horizontal() { 4.0 } else { 0.0 };

    Box::new(
        Flow::new(
            axis,
            vec![
                digits,
                cell(period_width, period.into(), font_size, secondary),
            ],
        )
        .align(Center)
        .justify(Center)
        .gap(period_gap)
        .into_envelope(padding, radius, Some(surface)),
    ) as Box<dyn Widget>
}

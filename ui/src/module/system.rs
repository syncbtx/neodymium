use amane::{Center, Cpu, Memory, Service, Text, Widget};
use model::bar::module::{SystemModule, SystemModuleStyle};

use crate::{
    Flow,
    color::resolve_color,
    component::progress,
    constants::{CPU_ICON, RAM_ICON},
    module::ModuleViewArgs,
    service::{self},
};

pub fn view(
    module_view_args: &ModuleViewArgs,
    system_module: SystemModule,
    style: SystemModuleStyle,
) -> Box<dyn Widget> {
    let ModuleViewArgs {
        thickness,
        radius,
        phase,
        palette,
        colors,
        axis,
        font,
        font_size,
        font_weight,
        padding,
        ..
    } = *module_view_args;

    let (key, pct, raw_ratio, icon) = match system_module {
        SystemModule::Cpu => {
            let pct = Cpu::read().percent();
            ("cpu", pct, (pct as f32 / 100.0).max(0.05), CPU_ICON)
        }
        SystemModule::Ram => {
            let pct = Memory::read().percent();
            ("ram", pct, (pct as f32 / 100.0).max(0.05), RAM_ICON)
        }
        SystemModule::Disk => todo!(),
        SystemModule::Temp => todo!(),
    };

    let ratio = service::motion::smooth(key, raw_ratio);

    let is_wavy = matches!(
        style,
        model::bar::module::SystemModuleStyle::WaveyRing
            | model::bar::module::SystemModuleStyle::WaveyBar
    );
    let phase = if is_wavy { phase } else { 0.0 };

    let primary = resolve_color(palette.primary);
    let surface_highest = resolve_color(palette.surface_container_highest);
    let surface = resolve_color(palette.surface_container);
    let secondary = resolve_color(palette.secondary);
    let error = resolve_color(palette.error);

    let shader_colors = [primary, surface_highest, secondary, error];

    let thresholds = [0.50, 0.75];

    let bg = resolve_color(colors.module_background);
    let fg = resolve_color(colors.module_foreground);

    let ring_thickness = 1.25f32;

    let icon_font_size = font_size * 1.0;

    let icon_text = Text::new(icon)
        .font(&font.icon_path_or_name)
        .size(icon_font_size)
        .color(resolve_color(palette.on_surface_variant))
        .tight();

    let rect = match style {
        SystemModuleStyle::Ring => progress::flat_ring(
            thickness,
            ring_thickness,
            ratio,
            phase,
            &shader_colors,
            thresholds,
        )
        .align_child(Center, Center)
        .child(icon_text),
        SystemModuleStyle::WaveyRing => progress::wavy_ring(
            thickness,
            ring_thickness,
            ratio,
            phase,
            &shader_colors,
            thresholds,
        )
        .align_child(Center, Center)
        .child(icon_text),
        SystemModuleStyle::Bar => {
            let bar = progress::flat_bar(
                thickness,
                ring_thickness,
                ratio,
                phase,
                &shader_colors,
                thresholds,
            );
            Flow::new(
                axis.flip(),
                vec![
                    Box::new(icon_text) as Box<dyn Widget>,
                    Box::new(bar) as Box<dyn Widget>,
                ],
            )
            .align(Center)
            .justify(Center)
            .gap(1.0)
            .into_envelope(padding, radius, Some(surface))
            .align_child(Center, Center)
        }
        SystemModuleStyle::WaveyBar | SystemModuleStyle::Graph => {
            let bar = progress::wavy_bar(
                thickness,
                ring_thickness,
                ratio,
                phase,
                &shader_colors,
                thresholds,
            );
            Flow::new(
                axis,
                vec![
                    Box::new(icon_text) as Box<dyn Widget>,
                    Box::new(bar) as Box<dyn Widget>,
                ],
            )
            .align(Center)
            .justify(Center)
            .gap(1.0)
            .into_envelope(padding, radius, Some(surface))
            .align_child(Center, Center)
        }
        SystemModuleStyle::Text => {
            let children = vec![
                Box::new(
                    Text::new(format!("{}", system_module.label().to_uppercase(),))
                        .size(font_size)
                        .weight(font_weight)
                        .color(resolve_color(palette.primary)),
                ) as Box<dyn Widget>,
                Box::new(
                    Text::new(format!("{:02}%", pct))
                        .size(font_size)
                        .weight(font_weight)
                        .color(fg),
                ) as Box<dyn Widget>,
            ];

            let container = Flow::new(axis, children)
                .cross_size(thickness)
                .align(Center)
                .justify(Center);

            return Box::new(container.into_envelope(padding, radius, Some(bg))) as Box<dyn Widget>;
        }
    };

    Box::new(rect) as Box<dyn Widget>
}

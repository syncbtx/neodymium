use amane::{Monitor, Padding, Rectangle, Widget};
use model::{
    Font,
    bar::{module::Module, section::BarSectionColors},
    palette::Palette,
};

mod bluetooth;
mod calendar;
mod focused;
mod icon;
mod media;
mod ring;
mod system;
mod workspaces;

use crate::{axis::Axis, color::resolve_color};

#[derive(Debug, Clone)]
pub struct ModuleViewArgs<'a> {
    monitor: &'a Monitor,
    module: Module,
    thickness: f32,
    axis: Axis,
    radius: f32,
    colors: &'a BarSectionColors,
    font: &'a Font,
    font_size: f32,
    font_weight: u16,
    phase: f32,
    palette: &'a Palette,
    padding: Padding,
}

impl<'a> ModuleViewArgs<'a> {
    pub fn new(
        monitor: &'a Monitor,
        module: Module,
        axis: Axis,
        thickness: f32,
        palette: &'a Palette,
        colors: &'a BarSectionColors,
        font: &'a Font,
    ) -> Self {
        let padding = match axis {
            crate::Axis::Horizontal => Padding {
                top: 0.0,
                right: 0.0,
                bottom: 0.0,
                left: 0.0,
            },
            crate::Axis::Vertical => Padding {
                top: 0.0,
                right: 0.0,
                bottom: 0.0,
                left: 0.0,
            },
        };
        Self {
            monitor,
            module,
            thickness,
            axis,
            radius: 0.0,
            colors,
            font,
            font_size: 12.0,
            font_weight: 400,
            phase: 0.0,
            palette,
            padding,
        }
    }

    pub fn module(mut self, module: Module) -> Self {
        self.module = module;
        self
    }

    pub fn monitor(mut self, monitor: &'a Monitor) -> Self {
        self.monitor = monitor;
        self
    }

    pub fn axis(mut self, axis: Axis) -> Self {
        self.axis = axis;
        self
    }

    pub fn thickness(mut self, thickness: f32) -> Self {
        self.thickness = thickness;
        self
    }

    pub fn radius(mut self, radius: f32) -> Self {
        self.radius = radius;
        self
    }

    pub fn colors(mut self, colors: &'a BarSectionColors) -> Self {
        self.colors = colors;
        self
    }

    pub fn font(mut self, font: &'a Font) -> Self {
        self.font = font;
        self
    }

    pub fn font_size(mut self, font_size: f32) -> Self {
        self.font_size = font_size;
        self
    }

    pub fn font_weight(mut self, font_weight: u16) -> Self {
        self.font_weight = font_weight;
        self
    }

    pub fn phase(mut self, phase: f32) -> Self {
        self.phase = phase;
        self
    }
    pub fn padding(mut self, padding: Padding) -> Self {
        self.padding = padding;
        self
    }
}

pub fn view(module_view_args: ModuleViewArgs) -> Box<dyn Widget> {
    match module_view_args.module {
        Module::Empty { main_axis_size } => {
            let ModuleViewArgs {
                thickness,
                axis,
                radius,
                colors,
                ..
            } = module_view_args;

            let (w, h) = axis.size(main_axis_size as f32, thickness);
            let r = axis.main(radius, 4.0);
            Box::new(
                Rectangle::new()
                    .width(w)
                    .height(h)
                    .radius(r)
                    .fill(resolve_color(colors.module_background)),
            ) as Box<dyn Widget>
        }
        Module::Spacer(gap) => {
            let ModuleViewArgs {
                thickness,
                axis,
                radius,
                ..
            } = module_view_args;
            let (w, h) = axis.size(gap as f32, thickness);
            Box::new(Rectangle::new().width(w).height(h).radius(radius)) as Box<dyn Widget>
        }
        Module::Workspaces { style, .. } => workspaces::view(&module_view_args, style),
        Module::FocusedWindowIcon => focused::view(&module_view_args),
        Module::Calendar { .. } => calendar::view(&module_view_args),

        Module::Divider => {
            let ModuleViewArgs {
                thickness,
                axis,
                palette,
                ..
            } = module_view_args;

            let girth = 1.0f32;
            let length = thickness * 0.8;
            let fill = resolve_color(palette.outline_variant);
            let (w, h) = axis.size(girth, length);
            Box::new(Rectangle::new().width(w).height(h).fill(fill)) as Box<dyn Widget>
        }
        Module::Ring => ring::view(&module_view_args),
        Module::Media => media::view(&module_view_args),
        Module::SystemModuleCapsule { module, style, .. } => {
            system::view(&module_view_args, module, style)
        }
        Module::Icon { icon, linked_panel } => icon::view(&module_view_args, icon, linked_panel),
        Module::Bluetooth => bluetooth::view(&module_view_args),
    }
}

use amane::{Align, Column, Rectangle, Row, Service, Text, Widget};
use hex_color::HexColor;
use model::{Font, bar::module::Module};

use crate::color::hex_to_color;

pub fn module_view(
    module: Module,
    is_bar_horizontal: bool,
    font: Font,
    background: HexColor,
    foreground: HexColor,
    border_radius: u32,
    spacing: u32,
) -> Box<dyn Widget> {
    let thickness: f32 = 24.0;

    match module {
        Module::Spacer(gap) => {
            if is_bar_horizontal {
                Box::new(Rectangle::new().width(gap as f32).height(thickness)) as Box<dyn Widget>
            } else {
                Box::new(Rectangle::new().width(thickness).height(gap as f32)) as Box<dyn Widget>
            }
        }
        Module::Workspaces { .. } => {
            let workspaces = amane::Workspaces::read();

            let button = |index: u32| -> Box<dyn Widget> {
                Box::new(
                    Rectangle::new()
                        .width(thickness)
                        .height(thickness)
                        .child(
                            Text::new(index.to_string())
                                .font(&font.path_or_name)
                                .weight(font.weight.value())
                                .color(hex_to_color(foreground))
                                .size(20.0),
                        )
                        .align_child(Align::Center, Align::Center)
                        .fill(hex_to_color(background))
                        .radius(border_radius as f32),
                ) as Box<dyn Widget>
            };

            let pills = workspaces
                .list()
                .iter()
                .map(|w| button(w.index()))
                .collect();

            if is_bar_horizontal {
                Box::new(Row::new(pills).gap(spacing as f32).align(Align::Center))
            } else {
                Box::new(Column::new(pills).gap(spacing as f32).align(Align::Center))
            }
        }
        Module::ActiveWindowTitle => Box::new(
            Text::new("Window Title")
                .font(&font.path_or_name)
                .weight(font.weight.value())
                .color(hex_to_color(foreground))
                .size(20.0),
        ) as Box<dyn Widget>,
        Module::Calendar { .. } => Box::new(
            Text::new("Time & Date")
                .font(&font.path_or_name)
                .weight(font.weight.value())
                .color(hex_to_color(foreground))
                .size(20.0),
        ) as Box<dyn Widget>,
    }
}

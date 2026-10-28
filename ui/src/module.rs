use amane::{Align, Center, Column, Rectangle, Row, Service, Text, Widget};
use model::{
    Font,
    bar::{module::Module, section::BarSectionColors},
};

use crate::{
    color::resolve_color,
    services::{self},
};

pub fn view(
    module: Module,
    thickness: f32,
    is_horizontal: bool,
    radius: f32,
    colors: &BarSectionColors,
    font: &Font,
    font_size: f32,
    font_weight: u16,
) -> Box<dyn Widget> {
    match module {
        Module::Empty { main_axis_size } => {
            if is_horizontal {
                Box::new(
                    Rectangle::new()
                        .width(main_axis_size as f32)
                        .height(thickness)
                        .radius(radius)
                        .fill(resolve_color(colors.module_background)),
                ) as Box<dyn Widget>
            } else {
                Box::new(
                    Rectangle::new()
                        .width(thickness)
                        .height(main_axis_size as f32)
                        .radius(4.0)
                        .fill(resolve_color(colors.module_background)),
                ) as Box<dyn Widget>
            }
        }
        Module::Spacer(gap) => {
            if is_horizontal {
                Box::new(
                    Rectangle::new()
                        .width(gap as f32)
                        .height(thickness)
                        .radius(radius),
                ) as Box<dyn Widget>
            } else {
                Box::new(
                    Rectangle::new()
                        .width(thickness)
                        .height(gap as f32)
                        .radius(radius),
                ) as Box<dyn Widget>
            }
        }
        Module::Workspaces { .. } => {
            let workspaces = amane::Workspaces::read();

            let pills = workspaces
                .list()
                .iter()
                .filter(|ws| ws.windows() > 0 || (ws.windows() == 0 && ws.focused()))
                .map(|ws| {
                    let (background, foreground) = if ws.active() {
                        (colors.module_background, colors.module_foreground)
                    } else {
                        (colors.module_foreground, colors.module_background)
                    };

                    Box::new(
                        Rectangle::new()
                            .width(thickness)
                            .height(thickness)
                            .radius(radius)
                            .fill(resolve_color(background))
                            .align_child(Align::Center, Align::Center)
                            .child(
                                Text::new(format!("{}", ws.index()))
                                    .size(font_size)
                                    .font(&font.path_or_name)
                                    .weight(font_weight)
                                    .color(resolve_color(foreground)),
                            ),
                    ) as Box<dyn Widget>
                })
                .collect();

            if is_horizontal {
                Box::new(Row::new(pills).height(thickness).gap(4.0)) as Box<dyn Widget>
            } else {
                Box::new(Column::new(pills).width(thickness).gap(4.0)) as Box<dyn Widget>
            }
        }
        Module::FocusedWindowIcon => {
            let focused = services::FocusedWindow::read();

            // let id = if focused.id == "null".to_string() {
            //     "".to_string()
            // } else {
            //     focused.id.clone()
            // };

            let inner = match focused.icon.clone() {
                Some(icon_path) => Rectangle::new()
                    .width(thickness)
                    .height(thickness)
                    .fill(amane::Image::stretch(icon_path)),
                None => Rectangle::new()
                    .width(0.0)
                    .height(0.0)
                    .fill(resolve_color(colors.module_foreground)),
            };

            if is_horizontal {
                Box::new(
                    inner, // Text::new(id)
                          //     .font(&font.path_or_name)
                          //     .size(font_size)
                          //     .weight(font_weight)
                          //     .color(resolve_color(colors.foreground))
                          //     .max_lines(1)
                          //     .elide(),
                ) as Box<dyn Widget>
            } else {
                Box::new(
                    inner,
                    // Text::new(id)
                    //     .size(font_size)
                    //     .weight(font_weight)
                    //     .color(resolve_color(colors.foreground))
                    //     .elide(),
                ) as Box<dyn Widget>
            }
        }
        Module::Calendar {
            time,
            date,
            format_24_hrs,
            ..
        } => {
            let clock = services::Clock::read();
            let out = if time && date {
                format!("{} {}", clock.time(true, format_24_hrs), clock.date())
            } else if time {
                format!("{}", clock.time(true, format_24_hrs))
            } else if date {
                format!("{}", clock.date())
            } else {
                format!("{}", "")
            };

            if is_horizontal {
                Box::new(
                    Text::new(&out)
                        .size(font_size)
                        .weight(font_weight)
                        .font(&font.path_or_name)
                        .color(resolve_color(colors.foreground))
                        .max_lines(1),
                ) as Box<dyn Widget>
            } else {
                Box::new(
                    Text::new(&out)
                        .font(&font.path_or_name)
                        .size(font_size)
                        .weight(font_weight)
                        .color(resolve_color(colors.foreground))
                        .elide(),
                ) as Box<dyn Widget>
            }
        }
        Module::Divider => {
            let girth = 1.0f32;
            let length = thickness * 0.8;

            let fill = resolve_color(colors.module_background);

            let rect = if is_horizontal {
                Box::new(Rectangle::new().width(girth).height(length).fill(fill)) as Box<dyn Widget>
            } else {
                Box::new(Rectangle::new().width(length).height(girth).fill(fill)) as Box<dyn Widget>
            };
            rect
        }
        Module::Ring => {
            let inner = thickness - 1.5f32;
            let radius = 9999.0f32;
            Box::new(
                Rectangle::new()
                    .width(thickness)
                    .height(thickness)
                    .radius(radius)
                    .align_child(Center, Center)
                    .fill(resolve_color(colors.module_background))
                    .child(
                        Rectangle::new()
                            .width(inner)
                            .height(inner)
                            .radius(radius)
                            .align_child(Center, Center)
                            .fill(resolve_color(colors.background)),
                    ),
            ) as Box<dyn Widget>
        }
    }
}

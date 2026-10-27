use amane::{Align, Column, Rectangle, Row, Service, Text, Widget};
use model::{
    Font,
    bar::{module::Module, section::BarSectionColors},
};

use crate::color::resolve_color;

pub struct FocusedWindow {
    pub id: String,
}

impl FocusedWindow {
    pub fn query() -> String {
        let out = amane::output("niri msg --json focused-window");
        match serde_json::from_str::<serde_json::Value>(&out) {
            Ok(v) => v.get("app_id").unwrap().to_string(),
            Err(_) => "focused title".to_string(),
        }
    }
}

impl amane::Service for FocusedWindow {
    fn new() -> Self {
        Self { id: Self::query() }
    }

    fn listen() {
        amane::lines("niri msg event-stream")
            .into_iter()
            .filter(|_| Self::read().id != Self::query())
            .for_each(|_| Self::write().id = Self::query());
    }
}

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
                    Box::new(
                        Rectangle::new()
                            .width(thickness)
                            .height(thickness)
                            .radius(radius)
                            .fill(resolve_color(colors.module_background))
                            .align_child(Align::Center, Align::Center)
                            .child(
                                Text::new(format!("{}", ws.index()))
                                    .size(font_size)
                                    .font(&font.path_or_name)
                                    .weight(font_weight)
                                    .color(resolve_color(colors.module_foreground)),
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
        Module::ActiveWindowTitle => {
            amane::DesktopApp::
            let title = FocusedWindow::read();
            if is_horizontal {
                Box::new(
                    Text::new(format!("{}", title.id))
                        .font(&font.path_or_name)
                        .size(font_size)
                        .weight(font_weight)
                        .color(resolve_color(colors.foreground))
                        .max_lines(1),
                ) as Box<dyn Widget>
            } else {
                Box::new(
                    Text::new(format!("{}", title.id))
                        .size(font_size)
                        .weight(font_weight)
                        .color(resolve_color(colors.foreground))
                        .elide(),
                ) as Box<dyn Widget>
            }
        }
        Module::Calendar { .. } => {
            let time = format!("{}", "hh:mm::ss");
            if is_horizontal {
                Box::new(
                    Text::new(&time)
                        .size(font_size)
                        .weight(font_weight)
                        .font(&font.path_or_name)
                        .color(resolve_color(colors.foreground))
                        .max_lines(1),
                ) as Box<dyn Widget>
            } else {
                Box::new(
                    Text::new(&time)
                        .font(&font.path_or_name)
                        .size(font_size)
                        .weight(font_weight)
                        .color(resolve_color(colors.foreground))
                        .elide(),
                ) as Box<dyn Widget>
            }
        }
    }
}

use amane::{
    Align, Center, Color, Column, Cpu, Media, Memory, Pointer, Rectangle, Row, Service, Text,
    Widget, Workspaces,
};
use hhmmss::Hhmmss;
use model::{
    Font,
    bar::{
        module::{Module, SystemModuleStyle},
        section::BarSectionColors,
    },
    palette::Palette,
};

pub const WEEKDAYS: [&str; 7] = [
    "Thursday",
    "Friday",
    "Saturday",
    "Sunday",
    "Monday",
    "Tuesday",
    "Wednesday",
];

pub const WEEKDAYS_SHORT: [&str; 7] = ["Thur", "Fri", "Sat", "Sun", "Mon", "Tue", "Wed"];

pub const MONTHS: [&str; 12] = [
    "Jan", "Feb", "Mar", "Apr", "May", "Jun", "Jul", "Aug", "Sep", "Oct", "Nov", "Dec",
];

use crate::{
    axis::{Axis, Flow},
    color::resolve_color,
    component::progress,
    services::{self},
};

use amane::Animation;
use std::{collections::HashMap, time::Duration};

struct PillColors {
    bg: Animation<Color>,
    fg: Animation<Color>,
}

pub struct PillHover {
    colors: HashMap<i64, PillColors>,
    scale: HashMap<i64, Animation<f32>>,
}

impl Service for PillHover {
    fn new() -> Self {
        Self {
            colors: HashMap::new(),
            scale: HashMap::new(),
        }
    }
    fn interval() -> Duration {
        Duration::from_secs(3600)
    }
}

impl PillHover {
    pub fn colors(&self, id: i64) -> Option<(Color, Color)> {
        self.colors.get(&id).map(|c| (c.bg.value(), c.fg.value()))
    }

    pub fn scale(&self, id: i64) -> Option<f32> {
        self.scale.get(&id).map(|s| s.value())
    }

    pub fn go(&mut self, id: i64, rest: (Color, Color), target: (Color, Color), scale: f32) {
        let p = self.colors.entry(id).or_insert_with(|| PillColors {
            bg: Animation::new(rest.0).duration(Duration::from_millis(500)),
            fg: Animation::new(rest.1).duration(Duration::from_millis(500)),
        });
        p.bg.to(target.0);
        p.fg.to(target.1);
        self.scale
            .entry(id)
            .or_insert_with(|| {
                Animation::new(1.0)
                    .duration(Duration::from_millis(500))
                    .easing(amane::Easing::Out)
            })
            .to(scale);
    }
}

pub fn view(
    module: Module,
    thickness: f32,
    axis: Axis,
    radius: f32,
    colors: &BarSectionColors,
    font: &Font,
    font_size: f32,
    font_weight: u16,
    phase: f32,
    palette: &Palette,
) -> Box<dyn Widget> {
    let font_size = font_size.min(thickness);

    match module {
        Module::Empty { main_axis_size } => {
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
            let (w, h) = axis.size(gap as f32, thickness);
            Box::new(Rectangle::new().width(w).height(h).radius(radius)) as Box<dyn Widget>
        }
        Module::Workspaces { .. } => {
            let guard = amane::Workspaces::read();

            let workspaces = guard.list();

            let pills = workspaces
                .iter()
                .filter(|ws| ws.windows() > 0 || (ws.windows() == 0 && ws.focused()))
                .map(|ws| {
                    let hover = PillHover::read();

                    let active = ws.active();

                    let id = ws.id();

                    let rest = (
                        resolve_color(colors.module_background),
                        resolve_color(colors.module_foreground),
                    );
                    let lit = (
                        resolve_color(palette.surface_container_highest),
                        resolve_color(palette.on_surface_variant),
                    );

                    let (background, foreground) = if active {
                        (
                            resolve_color(palette.primary),
                            resolve_color(palette.on_primary),
                        )
                    } else {
                        hover.colors(id).unwrap_or(rest)
                    };

                    Box::new(
                        Rectangle::new()
                            .width(thickness)
                            .height(thickness)
                            .radius(radius)
                            .fill(background)
                            .align_child(Align::Center, Align::Center)
                            .cursor(Pointer)
                            .on_hover(move |inside| {
                                PillHover::write().go(
                                    id,
                                    rest,
                                    if inside { lit } else { rest },
                                    if inside { 1.1 } else { 1.0 },
                                );
                            })
                            .on_click(move |_| {
                                Workspaces::focus(id);
                            })
                            .child(
                                Text::new(format!("{}", ws.index()))
                                    .size(font_size)
                                    .font(&font.path_or_name)
                                    .weight(font_weight)
                                    .color(foreground),
                            ),
                    ) as Box<dyn Widget>
                })
                .collect();

            Flow::new(axis, pills)
                .cross_size(thickness)
                .gap(4.0)
                .into_widget()
        }
        Module::FocusedWindowIcon => {
            let focused = services::FocusedWindow::read();

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

            Box::new(inner) as Box<dyn Widget>
        }
        Module::Calendar { format_24_hrs, .. } => {
            let clock = services::Clock::read();

            let time = clock.time();
            let date = clock.date();

            let primary = resolve_color(palette.primary);
            let secondary = resolve_color(palette.secondary);
            let surface_low = resolve_color(palette.surface_container_low);
            let tertiary = resolve_color(palette.tertiary);

            let large_font = font_size * 1.25;
            let small_font = font_size * 0.9;

            let inner_gap = 2.0f32;
            let weight = 700;

            let thickness = large_font + inner_gap;

            let sec_and_period = if !format_24_hrs {
                let period = if time.am { "AM" } else { "PM" };

                let radius = radius / 1.75;

                Box::new(
                    Rectangle::new()
                        .width(thickness)
                        .height((thickness - inner_gap) / 2.0)
                        .fill(surface_low)
                        .radius(radius)
                        .align_child(Center, Center)
                        .child(
                            Text::new(format!("{}", period))
                                .size(large_font)
                                .color(primary)
                                .weight(weight)
                                .tight(),
                        ),
                ) as Box<dyn Widget>
            } else {
                Box::new(Column::new(vec![Box::new(
                    Rectangle::new()
                        .width(0.0)
                        .height(0.0)
                        .fill(surface_low)
                        .radius(radius), // .align_child(Center, Center)
                                         // .child(
                                         //     Text::new(format!("{:02}", time.sec))
                                         //         .size(large_font)
                                         //         .color(primary)
                                         //         .weight(weight)
                                         //         .tight(),
                                         // ),
                ) as Box<dyn Widget>])) as Box<dyn Widget>
            };

            let time = Row::new(vec![
                Box::new(
                    Rectangle::new()
                        .width(thickness)
                        .height(thickness)
                        .fill(surface_low)
                        .radius(radius)
                        .align_child(Center, Center)
                        .child(
                            Text::new(format!("{:02}", time.hour))
                                .size(large_font)
                                .weight(weight)
                                .color(primary)
                                .tight(),
                        ),
                ) as Box<dyn Widget>,
                Box::new(
                    Rectangle::new()
                        .width(thickness)
                        .height(thickness)
                        .radius(radius)
                        .fill(surface_low)
                        .align_child(Center, Center)
                        .child(
                            Text::new(format!("{:02}", time.min))
                                .size(large_font)
                                .weight(weight)
                                .color(secondary)
                                .tight(),
                        ),
                ) as Box<dyn Widget>,
                sec_and_period,
            ])
            .align(Center)
            .justify(Center)
            .gap(0.0);

            let _date = Row::new(vec![
                Box::new(
                    Rectangle::new()
                        .width(thickness)
                        .height(thickness)
                        .fill(surface_low)
                        .align_child(Center, Center)
                        .child(
                            Text::new(format!("{}", WEEKDAYS_SHORT[date.weekday].to_uppercase()))
                                .size(small_font)
                                .weight(weight)
                                .color(primary)
                                .tight(),
                        ),
                ) as Box<dyn Widget>,
                Box::new(
                    Rectangle::new()
                        .width(thickness)
                        .height(thickness)
                        .fill(surface_low)
                        .align_child(Center, Center)
                        .child(
                            Text::new(format!("{:02}", date.day))
                                .size(large_font)
                                .weight(weight)
                                .color(secondary)
                                .tight(),
                        ),
                ) as Box<dyn Widget>,
                Box::new(
                    Column::new(vec![
                        Box::new(
                            Rectangle::new()
                                .width(thickness)
                                .height((thickness - inner_gap) / 2.0)
                                .fill(surface_low)
                                .align_child(Center, Center)
                                .child(
                                    Text::new(format!("{}", MONTHS[date.month].to_uppercase()))
                                        .size(small_font)
                                        .color(primary)
                                        .weight(weight)
                                        .tight(),
                                ),
                        ) as Box<dyn Widget>,
                        Box::new(
                            Rectangle::new()
                                .width(thickness)
                                .height((thickness - inner_gap) / 2.0)
                                .fill(surface_low)
                                .align_child(Center, Center)
                                .child(
                                    Text::new(format!("{}", date.year))
                                        .size(small_font)
                                        .color(tertiary)
                                        .weight(weight)
                                        .tight(),
                                ),
                        ) as Box<dyn Widget>,
                    ])
                    .gap(2.0),
                ) as Box<dyn Widget>,
            ])
            .align(Center)
            .justify(Center)
            .gap(2.0);

            Box::new(
                Row::new(vec![
                    Box::new(time) as Box<dyn Widget>,
                    // Box::new(date) as Box<dyn Widget>,
                ])
                .gap(inner_gap),
            ) as Box<dyn Widget>
        }

        Module::Divider => {
            let girth = 1.0f32;
            let length = thickness * 0.8;
            let fill = resolve_color(palette.outline_variant);
            let (w, h) = axis.size(girth, length);
            Box::new(Rectangle::new().width(w).height(h).fill(fill)) as Box<dyn Widget>
        }
        Module::Ring => {
            let colors = [
                resolve_color(palette.primary),
                Color::TRANSPARENT,
                resolve_color(palette.secondary),
                resolve_color(palette.error),
            ];

            let thresholds = [0.0, 1.0];

            Box::new(progress::flat_ring(
                thickness, 1.5, 1.0, -phase, &colors, thresholds,
            )) as Box<dyn Widget>
        }
        Module::Media => {
            let media = Media::read();

            match media.active() {
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
                    Box::new(
                        Text::new(&info)
                            .font(&font.path_or_name)
                            .size(font_size)
                            .weight(font_weight)
                            .color(resolve_color(colors.foreground)),
                    ) as Box<dyn Widget>
                }
                None => Box::new(
                    Text::new("Nothing is playing")
                        .font(&font.path_or_name)
                        .size(font_size)
                        .weight(font_weight)
                        .color(resolve_color(colors.foreground)),
                ) as Box<dyn Widget>,
            }
        }
        Module::SystemModuleCapsule { module, style, .. } => {
            let (key, raw_ratio, icon) = match module {
                model::bar::module::SystemModule::Cpu => {
                    let pct = Cpu::read().percent();
                    ("cpu", pct as f32 / 100.0, format!("{}", "\u{f4bc}"))
                }
                model::bar::module::SystemModule::Ram => {
                    let pct = Memory::read().percent();
                    ("ram", pct as f32 / 100.0, format!("{}", "\u{efc5}"))
                }
                model::bar::module::SystemModule::Disk => todo!(),
                model::bar::module::SystemModule::Temp => todo!(),
            };

            let ratio = services::motion::smooth(key, raw_ratio);

            let is_wavy = matches!(
                style,
                model::bar::module::SystemModuleStyle::WaveyRing
                    | model::bar::module::SystemModuleStyle::WaveyBar
            );
            let phase = if is_wavy { phase } else { 0.0 };

            let colors = [
                resolve_color(palette.primary),
                resolve_color(palette.surface_container_highest),
                resolve_color(palette.secondary),
                resolve_color(palette.error),
            ];

            let thresholds = [0.50, 0.75];

            let rect = match style {
                SystemModuleStyle::Ring => {
                    progress::flat_ring(thickness, 1.5, ratio, phase, &colors, thresholds)
                        .align_child(Center, Center)
                        .child(
                            Text::new(icon)
                                .color(resolve_color(palette.on_surface_variant))
                                .size(10.0)
                                .tight(),
                        )
                }
                SystemModuleStyle::WaveyRing => {
                    progress::wavy_ring(thickness, 1.5, ratio, phase, &colors, thresholds)
                        .align_child(Center, Center)
                        .child(
                            Text::new(icon)
                                .color(resolve_color(palette.on_surface_variant))
                                .size(10.0)
                                .tight(),
                        )
                }
                SystemModuleStyle::Bar => {
                    progress::flat_bar(thickness, 1.5, ratio, phase, &colors, thresholds)
                }
                SystemModuleStyle::WaveyBar => {
                    progress::wavy_bar(thickness, 1.5, ratio, phase, &colors, thresholds)
                }
                SystemModuleStyle::Graph => {
                    progress::wavy_bar(thickness, 1.5, ratio, phase, &colors, thresholds)
                }
            };

            Box::new(rect) as Box<dyn Widget>
        }
    }
}

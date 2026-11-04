use amane::{
    Align, Center, Column, Cpu, Media, Memory, Radius, Rectangle, Service, Stack, Text, Widget,
    children,
};
use hex_color::HexColor;
use hhmmss::Hhmmss;
use model::{
    Font,
    bar::{
        module::{Module, SystemModule, SystemModuleStyle},
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
    services::{self, DateParts, TimeParts},
};

pub fn view(
    module: Module,
    thickness: f32,
    axis: Axis,
    radius: f32,
    colors: &BarSectionColors,
    font: &Font,
    font_size: f32,
    font_weight: u16,
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
            let workspaces = amane::Workspaces::read();

            let pills = workspaces
                .list()
                .iter()
                .filter(|ws| ws.windows() > 0 || (ws.windows() == 0 && ws.focused()))
                .map(|ws| {
                    let (background, foreground) = if ws.active() {
                        (palette.primary, palette.on_primary)
                    } else {
                        (colors.module_background, colors.module_foreground)
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
        Module::Calendar {
            time,
            date,
            format_24_hrs,
            ..
        } => {
            let clock = services::Clock::read();

            let mut sections = Vec::new();

            if time {
                sections.push(time_section(&clock.time(), !format_24_hrs));
            }

            if date {
                sections.push(date_section(&clock.date()));
            }

            if axis.is_horizontal() {
                let line = sections
                    .iter()
                    .map(TimeSection::line)
                    .collect::<Vec<_>>()
                    .join("  ");

                Box::new(
                    Text::new(&line.to_uppercase())
                        .size(font_size)
                        .weight(font_weight)
                        .font(&font.path_or_name)
                        .color(resolve_color(colors.foreground))
                        .max_lines(1),
                ) as Box<dyn Widget>
            } else {
                let rows = sections
                    .into_iter()
                    .flat_map(TimeSection::stacked)
                    .map(|row| {
                        Box::new(
                            Text::new(&row.to_uppercase())
                                .font(&font.path_or_name)
                                .size(font_size)
                                .weight(font_weight)
                                .color(resolve_color(colors.foreground)),
                        ) as Box<dyn Widget>
                    })
                    .collect();

                Box::new(Column::new(rows).align(Center)) as Box<dyn Widget>
            }
        }

        Module::Divider => {
            let girth = 1.0f32;
            let length = thickness * 0.8;
            let fill = resolve_color(palette.outline_variant);
            let (w, h) = axis.size(girth, length);
            Box::new(Rectangle::new().width(w).height(h).fill(fill)) as Box<dyn Widget>
        }
        Module::Ring => {
            let inner = thickness - 1.5f32;
            Box::new(
                Rectangle::new()
                    .width(thickness)
                    .height(thickness)
                    .radius(Radius::Full)
                    .align_child(Center, Center)
                    .fill(resolve_color(palette.outline_variant))
                    .child(
                        Rectangle::new()
                            .width(inner)
                            .height(inner)
                            .radius(Radius::Full)
                            .align_child(Center, Center)
                            .fill(resolve_color(colors.background)),
                    )
                    .on_hover(|hovered| if hovered {}),
            ) as Box<dyn Widget>
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
        Module::SystemModuleCapsule {
            label,
            module,
            style,
        } => {
            let color = |ratio: f32| -> HexColor {
                if ratio > 0.85 {
                    palette.error
                } else if ratio > 0.70 {
                    palette.tertiary
                } else {
                    palette.primary
                }
            };

            let (key, raw_ratio) = match module {
                model::bar::module::SystemModule::Cpu => {
                    let pct = Cpu::read().percent();
                    ("cpu", pct as f32 / 100.0)
                }
                model::bar::module::SystemModule::Ram => {
                    let pct = Memory::read().percent();
                    ("ram", pct as f32 / 100.0)
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
            let phase = if is_wavy {
                services::motion::phase()
            } else {
                0.0
            };

            let pct_val = (ratio * 100.0) as u32;
            let track_color = resolve_color(palette.surface_container_highest);
            let fill_color = resolve_color(color(ratio));

            match style {
                SystemModuleStyle::Ring | SystemModuleStyle::WaveyRing => {
                    let ring_size = 32.0f32.min(thickness).max(20.0);
                    let (amplitude, waves) = if is_wavy { (1.0, 8.0) } else { (0.0, 0.0) };
                    let ring_style = progress::Style {
                        size: ring_size,
                        thickness: 2.0,
                        amplitude,
                        waves,
                        track: track_color,
                        color: fill_color,
                    };

                    let ring_font_size = (ring_size * 0.42).min(font_size).max(10.0);
                    let icon = match module {
                        SystemModule::Cpu => "\u{f035b}",
                        SystemModule::Ram => "\u{f035b}",
                        SystemModule::Disk => todo!(),
                        SystemModule::Temp => todo!(),
                    };
                    let pct_text = Rectangle::new()
                        .width(ring_size - ring_style.thickness)
                        .height(ring_size)
                        .align_child(Center, Center)
                        .child(
                            Text::new(format!("{}", icon))
                                .font(&font.path_or_name)
                                .size(ring_font_size)
                                .weight(font_weight)
                                .color(resolve_color(colors.foreground)),
                        );

                    let ring_widget = Stack::new(children![
                        progress::ring(ring_style, ratio, phase),
                        pct_text
                    ]);

                    if label {
                        let label_widget = Text::new(module.label().to_uppercase())
                            .font(&font.path_or_name)
                            .size(font_size)
                            .weight(font_weight)
                            .color(resolve_color(colors.foreground));

                        Flow::new(
                            axis,
                            vec![
                                Box::new(label_widget) as Box<dyn Widget>,
                                Box::new(ring_widget) as Box<dyn Widget>,
                            ],
                        )
                        .gap(6.0)
                        .align(Align::Center)
                        .into_widget()
                    } else {
                        Box::new(ring_widget) as Box<dyn Widget>
                    }
                }
                model::bar::module::SystemModuleStyle::Bar
                | model::bar::module::SystemModuleStyle::WaveyBar
                | model::bar::module::SystemModuleStyle::Graph => {
                    let bar_width = if axis.is_horizontal() {
                        64.0
                    } else {
                        (thickness - 8.0).max(24.0)
                    };
                    let (amplitude, wavelength) = if is_wavy { (1.5, 14.0) } else { (0.0, 1.0) };
                    let bar_style = progress::BarStyle {
                        width: bar_width,
                        thickness: 2.0,
                        amplitude,
                        wavelength,
                        track: track_color,
                        color: fill_color,
                    };
                    let bar_widget = progress::bar(bar_style, ratio, phase);

                    let pct_widget = Text::new(format!("{}", pct_val))
                        .font(&font.path_or_name)
                        .size(font_size)
                        .weight(font_weight)
                        .color(resolve_color(colors.foreground));

                    if label {
                        let label_widget = Text::new(module.label().to_uppercase())
                            .font(&font.path_or_name)
                            .size(font_size)
                            .weight(font_weight)
                            .color(resolve_color(colors.foreground));

                        Flow::new(
                            axis,
                            vec![
                                Box::new(label_widget) as Box<dyn Widget>,
                                Box::new(bar_widget) as Box<dyn Widget>,
                                Box::new(pct_widget) as Box<dyn Widget>,
                            ],
                        )
                        .gap(8.0)
                        .align(Align::Center)
                        .into_widget()
                    } else {
                        Flow::new(
                            axis,
                            vec![
                                Box::new(bar_widget) as Box<dyn Widget>,
                                Box::new(pct_widget) as Box<dyn Widget>,
                            ],
                        )
                        .gap(6.0)
                        .align(Align::Center)
                        .into_widget()
                    }
                }
            }
        }
    }
}

struct TimeSection {
    fields: Vec<String>,
    sep: &'static str,
    suffix: Option<&'static str>,
}

impl TimeSection {
    fn line(&self) -> String {
        let mut line = self.fields.join(self.sep);

        if let Some(suffix) = self.suffix {
            line.push(' ');
            line.push_str(suffix);
        }

        line
    }

    fn stacked(self) -> Vec<String> {
        let mut rows = self.fields;
        rows.extend(self.suffix.map(String::from));
        rows
    }
}

fn time_section(time: &TimeParts, twelve_hour: bool) -> TimeSection {
    let (hour, suffix) = if twelve_hour {
        let hour = match time.hour % 12 {
            0 => 12,
            hour => hour,
        };

        (
            format!("{:02}", hour),
            Some(if time.am { "AM" } else { "PM" }),
        )
    } else {
        (format!("{:02}", time.hour), None)
    };

    TimeSection {
        fields: vec![hour, format!("{:02}", time.min), format!("{:02}", time.sec)],
        sep: ":",
        suffix,
    }
}

fn date_section(date: &DateParts) -> TimeSection {
    TimeSection {
        fields: vec![
            WEEKDAYS_SHORT[date.weekday].to_string(),
            date.day.to_string(),
            MONTHS[date.month].to_string(),
            date.year.to_string(),
        ],
        sep: " ",
        suffix: None,
    }
}

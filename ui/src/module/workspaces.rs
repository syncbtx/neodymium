use amane::{Center, Color, Cursor::Pointer, Rectangle, Service, Text, Widget, Workspaces};
use model::bar::module::WorkspacePillStyle;

use crate::{Flow, color::resolve_color, module::ModuleViewArgs};

use amane::Animation;
use std::{collections::HashMap, time::Duration};

struct PillColors {
    bg: Animation<Color>,
    fg: Animation<Color>,
}

pub struct PillHover {
    colors: HashMap<i64, PillColors>,
}

impl Service for PillHover {
    fn new() -> Self {
        Self {
            colors: HashMap::new(),
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

    pub fn go(&mut self, id: i64, rest: (Color, Color), target: (Color, Color)) {
        let p = self.colors.entry(id).or_insert_with(|| PillColors {
            bg: Animation::new(rest.0).duration(Duration::from_millis(500)),
            fg: Animation::new(rest.1).duration(Duration::from_millis(500)),
        });
        p.bg.to(target.0);
        p.fg.to(target.1);
    }
}

pub fn view(module_view_args: &ModuleViewArgs, style: WorkspacePillStyle) -> Box<dyn Widget> {
    let ModuleViewArgs {
        monitor,
        thickness,
        axis,
        radius,
        colors,
        font,
        font_size,
        font_weight,
        palette,
        ..
    } = module_view_args.clone();

    let radius = match style {
        WorkspacePillStyle::Circle => 9999.0f32,
        WorkspacePillStyle::Square => radius,
    };

    let guard = amane::Workspaces::read();

    let workspaces = guard.list();

    let pills = workspaces
        .iter()
        .filter(|ws| ws.output() == Some(monitor.name.as_str()))
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
                    .align_child(Center, Center)
                    .cursor(Pointer)
                    .on_hover(move |inside| {
                        PillHover::write().go(id, rest, if inside { lit } else { rest });
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

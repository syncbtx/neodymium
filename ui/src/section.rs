use amane::{Align, Column, Justify, Padding, Row, Size, Widget};
use model::bar::{Bar, section::BarSection};

use crate::{card, color::resolve_color, module};

pub fn view(section: &BarSection, parent: &Bar) -> Box<dyn Widget> {
    let is_bar_horizontal = Bar::is_horizontal(parent.edge);
    let font = parent.font.clone().unwrap_or_default();
    let cross_size = parent.section_cross_axis_size as f32;

    let padding = Padding {
        top: section.padding.top as f32,
        right: section.padding.right as f32,
        bottom: section.padding.bottom as f32,
        left: section.padding.left as f32,
    };

    let (inner_cross, module_thickness) = if is_bar_horizontal {
        let t = (cross_size - padding.top - padding.bottom).max(8.0);
        (Size::Fixed(t), t)
    } else {
        let t = (cross_size - padding.left - padding.right).max(8.0);
        (Size::Fixed(t), t)
    };

    let module_views = section
        .modules
        .iter()
        .map(|m| {
            module::view(
                *m,
                module_thickness,
                is_bar_horizontal,
                section.border_radius.unwrap_or_default() as f32,
                &section.colors,
                &font,
                12.0,
                700,
            )
        })
        .collect();

    let gap = section.spacing as f32;

    if is_bar_horizontal {
        let inner = Row::new(module_views)
            .gap(gap)
            .align(Align::Center)
            .justify(Justify::Center)
            .height(inner_cross);

        Box::new(card::view(
            inner,
            padding,
            section.border_radius.unwrap_or_default() as f32,
            Some(resolve_color(section.colors.background)),
        ))
    } else {
        let inner = Column::new(module_views)
            .gap(gap)
            .align(Align::Center)
            .justify(Justify::Center)
            .width(inner_cross);

        Box::new(card::view(
            inner,
            padding,
            section.border_radius.unwrap_or_default() as f32,
            Some(resolve_color(section.colors.background)),
        ))
    }
}

use amane::{Align, Justify, Padding, Service, Size, Widget};
use model::bar::{
    Bar,
    section::{BarSection, BarSectionColors, SectionTint},
};

use crate::{
    axis::{Axis, Flow},
    color::resolve_color_with_alpha,
    module,
    services::Theme,
};

pub fn view(section: &BarSection, parent: &Bar) -> Box<dyn Widget> {
    let axis = Axis::of(parent.edge);
    let font = section
        .font
        .clone()
        .or_else(|| parent.font.clone())
        .unwrap_or_default();
    let cross_size = parent.section_cross_axis_size as f32;

    let padding = Padding {
        top: section.padding.top as f32,
        right: section.padding.right as f32,
        bottom: section.padding.bottom as f32,
        left: section.padding.left as f32,
    };

    let module_thickness = (cross_size - axis.cross_padding(padding)).max(16.0);
    let inner_cross = Size::Fixed(module_thickness);

    let theme = Theme::read();
    let palette = theme.active();

    let (card_bg, pill_bg, text_fg, text_muted) = match section.tint {
        SectionTint::Surface => (
            palette.surface_container_low,
            palette.surface_container,
            palette.on_surface,
            palette.on_surface_variant,
        ),
        SectionTint::Primary => (
            palette.primary,
            palette.surface_container_high,
            palette.on_primary,
            palette.on_primary,
        ),
        SectionTint::Secondary => (
            palette.secondary,
            palette.surface_container_high,
            palette.on_secondary,
            palette.on_secondary,
        ),
        SectionTint::Tertiary => (
            palette.tertiary,
            palette.surface_container_high,
            palette.on_tertiary,
            palette.on_tertiary,
        ),
        SectionTint::Error => (
            palette.error,
            palette.surface_container_high,
            palette.on_error,
            palette.on_error,
        ),
    };

    let resolved_colors = BarSectionColors {
        background: section.background.unwrap_or(card_bg),
        foreground: section.foreground.unwrap_or(text_fg),
        module_background: pill_bg,
        module_foreground: text_muted,
    };

    let module_views = section
        .modules
        .iter()
        .map(|m| {
            module::view(
                *m,
                module_thickness,
                axis,
                section.border_radius.unwrap_or_default() as f32,
                &resolved_colors,
                &font,
                9.0,
                700,
            )
        })
        .collect();

    let gap = section.spacing as f32;

    let radius = match section.border_radius {
        Some(r) => r as f32,
        None => parent.section_border_radius as f32,
    };

    Box::new(
        Flow::new(axis, module_views)
            .gap(gap)
            .align(Align::Center)
            .justify(Justify::Center)
            .cross_size(inner_cross)
            .into_card(
                padding,
                radius,
                Some(resolve_color_with_alpha(
                    resolved_colors.background,
                    theme.opacity(),
                )),
            ),
    )
}

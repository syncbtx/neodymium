use amane::{Align, Justify, Monitor, Padding, Size, Widget};
use model::{
    bar::{
        Bar,
        section::{BarSection, BarSectionColors, SectionTint},
    },
    palette::Palette,
};

use crate::{
    axis::{Axis, Flow},
    color,
    module::{self, ModuleViewArgs},
};

pub fn view(
    monitor: &Monitor,
    section: &BarSection,
    parent: &Bar,
    palette: &Palette,
    phase: f32,
    opacity: u8,
) -> Box<dyn Widget> {
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
        background: card_bg,
        foreground: text_fg,
        module_background: pill_bg,
        module_foreground: text_muted,
    };

    let module_views = section
        .modules
        .iter()
        .map(|m| {
            let args = ModuleViewArgs::new(
                monitor,
                *m,
                axis,
                module_thickness,
                palette,
                &resolved_colors,
                &font,
            )
            .radius(section.module_border_radius.unwrap_or_default() as f32)
            .font_size(9.5)
            .font_weight(700)
            .phase(phase);

            module::view(args)
        })
        .collect();

    let gap = section.spacing as f32;

    let radius = match section.border_radius {
        Some(r) => r as f32,
        None => parent.section_border_radius as f32,
    };

    let theme_alpha = opacity;
    let eff_alpha = ((section.opacity.0 as u16 * theme_alpha as u16) / 255) as u8;
    let card_color = color::resolve_color_with_alpha(resolved_colors.background, eff_alpha);

    Box::new(
        Flow::new(axis, module_views)
            .gap(gap)
            .align(Align::Center)
            .justify(Justify::Center)
            .cross_size(inner_cross)
            .into_envelope(padding, radius, Some(card_color)),
    )
}

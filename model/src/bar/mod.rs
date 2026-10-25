pub mod module;
pub mod section;

use std::num::NonZeroU32;

use configuration::{
    domain::{
        bars::{
            AttachmentMode, BarBackdrop, BarBlend, BarConfig, BarLayer, BarsConfig, MajorAxisSize,
            ScreenCornerClaimer,
        },
        general::Font,
    },
    shared::{Alpha, Border, Edge},
};
use hex_color::HexColor;

use crate::bar::section::BarSection;

#[derive(Debug)]
pub struct FourSideData {
    pub top: u32,
    pub right: u32,
    pub bottom: u32,
    pub left: u32,
}

#[derive(Debug)]
pub struct Bar {
    pub enabled: bool,
    pub exclusive_zone: bool,
    pub edge: Edge,
    pub layer: BarLayer,
    pub islands: bool,
    pub autohide: bool,
    pub border: Border,
    pub border_radius: u32,
    pub section_border_radius: u32,
    pub backdrop: BarBackdrop,
    pub background: HexColor,
    pub foreground: HexColor,
    pub contrast: bool,
    pub opacity: Alpha,
    pub attachment_mode: AttachmentMode,
    pub major_axis_size: MajorAxisSize,
    pub minor_axis_size: NonZeroU32,
    pub margin: FourSideData,
    pub padding: FourSideData,
    pub font: Option<Font>,
    pub sections: Vec<BarSection>,
}

impl Bar {
    pub fn is_horizontal(edge: Edge) -> bool {
        matches!(edge, Edge::Top | Edge::Bottom)
    }

    pub fn from_config(config: &BarConfig) -> Self {
        let mut sections = Vec::new();
        for cfg in config.sections.iter() {
            sections.push(BarSection::from_config(&cfg));
        }
        let is_horizontal = Bar::is_horizontal(config.edge);
        let margin = if is_horizontal {
            FourSideData {
                top: config.minor_axis_margin,
                right: config.major_axis_margin,
                bottom: config.minor_axis_margin,
                left: config.major_axis_margin,
            }
        } else {
            FourSideData {
                top: config.major_axis_margin,
                right: config.minor_axis_margin,
                bottom: config.major_axis_margin,
                left: config.minor_axis_margin,
            }
        };
        let padding = if is_horizontal {
            FourSideData {
                top: config.minor_axis_padding,
                right: config.major_axis_padding,
                bottom: config.minor_axis_padding,
                left: config.major_axis_padding,
            }
        } else {
            FourSideData {
                top: config.major_axis_padding,
                right: config.minor_axis_padding,
                bottom: config.major_axis_padding,
                left: config.minor_axis_padding,
            }
        };

        Self {
            enabled: config.enabled,
            exclusive_zone: true,
            edge: config.edge,
            layer: config.layer,
            islands: config.islands,
            autohide: config.autohide,
            border: config.border,
            border_radius: config.border_radius,
            section_border_radius: config.section_border_radius,
            backdrop: config.backdrop,
            background: config.background,
            foreground: config.foreground,
            contrast: config.contrast,
            opacity: config.opacity,
            attachment_mode: config.attachment_mode,
            major_axis_size: config.major_axis_size,
            minor_axis_size: config.minor_axis_size,
            margin,
            padding,
            font: None,
            sections,
        }
    }
}

#[derive(Debug)]
pub struct Bars {
    pub top: Bar,
    pub right: Bar,
    pub bottom: Bar,
    pub left: Bar,
    pub screen_corner_claimer: ScreenCornerClaimer,
    pub blend: BarBlend,
}

impl Bars {
    pub fn from_config(config: &BarsConfig) -> Self {
        Self {
            top: Bar::from_config(&config.top),
            right: Bar::from_config(&config.right),
            bottom: Bar::from_config(&config.bottom),
            left: Bar::from_config(&config.left),
            screen_corner_claimer: config.screen_corner_claimer,
            blend: config.blend,
        }
    }
}

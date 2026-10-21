pub mod module;
pub mod section;

use std::num::NonZeroU32;

use configuration::domain::{
    bars::{AttachmentMode, BarBackdrop, BarConfig, BarLayer, Border, Edge, MajorAxisSize},
    general::Font,
};
use hex_color::HexColor;

use crate::bar::section::BarSection;

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
    pub opacity: u32,
    pub attachment_mode: AttachmentMode,
    pub major_axis_size: MajorAxisSize,
    pub minor_axis_size: NonZeroU32,
    pub major_axis_margin: u32,
    pub minor_axis_margin: u32,
    pub major_axis_padding: u32,
    pub minor_axis_padding: u32,
    pub font: Option<Font>,
    pub sections: Vec<BarSection>,
}

impl Bar {
    pub fn from_config(config: &BarConfig) -> Self {
        let sections = Vec::new();
        for cfg in config.sections {
            sections.push(BarSection::from_config(&cfg));
        }
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
            opacity: config.opacity,
            attachment_mode: config.attachment_mode,
            major_axis_size: config.major_axis_size,
            minor_axis_size: config.minor_axis_size,
            major_axis_margin: config.major_axis_margin,
            minor_axis_margin: config.minor_axis_margin,
            major_axis_padding: config.major_axis_padding,
            minor_axis_padding: config.minor_axis_padding,
            font: None,
            sections,
        }
    }
}

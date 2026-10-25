pub mod module;
pub mod section;

use std::num::NonZeroU32;

use hex_color::HexColor;

use crate::{
    Alpha, AttachmentMode, Backdrop, BarBlend, Border, Edge, Font, Insets, Layer, MajorAxisSize,
    ScreenCornerClaimer, bar::section::BarSection,
};

#[derive(Debug)]
pub struct Bar {
    pub enabled: bool,
    pub exclusive_zone: bool,
    pub edge: Edge,
    pub layer: Layer,
    pub islands: bool,
    pub autohide: bool,
    pub border: Border,
    pub border_radius: u32,
    pub section_border_radius: u32,
    pub backdrop: Backdrop,
    pub background: HexColor,
    pub foreground: HexColor,
    pub contrast: bool,
    pub opacity: Alpha,
    pub attachment_mode: AttachmentMode,
    pub size: Insets<u32>,
    pub margin: Insets<u32>,
    pub padding: Insets<u32>,
    pub section_padding: Insets<u32>,
    pub font: Option<Font>,
    pub sections: Vec<BarSection>,
}

<<<<<<< Updated upstream
impl Bar {
    pub fn is_horizontal(edge: Edge) -> bool {
        matches!(edge, Edge::Top | Edge::Bottom)
=======
impl Default for Bar {
    fn default() -> Self {
        Self {
            enabled: Default::default(),
            exclusive_zone: Default::default(),
            edge: Default::default(),
            layer: Default::default(),
            islands: Default::default(),
            autohide: Default::default(),
            border: Default::default(),
            border_radius: Default::default(),
            section_border_radius: Default::default(),
            backdrop: Default::default(),
            background: Default::default(),
            foreground: Default::default(),
            opacity: Default::default(),
            attachment_mode: Default::default(),
            major_axis_size: Default::default(),
            minor_axis_size: NonZeroU32::new(24).unwrap(),
            major_axis_margin: Default::default(),
            minor_axis_margin: Default::default(),
            major_axis_padding: Default::default(),
            minor_axis_padding: Default::default(),
            font: Default::default(),
            sections: Default::default(),
        }
>>>>>>> Stashed changes
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

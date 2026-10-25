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

impl Bar {
    pub fn is_horizontal(edge: Edge) -> bool {
        matches!(edge, Edge::Top | Edge::Bottom)
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

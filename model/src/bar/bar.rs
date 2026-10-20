use std::num::NonZeroU32;

use configuration::domain::{
    bars::{AttachmentMode, BarBackdrop, BarLayer, Border, Edge, MajorAxisSize},
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

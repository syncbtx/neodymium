pub mod module;
pub mod section;

use hex_color::HexColor;

use crate::{
    Alpha, AttachmentMode, Backdrop, BarBlend, Border, Edge, Font, Insets, Layer, MainAxisSize,
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
    pub main_axis_size: MainAxisSize,
    pub cross_axis_size: u32,
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

impl Default for Bar {
    fn default() -> Self {
        Self {
            enabled: false,
            exclusive_zone: true,
            edge: Edge::default(),
            layer: Layer::default(),
            islands: true,
            autohide: false,
            border: Border::default(),
            border_radius: 8,
            section_border_radius: 8,
            backdrop: Backdrop::None,
            background: HexColor::BLACK,
            foreground: HexColor::WHITE,
            contrast: true,
            opacity: Alpha::default(),
            attachment_mode: AttachmentMode::default(),
            main_axis_size: MainAxisSize::default(),
            cross_axis_size: 8,
            margin: Insets {
                top: 8,
                right: 8,
                bottom: 8,
                left: 8,
            },
            padding: Insets {
                top: 8,
                right: 8,
                bottom: 8,
                left: 8,
            },
            section_padding: Insets {
                top: 8,
                right: 8,
                bottom: 8,
                left: 8,
            },
            font: Some(Font::default()),
            sections: Vec::new(),
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

impl IntoIterator for Bars {
    type Item = Bar;

    type IntoIter = std::array::IntoIter<Bar, 4>;

    fn into_iter(self) -> Self::IntoIter {
        [self.top, self.right, self.bottom, self.left].into_iter()
    }
}

impl<'a> IntoIterator for &'a mut Bars {
    type Item = &'a mut Bar;
    type IntoIter = std::array::IntoIter<&'a mut Bar, 4>;

    fn into_iter(self) -> Self::IntoIter {
        [
            &mut self.top,
            &mut self.right,
            &mut self.bottom,
            &mut self.left,
        ]
        .into_iter()
    }
}

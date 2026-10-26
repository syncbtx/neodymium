use amane::{Align, Column, Justify, Row, Widget};
use hex_color::HexColor;
use model::{Font, bar::section::BarSection};

use crate::module::module_view;

pub fn section_view(section: &BarSection, is_bar_horizontal: bool, font: Font) -> Box<dyn Widget> {
    let module_views = section
        .modules
        .iter()
        .map(|m| {
            module_view(
                *m,
                is_bar_horizontal,
                font.clone(),
                HexColor::YELLOW,
                HexColor::BLACK,
                12,
                12,
            )
        })
        .collect();

    if is_bar_horizontal {
        Box::new(
            Row::new(module_views)
                .gap(12.0)
                .align(Align::Center)
                .justify(Justify::Center),
        )
    } else {
        Box::new(
            Column::new(module_views)
                .gap(12.0)
                .align(Align::Center)
                .justify(Justify::Center),
        )
    }
}

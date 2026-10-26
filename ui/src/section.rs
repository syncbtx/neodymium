use amane::{Align, Column, Justify, LayerWindow, Rectangle, Row, Widget};
use model::bar::section::BarSection;

use crate::module;

fn horizontal_view(section: &BarSection) -> Row {
    let modules: Vec<Box<dyn Widget>> = section
        .modules
        .iter()
        .map(|m| Box::new(module::horizontal_view(m)) as Box<dyn Widget>)
        .collect();

    Row::new(modules)
        .gap(section.spacing as f32)
        .justify(Justify::Center)
        .align(Align::Center)
}

// fn vertical_view(section: &BarSection) -> Column {
//     let modules: Vec<Box<dyn Widget>> = section
//         .modules
//         .iter()
//         .map(|m| Box::new(module::vertical_view(m)) as Box<dyn Widget>)
//         .collect();

//     Column::new(modules)
//         .gap(section.spacing as f32)
//         .justify(Justify::Center)
//         .align(Align::Center)
// }

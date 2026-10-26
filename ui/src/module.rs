use amane::{Color, Rectangle, Text};
use model::bar::module::Module;

pub fn horizontal_view(module: &Module) -> Rectangle {
    let height = 20.0;
    match module {
        Module::Spacer(gap) => Rectangle::new().width(*gap as f32).height(height),
        Module::Workspaces { .. } => Rectangle::new().width(height * 2.0).height(height),
        Module::ActiveWindowTitle => Rectangle::new()
            .width(height * 2.0)
            .height(height)
            .child(Text::new("Window Title").color(Color::BLUE)),
        _ => Rectangle::new().width(height).height(height),
    }
}

// pub fn vertical_view(module: &Module) -> Rectangle {
//     let width = 20.0;
//     match module {
//         Module::Spacer(gap) => Rectangle::new().width(width).height(*gap as f32),
//         Module::Workspaces(_) => Rectangle::new().width(width).height(width * 2.0),
//         Module::WindowTitle(_) => Rectangle::new()
//             .width(width)
//             .height(width * 2.0)
//             .child(Text::new("Window Title").color(Color::BLUE)),
//     }
// }

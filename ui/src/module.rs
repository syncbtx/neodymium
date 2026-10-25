use amane::{Color, Rectangle, Text};
use model::bar::module::BarModule;

pub fn horizontal_view(module: &BarModule) -> Rectangle {
    let height = 20.0;
    match module {
        BarModule::Spacer(gap) => Rectangle::new().width(*gap as f32).height(height),
        BarModule::Workspaces(_) => Rectangle::new().width(height * 2.0).height(height),
        BarModule::WindowTitle(_) => Rectangle::new()
            .width(height * 2.0)
            .height(height)
            .child(Text::new("Window Title").color(Color::BLUE)),
    }
}

pub fn vertical_view(module: &BarModule) -> Rectangle {
    let width = 20.0;
    match module {
        BarModule::Spacer(gap) => Rectangle::new().width(width).height(*gap as f32),
        BarModule::Workspaces(_) => Rectangle::new().width(width).height(width * 2.0),
        BarModule::WindowTitle(_) => Rectangle::new()
            .width(width)
            .height(width * 2.0)
            .child(Text::new("Window Title").color(Color::BLUE)),
    }
}

use crate::bar::module::{window_title::WindowTitle, workspaces::Workspaces};

pub mod window_title;
pub mod workspaces;

pub enum BarModule {
    Spacer(u32),
    WindowTitle(WindowTitle),
    Workspaces(Workspaces),
}

use configuration::domain::bars::ModuleConfig;

use crate::bar::module::{window_title::WindowTitle, workspaces::Workspaces};

pub mod window_title;
pub mod workspaces;

#[derive(Debug)]
pub enum BarModule {
    Spacer(u32),
    Workspaces(Workspaces),
    WindowTitle(WindowTitle),
}

impl BarModule {
    pub fn from_config(config: &ModuleConfig) -> Self {
        match config {
            ModuleConfig::WindowTitle => Self::WindowTitle(WindowTitle),
            ModuleConfig::Workspaces { .. } => Self::Workspaces(Workspaces),
            ModuleConfig::Spacer(gap) => Self::Spacer(*gap),
        }
    }
}

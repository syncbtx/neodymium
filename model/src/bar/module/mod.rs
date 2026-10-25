use crate::bar::module::workspaces::Workspaces;

pub mod workspaces;

#[derive(Debug)]
pub enum BarModule {
    Spacer(u32),
    Workspaces(Workspaces),
}

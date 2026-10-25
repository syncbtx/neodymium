#[derive(Debug)]
pub enum Module {
    Workspaces {
        collape_unused: bool,
    },
    ActiveWindowTitle,
    Calendar {
        time: bool,
        date: bool,
        compact: bool,
    },
}

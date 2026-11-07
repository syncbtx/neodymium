use amane::{Image, Rectangle, Widget};
use model::bar::module::{
    LinkedPanel,
    ModelIcon::{self},
};

use crate::module::ModuleViewArgs;

pub fn view(
    module_view_args: &ModuleViewArgs,
    icon: ModelIcon,
    linked_panel: LinkedPanel,
) -> Box<dyn Widget> {
    let ModuleViewArgs { thickness, .. } = *module_view_args;

    let _ = linked_panel;

    let path = match icon {
        ModelIcon::Distro => "/usr/share/icons/cachyos.svg",
        ModelIcon::Compositor | ModelIcon::Shell => {
            &concat!(env!("CARGO_MANIFEST_DIR"), "/assets/icons/niri.svg")
        }
    };

    Box::new(
        Rectangle::new()
            .width(thickness)
            .height(thickness)
            .fill(Image::cover(path).thumbnail(thickness as u32, thickness as u32)),
    ) as Box<dyn Widget>
}

mod clock;
pub mod distro;
mod focused;
pub mod motion;
mod system;
mod theme;

pub use clock::*;
pub use focused::*;
pub use motion::{Spec, animate, phase, smooth};
pub use theme::*;

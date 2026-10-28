use amane::{LayerWindow, Monitor};
use model::ScreenCornerClaimer;

pub fn ordered(
    top: fn(&Monitor) -> LayerWindow,
    right: fn(&Monitor) -> LayerWindow,
    bottom: fn(&Monitor) -> LayerWindow,
    left: fn(&Monitor) -> LayerWindow,
    scc: ScreenCornerClaimer,
) -> [fn(&Monitor) -> LayerWindow; 4] {
    match scc {
        ScreenCornerClaimer::HorizontalBars => [top, bottom, right, left],
        ScreenCornerClaimer::VerticalBars => [right, left, top, bottom],
    }
}

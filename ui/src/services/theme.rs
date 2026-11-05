use amane::Service;
use model::palette::Palette;

#[derive(Debug, Clone)]
pub struct Theme {
    pub palette: Palette,
    pub opacity: u8,
}

impl Default for Theme {
    fn default() -> Self {
        Self {
            palette: Palette::fallback_dark(),
            opacity: 255,
        }
    }
}

impl Service for Theme {
    fn new() -> Self {
        Self::default()
    }
}

impl Theme {
    pub fn active(&self) -> &Palette {
        &self.palette
    }

    pub fn opacity(&self) -> u8 {
        self.opacity
    }

    pub fn set(&mut self, palette: Palette, opacity: u8) {
        self.palette = palette;
        self.opacity = opacity;
    }
}

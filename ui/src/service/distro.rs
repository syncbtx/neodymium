use amane::{Image, Service};

pub struct Distro {
    pub name: String,
    pub icon: Image,
}

impl Service for Distro {
    fn new() -> Self {
        let name = "".to_string();
        let icon = Image::contain("/usr/share/icons/cachyos.svg");
        Self { name, icon }
    }
}

impl Distro {
    pub fn name(&self) -> String {
        self.name.clone()
    }

    pub fn icon(self, width: u32, height: u32) -> Image {
        self.icon.thumbnail(width, height)
    }
}

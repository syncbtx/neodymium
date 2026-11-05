use hex_color::HexColor;
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

#[derive(Debug, Copy, Clone, PartialEq, Eq, Serialize, Deserialize, JsonSchema)]
#[serde(default, rename_all = "camelCase")]
pub struct Palette<T: Default + 'static = HexColor> {
    pub primary: T,
    #[serde(alias = "on_primary")]
    pub on_primary: T,
    pub secondary: T,
    #[serde(alias = "on_secondary")]
    pub on_secondary: T,
    pub tertiary: T,
    #[serde(alias = "on_tertiary")]
    pub on_tertiary: T,
    pub error: T,
    #[serde(alias = "on_error")]
    pub on_error: T,
    pub surface: T,
    #[serde(alias = "surface_container")]
    pub surface_container: T,
    #[serde(alias = "surface_container_low")]
    pub surface_container_low: T,
    #[serde(alias = "surface_container_high")]
    pub surface_container_high: T,
    #[serde(alias = "surface_container_highest")]
    pub surface_container_highest: T,
    #[serde(alias = "on_surface")]
    pub on_surface: T,
    #[serde(alias = "on_surface_variant")]
    pub on_surface_variant: T,
    pub outline: T,
    #[serde(alias = "outline_variant")]
    pub outline_variant: T,
}

impl<T: Default> Palette<T> {
    pub fn map<U: Default>(self, mut f: impl FnMut(T) -> U) -> Palette<U> {
        Palette {
            primary: f(self.primary),
            on_primary: f(self.on_primary),
            secondary: f(self.secondary),
            on_secondary: f(self.on_secondary),
            tertiary: f(self.tertiary),
            on_tertiary: f(self.on_tertiary),
            error: f(self.error),
            on_error: f(self.on_error),
            surface: f(self.surface),
            surface_container: f(self.surface_container),
            surface_container_low: f(self.surface_container_low),
            surface_container_high: f(self.surface_container_high),
            surface_container_highest: f(self.surface_container_highest),
            on_surface: f(self.on_surface),
            on_surface_variant: f(self.on_surface_variant),
            outline: f(self.outline),
            outline_variant: f(self.outline_variant),
        }
    }
}

impl Palette {
    pub const fn fallback_dark() -> Self {
        Self {
            primary: HexColor::rgb(0xb4, 0xc5, 0xff),
            on_primary: HexColor::rgb(0x1b, 0x2e, 0x60),
            secondary: HexColor::rgb(0xbe, 0xc6, 0xdc),
            on_secondary: HexColor::rgb(0x28, 0x31, 0x41),
            tertiary: HexColor::rgb(0xe2, 0xbb, 0xdb),
            on_tertiary: HexColor::rgb(0x42, 0x27, 0x40),
            error: HexColor::rgb(0xff, 0xb4, 0xab),
            on_error: HexColor::rgb(0x69, 0x00, 0x05),
            surface: HexColor::rgb(0x12, 0x13, 0x18),
            surface_container_low: HexColor::rgb(0x1a, 0x1b, 0x21),
            surface_container: HexColor::rgb(0x1e, 0x1f, 0x25),
            surface_container_high: HexColor::rgb(0x28, 0x2a, 0x2f),
            surface_container_highest: HexColor::rgb(0x34, 0x34, 0x3a),
            on_surface: HexColor::rgb(0xe3, 0xe2, 0xe9),
            on_surface_variant: HexColor::rgb(0xc6, 0xc6, 0xd0),
            outline: HexColor::rgb(0x8f, 0x90, 0x9a),
            outline_variant: HexColor::rgb(0x45, 0x46, 0x4f),
        }
    }

    pub const fn fallback_light() -> Self {
        Self {
            primary: HexColor::rgb(0x41, 0x5f, 0x91),
            on_primary: HexColor::rgb(0xff, 0xff, 0xff),
            secondary: HexColor::rgb(0x56, 0x5f, 0x71),
            on_secondary: HexColor::rgb(0xff, 0xff, 0xff),
            tertiary: HexColor::rgb(0x70, 0x55, 0x74),
            on_tertiary: HexColor::rgb(0xff, 0xff, 0xff),
            error: HexColor::rgb(0xba, 0x1a, 0x1a),
            on_error: HexColor::rgb(0xff, 0xff, 0xff),
            surface: HexColor::rgb(0xf9, 0xf9, 0xff),
            surface_container_low: HexColor::rgb(0xf3, 0xf3, 0xfa),
            surface_container: HexColor::rgb(0xed, 0xed, 0xf4),
            surface_container_high: HexColor::rgb(0xe7, 0xe8, 0xee),
            surface_container_highest: HexColor::rgb(0xe2, 0xe2, 0xe9),
            on_surface: HexColor::rgb(0x19, 0x1c, 0x20),
            on_surface_variant: HexColor::rgb(0x44, 0x47, 0x4f),
            outline: HexColor::rgb(0x74, 0x77, 0x7f),
            outline_variant: HexColor::rgb(0xc4, 0xc6, 0xd0),
        }
    }
}

impl<T: Default + 'static> Default for Palette<T> {
    fn default() -> Self {
        if std::any::TypeId::of::<T>() == std::any::TypeId::of::<HexColor>() {
            let dark = Palette::<HexColor>::fallback_dark();
            unsafe { std::mem::transmute_copy(&dark) }
        } else {
            Self {
                primary: T::default(),
                on_primary: T::default(),
                secondary: T::default(),
                on_secondary: T::default(),
                tertiary: T::default(),
                on_tertiary: T::default(),
                error: T::default(),
                on_error: T::default(),
                surface: T::default(),
                surface_container: T::default(),
                surface_container_low: T::default(),
                surface_container_high: T::default(),
                surface_container_highest: T::default(),
                on_surface: T::default(),
                on_surface_variant: T::default(),
                outline: T::default(),
                outline_variant: T::default(),
            }
        }
    }
}

#[derive(Debug, Copy, Clone, PartialEq, Eq, Serialize, Deserialize, JsonSchema)]
#[serde(default, rename_all = "camelCase")]
pub struct ModelTheme {
    pub dark: Palette,
    pub light: Palette,
}

impl Default for ModelTheme {
    fn default() -> Self {
        Self {
            dark: Palette::fallback_dark(),
            light: Palette::fallback_light(),
        }
    }
}

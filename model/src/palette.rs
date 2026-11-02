use hex_color::HexColor;
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

#[derive(Debug, Copy, Clone, PartialEq, Eq, Serialize, Deserialize, JsonSchema)]
#[serde(default, rename_all = "camelCase")]
pub struct Palette {
    pub primary: HexColor,
    pub on_primary: HexColor,
    pub secondary: HexColor,
    pub on_secondary: HexColor,
    pub tertiary: HexColor,
    pub on_tertiary: HexColor,
    pub error: HexColor,
    pub on_error: HexColor,
    pub surface: HexColor,
    pub surface_container: HexColor,
    pub surface_container_low: HexColor,
    pub surface_container_high: HexColor,
    pub surface_container_highest: HexColor,
    pub on_surface: HexColor,
    pub on_surface_variant: HexColor,
    pub outline: HexColor,
    pub outline_variant: HexColor,
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

impl Default for Palette {
    fn default() -> Self {
        Self::fallback_dark()
    }
}

#[derive(Debug, Copy, Clone, PartialEq, Eq, Serialize, Deserialize, JsonSchema)]
#[serde(default, rename_all = "camelCase")]
pub struct Theme {
    pub dark: Palette,
    pub light: Palette,
}

impl Default for Theme {
    fn default() -> Self {
        Self {
            dark: Palette::fallback_dark(),
            light: Palette::fallback_light(),
        }
    }
}

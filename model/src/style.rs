use hex_color::HexColor;
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

#[derive(Debug, Copy, Clone, PartialEq, Eq, Default, Serialize, Deserialize, JsonSchema)]
#[serde(rename_all = "camelCase")]
pub enum FontWeight {
    Thin,
    ExtraLight,
    Light,
    #[default]
    Regular,
    Medium,
    SemiBold,
    Bold,
    ExtraBold,
    Black,
}

impl FontWeight {
    pub const fn value(self) -> u16 {
        match self {
            Self::Thin => 100,
            Self::ExtraLight => 200,
            Self::Light => 300,
            Self::Regular => 400,
            Self::Medium => 500,
            Self::SemiBold => 600,
            Self::Bold => 700,
            Self::ExtraBold => 800,
            Self::Black => 900,
        }
    }
}

#[derive(Debug, Copy, Clone, PartialEq, Eq, Default, Serialize, Deserialize, JsonSchema)]
#[serde(rename_all = "camelCase")]
pub enum FontStyle {
    #[default]
    Regular,
    Italic,
    Oblique,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, JsonSchema)]
#[serde(default, deny_unknown_fields, rename_all = "camelCase")]
pub struct Font {
    pub path_or_name: String,
    pub weight: FontWeight,
    pub style: FontStyle,
}

impl Default for Font {
    fn default() -> Self {
        Self {
            path_or_name: "sans-serif".to_owned(),
            weight: FontWeight::default(),
            style: FontStyle::default(),
        }
    }
}

#[derive(Debug, Copy, Clone, PartialEq, Eq, Default, Serialize, Deserialize, JsonSchema)]
#[serde(rename_all = "camelCase")]
pub enum Border {
    #[default]
    None,
    Surround {
        thickness: u32,
        color: HexColor,
    },
    Brackets {
        thickness: u32,
        color: HexColor,
        arm: u32,
    },
}

#[derive(Debug, Copy, Clone, PartialEq, Serialize, Deserialize, JsonSchema)]
#[serde(rename_all = "camelCase")]
pub enum AttachmentMode {
    Floating { margin: u32 },
    Attached { morph: bool },
}

impl Default for AttachmentMode {
    fn default() -> Self {
        Self::Floating { margin: 12 }
    }
}

#[derive(Debug, Copy, Clone, PartialEq, Serialize, JsonSchema)]
#[serde(rename_all = "camelCase")]
pub struct Alpha(pub u8);

impl Default for Alpha {
    fn default() -> Self {
        Self(255)
    }
}

impl<'de> Deserialize<'de> for Alpha {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        let n = u64::deserialize(deserializer)?;
        if n <= 255 {
            return Ok(Alpha(n as u8));
        } else {
            return Err(serde::de::Error::custom(format!(
                "opacity {n} is out of the valid range: 0..255",
            )));
        }
    }
}

#[derive(Debug, Clone, PartialEq, Default, Serialize, Deserialize, JsonSchema)]
#[serde(rename_all = "camelCase")]
pub enum Backdrop {
    #[default]
    None,
    Blur {
        radius: u32,
    },
    Waveform {
        color: HexColor,
        opacity: Alpha,
    },
    Shader {
        name_or_path: String,
    },
}

#[derive(Debug, Copy, Clone, PartialEq, Default, Serialize, Deserialize, JsonSchema)]
#[serde(default, deny_unknown_fields, rename_all = "camelCase")]
pub struct BarBlend {
    pub enabled: bool,
    pub radius: u32,
}

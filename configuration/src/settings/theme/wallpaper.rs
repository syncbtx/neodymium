use hex_color::HexColor;
use model::{Alpha, ModelTheme};
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

use crate::{
    resolution,
    resolve::Resolve,
    settings::{ColorPreference, MatugenOutput, ResizeFilter, SchemeType},
    validate::Sanitize,
};

#[derive(Debug, Copy, Clone, PartialEq, Serialize, Deserialize, JsonSchema)]
#[serde(default, deny_unknown_fields, rename_all = "camelCase")]
pub struct WallpaperTheme {
    pub index: Option<usize>,
    pub contrast: f32,
    pub opacity: Alpha,
    pub scheme_type: SchemeType,
    pub prefer: Option<ColorPreference>,
    pub resize_filter: Option<ResizeFilter>,
    pub lightness_dark: Option<f32>,
    pub lightness_light: Option<f32>,
    pub fallback_color: Option<HexColor>,
}

impl Default for WallpaperTheme {
    fn default() -> Self {
        Self {
            index: None,
            contrast: 0.0,
            opacity: Alpha::default(),
            scheme_type: SchemeType::SchemeTonalSpot,
            prefer: None,
            resize_filter: None,
            lightness_dark: None,
            lightness_light: None,
            fallback_color: None,
        }
    }
}

impl Resolve for WallpaperTheme {
    type Output = Option<ModelTheme>;

    fn resolve(&self) -> Self::Output {
        let path = resolution::resolve_active_wallpaper()?;

        let mut cmd = std::process::Command::new("matugen");

        cmd.arg("image")
            .arg(path)
            .args(["-j", "hex", "-q", "--dry-run"])
            .args(["-t", self.scheme_type.as_str()])
            .args(["--contrast", &self.contrast.to_string()]);

        if let Some(idx) = self.index {
            cmd.args(["--source-color-index", &idx.to_string()]);
        }

        if let Some(pref) = self.prefer {
            cmd.args(["--prefer", pref.as_str()]);
        }

        if let Some(rf) = self.resize_filter {
            cmd.args(["--resize-filter", rf.as_str()]);
        }

        if let Some(ld) = self.lightness_dark {
            cmd.args(["--lightness-dark", &ld.to_string()]);
        }

        if let Some(ll) = self.lightness_light {
            cmd.args(["--lightness-light", &ll.to_string()]);
        }

        if let Some(fb) = self.fallback_color {
            let hex = &format!("{:02x}{:02x}{:02x}", fb.r, fb.g, fb.b);
            cmd.args(["--fallback-color", hex]);
        }

        let output = match cmd.output() {
            Ok(out) => out,
            Err(e) => {
                eprintln!("{e}");
                return None;
            }
        };

        if !output.status.success() {
            return None;
        }

        let parsed: MatugenOutput = serde_json::from_slice(&output.stdout).ok()?;

        Some(ModelTheme {
            dark: parsed.colors.map(|e| e.dark.color),
            light: parsed.colors.map(|e| e.light.color),
        })
    }
}

impl Sanitize for WallpaperTheme {
    fn sanitize(&mut self) {
        if let Some(idx) = &mut self.index {
            *idx = (*idx).min(4);
        }
        self.contrast = self.contrast.clamp(-1.0, 1.0);
        if let Some(ld) = &mut self.lightness_dark {
            *ld = ld.min(1.0);
        }
        if let Some(ll) = &mut self.lightness_light {
            *ll = ll.max(-1.0);
        }
    }
}

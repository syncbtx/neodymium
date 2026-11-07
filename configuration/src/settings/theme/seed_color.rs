use hex_color::HexColor;
use model::{Alpha, ModelTheme};
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

use crate::{
    resolve::Resolve,
    settings::{MatugenOutput, SchemeType},
    validate::Sanitize,
};

#[derive(Debug, Copy, Clone, PartialEq, Serialize, Deserialize, JsonSchema)]
#[serde(default, deny_unknown_fields, rename_all = "camelCase")]
pub struct SeedColorTheme {
    pub seed_color: HexColor,
    pub contrast: f32,
    pub opacity: Alpha,
    pub scheme_type: SchemeType,
    pub lightness_dark: Option<f32>,
    pub lightness_light: Option<f32>,
}

impl Default for SeedColorTheme {
    fn default() -> Self {
        Self {
            seed_color: HexColor::rgb(0x41, 0x5f, 0x91),
            contrast: 0.0,
            opacity: Alpha::default(),
            scheme_type: SchemeType::SchemeTonalSpot,
            lightness_dark: None,
            lightness_light: None,
        }
    }
}

impl Resolve for SeedColorTheme {
    type Output = Option<ModelTheme>;

    fn resolve(&self) -> Self::Output {
        let seed = self.seed_color;

        let hex = &format!("{:02x}{:02x}{:02x}", seed.r, seed.g, seed.b);

        let mut cmd = std::process::Command::new("matugen");

        cmd.arg("color")
            .arg("hex")
            .arg(hex)
            .args(["-j", "hex", "-q", "--dry-run"])
            .args(["-t", self.scheme_type.as_str()])
            .args(["--contrast", &self.contrast.to_string()]);

        if let Some(ld) = self.lightness_dark {
            cmd.args(["--lightness-dark", &ld.to_string()]);
        }

        if let Some(ll) = self.lightness_light {
            cmd.args(["--lightness-light", &ll.to_string()]);
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

impl Sanitize for SeedColorTheme {
    fn sanitize(&mut self) {
        self.contrast = self.contrast.clamp(-1.0, 1.0);
        if let Some(ld) = &mut self.lightness_dark {
            *ld = ld.min(1.0);
        }
        if let Some(ll) = &mut self.lightness_light {
            *ll = ll.max(-1.0);
        }
    }
}

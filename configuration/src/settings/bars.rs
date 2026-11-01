use model::{BarBlend, Edge, ScreenCornerClaimer, bar::Bars};
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

use crate::{resolve::Resolve, settings::bar::BarSettings, validate::Sanitize};

#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema)]
#[serde(default, deny_unknown_fields, rename_all = "camelCase")]
pub struct BarsSettings {
    pub top: BarSettings,
    pub right: BarSettings,
    pub bottom: BarSettings,
    pub left: BarSettings,
    pub screen_corner_claimer: ScreenCornerClaimer,
    pub blend: BarBlend,
}

impl Default for BarsSettings {
    fn default() -> Self {
        let mut top = BarSettings::default();
        top.enabled = true;
        Self {
            top: top,
            right: BarSettings::default(),
            bottom: BarSettings::default(),
            left: BarSettings::default(),
            screen_corner_claimer: ScreenCornerClaimer::default(),
            blend: BarBlend::default(),
        }
    }
}

impl<'a> IntoIterator for &'a BarsSettings {
    type Item = &'a BarSettings;
    type IntoIter = std::array::IntoIter<&'a BarSettings, 4>;

    fn into_iter(self) -> Self::IntoIter {
        [&self.top, &self.right, &self.bottom, &self.left].into_iter()
    }
}

impl<'a> IntoIterator for &'a mut BarsSettings {
    type Item = &'a mut BarSettings;
    type IntoIter = std::array::IntoIter<&'a mut BarSettings, 4>;

    fn into_iter(self) -> Self::IntoIter {
        [
            &mut self.top,
            &mut self.right,
            &mut self.bottom,
            &mut self.left,
        ]
        .into_iter()
    }
}

impl BarsSettings {
    pub fn iter(&self) -> std::array::IntoIter<&BarSettings, 4> {
        self.into_iter()
    }

    pub fn iter_mut(&mut self) -> std::array::IntoIter<&mut BarSettings, 4> {
        self.into_iter()
    }
}

impl Resolve for BarsSettings {
    type Output = Bars;

    fn resolve(&self) -> Self::Output {
        Bars {
            top: self.top.resolve(),
            right: self.right.resolve(),
            bottom: self.bottom.resolve(),
            left: self.left.resolve(),
            screen_corner_claimer: self.screen_corner_claimer,
            blend: self.blend,
        }
    }
}

impl Sanitize for BarsSettings {
    fn sanitize(&mut self) {
        self.top.edge = Edge::Top;
        self.right.edge = Edge::Right;
        self.bottom.edge = Edge::Bottom;
        self.left.edge = Edge::Left;
    }
}

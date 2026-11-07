use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

#[derive(Debug, Copy, Clone, PartialEq, Eq, Serialize, Deserialize, JsonSchema)]
#[serde(rename_all = "camelCase")]
pub enum SystemModule {
    Cpu,
    Ram,
    Disk,
    Temp,
}

impl SystemModule {
    pub fn label(&self) -> &str {
        match self {
            SystemModule::Cpu => "cpu",
            SystemModule::Ram => "ram",
            SystemModule::Disk => "disk",
            SystemModule::Temp => "temp",
        }
    }
}

#[derive(Debug, Copy, Clone, PartialEq, Eq, Serialize, Deserialize, JsonSchema, Default)]
#[serde(rename_all = "camelCase")]
pub enum SystemModuleStyle {
    #[default]
    Ring,
    #[serde(alias = "wavyRing", alias = "wavy_ring", alias = "wavey_ring")]
    WaveyRing,
    Bar,
    #[serde(alias = "wavyBar", alias = "wavy_bar", alias = "wavey_bar")]
    WaveyBar,
    Graph,
    Text,
}

#[derive(Debug, Copy, Clone, PartialEq, Eq, Serialize, Deserialize, JsonSchema, Default)]
#[serde(rename_all = "camelCase")]
pub enum WorkspacePillStyle {
    Circle,
    #[default]
    Square,
}

#[derive(Debug, Copy, Clone, PartialEq, Eq, Serialize, Deserialize, JsonSchema, Default)]
#[serde(rename_all = "camelCase")]
pub enum LinkedPanel {
    #[default]
    None,
    PowerMenu,
    Launcher,
}

#[derive(Debug, Copy, Clone, PartialEq, Eq, Serialize, Deserialize, JsonSchema, Default)]
#[serde(rename_all = "camelCase")]
pub enum ModelIcon {
    #[default]
    Distro,
    Compositor,
    Shell,
}

#[derive(Debug, Copy, Clone, PartialEq, Eq, Serialize, Deserialize, JsonSchema)]
#[serde(rename_all = "camelCase")]
pub enum Module {
    Empty {
        main_axis_size: u32,
    },
    Spacer(u32),
    Icon {
        icon: ModelIcon,
        linked_panel: LinkedPanel,
    },
    #[serde(rename_all = "camelCase")]
    Workspaces {
        #[serde(
            default,
            alias = "collape_unused",
            alias = "collapeUnused",
            alias = "collapse_unused"
        )]
        collapse_unused: bool,
        style: WorkspacePillStyle,
    },
    FocusedWindowIcon,
    #[serde(rename_all = "camelCase")]
    Calendar {
        #[serde(default)]
        show_time: bool,
        #[serde(default)]
        format_24_hrs: bool,
        #[serde(default)]
        show_date: bool,
        #[serde(default)]
        compact: bool,
    },
    SystemModuleCapsule {
        #[serde(default)]
        label: bool,
        module: SystemModule,
        #[serde(default)]
        style: SystemModuleStyle,
    },
    Media,
    Bluetooth,
    Divider,
    Ring,
}

impl Default for Module {
    fn default() -> Self {
        Self::FocusedWindowIcon
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_system_module_style_deserialization() {
        let json_ring = r#"{"systemModuleCapsule": {"module": "cpu", "style": "ring"}}"#;
        let m: Module = serde_json::from_str(json_ring).unwrap();
        assert_eq!(
            m,
            Module::SystemModuleCapsule {
                label: false,
                module: SystemModule::Cpu,
                style: SystemModuleStyle::Ring,
            }
        );

        let json_wavy_ring =
            r#"{"systemModuleCapsule": {"label": true, "module": "ram", "style": "wavyRing"}}"#;
        let m: Module = serde_json::from_str(json_wavy_ring).unwrap();
        assert_eq!(
            m,
            Module::SystemModuleCapsule {
                label: true,
                module: SystemModule::Ram,
                style: SystemModuleStyle::WaveyRing,
            }
        );

        let json_wavy_bar = r#"{"systemModuleCapsule": {"module": "cpu", "style": "wavyBar"}}"#;
        let m: Module = serde_json::from_str(json_wavy_bar).unwrap();
        assert_eq!(
            m,
            Module::SystemModuleCapsule {
                label: false,
                module: SystemModule::Cpu,
                style: SystemModuleStyle::WaveyBar,
            }
        );

        let json_default = r#"{"systemModuleCapsule": {"module": "cpu"}}"#;
        let m: Module = serde_json::from_str(json_default).unwrap();
        assert_eq!(
            m,
            Module::SystemModuleCapsule {
                label: false,
                module: SystemModule::Cpu,
                style: SystemModuleStyle::Ring,
            }
        );
    }
}

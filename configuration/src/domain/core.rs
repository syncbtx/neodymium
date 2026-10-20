use crate::{
    domain::{bars::BarsConfig, general::GeneralConfig},
    resolution::resolve_domain,
    validate::ConfigValidate,
};

#[derive(Debug, Default)]
pub struct CoreConfig {
    pub bars: BarsConfig,
    pub general: GeneralConfig,
}

impl CoreConfig {
    pub fn aggregate() -> Result<Self, String> {
        Ok(Self {
            bars: resolve_domain::<BarsConfig>()?,
            general: resolve_domain::<GeneralConfig>()?,
        })
    }
}

impl ConfigValidate for CoreConfig {
    fn validate(&self) -> Result<(), Vec<crate::validate::ValidationError>> {
        let mut errors = Vec::new();

        if let Err(e) = self.bars.validate() {
            errors.extend(e);
        }
        if let Err(e) = self.general.validate() {
            errors.extend(e);
        }

        if errors.is_empty() {
            Ok(())
        } else {
            Err(errors)
        }
    }
}

use configuration::ConfigSchema;

fn main() {
    let _ = configuration::bars::BarsConfig::save_schema();
    let _ = configuration::general::GeneralConfig::save_schema();
}

use configuration::domain::ConfigDomain;

fn main() {
    let _ = configuration::domain::bars::BarsConfig::save_schema();
    let _ = configuration::domain::general::GeneralConfig::save_schema();
}

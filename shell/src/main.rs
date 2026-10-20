use configuration::{engine::ConfigEngine, validate::ConfigValidate};

fn main() {
    match ConfigEngine::init() {
        Ok(engine) => {
            println!("Config Initialized {:#?}", engine.core_config);
            match engine.core_config.validate() {
                Ok(_) => {}
                Err(errors) => {
                    if errors.is_empty() {
                        println!("\x1b[32mconfig validation is successful\x1b[32m")
                    } else {
                        for e in errors {
                            println!("\x1b[30m{}\x1b[0m", e)
                        }
                    }
                }
            }
        }
        Err(e) => eprintln!("Error {e}"),
    }
}

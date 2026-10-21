use configuration::engine::ConfigEngine;

use model::bar::{self, Bar};

#[derive(Debug, Default)]
pub struct Bars {
    pub top: Bar,
    pub right: Bar,
    pub bottom: Bar,
    pub left: Bar,
}

#[derive(Debug)]
pub struct Shell {
    pub config_engine: ConfigEngine,
    pub bars: Bars,
}

impl Default for Shell {
    fn default() -> Self {
        Self {
            config_engine: Default::default(),
            bars: Default::default(),
        }
    }
}

impl Shell {
    pub fn init(&mut self) -> Result<(), String> {
        match ConfigEngine::init() {
            Ok(engine) => {
                let bars = Bars {
                    top: Bar::from_config(&engine.core_config.bars.top),
                    right: Bar::from_config(&engine.core_config.bars.right),
                    bottom: Bar::from_config(&engine.core_config.bars.bottom),
                    left: Bar::from_config(&engine.core_config.bars.left),
                };
                self.bars = bars;
                Ok(())
            }
            Err(e) => Err(e),
        }
    }
}

fn main() {
    let mut shell = Shell::default();
    shell.init();
    println!("{:#?}", shell);
}

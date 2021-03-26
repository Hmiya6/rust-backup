use crate::config;
use config::Config;
// use crate::Backupper;
use std::path::Path;
use anyhow::Result;

pub struct App {
    // interface: DebugInterface,
    config: Config,
    // backupper: Backupper,
    // snapshot: String,
}

impl App {
    pub fn new(config_path: &Path) -> Result<Self> {
        let config = Config::new(config_path)?;
        Ok(Self {
            config,
        })
        
    }

    pub fn run(&self) {
        loop {
            println!("running");
            std::thread::sleep(std::time::Duration::from_secs(60));
        }
    }
}


#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_new_app() {
        let _ = App::new(&std::path::Path::new("test_config.toml")).unwrap();
    }
}

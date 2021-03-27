use crate::config;
use config::Config;
use crate::Backup;
use std::path::Path;
use anyhow::Result;

// App -------------------------------------
pub struct App {
    // interface: DebugInterface,
    config: Config,
    backup: Backup,
}

impl App {
    pub fn new(config_path: &Path) -> Result<Self> {
        let config = Config::new(config_path)?;
        let (src, dst) = config.get_config();
        let backup = Backup::new(src, dst)?;
        Ok(Self {
            config,
            backup,
        })
        
    }

    pub fn run(&mut self) {
        loop {
            println!("running");
            match self.backup.run() {
                Ok(_) => {
                    println!("success");
                },
                Err(e) => {
                    println!("error!: {}", e);
                }
            }
            std::thread::sleep(std::time::Duration::from_secs(60*60*60));
        }
    }
}
// ----------------------------------------

// tests ----------------------------------
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_new_app() {
        let _ = App::new(&std::path::Path::new("test_config.toml")).unwrap();
    }
}

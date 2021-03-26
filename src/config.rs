use std::fs;
use std::path::Path;
use anyhow::Result;
use toml;
use serde::Deserialize;

/*
enum FileType {
    Zip(String),
    Tar(String),
}
*/

#[derive(Deserialize)]
pub struct Config {
    src: String,
    dst: String,
    file_name: String,
}

impl Config {
    pub fn new(toml_path: &Path) -> Result<Self> {
        let toml_string = match fs::read_to_string(toml_path) {
            Ok(string) => string,
            Err(_) => {
                return Ok(Config::default())
            },
        };
        let config: Config = toml::from_str(&toml_string)?;
        Ok(config)
    }

    pub fn default() -> Self {
        Self {
            src: String::from("."),
            dst: String::from("~/rust-backup"),
            file_name: String::from("backup"),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_config_from_toml() {
        let config = Config::new(&Path::new("test_config.toml")).unwrap();
        assert_eq!(config.src, String::from("foo"));
        assert_eq!(config.dst, String::from("bar"));
        assert_eq!(config.file_name, String::from("poyoyo"));
    }
    
    #[test]
    fn test_default_config() {
        let config = Config::default();
        assert_eq!(config.dst, String::from("~/rust-backup"));
    }
}

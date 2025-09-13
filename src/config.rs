use crate::error::{AstgenError, Result};
use serde::{Deserialize, Serialize};
use std::path::PathBuf;

#[derive(Serialize, Deserialize, Debug, Default)]
pub struct Config {
    pub patterns: Option<PatternConfig>,
    pub ignore: Option<IgnoreConfig>,
    pub output: Option<OutputConfig>,
    pub performance: Option<PerformanceConfig>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct PatternConfig {
    pub rust: Option<Vec<String>>,
    pub python: Option<Vec<String>>,
    pub javascript: Option<Vec<String>>,
    pub java: Option<Vec<String>>,
    pub go: Option<Vec<String>>,
    pub typescript: Option<Vec<String>>,
    pub csharp: Option<Vec<String>>,
    pub ruby: Option<Vec<String>>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct IgnoreConfig {
    pub patterns: Option<Vec<String>>,
    pub directories: Option<Vec<String>>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct OutputConfig {
    pub format: Option<String>,
    pub truncate: Option<usize>,
    pub pretty: Option<bool>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct PerformanceConfig {
    pub max_threads: Option<usize>,
    pub max_file_size_mb: Option<usize>,
    pub parser_pool_size: Option<usize>,
}

impl Config {
    pub fn load(path: &PathBuf) -> Result<Self> {
        let content = std::fs::read_to_string(path).map_err(|e| {
            AstgenError::ConfigError(format!("Cannot read config file {}: {}", path.display(), e))
        })?;
        let config: Config = toml::from_str(&content)
            .map_err(|e| AstgenError::ConfigError(format!("Invalid config file {}: {}\n\nCheck the TOML syntax and ensure all required fields are present.", path.display(), e)))?;
        Ok(config)
    }

    pub fn find_default() -> Option<PathBuf> {
        // Look for .astgenrc in current directory, then home directory
        if let Ok(current_dir) = std::env::current_dir() {
            let config_file = current_dir.join(".astgenrc");
            if config_file.exists() {
                return Some(config_file);
            }
        }

        if let Some(home_dir) = dirs::home_dir() {
            let home_config = home_dir.join(".astgenrc");
            if home_config.exists() {
                return Some(home_config);
            }
        }

        None
    }

    pub fn load_default() -> Result<Self> {
        if let Some(config_path) = Self::find_default() {
            Self::load(&config_path)
        } else {
            Ok(Self::default())
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs::File;
    use std::io::Write;
    use tempfile::tempdir;

    #[test]
    fn test_load_valid_config() {
        let dir = tempdir().unwrap();
        let file_path = dir.path().join("test.astgenrc");
        let content = "[output]\nformat = 'json'\ntruncate = 100\n".replace("\\n", "\n");
        let mut file = File::create(&file_path).unwrap();
        file.write_all(content.as_bytes()).unwrap();
        let config = Config::load(&file_path).unwrap();
        assert_eq!(config.output.unwrap().format.unwrap(), "json");
    }

    #[test]
    fn test_load_invalid_config() {
        let dir = tempdir().unwrap();
        let file_path = dir.path().join("bad.astgenrc");
        let content = "not a toml file";
        let mut file = File::create(&file_path).unwrap();
        file.write_all(content.as_bytes()).unwrap();
        let err = Config::load(&file_path).unwrap_err();
        match err {
            AstgenError::ConfigError(_) => {}
            _ => panic!("Expected ConfigError, got: {:?}", err),
        }
    }

    #[test]
    fn test_find_default_none() {
        // Should not find a config in a temp dir with none present
        let dir = tempdir().unwrap();
        let _ = std::env::set_current_dir(&dir);
        assert!(Config::find_default().is_none());
    }
}

use std::collections::HashMap;
use std::path::Path;
use std::fs;
use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize)]
enum AppendType {
    Top,
    Bottom,
}

#[derive(Serialize, Deserialize)]
pub struct ConfigFile {
    default_upgrade: Option<String>,
    version_type: HashMap<String, String>,
    version_format: String,
    changelog_paths: Vec<String>,
    categories: Vec<String>,
    append_position: AppendType,
}

impl ConfigFile {
    pub fn new(path: &Path) -> Result<ConfigFile, Box<dyn std::error::Error + 'static>> {
        let contents = fs::read_to_string(path)?;
        let config: ConfigFile = serde_json::from_str(&contents)?;
        Ok(config)
    }
}

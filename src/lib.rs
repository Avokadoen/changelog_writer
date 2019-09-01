use std::collections::HashMap;
use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize)]
pub struct ConfigFile {
    default_upgrade: Option<String>,
    version_types: HashMap<String, String>,
    version_format: String,
    changelog_paths: Vec<String>,
    categories: Vec<String>,
    append_position: String,
}

impl ConfigFile {
    pub fn new(json_string: String) -> Result<ConfigFile, Box<dyn std::error::Error + 'static>> {
        //let contents = fs::read_to_string(path)?;
        let config: ConfigFile = serde_json::from_str(&json_string)?;
        Ok(config)
    }
}

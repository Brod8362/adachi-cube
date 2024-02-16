use std::{error::Error, path::Path, fs::read_to_string};

use serde::{Serialize, Deserialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct BotConfig {
    pub token: String
}

pub fn parse_file(path: &Path) -> Result<BotConfig, Box<dyn Error>> {
    let file_contents = read_to_string(path)?;
    let r: Result<BotConfig, toml::de::Error> = toml::from_str(&file_contents);
    Ok(r?)
}
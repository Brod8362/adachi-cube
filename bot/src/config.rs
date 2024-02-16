use std::{error::Error, path::Path, fs::read_to_string};

use serde::Deserialize;

const TRUE_DEFAULT: &'static str = "true";
const MAYBE_DEFAULT: &'static str = "maybe";
const FALSE_DEFAULT: &'static str = "false";

fn true_default_path() -> String {
    String::from(TRUE_DEFAULT)
}

fn maybe_default_path() -> String {
    String::from(MAYBE_DEFAULT)
}

fn false_default_path() -> String {
    String::from(FALSE_DEFAULT)
}

#[derive(Debug, Deserialize)]
pub struct BotConfig {
    pub token: String,
    pub influx_host: Option<String>,
    pub influx_database: Option<String>,
    #[serde(default = "true_default_path")]
    pub true_folder_path: String,
    #[serde(default = "maybe_default_path")]
    pub maybe_folder_path: String,
    #[serde(default = "false_default_path")]
    pub false_folder_path: String
}

pub fn parse_file(path: &Path) -> Result<BotConfig, Box<dyn Error>> {
    let file_contents = read_to_string(path)?;
    let r: Result<BotConfig, toml::de::Error> = toml::from_str(&file_contents);
    Ok(r?)
}
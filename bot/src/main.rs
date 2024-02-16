use std::{env, path::PathBuf};

mod config;
mod analytics;

static DEFAULT_CONFIG_PATH: &'static str = "./config.toml";

pub type DiscordClient = serenity::Client;

fn main() {
    let config_path_str = match env::var("ADACHI_CONFIG_PATH") {
        Ok(t) => t,
        _ => String::from(DEFAULT_CONFIG_PATH)
    };
    let config_path = PathBuf::from(config_path_str);

    let bot_config = match config::parse_file(&config_path)  {
        Ok(c) => c,
        Err(e) => {
            panic!("Failed to parse config file at {}: {}", config_path.to_str().unwrap(), e)
        }
    };

    println!("token: {}", bot_config.token);
}

use std::{env, path::PathBuf, error::Error};

use analytics::AnalyticsClient;
use serenity::prelude::GatewayIntents;

mod analytics;
mod commands;
mod config;

static DEFAULT_CONFIG_PATH: &'static str = "./config.toml";

pub type DiscordClient = serenity::Client;

pub struct BotData {
    pub analytics: AnalyticsClient
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    //parse config
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

    let analytics = AnalyticsClient::new::<String>(None, None)?; //TODO not None here

    let intents = GatewayIntents::non_privileged();

    let framework = poise::Framework::builder()
        .options(poise::FrameworkOptions {
            commands: vec![commands::roll()],
            ..Default::default()
        })
        .setup(|ctx, _ready, framework| {
            Box::pin(async move {
                poise::builtins::register_globally(ctx, &framework.options().commands).await?;
                Ok( BotData {
                    analytics: analytics
                })
            })
        })
        .build();
    
    let client = serenity::Client::builder(bot_config.token, intents)
        .framework(framework)
        .await;
    client.unwrap().start().await.unwrap();
    Ok(())
}

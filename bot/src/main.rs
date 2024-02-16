use std::{env, path::PathBuf, error::Error};

use analytics::AnalyticsClient;
use folders::Folders;
use serenity::prelude::GatewayIntents;

mod analytics;
mod commands;
mod config;
mod folders;

static DEFAULT_CONFIG_PATH: &'static str = "./config.toml";

pub type DiscordClient = serenity::Client;

pub struct BotData {
    pub analytics: AnalyticsClient,
    pub folders: Folders
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error + Send + Sync>> {
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

    let folders = Folders::new(&bot_config.true_folder_path, &bot_config.maybe_folder_path, &bot_config.false_folder_path)?;

    let analytics = AnalyticsClient::new::<String>(bot_config.influx_host, bot_config.influx_database, &bot_config.analytics_identifier)?; //TODO not None here

    let intents = GatewayIntents::non_privileged();

    let framework = poise::Framework::builder()
        .options(poise::FrameworkOptions {
            commands: vec![commands::ask(), commands::invite()],
            event_handler: |ctx, event, framework, data| {
                Box::pin(commands::event_handler(ctx, event, framework, data))
            },
            ..Default::default()
        })
        .setup(|ctx, _ready, framework| {
            Box::pin(async move {
                poise::builtins::register_globally(ctx, &framework.options().commands).await?;
                Ok( BotData {
                    analytics: analytics,
                    folders: folders
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

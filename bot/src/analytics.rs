use chrono::{DateTime, Utc};
use influxdb::InfluxDbWriteable;
use thiserror::Error;

use crate::BotData;

pub struct AnalyticsClient {
    influx_client: Option<influxdb::Client>,
    identifier: String,
}

enum LogLevel {
    Info,
    Warn,
    Error
}

type Error = Box<dyn std::error::Error + Send + Sync>;

impl LogLevel {
    fn as_str(&self) -> &'static str {
        match self {
            LogLevel::Info => "info",
            LogLevel::Error => "error",
            LogLevel::Warn => "warn"
        }
    }

    fn ansi_code(&self) -> &'static str {
        match self {
            LogLevel::Error => "\x1b[33m",
            LogLevel::Warn => "\x1b[31m",
            _ => "\x1b[32m"
        }
    }
}

#[derive(Error, Debug)]
pub enum AnalyticsClientError {
    #[error("missing host")]
    MissingHost,
    #[error("missing database")]
    MissingDatabase
}

#[derive(InfluxDbWriteable)]
struct GuildCountReading {
    time: DateTime<Utc>,
    value: i32,
    #[influxdb(tag)]
    bot: String,
    #[influxdb(tag)]
    shard_id: String
}

#[derive(InfluxDbWriteable)]
struct BotUsageReading {
    time: DateTime<Utc>,
    value: i32,
    #[influxdb(tag)]
    bot: String,
    #[influxdb(tag)]
    guild: String
}

#[derive(InfluxDbWriteable)]
struct LogEntry {
    time: DateTime<Utc>,
    value: String,
    #[influxdb(tag)]
    bot: String,
    #[influxdb(tag)]
    level: String
}

impl AnalyticsClient {
    pub fn new<S: Into<String>>(host: Option<S>, database: Option<S>, identifier: &String) -> Result<AnalyticsClient, Error> {
        if host.is_some() ^ database.is_some() {
            //both are required
            if host.is_none() {
                return Err(Box::new(AnalyticsClientError::MissingHost))
            }
            if database.is_none() {
                return Err(Box::new(AnalyticsClientError::MissingDatabase))
            }
        }
        // if neither are set, default to no-op
        if host.is_none() && database.is_none() {
            println!("defaulting to analytics no-op");
            return Ok(
                Self {
                    influx_client: None,
                    identifier: identifier.clone()
                }
            )
        }

        let client = influxdb::Client::new(host.unwrap(), database.unwrap());
        Ok(Self {
            influx_client: Some(client),
            identifier: identifier.clone()
        })
    }

    pub async fn update_guilds(&self, count: usize, shard_id: u32) -> Result<(), Error> {
        if self.influx_client.is_none() {
            return Ok(());
        }
        let r = GuildCountReading {
            time: Utc::now(),
            value: count as i32,
            bot: self.identifier.clone(),
            shard_id: shard_id.to_string(),
        }.into_query("guild_count");
        if let Some(client) = &self.influx_client {
            client.query(r).await?;
        }
        Ok(())
    }

    pub async fn update_usage(&self, guild_id: u64, _channel_id: u64) -> Result<(), Error> {
        if self.influx_client.is_none() {
            return Ok(());
        }
        let r = BotUsageReading {
            time: Utc::now(),
            value: 1,
            bot: self.identifier.clone(),
            guild: guild_id.to_string(),
        }.into_query("use");
        if let Some(client) = &self.influx_client {
            client.query(r).await?;
        }
        Ok(())
    }

    pub async fn update_guild_name<S: Into<String>>(&self, _guild_id: u64, _name: S) -> Result<(), Error> {
        if self.influx_client.is_none() {
            return Ok(());
        }
        todo!()
    }

    async fn log_generic<S: Into<String>>(&self, message: S, level: LogLevel) -> Result<(), Error> {
        let now = Utc::now();
        let msg: String = message.into();
        println!("[{} {}{}\x1b[0m] {}", now, level.ansi_code(), level.as_str(), msg);
        if self.influx_client.is_none() {
            return Ok(());
        }
        let r = LogEntry {
            time: now,
            value: msg,
            bot: self.identifier.clone(),
            level: String::from(level.as_str()),
        }.into_query("log");
        if let Some(client) = &self.influx_client {
            client.query(r).await?;
        }
        Ok(())
    }

    pub async fn info<S: Into<String>>(&self, message: S) -> Result<(), Error> {
        self.log_generic(message, LogLevel::Info).await
    }

    pub async fn warn<S: Into<String>>(&self, message: S) -> Result<(), Error> {
        self.log_generic(message, LogLevel::Warn).await
    }

    pub async fn error<S: Into<String>>(&self, message: S) -> Result<(), Error> {
        self.log_generic(message, LogLevel::Error).await
    }
}

pub async fn on_error(error: poise::FrameworkError<'_, BotData, Error>) {
    // This is our custom error handler
    // They are many errors that can occur, so we only handle the ones we want to customize
    // and forward the rest to the default handler
    match error {
        poise::FrameworkError::Setup { error, ctx: _, framework, ..} => {
            let msg = format!("failed to start bot: {:?}", error);
            framework.user_data().await.analytics.error(&msg).await.unwrap();
            panic!("{}", msg);

        },
        poise::FrameworkError::Command { error, ctx, .. } => {
            let log = format!("Error in command `{}`: {:?}", ctx.command().name, error);
            ctx.data().analytics.warn(log).await.unwrap();
        }
        error => {
            let analytics = &error.ctx().unwrap().data().analytics;
            let log = format!("unknown error: {:#?}", &error.to_string());
            analytics.error(log).await.unwrap();
            if let Err(e) = poise::builtins::on_error(error).await {
                let inner_err = format!("Error while handling error: {}", e);
                analytics.error(inner_err).await.unwrap();
            }
        }
    }
}
//TODO: implement a generic error handler
// https://github.com/serenity-rs/poise/blob/f6b94ca83bc9f98815112456adab0d3031e37319/examples/basic_structure/main.rs#L22
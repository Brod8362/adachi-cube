use chrono::{DateTime, Utc};
use influxdb::InfluxDbWriteable;
use thiserror::Error;

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

    pub async fn update_usage(&self, guild_id: usize, _channel_id: usize) -> Result<(), Error> {
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

    pub async fn update_guild_name<S: Into<String>>(&self, _guild_id: usize, _name: S) -> Result<(), Error> {
        if self.influx_client.is_none() {
            return Ok(());
        }
        todo!()
    }

    async fn log_generic<S: Into<String>>(&self, message: S, level: LogLevel) -> Result<(), Error> {
        println!("[{}{}\x1b[0m] {}", level.ansi_code(), level.as_str(), message.into());
        if self.influx_client.is_none() {
            return Ok(());
        }
        todo!()
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
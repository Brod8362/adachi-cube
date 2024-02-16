use std::{error::Error, sync::{Arc, Mutex}};

use thiserror::Error;

pub struct AnalyticsClient {
    influx_client: Option<Arc<Mutex<influxdb::Client>>>
}

enum LogLevel {
    Log,
    Warn,
    Error
}

impl LogLevel {
    fn as_str(&self) -> &'static str {
        match self {
            LogLevel::Log => "log",
            LogLevel::Error => "error",
            LogLevel::Warn => "warn"
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

impl AnalyticsClient {
    pub fn new<S: Into<String>>(host: Option<S>, database: Option<S>) -> Result<AnalyticsClient, Box<dyn Error>> {
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
            return Ok(Self { influx_client: None })
        }

        let client = influxdb::Client::new(host.unwrap(), database.unwrap());
        Ok(Self {
            influx_client: Some(Arc::new(Mutex::new(client)))
        })
    }

    pub fn update_guilds(&self, count: usize, shard_id: usize) -> Result<(), Box<dyn Error>> {
        if self.influx_client.is_none() {
            return Ok(());
        }
        todo!()
    }

    pub fn update_usage(&self, guild_id: usize, channel_id: usize) -> Result<(), Box<dyn Error>> {
        if self.influx_client.is_none() {
            return Ok(());
        }
        todo!()
    }

    pub fn update_guild_name<S: Into<String>>(&self, guild_id: usize, name: S) -> Result<(), Box<dyn Error>> {
        if self.influx_client.is_none() {
            return Ok(());
        }
        todo!()
    }

    fn log_generic<S: Into<String>>(&self, message: S, level: LogLevel) -> Result<(), Box<dyn Error>> {
        if self.influx_client.is_none() {
            return Ok(());
        }
        todo!()
    }
}
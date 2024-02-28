use serde::Deserialize;

use crate::{
    cron::Interval,
    job::Job,
};

#[derive(Debug, Deserialize)]
pub struct Config {
    #[serde(default = "Interval::default")]
    pub interval: Interval,
    pub base_url: String,
    pub jobs: Vec<Job>,
}

impl Default for Config {
    fn default() -> Self {
        let config_str = std::fs::read_to_string("config.toml")
            .expect("failed to read config");
        toml::from_str(&config_str)
            .expect("failed to parse config")
    }
}


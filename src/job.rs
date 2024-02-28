use serde::{Deserialize, Serialize};

use crate::cron::Cron;

#[derive(Debug, Default, Deserialize, Serialize)]
pub struct Job {
    pub route: String,
    #[serde(default = "Cron::default")]
    pub cron: Cron,
}

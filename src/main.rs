use std::time::Duration;

use anyhow::Result;
use chrono::Utc;
use reqwest::Client;
use tokio::time::sleep;

mod config;
mod cron;
mod job;
mod schedule;

use config::Config;
use schedule::Schedule;

#[tokio::main]
async fn main() -> Result<()> {
    let config = Config::default();
    let interval = config.interval.0; 
    let base_url = config.base_url.clone(); 
    let mut schedule = Schedule::from(config);

    let client = Client::new();
    loop {
        if let Some(job) = schedule.queue.pop_front() {
            if let Some(next_run_time) = schedule.next_run_time {
                let now = Utc::now();
                if next_run_time <= now {
                    client
                        .post(format!("{}{}", base_url, job.route.clone()))
                        .send()
                        .await?;
                    schedule.add(job);
                } else {
                    schedule.queue.insert(0, job);
                }
            }
        }
        sleep(Duration::from_millis(interval)).await;
    }
}


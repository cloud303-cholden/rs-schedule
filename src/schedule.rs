use std::collections::VecDeque;

use chrono::{
    DateTime,
    Utc,
};

use crate::{
    config::Config,
    cron::Interval,
    job::Job,
};

#[derive(Debug, Default)]
pub struct Schedule {
    pub queue: VecDeque<Job>,
    pub next_run_time: Option<DateTime<Utc>>,
    pub interval: Interval,
}

impl Schedule {
    pub fn add(&mut self, job: Job) {
        self
            .queue
            .push_back(job);
        self
            .queue
            .make_contiguous()
            .sort_by_key(|j| j.cron.next_run_time());
        self.next_run_time = self
            .queue
            .front()
            .and_then(|j| j.cron.next_run_time());
    }
}

impl From<Config> for Schedule {
    fn from(config: Config) -> Self {
        let mut schedule = Self {
            interval: config.interval,
            ..Default::default()
        };
        for job in config.jobs {
            schedule.add(job);
        }
        schedule
    }
}

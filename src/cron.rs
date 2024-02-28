use chrono::{
    Datelike,
    DateTime,
    Duration,
    NaiveDate,
    Timelike,
    TimeZone,
    Utc,
};
use serde::{Deserialize, Serialize};
use strum::EnumString;

#[derive(Debug, Deserialize)]
pub struct Interval(pub u64);

impl Default for Interval {
    fn default() -> Self {
        Self(100)
    }
}

#[derive(EnumString)]
#[strum(serialize_all = "lowercase")]
enum RateUnit {
    Seconds,
    Minutes,
    Hours,
    Days,
}

impl RateUnit {
    fn to_duration(&self, duration: i64) -> Duration {
        match self {
            RateUnit::Seconds => Duration::seconds(duration),
            RateUnit::Minutes => Duration::minutes(duration),
            RateUnit::Hours => Duration::hours(duration),
            RateUnit::Days => Duration::days(duration),
        }
    }
}

#[derive(Debug, Deserialize, Serialize)]
#[serde(rename_all = "lowercase")]
#[serde(tag = "type", content = "expr")]
pub enum Cron {
    Time(String),
    Rate(String),
}

impl Default for Cron {
    fn default() -> Self {
        Self::Time(String::from("0 0"))
    }
}

impl Cron {
    pub fn next_run_time(&self) -> Option<DateTime<Utc>> {
        match self {
            Self::Time(expr) => {
                let (minute, hour) = expr.split_once(' ')?;
                let minute = minute.parse().ok()?;
                let hour = hour.parse().ok()?;

                let now = Utc::now();
                let mut day = now.day();
                let mut month = now.month();
                let mut year = now.year();

                if (minute <= now.minute() && hour <= now.hour()) || hour < now.hour() {
                    day = if day < days_in_month(month, year) {
                        day + 1
                    } else {
                        month = if month < 12 {
                            month + 1
                        } else {
                            year += 1;
                            1
                        };
                        1
                    };
                };
                Utc.with_ymd_and_hms(year, month, day, hour, minute, 0).single()
            },
            Self::Rate(expr) => {
                let (duration, rate_unit) = expr.split_once(' ')?;
                let duration: i64 = duration.parse().ok()?;
                let rate_unit: RateUnit = rate_unit.parse().ok()?;
                Some(Utc::now() + rate_unit.to_duration(duration))
            },
        }
    }
}

// https://stackoverflow.com/a/53687279/19027916
fn days_in_month(m: u32, year: i32) -> u32 {
    if m == 12 {
        NaiveDate::from_ymd_opt(year + 1, 1, 1)
    } else {
        NaiveDate::from_ymd_opt(year, m + 1, 1)
    }.expect("invalid date")
    .signed_duration_since(NaiveDate::from_ymd_opt(year, m, 1).unwrap())
    .num_days() as u32
}

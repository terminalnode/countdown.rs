use crate::helpers::get_system_timezone;
use chrono::{DateTime, Local, NaiveDate, NaiveDateTime, NaiveTime};
use chrono_tz::Tz;
use clap::Parser;

/// Calculate the time between now and a specified target time
#[derive(Parser, Debug)]
#[command(name = "countdown", version, about)]
pub struct Args {
    /// The date to use for the target time [default: current local date]
    #[arg(short = 'd', long = "date", value_name = "YYYY-mm-dd")]
    pub date: Option<NaiveDate>,

    /// The time to use for the target time [default: current local time]
    #[arg(short = 't', long = "time", value_name = "HH:MM[:SS]")]
    pub time: Option<NaiveTime>,

    /// The timezone to use for the target time [default: system timezone]
    #[arg(short = 'z', long = "zone")]
    pub zone: Option<Tz>,

    /// If set, print current time and target time above the remaining time
    #[arg(short = 'v', long = "verbose")]
    pub verbose: bool,

    /// If set, print the remaining time every second
    #[arg(short = 'c', long = "continuous")]
    pub continuous: bool,

    /// If set in combination with -c / --continuous, overwrite the previous line instead of printing a new line
    #[arg(short = 'o', long = "overwrite")]
    pub overwrite: bool,
}

impl Args {
    pub fn get_target(&self) -> Result<DateTime<Tz>, String> {
        let now = Local::now();
        let date = self.date.unwrap_or_else(|| now.date_naive());
        let time = self.time.unwrap_or_else(|| now.time());
        let zone = self.zone.map(Ok).unwrap_or_else(get_system_timezone)?;

        Ok(NaiveDateTime::new(date, time)
            .and_local_timezone(zone)
            .unwrap())
    }
}

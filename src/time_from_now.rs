use crate::helpers::divmod;
use chrono::{DateTime, Local};
use chrono_tz::Tz;
use std::cmp::max;

pub struct TimeFromNow {
    pub time_remaining: String,
    pub time_now: String,
    pub time_target: String,
    pub millis: u64,
}

impl TimeFromNow {
    pub fn from(target: DateTime<Tz>) -> Result<Self, String> {
        let now = Local::now();
        let signed_millis = target.signed_duration_since(now).num_milliseconds();
        let sign = if signed_millis < 0 { "-" } else { "" };
        let millis = signed_millis.abs();

        let (days, millis) = divmod(millis, 86_400_000);
        let (hours, millis) = divmod(millis, 3_600_000);
        let (minutes, millis) = divmod(millis, 60_000);
        let (seconds, millis) = divmod(millis, 1_000);

        // If we don't add +1 to millis the sleep time is too short,
        // and we will print the same time many times over.
        let millis = if signed_millis < 0 {
            max(0, 1000 - millis)
        } else {
            max(0, millis + 1)
        } as u64;

        Ok(TimeFromNow {
            time_remaining: format!("{sign}{days} days {hours:02}:{minutes:02}:{seconds:02}"),
            time_now: now.format("Now:    %Y-%m-%d %H:%M:%S (%Z)").to_string(),
            time_target: target.format("Target: %Y-%m-%d %H:%M:%S (%Z)").to_string(),
            millis,
        })
    }

    pub fn formatted(&self, verbose: bool) -> String {
        if verbose {
            format!(
                "{}\n{}\n{}",
                self.time_now, self.time_target, self.time_remaining
            )
        } else {
            self.time_remaining.clone()
        }
    }
}

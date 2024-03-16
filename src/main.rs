use std::process::exit;
use std::str::FromStr;

use chrono::{Local, NaiveDate, NaiveDateTime, NaiveTime};
use chrono_tz::Tz;
use clap::Parser;
use iana_time_zone::get_timezone;

/// Simple program to calculate the time until a target specified target time
#[derive(Parser, Debug)]
#[command(version, about)]
struct Args {
    /// The date to use for the target time [default: current local date]
    #[arg(short = 'd', long = "date", value_name = "YYYY-mm-dd")]
    date: Option<NaiveDate>,

    /// The time to use for the target time [default: current local time]
    #[arg(short = 't', long = "time", value_name = "HH:MM[:SS]")]
    time: Option<NaiveTime>,

    /// The timezone to use for the target time [default: system timezone]
    #[arg(short = 'z', long = "zone")]
    zone: Option<Tz>,

    /// If set, print current time and target time above the remaining time
    #[arg(short = 'v', long = "verbose")]
    verbose: bool,
}

fn main() {
    match run() {
        Ok(output) => {
            println!("{}", output);
            exit(0);
        }
        Err(error) => {
            eprintln!("Error:\n{}", error);
            exit(1);
        }
    }
}

fn run() -> Result<String, String> {
    let args = Args::parse();
    let now = Local::now();
    let zone = match args.zone {
        Some(tz) => tz,
        None => {
            let raw = get_timezone().or_else(|_| Err("Failed to get system timezone".to_string()))?;
            Tz::from_str(&raw).or_else(|_| Err(format!("Failed to parse timezone {}", &raw)))?
        }
    };

    let target = NaiveDateTime::new(
        args.date.unwrap_or_else(|| now.date_naive()),
        args.time.unwrap_or_else(|| now.time()),
    ).and_local_timezone(zone).unwrap();

    let mut seconds = target.signed_duration_since(now).num_seconds();
    let days = seconds / 86400;
    seconds %= 86400;
    let hours = seconds / 3600;
    seconds %= 3600;
    let minutes = seconds / 60;
    seconds %= 60;

    let remaining = format!("{days} days {hours:02}:{minutes:02}:{seconds:02}");
    let out = if args.verbose {
        let now = now.format("Now:    %Y-%m-%d %H:%M:%S (%Z)");
        let target = target.format("Target: %Y-%m-%d %H:%M:%S (%Z)");
        format!("{now}\n{target}\n{remaining}")
    } else {
        remaining
    };
    Ok(out)
}
use std::process::exit;
use std::str::FromStr;

use chrono::{DateTime, Local, NaiveDate, NaiveDateTime, NaiveTime};
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
    let target = get_target(&args)?;
    get_time_from_now(target, args.verbose)
}

fn get_target(args: &Args) -> Result<DateTime<Tz>, String> {
    let now = Local::now();
    let date = args.date.unwrap_or_else(|| now.date_naive());
    let time = args.time.unwrap_or_else(|| now.time());
    let zone = args.zone.map(Ok).unwrap_or_else(get_system_timezone)?;

    Ok(NaiveDateTime::new(date, time)
        .and_local_timezone(zone)
        .unwrap())
}

fn get_system_timezone() -> Result<Tz, String> {
    get_timezone()
        .or_else(|_| Err("Failed to get system timezone".to_string()))
        .and_then(|tz| Tz::from_str(&tz).or_else(|_| Err(format!("Failed to parse timezone {tz}"))))
}

fn get_time_from_now(target: DateTime<Tz>, verbose: bool) -> Result<String, String> {
    let now = Local::now();
    let mut seconds = target.signed_duration_since(now).num_seconds();
    let days = seconds / 86400;
    seconds %= 86400;
    let hours = seconds / 3600;
    seconds %= 3600;
    let minutes = seconds / 60;
    seconds %= 60;

    let remaining = format!("{days} days {hours:02}:{minutes:02}:{seconds:02}");
    Ok(if verbose {
        let now = now.format("Now:    %Y-%m-%d %H:%M:%S (%Z)");
        let target = target.format("Target: %Y-%m-%d %H:%M:%S (%Z)");
        format!("{now}\n{target}\n{remaining}")
    } else {
        remaining
    })
}

use std::cmp::max;
use std::process::exit;
use std::str::FromStr;
use std::thread::sleep;
use std::time::Duration;

use chrono::{DateTime, Local, NaiveDate, NaiveDateTime, NaiveTime};
use chrono_tz::Tz;
use clap::Parser;
use iana_time_zone::get_timezone;

/// Calculate the time between now and a specified target time
#[derive(Parser, Debug)]
#[command(name = "countdown", version, about)]
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

    /// If set, print the remaining time every second
    #[arg(short = 'c', long = "continuous")]
    continuous: bool,
}

struct TimeFromNow {
    formatted: String,
    millis: u64,
}

fn main() {
    match run() {
        Ok(_) => exit(0),
        Err(error) => {
            eprintln!("Error:\n{}", error);
            exit(1);
        }
    }
}

fn run() -> Result<(), String> {
    let args = Args::parse();
    let target = get_target(&args)?;

    if args.continuous {
        loop {
            let remaining = get_time_from_now(target, args.verbose)?;
            println!("{}", remaining.formatted);
            sleep(Duration::from_millis(remaining.millis));
        }
    } else {
        let remaining = get_time_from_now(target, args.verbose)?;
        println!("{}", remaining.formatted);
    }

    Ok(())
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

fn get_time_from_now(target: DateTime<Tz>, verbose: bool) -> Result<TimeFromNow, String> {
    let now = Local::now();
    let signed_millis = target.signed_duration_since(now).num_milliseconds();
    let sign = if signed_millis < 0 { "-" } else { "" };
    let millis = signed_millis.abs();

    let (days, millis) = (millis / 86_400_000, millis % 86_400_000);
    let (hours, millis) = (millis / 3_600_000, millis % 3_600_000);
    let (minutes, millis) = (millis / 60_000, millis % 60_000);
    let (seconds, millis) = (millis / 1_000, millis % 1_000);

    let remaining = format!("{sign}{days} days {hours:02}:{minutes:02}:{seconds:02}");
    let formatted = if verbose {
        let now = now.format("Now:    %Y-%m-%d %H:%M:%S (%Z)");
        let target = target.format("Target: %Y-%m-%d %H:%M:%S (%Z)");
        format!("{now}\n{target}\n{remaining}")
    } else {
        remaining
    };

    // If we don't add +1 to millis the sleep time is too short,
    // and we will print the same time many times over.
    let millis = if signed_millis < 0 {
        max(0, 1000 - millis)
    } else {
        max(0, millis + 1)
    } as u64;

    Ok(TimeFromNow { formatted, millis })
}

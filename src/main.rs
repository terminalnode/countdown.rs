use std::cmp::max;
use std::io::stdout;
use std::process::exit;
use std::str::FromStr;
use std::thread::sleep;
use std::time::Duration;

use chrono::{DateTime, Local, NaiveDate, NaiveDateTime, NaiveTime};
use chrono_tz::Tz;
use clap::Parser;
use crossterm::cursor::MoveLeft;
use crossterm::execute;
use crossterm::style::Print;
use crossterm::terminal::{Clear, ClearType};
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

    /// If set in combination with -c / --continuous, overwrite the previous line instead of printing a new line
    #[arg(short = 'o', long = "overwrite")]
    overwrite: bool,
}

struct TimeFromNow {
    time_remaining: String,
    time_now: String,
    time_target: String,
    millis: u64,
}

impl TimeFromNow {
    fn formatted(&self, verbose: bool) -> String {
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
        let mut first = true;

        loop {
            let remaining = get_time_from_now(target)?;
            let formatted = remaining.formatted(args.verbose && first);

            if args.overwrite && !first {
                let execution = execute!(
                    stdout(),
                    Clear(ClearType::CurrentLine),
                    MoveLeft(100),
                    Print(formatted),
                );
                execution.or_else(|x| Err(x.to_string()))?;
            } else {
                if args.overwrite {
                    execute!(stdout(), Print(formatted)).or_else(|x| Err(x.to_string()))?;
                } else {
                    println!("{}", formatted);
                }
                first = false;
            };
            sleep(Duration::from_millis(remaining.millis));
        }
    } else {
        let remaining = get_time_from_now(target)?;
        println!("{}", remaining.formatted(args.verbose));
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

fn get_time_from_now(target: DateTime<Tz>) -> Result<TimeFromNow, String> {
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

/// Return the quotient and remainder of the division of dividend by divisor
fn divmod(dividend: i64, divisor: i64) -> (i64, i64) {
    (dividend / divisor, dividend % divisor)
}

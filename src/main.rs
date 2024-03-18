use std::io::stdout;
use std::process::exit;
use std::thread::sleep;
use std::time::Duration;

use clap::Parser;
use crossterm::cursor::MoveLeft;
use crossterm::execute;
use crossterm::style::Print;
use crossterm::terminal::{Clear, ClearType};

use time_from_now::TimeFromNow;

use crate::args::Args;

mod args;
mod helpers;
mod time_from_now;

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
    let target = args.get_target()?;

    if args.continuous {
        let mut first = true;

        loop {
            let remaining = TimeFromNow::from(target)?;
            let formatted = remaining.formatted(args.verbose && first);

            let execution = if args.overwrite && !first {
                execute!(
                    stdout(),
                    Clear(ClearType::CurrentLine),
                    MoveLeft(100),
                    Print(formatted)
                )
            } else {
                first = false;
                if args.overwrite {
                    execute!(stdout(), Print(formatted))
                } else {
                    execute!(stdout(), Print(formatted), Print("\n"))
                }
            };

            execution.or_else(|x| Err(x.to_string()))?;
            sleep(Duration::from_millis(remaining.millis));
        }
    } else {
        let remaining = TimeFromNow::from(target)?;
        println!("{}", remaining.formatted(args.verbose));
    }

    Ok(())
}

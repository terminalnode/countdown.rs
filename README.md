![cargo build](https://github.com/terminalnode/countdown.rs/actions/workflows/cargo-build.yml/badge.svg)
![cargo fmt](https://github.com/terminalnode/countdown.rs/actions/workflows/cargo-fmt.yml/badge.svg)

# countdown.rs
Ever wanted to know how many days hours:minutes:seconds are left until a certain date and time?
Look no further, the solution is here! This is a simple Rust program that does just that.

## Build and install
Just build the program with `cargo build -r`, then move the resulting binary from
`./target/release/countdown` to some desired directory on your `$PATH`.

## Usage
```
$ countdown -V
countdown 0.3.0

$ countdown -h
Calculate the time between now and a specified target time

Usage: countdown [OPTIONS]

Options:
  -d, --date <YYYY-mm-dd>  The date to use for the target time [default: current local date]
  -t, --time <HH:MM[:SS]>  The time to use for the target time [default: current local time]
  -z, --zone <ZONE>        The timezone to use for the target time [default: system timezone]
  -v, --verbose            If set, print current time and target time above the remaining time
  -c, --continuous         If set, print the remaining time every second
  -o, --overwrite          If set in combination with -c / --continuous, overwrite the previous line instead of printing a new line
  -h, --help               Print help
  -V, --version            Print version

$ countdown -d 2024-03-23 -t 10:00
4 days 23:19:17

$ countdown -d 2024-03-23 -t 10:00 -v
Now:    2024-03-18 10:40:44 (+01:00)
Target: 2024-03-23 10:00:00 (CET)
4 days 23:19:15

$ countdown -d 2024-03-23 -t 10:00 -cv
Now:    2024-03-18 10:40:47 (+01:00)
Target: 2024-03-23 10:00:00 (CET)
4 days 23:19:12
4 days 23:19:11
4 days 23:19:10
4 days 23:19:09
4 days 23:19:08
4 days 23:19:07
4 days 23:19:06
^C

# This variant will update the remaining time on the same line, instead of printing a new line every second
$ countdown -d 2024-03-23 -t 10:00 -cvo
Now:    2024-03-18 10:40:58 (+01:00)
Target: 2024-03-23 10:00:00 (CET)
4 days 23:18:54
```

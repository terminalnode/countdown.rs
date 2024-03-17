# countdown.rs
Ever wanted to know how many days hours:minutes:seconds are left until a certain date and time?
Look no further, the solution is here! This is a simple Rust program that does just that.

## Build and install
Just build the program with `cargo build -r`, then move the resulting binary from
`./target/release/countdown` to some desired directory on your `$PATH`.

## Usage
```
`$ countdown -V
countdown 0.1.1

$ countdown -h
Calculate the time between now and a specified target time

Usage: countdown [OPTIONS]

Options:
  -d, --date <YYYY-mm-dd>  The date to use for the target time [default: current local date]
  -t, --time <HH:MM[:SS]>  The time to use for the target time [default: current local time]
  -z, --zone <ZONE>        The timezone to use for the target time [default: system timezone]
  -v, --verbose            If set, print current time and target time above the remaining time
  -h, --help               Print help
  -V, --version            Print version

$ countdown -d '2024-03-23' -t '10:00'
5 days 23:42:03

$ countdown -d '2024-03-23' -t '10:00' -v
Now:    2024-03-17 10:18:00 (+01:00)
Target: 2024-03-23 10:00:00 (CET)
5 days 23:41:59

$ countdown -d 2024-03-13 -t 13:20 -v
Now:    2024-03-17 10:18:59 (+01:00)
Target: 2024-03-13 13:20:00 (CET)
-3 days 20:58:59`
```

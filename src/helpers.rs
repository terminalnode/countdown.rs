use chrono_tz::Tz;
use iana_time_zone::get_timezone;
use std::str::FromStr;

/// Get the system timezone. Return an Err(String) if no system timezone can be found or
/// if the timezone cannot be parsed as a [Tz].
pub fn get_system_timezone() -> Result<Tz, String> {
    get_timezone()
        .or_else(|_| Err("Failed to get system timezone".to_string()))
        .and_then(|tz| Tz::from_str(&tz).or_else(|_| Err(format!("Failed to parse timezone {tz}"))))
}

/// Return the quotient and remainder of the division of dividend by divisor
pub fn divmod(dividend: i64, divisor: i64) -> (i64, i64) {
    (dividend / divisor, dividend % divisor)
}

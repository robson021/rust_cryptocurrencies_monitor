use chrono::{DateTime, Local};
use std::time::{Duration, SystemTime, UNIX_EPOCH};

const DATE_FORMAT: &str = "%Y-%m-%d %H:%M:%S";

pub fn get_formatted_local_time(timestamp_millis: u64) -> String {
    let system_time = get_system_time(timestamp_millis);
    let date = DateTime::<Local>::from(system_time);
    date.format(DATE_FORMAT).to_string()
}

#[inline(always)]
fn get_system_time(timestamp: u64) -> SystemTime {
    UNIX_EPOCH + Duration::from_millis(timestamp)
}

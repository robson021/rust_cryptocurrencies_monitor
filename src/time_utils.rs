use chrono::{DateTime, Local};
use rust_decimal::prelude::ToPrimitive;
use std::time::{Duration, SystemTime, UNIX_EPOCH};

const DATE_FORMAT: &str = "%Y-%m-%d %H:%M:%S";

pub fn get_formatted_local_time(timestamp_millis: u64) -> String {
    let timestamp = timestamp_millis.to_u64().unwrap();
    let system_time = get_system_time(timestamp);
    let date = DateTime::<Local>::from(system_time);
    date.format(DATE_FORMAT).to_string()
}

#[inline(always)]
fn get_system_time(timestamp: u64) -> SystemTime {
    UNIX_EPOCH + Duration::from_millis(timestamp)
}

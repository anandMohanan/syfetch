use colored::Colorize;
use std::fs::read_to_string;

/// Obtain the uptime of the system
pub fn uptime_info() {
    let up_val = read_to_string("/proc/uptime").unwrap();
    let val: Vec<&str> = up_val.split(".").collect();
    let uptime = val[0].replace("\n", "").parse::<i64>().unwrap();
    let days = if uptime > 86400 {
        let days_pre = uptime / 60 / 60 / 24;
        days_pre.to_string() + "d"
    } else {
        "0d".to_string()
    };
    let hours = if uptime > 3600 {
        let hours_pre = (uptime / 60 / 60) % 24;
        hours_pre.to_string() + "h"
    } else {
        "0h".to_string()
    };
    let minutes = if uptime > 60 {
        let minutes_pre = (uptime / 60) % 60;
        minutes_pre.to_string() + "m"
    } else {
        "0m".to_string()
    };

    println!(
        "{} {days} {hours} {minutes}",
        "uptime:".bright_cyan().bold().italic()
    );
}

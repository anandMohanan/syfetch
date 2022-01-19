use std::fs::read_to_string;

use colored::Colorize;

/// Obtain the CPU temperature
pub fn temp_info() {
    let temp_val = read_to_string("/sys/class/thermal/thermal_zone0/temp").unwrap();
    let temp_val = temp_val.trim().parse::<f64>().unwrap() / 1000.00;
    println!(
        "{} {temp_val}c",
        "cpu temperature:".bright_cyan().bold().italic()
    );
}

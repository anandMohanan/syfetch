use std::fs::read_to_string;

pub fn temp_info() {
    let temp_val = read_to_string("/sys/class/thermal/thermal_zone0/temp").unwrap();
    let temp_val = temp_val.trim().parse::<f64>().unwrap() / 1000.00;
    println!("| cpu temperature: {}c", temp_val);
}

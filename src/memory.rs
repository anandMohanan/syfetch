use std::{
    fs::File,
    io::{self, BufRead, BufReader, Result},
    panic,
};

fn mem_available() -> Result<i64> {
    let file = File::open("/proc/meminfo").unwrap();
    let f = BufReader::new(file);
    for line in f.lines() {
        match line {
            Ok(line) => {
                if line.starts_with("MemAvailable") {
                    if let Some(name) = line.split(":").last() {
                        // println!(
                        //     "{} mb",
                        let val = name
                            .chars()
                            .filter(|c| !c.is_whitespace())
                            .collect::<String>()
                            .replace("kB", "")
                            .trim()
                            .parse::<i64>()
                            .unwrap()
                            / 1024;
                        return Ok(val);
                        // );
                    } else {
                        println!("error")
                    }
                }
            }
            Err(e) => panic!("Error reading fie: {}", e),
        }
    }
    const ERROR_01: &str = "no MemTotal line found in /proc/meminfo!";
    Err(io::Error::new(std::io::ErrorKind::Other, ERROR_01))
}

fn mem_total() -> Result<i64> {
    let file = File::open("/proc/meminfo").unwrap();
    let f = BufReader::new(file);
    for line in f.lines() {
        match line {
            Ok(line) => {
                if line.starts_with("MemTotal") {
                    if let Some(name) = line.split(":").last() {
                        // println!(
                        //     "{} mb",
                        let val = name
                            .chars()
                            .filter(|c| !c.is_whitespace())
                            .collect::<String>()
                            .replace("kB", "")
                            .trim()
                            .parse::<i64>()
                            .unwrap()
                            / 1024;
                        return Ok(val);
                    } else {
                        println!("error")
                    }
                }
            }
            Err(e) => panic!("Error reading fie: {}", e),
        }
    }
    const ERROR_01: &str = "no MemTotal line found in /proc/meminfo!";
    Err(io::Error::new(std::io::ErrorKind::Other, ERROR_01))
}

/// Obtain the amount of memory in the system
pub fn mem_info() {
    let mem_total = match mem_total() {
        Ok(yes) => yes,
        Err(e) => panic!("{}", e),
    };

    let mem_available = match mem_available() {
        Ok(yes) => yes,
        Err(e) => panic!("{}", e),
    };

    println!("| mem: {}mb/{}mb", mem_total - mem_available, mem_total);
}

use std::{
    fs::File,
    io::{BufRead, BufReader},
    panic,
};

/// Obtain the name of the CPU
pub fn cpu_info() {
    let file = File::open("/proc/cpuinfo").unwrap();
    let f = BufReader::new(file);
    for line in f.lines() {
        match line {
            Ok(line) => {
                if line.starts_with("model name") {
                    if let Some(name) = line.split(":").last() {
                        let name = name
                            .char_indices()
                            .next()
                            .and_then(|(i, _)| name.get(i + 1..))
                            .unwrap_or("");

                        println!(
                            "| cpu name: {}",
                            name
                        );

                        break;
                    } else {
                        println!("error")
                    }
                }
            }
            Err(e) => panic!("Error reading fie: {}", e),
        }
    }
}

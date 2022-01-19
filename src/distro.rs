use std::fs::read_to_string;

use colored::Colorize;

/// Obtain the name of the DISTRO
pub fn distro_info() {
    let distro_name = read_to_string("/proc/sys/kernel/hostname").unwrap();

    println!(
        "{} {}",
        "os:".bright_cyan().bold().italic(),
        distro_name.replace("\n", "")
    );
}

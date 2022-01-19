use std::fs::read_to_string;

use colored::Colorize;

/// Obtain the name and version of the KERNEL
pub fn kernel_info() {
    let distro_name = read_to_string("/proc/sys/kernel/osrelease").unwrap();

    println!(
        "{} {}",
        "kernel:".bright_cyan().bold().italic(),
        distro_name.replace("\n", "")
    );
}

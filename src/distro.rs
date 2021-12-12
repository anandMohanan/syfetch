use std::fs::read_to_string;

/// Obtain the name of the DISTRO
pub fn distro_info() {
    let distro_name = read_to_string("/proc/sys/kernel/hostname").unwrap();

    println!("| os: {}", distro_name.replace("\n", ""));
}

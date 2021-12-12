use std::fs::read_to_string;

pub fn kernel_info() {
    let distro_name = read_to_string("/proc/sys/kernel/osrelease").unwrap();

    println!("| kernel: {}", distro_name.replace("\n", ""));
}

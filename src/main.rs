// use std::{fmt::Error, io, process};

// use nixinfo::{
//     cpu, device, distro, environment, gpu, hostname, kernel, memory, packages, temp, terminal,
//     uptime,
// };

// fn output(f: fn() -> io::Result<String>) -> String {
//     let val = f();
//     let val = match val {
//         Ok(success) => success,
//         Err(err) => {
//             println!("error: {}", err);
//             process::exit(1);
//         }
//     };
//     val
// }

// fn main() {
//     println!("{}", output(gpu));
//     println!("{}", output(temp));
// }
mod cpu;
mod distro;
mod gpu;
mod kernel;
mod memory;
mod temp;
mod uptime;
fn main() {
    println!("------------------------------------------------------------");
    distro::distro_info();
    cpu::cpu_info();
    temp::temp_info();
    uptime::uptime_info();
    kernel::kernel_info();
    gpu::gpu_info();
    memory::mem_info();
    println!("------------------------------------------------------------")
}

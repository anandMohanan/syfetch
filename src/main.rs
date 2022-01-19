mod cpu;
mod distro;
mod gpu;
mod kernel;
mod memory;
mod temp;
mod uptime;
fn main() {
    distro::distro_info();
    cpu::cpu_info();
    temp::temp_info();
    uptime::uptime_info();
    kernel::kernel_info();
    gpu::gpu_info();
    memory::mem_info();
}

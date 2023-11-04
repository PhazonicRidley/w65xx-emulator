use crate::peripherals::memory::VirtualMemory;

pub mod cpu;
pub mod instruction;
pub mod io;
pub mod register;

pub fn test() {
    println!("Hello from core.");
}

pub fn initialize() {
    // Init memory and other virtual devices if needed.
    let _virtual_memory = VirtualMemory::new();
    // Parse program and load ROM.
    // Perform boot cycle
    // TODO: await 7 clock cycles
    // Begin program
}

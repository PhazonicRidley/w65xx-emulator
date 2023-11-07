use std::sync::{Arc, Mutex};

use w65xx_emulator::core::cpu::*;
use w65xx_emulator::core::register::{Accumulator, Register};
use w65xx_emulator::peripherals::memory::{self, VirtualMemory};

#[test]
fn verify_boot_cycle() {
    // Set up
    let mem_arc = Arc::new(Mutex::new(VirtualMemory::new()));
    let rom: Vec<u8> = vec![0x00, 0x80];
    let mut cpu = CPU::new(mem_arc.clone());

    // Execute
    mem_arc.lock().unwrap().load_rom(rom, 0xFFFC).unwrap();
    cpu.boot_cycle();

    // Verify
    assert_eq!(mem_arc.lock().unwrap().read_word(0xFFFC), 0x8000);
    assert_eq!(cpu.program_counter.get_data(), 0x8000);
}

fn address_mode_setup() -> CPU {
    let mem_arc = Arc::new(Mutex::new(VirtualMemory::new()));
    // Make a test rom that is padded from 0 to 0xFEFF with 0xEA and
    // Then 0xFEFF to 0xFFFB is just counted up from 0, finally 0xFFFC is 0x00 and 0xFFFD is 0xFF
    let padding: Vec<u8> = vec![0xEA; 0xFF00];
    let test_rom: Vec<u8> = (0..0xFC).collect();
    let reset_vector: Vec<u8> = vec![0x00, 0xFF];
    let rom = vec![padding, test_rom, reset_vector].concat();
    // let mut i = 0;
    // for byte in &rom {
    //     println!("{:#04x}: {:#02x}", i, byte);
    //     i += 1;
    // }
    assert_eq!(rom[0xFF00], 0x00);
    assert_eq!(rom[0xFFFC], 0x00);
    assert_eq!(rom[0xFFFD], 0xFF);
    mem_arc.lock().unwrap().load_rom(rom, 0x0).unwrap();
    let mut cpu = CPU::new(mem_arc.clone());
    cpu.boot_cycle();
    assert_eq!(cpu.program_counter.get_data(), 0xFF00);

    return cpu;
}

#[test]
fn addr_setup_test() {
    address_mode_setup();
}

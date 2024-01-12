use std::sync::{Arc, Mutex};

use w65xx_emulator::core::cpu::CPU;
use w65xx_emulator::peripherals::memory::VirtualMemory;

fn alu_test_setup() -> CPU {
    let memory_arc = Arc::new(Mutex::new(VirtualMemory::new()));
    {
        let mut memory = memory_arc.lock().unwrap();

        let paddng: Vec<u8> = vec![0xEA; 0xFF00];
        let working_data: Vec<u8> = (0..0xFC).collect();
        let reset_vector: Vec<u8> = vec![0x00, 0xFF];
        let test_rom = vec![paddng, working_data, reset_vector].concat();
        //   for i in 0..=0xFFFD {
        //       println!("{:4x}: {:?}", i, test_rom[i]);
        //   }
        memory.load_rom(test_rom, 0x00).unwrap();
    }
    let mut cpu = CPU::new(memory_arc.clone());
    cpu.boot_cycle();
    //  println!("{:4x}", cpu.program_counter.get_data());
    return cpu;
}

#[test]
fn adc_test() {
    // Setup
    alu_test_setup();
    // Execute
}

use std::ops::Add;
use std::sync::{Arc, Mutex};

use w65xx_emulator::core::instructions::utils::AddressingModes;
use w65xx_emulator::core::register::Register;
use w65xx_emulator::core::{cpu::CPU, instructions::alu};
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
    let mut cpu = alu_test_setup();
    cpu.program_counter.increment(0); // PC set to 0xfff1, operand is at 0xfff2, which is 2.
    cpu.accumulator.load_data(1);

    // Execute
    alu::add_with_carry(&AddressingModes::Immediate, &mut cpu);

    // Verify
    assert_eq!(cpu.accumulator.get_data(), 3);
}

#[test]
fn sbc_test() {
    // Setup
    let mut cpu = alu_test_setup();
    cpu.program_counter.increment(0xee); // PC set to 0xffef, operand is at 0xfff0
    cpu.accumulator.load_data(0x50);
    cpu.processor_status_flags.set_flag('c').unwrap(); // Programmer is responsible for setting the carry flag to "complete" 2s compliment.

    // Execute
    alu::sub_with_carry(&AddressingModes::Immediate, &mut cpu);

    // Verify
    assert_eq!(cpu.accumulator.get_data(), 0x60);
}

#[test]
fn sbc_borrow_test() {
    // Setup
    let mut cpu = alu_test_setup();
    cpu.program_counter.increment(0x6e); // PC set to 0xff6f, operand is at 0xff70
    cpu.accumulator.load_data(0xd0);
    cpu.processor_status_flags.set_flag('c').unwrap(); // Programmer is responsible for setting the carry flag to "complete" 2s compliment.

    // Execute
    alu::sub_with_carry(&AddressingModes::Immediate, &mut cpu);

    // Verify
    assert_eq!(cpu.accumulator.get_data(), 0x60);
    assert!(cpu.processor_status_flags.check_flag('v').unwrap_or(false));
}

// Bitwise operations

#[test]
fn bitwise_and_test() {
    // Set up
    let mut cpu = alu_test_setup();
    cpu.accumulator.load_data(3);

    // Execute
    alu::bitwise_and(&AddressingModes::Immediate, &mut cpu);

    // Verify
    assert_eq!(cpu.accumulator.get_data(), 1);
}

#[test]
fn bitwise_or_test() {
    // Set up
    let mut cpu = alu_test_setup();
    cpu.accumulator.load_data(2);

    // Execute
    alu::bitwise_or(&AddressingModes::Immediate, &mut cpu);

    // Verify
    assert_eq!(cpu.accumulator.get_data(), 3);
}

#[test]
fn bitwise_exclusive_or_test() {
    // Set up
    let mut cpu = alu_test_setup();
    cpu.accumulator.load_data(0);

    // Execute
    alu::bitwise_exclusive_or(&AddressingModes::Immediate, &mut cpu);

    // Verify
    assert_eq!(cpu.accumulator.get_data(), 1);
}

#[test]
fn arithmetic_shift_lift_test() {
    // Setup
    let mut cpu = alu_test_setup();
    cpu.accumulator.load_data(0x81);

    // Execute
    alu::left_shift(&AddressingModes::Accumulator, &mut cpu, false);

    // Verify
    assert!(cpu.processor_status_flags.check_flag('c').unwrap_or(false));
    assert_eq!(cpu.accumulator.get_data(), 2);
}

#[test]
fn rotate_left_test() {
    // Setup
    let mut cpu = alu_test_setup();
    cpu.accumulator.load_data(0x81);
    cpu.processor_status_flags.set_flag('c').unwrap();

    // Execute
    alu::left_shift(&AddressingModes::Accumulator, &mut cpu, true);
    alu::left_shift(&AddressingModes::Accumulator, &mut cpu, true);

    // Verify
    assert!(!cpu.processor_status_flags.check_flag('c').unwrap_or(true));
    assert_eq!(cpu.accumulator.get_data(), 0x7);
}

#[test]
fn shift_right_test() {
    // Setup
    let mut cpu = alu_test_setup();
    cpu.accumulator.load_data(3);

    // Execute
    alu::right_shift(&AddressingModes::Accumulator, &mut cpu, false);

    // Verify
    assert!(cpu.processor_status_flags.check_flag('c').unwrap_or(false));
    assert_eq!(cpu.accumulator.get_data(), 1);
}

#[test]
fn rotate_right_test() {
    // Setup
    let mut cpu = alu_test_setup();
    cpu.accumulator.load_data(0x83);

    // Execute
    alu::right_shift(&AddressingModes::Accumulator, &mut cpu, true);
    alu::right_shift(&AddressingModes::Accumulator, &mut cpu, true);

    // Verify
    assert!(cpu.processor_status_flags.check_flag('c').unwrap_or(false));
    assert_eq!(cpu.accumulator.get_data(), 0xa0)
}

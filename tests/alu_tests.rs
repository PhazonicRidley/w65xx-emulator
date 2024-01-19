use std::sync::{Arc, Mutex};

use w65xx_emulator::core::instructions::utils::AddressingModes;
use w65xx_emulator::core::register::{Register, StatusFlags};
use w65xx_emulator::core::{
    cpu::CPU,
    instructions::{alu, control_flow},
};
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
    cpu.processor_status_flags.clear_flag(StatusFlags::Carry); // Programmer is responsible for clearing the carry flag before adding

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
    cpu.processor_status_flags.set_flag(StatusFlags::Carry); // Programmer is responsible for setting the carry flag to "complete" 2s compliment.

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
    cpu.processor_status_flags.set_flag(StatusFlags::Carry); // Programmer is responsible for setting the carry flag to "complete" 2s compliment.

    // Execute
    alu::sub_with_carry(&AddressingModes::Immediate, &mut cpu);

    // Verify
    assert_eq!(cpu.accumulator.get_data(), 0x60);
    assert!(cpu.processor_status_flags.check_flag(StatusFlags::Overflow));
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
    assert!(cpu.processor_status_flags.check_flag(StatusFlags::Carry));
    assert_eq!(cpu.accumulator.get_data(), 2);
}

#[test]
fn rotate_left_test() {
    // Setup
    let mut cpu = alu_test_setup();
    cpu.accumulator.load_data(0x81);
    cpu.processor_status_flags.set_flag(StatusFlags::Carry);

    // Execute
    alu::left_shift(&AddressingModes::Accumulator, &mut cpu, true);
    alu::left_shift(&AddressingModes::Accumulator, &mut cpu, true);

    // Verify
    assert!(!cpu.processor_status_flags.check_flag(StatusFlags::Carry));
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
    assert!(cpu.processor_status_flags.check_flag(StatusFlags::Carry));
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
    assert!(cpu.processor_status_flags.check_flag(StatusFlags::Carry));
    assert_eq!(cpu.accumulator.get_data(), 0xa0)
}

#[test]
fn increment_test() {
    // Setup
    let mut cpu = alu_test_setup();
    cpu.x_register.load_data(1);

    // Execute
    alu::increment_memory(&AddressingModes::ZeroPage, &mut cpu);
    alu::increment_register(&mut cpu.x_register);

    // Verify
    let memory = cpu.memory_arc.lock().unwrap();
    assert_eq!(cpu.x_register.get_data(), 2);
    assert_eq!(memory[0x0001], 0xeb);
}

#[test]
fn bit_test() {
    // Setup
    let mut cpu = alu_test_setup();
    cpu.accumulator.load_data(0xff);

    // Execute
    alu::bit_instruction(&AddressingModes::ZeroPage, &mut cpu);

    // Verify
    let status_register = &cpu.processor_status_flags;
    assert!(!status_register.check_flag(StatusFlags::Zero));
    assert!(status_register.check_flag(StatusFlags::Negative));
    assert!(status_register.check_flag(StatusFlags::Overflow));
}

// #[test]
// fn compare_test() {
//     // Setup
//     let mut cpu = alu_test_setup();
//     cpu.accumulator.load_data(1);
//     let accumulator = &cpu.accumulator;

//     // Execute
//     control_flow::compare(&AddressingModes::Immediate, accumulator, &mut cpu);

//     // Verify
//     assert_eq!(
//         cpu.processor_status_flags.check_flag(StatusFlags::Zero),
//         true
//     );
// }

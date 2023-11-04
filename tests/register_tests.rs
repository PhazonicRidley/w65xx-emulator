use std::sync::{Arc, Mutex};

use w65xx_emulator::core::register::*;
use w65xx_emulator::peripherals::memory::VirtualMemory;

#[test]
fn register_read_write_test() {
    // Set up
    let mut accumulator = Accumulator::new();
    // Execute
    accumulator.load_data(0x13);

    // Verify
    assert_eq!(accumulator.get_data(), 0x13 as u8);
}

#[test]
fn reset_register_test() {
    // Set up
    let mut accumulator = Accumulator::new();

    // Execute
    accumulator.load_data(0xFF);
    accumulator.reset_register();

    // Verify
    assert_eq!(accumulator.get_data(), 0);
}

#[test]
fn index_register_test() {
    // Set up
    let mut x_register = IndexRegister::new('x');
    let mut y_register = IndexRegister::new('y');

    // Execute
    x_register.load_data(0x7F);
    y_register.load_data(0xFF);

    // Verify
    assert_eq!(x_register.get_data(), 0x7F);
    assert_eq!(y_register.get_data(), 0xFF);
    assert_ne!(x_register.get_name(), y_register.get_name());
}

#[test]
fn program_counter_byte_manip_test() {
    // Set up
    let mut pc = ProgramCounter::from(0x8002);
    let new_pc_value: u16 = 0x6004;
    let new_high_byte: u8 = 0x60;
    let new_low_byte: u8 = 0x04;

    // Execute
    pc.set_pch(new_high_byte);
    pc.set_pcl(new_low_byte);

    // Verify
    assert_eq!(pc.get_pch(), new_high_byte);
    assert_eq!(pc.get_pcl(), new_low_byte);
    assert_eq!(pc.get_data(), new_pc_value);
}

#[test]
fn program_counter_increment_test() {
    // Setup
    let mut pc = ProgramCounter::from(0x8002);
    let param_counter = 2;

    // Execute
    pc.increment(param_counter);

    // Verify
    assert_eq!(pc.get_data(), 0x8004);
}

#[test]
fn stack_push_test() {
    // Setup
    let memory: Arc<Mutex<VirtualMemory>> = Arc::new(Mutex::new(VirtualMemory::new()));
    let mut stack_ptr = StackPointerRegister::new(0x01, 0xFF, memory.clone());

    // Execute
    stack_ptr.push(0x01);
    stack_ptr.push(0x02);
    stack_ptr.push(0x03);

    // Verify
    let mem_inner = memory.lock().unwrap();
    assert_eq!(mem_inner[0xFF], 0x01);
    assert_eq!(mem_inner[0xFE], 0x02);
    assert_eq!(mem_inner[0xFD], 0x03);
}

#[test]
fn stack_pull_test() {
    let memory: Arc<Mutex<VirtualMemory>> = Arc::new(Mutex::new(VirtualMemory::new()));
    let mut stack_ptr = StackPointerRegister::new(0x01, 0xFF, memory.clone());

    // Execute
    stack_ptr.push(0x01);
    stack_ptr.push(0x02);
    stack_ptr.push(0x03);

    // Verify
    assert_eq!(stack_ptr.pop(), 0x03);
    assert_eq!(stack_ptr.pop(), 0x02);
    assert_eq!(stack_ptr.pop(), 0x01);
}

#[test]
fn status_flag_set_test() {
    // Set up
    let mut flag_register = ProcessorStatusRegister::new();
    let flags = ['c', 'z', 'i', 'd', 'b', 'v', 'n'];

    // Execute
    for flag in flags {
        flag_register.set_flag(flag).unwrap();
    }

    // Verify
    for flag in flags {
        assert_eq!(flag_register.check_flag(flag).unwrap(), true);
    }
}

#[test]
fn status_flag_check() {
    // Set up
    let mut flag_register = ProcessorStatusRegister::new();
    let expected_flags: u8 = 0b01100011;
    let flags = ['c', 'z', 'i', 'd', 'b', '1', 'v', 'n'];
    let set_flags = ['z', 'c', 'v'];

    // Execute
    for flag in set_flags {
        flag_register.set_flag(flag).unwrap();
    }

    // Verify
    for position in 0..7 {
        let expected_flag = (expected_flags & (1 << position)) != 0;
        let flag = flags[position];
        assert_eq!(flag_register.check_flag(flag).unwrap(), expected_flag);
    }
}

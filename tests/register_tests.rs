use std::cell::RefCell;
use std::rc::Rc;

use strum::IntoEnumIterator;
use w65xx_emulator::core::register::*;
use w65xx_emulator::peripherals::memory::VirtualMemory;

#[test]
fn register_read_write_test() {
    // Set up
    let mut accumulator = DataRegister::new("a");
    // Execute
    accumulator.m_value = 0x13;
    accumulator.m_value = 0x13;

    // Verify
    assert_eq!(accumulator.m_value, 0x13 as u8);
}

#[test]
fn reset_register_test() {
    // Set up
    let mut accumulator = DataRegister::new("a");

    // Execute
    accumulator.m_value = 0xFF;
    accumulator.m_value = 0xFF;
    accumulator.reset_register();

    // Verify
    assert_eq!(accumulator.m_value, 0);
}

#[test]
fn index_register_test() {
    // Set up
    let mut x_register = DataRegister::new('x');
    let mut y_register = DataRegister::new('y');
    let mut x_register = DataRegister::new('x');
    let mut y_register = DataRegister::new('y');

    // Execute
    x_register.m_value = 0x7F;
    y_register.m_value = 0xFF;
    x_register.m_value = 0x7F;
    y_register.m_value = 0xFF;

    // Verify
    assert_eq!(x_register.m_value, 0x7F);
    assert_eq!(y_register.m_value, 0xFF);
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
    assert_eq!(pc.m_value, new_pc_value);
}

#[test]
fn program_counter_param_increment_test() {
    // Setup
    let mut pc = ProgramCounter::from(0x8002);
    let parms = 1;

    // Execute
    pc.increment(parms);

    // Verify
    assert_eq!(pc.m_value, 0x8004);
}

#[test]
fn program_counter_increment_test() {
    // Setup
    let mut pc = ProgramCounter::from(0x8002);

    // Execute
    pc.increment(0); // No params, instruction only has op code

    // Verify
    assert_eq!(pc.m_value, 0x8003);
}

#[test]
fn stack_push_test() {
    // Setup
    let memory = Rc::new(RefCell::new(VirtualMemory::new()));
    let mut stack_ptr = StackPointerRegister::new(0x01, 0xFF, memory.clone());

    // Execute
    stack_ptr.push(0x01);
    stack_ptr.push(0x02);
    stack_ptr.push(0x03);

    // Verify
    let mem_inner = memory.borrow();
    assert_eq!(mem_inner[0xFF], 0x01);
    assert_eq!(mem_inner[0xFE], 0x02);
    assert_eq!(mem_inner[0xFD], 0x03);
}

#[test]
fn stack_pull_test() {
    let memory = Rc::new(RefCell::new(VirtualMemory::new()));
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
    let mut flag_register = StatusRegister::new();

    // Execute
    for flag in StatusFlags::iter() {
        flag_register.set_flag(flag);
    }

    // Verify
    for flag in StatusFlags::iter() {
        assert_eq!(flag_register.check_flag(flag), true);
    }
}

#[test]
fn status_flag_check() {
    // Set up
    let mut flag_register = StatusRegister::new();
    let expected_flags: u8 = 0b01100011;
    let set_flags = [StatusFlags::Zero, StatusFlags::Carry, StatusFlags::Overflow];

    // Execute
    for flag in set_flags {
        flag_register.set_flag(flag);
    }

    // Verify
    for (mut position, flag) in StatusFlags::iter().enumerate() {
        if position >= 5 {
            position += 1;
        }
        let expected_flag = (expected_flags & (1 << position)) != 0;
        assert_eq!(flag_register.check_flag(flag), expected_flag);
    }
}

#[test]
fn set_flag_mask_test() {
    // Set up
    let mut flag_register = StatusRegister::new();
    let mask: u8 = 0b01100011;

    // Execute
    flag_register.set_mask(mask);

    // Verify
    assert!(flag_register.check_mask(mask));
}

#[test]
fn clear_flag_mask_test() {
    // Set up
    let mut flag_register = StatusRegister::new();
    let mask: u8 = 0b01100011;
    let expected_flag_state = 0b00100000;

    // Execute
    flag_register.clear_mask(mask);

    // Verify
    assert!(flag_register.check_mask(expected_flag_state));
}

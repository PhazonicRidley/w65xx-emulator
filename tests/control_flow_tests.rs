use std::{cell::RefCell, rc::Rc};
use w65xx_emulator::core::cpu::CPU;
use w65xx_emulator::core::instructions::utils::AddressingModes;
use w65xx_emulator::core::register::StatusFlags;
use w65xx_emulator::peripherals::memory::VirtualMemory;

fn test_setup() -> CPU {
    let memory_rc = Rc::new(RefCell::new(VirtualMemory::new()));
    {
        let mut memory = memory_rc.borrow_mut();

        let paddng: Vec<u8> = vec![0xEA; 0xFF00];
        let working_data: Vec<u8> = (0..0xFC).collect();
        let reset_vector: Vec<u8> = vec![0x00, 0xFF];
        let test_rom = vec![paddng, working_data, reset_vector].concat();
        memory.load_rom(test_rom, 0x00).unwrap();
    }
    let mut cpu = CPU::new(memory_rc.clone());
    cpu.boot_cycle(); // PC starts at 0xFF00
    return cpu;
}

#[test]
fn compare_gt_test() {
    // Setup
    let mut cpu = test_setup();

    // Execute
    cpu.accumulator_cell.borrow_mut().m_value = 0x2;
    cpu.compare(&AddressingModes::Immediate, cpu.accumulator_cell.clone());

    // Verify
    assert!(
        cpu.processor_status_flags.check_flag(StatusFlags::Carry)
            && !cpu.processor_status_flags.check_flag(StatusFlags::Zero)
    );
}

#[test]
fn compare_lt_test() {
    // Setup
    let mut cpu = test_setup();

    // Execute
    cpu.accumulator_cell.borrow_mut().m_value = 1;
    cpu.x_cell.borrow_mut().m_value = 2;
    cpu.compare(
        &AddressingModes::AbsoluteXIndex,
        cpu.accumulator_cell.clone(),
    );

    // Verify
    assert!(
        !cpu.processor_status_flags.check_flag(StatusFlags::Carry)
            && !cpu.processor_status_flags.check_flag(StatusFlags::Zero)
    );
}

#[test]
fn compare_eq_test() {
    // Setup
    let mut cpu = test_setup();

    // Execute
    cpu.accumulator_cell.borrow_mut().m_value = 1;
    cpu.compare(&AddressingModes::Immediate, cpu.accumulator_cell.clone());

    // Verify
    let sfr = &cpu.processor_status_flags;
    assert!(
        sfr.check_flag(StatusFlags::Carry)
            && sfr.check_flag(StatusFlags::Zero)
            && !sfr.check_flag(StatusFlags::Negative)
    );
}

#[test]
fn jump_test() {
    // Setup
    let mut cpu = test_setup();
}

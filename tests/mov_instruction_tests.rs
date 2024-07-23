use std::{cell::RefCell, rc::Rc};
use w65xx_emulator::core::cpu::CPU;
use w65xx_emulator::core::instructions::utils::AddressingModes;
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
fn load_test() {
    // Setup
    let mut cpu = test_setup();

    // Execute
    cpu.load_instruction(AddressingModes::Immediate, cpu.accumulator_cell.clone());

    // Verify
    assert_eq!(cpu.accumulator_cell.borrow().m_value, 0x1);
}

#[test]
fn store_test() {
    // Setup
    let mut cpu = test_setup();
    cpu.accumulator_cell.borrow_mut().m_value = 0x5;

    // Execute
    cpu.store_instruction(AddressingModes::Immediate, cpu.accumulator_cell.clone());

    // Verify
    assert_eq!(cpu.memory_rc.borrow()[0xFF01], 0x5);
}

#[test]
fn transfer_test() {
    // Setup
    let mut cpu = test_setup();
    cpu.accumulator_cell.borrow_mut().m_value = 0x5;
    cpu.x_cell.borrow_mut().m_value = 0xA;

    // Execute
    cpu.transfer_register(cpu.accumulator_cell.clone(), cpu.x_cell.clone());

    // Verify
    assert_eq!(cpu.x_cell.borrow().m_value, 0x5);
}

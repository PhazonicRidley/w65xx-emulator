use std::{cell::RefCell, rc::Rc};
use w65xx_emulator::core::cpu::CPU;
use w65xx_emulator::core::instructions::utils::{AddressingModes, BranchMode};
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
    let value: u8 = 0x81;
    cpu.accumulator_cell.borrow_mut().value = value;

    // Execute
    cpu.compare(&AddressingModes::Immediate, cpu.accumulator_cell.clone());

    // Verify
    let sfr = &cpu.processor_status_flags;
    assert!(
        !sfr.check_flag(StatusFlags::Zero)
            && sfr.check_flag(StatusFlags::Carry)
            && sfr.check_flag(StatusFlags::Negative) == ((value & 128) != 0)
    );
}

#[test]
fn compare_lt_test() {
    // Setup
    let mut cpu = test_setup();
    let value: u8 = 1;
    cpu.accumulator_cell.borrow_mut().value = value;
    cpu.x_cell.borrow_mut().value = 2;

    // Execute
    cpu.compare(
        &AddressingModes::AbsoluteXIndex,
        cpu.accumulator_cell.clone(),
    );

    // Verify
    let sfr = &cpu.processor_status_flags;
    assert!(
        !sfr.check_flag(StatusFlags::Zero)
            && !sfr.check_flag(StatusFlags::Carry)
            && sfr.check_flag(StatusFlags::Negative) == ((value & 128) != 0)
    );
}

#[test]
fn compare_eq_test() {
    // Setup
    let mut cpu = test_setup();

    // Execute
    cpu.accumulator_cell.borrow_mut().value = 1;
    cpu.compare(&AddressingModes::Immediate, cpu.accumulator_cell.clone());

    // Verify
    let sfr = &cpu.processor_status_flags;
    assert!(
        sfr.check_flag(StatusFlags::Zero)
            && sfr.check_flag(StatusFlags::Carry)
            && !sfr.check_flag(StatusFlags::Negative)
    );
}

#[test]
fn jump_test() {
    // Setup
    let mut cpu = test_setup();
    cpu.memory_rc.borrow_mut()[0x0201] = !2 + 1;

    // Execute
    cpu.jump(&AddressingModes::Absolute, false, false);

    // Verify
    assert_eq!(cpu.program_counter.value, 0xfefe);
}

#[test]
fn subroutine_test() {
    // Setup
    let mut cpu = test_setup();
    cpu.memory_rc.borrow_mut()[0x0201] = 20;
    let next_pc = cpu.program_counter.value + 1;

    // Execute (JSR)
    cpu.jump(&AddressingModes::Absolute, true, false);

    // Verify (JSR)
    assert_eq!(cpu.program_counter.value, 0xff14);
    {
        let sp = 0x0100 | cpu.stack_pointer.get_pointer() as u16;
        let memory = cpu.memory_rc.borrow();
        let pcl = memory[sp + 1];
        let pch = memory[sp + 2];
        assert_eq!(pcl, 0x00);
        assert_eq!(pch, 0xff);
    }

    // Execute (RTS)
    cpu.subroutine_return();

    // Verify (RTS)
    assert_eq!(cpu.program_counter.value, next_pc);
    let sp = 0x0100 | cpu.stack_pointer.get_pointer() as u16;
    assert_eq!(sp, 0x01ff);
}

#[test]
fn branching_test() {
    // Setup
    let mut cpu = test_setup();
    let offset: u8 = 0x20;
    cpu.memory_rc.borrow_mut()[0xff01] = offset;

    // Execute
    let expected_value = cpu.program_counter.value + (offset as u16);
    cpu.processor_status_flags.set_flag(StatusFlags::Carry);
    cpu.branch_exec(BranchMode::BCS);

    // Verify
    assert_eq!(cpu.program_counter.value, expected_value);
}

#[test]
fn subtractive_branching_test() {
    // Setup
    let mut cpu = test_setup();
    let offset: u8 = !0x20 + 1;
    cpu.memory_rc.borrow_mut()[0xff01] = 255;

    // Execute
    println!(
        "offset: {:x} Original PC: {:x}",
        offset, cpu.program_counter.value
    );
    let expected_value: u16 = cpu.program_counter.value + (offset as u16);
    println!(
        "Test 0xff00 - 0x20 = = {:x}, 0xff00 + (~0x20 + 1) = {:x}",
        0xff00_u16 - 0x20_u16,
        (!0x20_u16 + 1)
    );
    cpu.processor_status_flags.set_flag(StatusFlags::Carry);
    cpu.branch_exec(BranchMode::BCS);

    // Verify
    println!(
        "expected_value: {:x} New PC: {:x}",
        expected_value, cpu.program_counter.value
    );
    //assert_eq!(expected_value, cpu.program_counter.value);
}

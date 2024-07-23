use std::{cell::RefCell, rc::Rc};
use w65xx_emulator::core::cpu::*;
use w65xx_emulator::core::instructions::utils::AddressingModes;
use w65xx_emulator::peripherals::memory::VirtualMemory;

#[test]
fn verify_boot_cycle() {
    // Set up
    let mem_rc = Rc::new(RefCell::new(VirtualMemory::new()));
    let rom: Vec<u8> = vec![0x00, 0x80];
    let mut cpu = CPU::new(mem_rc.clone());

    // Execute
    mem_rc.borrow_mut().load_rom(rom, 0xFFFC).unwrap();
    cpu.boot_cycle();

    // Verify
    assert_eq!(mem_rc.borrow().read_word(0xFFFC), 0x8000);
    assert_eq!(cpu.program_counter.m_value, 0x8000);
}

fn address_mode_setup() -> CPU {
    let mem_rc = Rc::new(RefCell::new(VirtualMemory::new()));
    // Make a test rom that is padded from 0 to 0xFEFF with 0xEA and
    // Then 0xFEFF to 0xFFFB is just counted up from 0, finally 0xFFFC is 0x00 and 0xFFFD is 0xFF
    let zp_padding: Vec<u8> = (0x00..=0xFF).collect();
    let mut stack_page_padding: Vec<u8> = (0x00..=0xFF).collect();
    stack_page_padding.push(0x00);
    let padding: Vec<u8> = vec![0xEA; 0xFCFF];
    let test_rom: Vec<u8> = (0..0xFC).collect();
    let reset_vector: Vec<u8> = vec![0x00, 0xFF];
    let rom = vec![
        zp_padding,
        stack_page_padding,
        padding,
        test_rom,
        reset_vector,
        vec![0x00],
    ]
    .concat();
    // let mut i = 0;
    // for byte in &rom {
    //     println!("{:#04x}: {:#02x}", i, byte);
    //     i += 1;
    // }

    // Sanity checks
    assert_eq!(rom.len(), 0xFFFF);
    assert_eq!(rom[0xFF00], 0x00);
    assert_eq!(rom[0xFFFC], 0x00);
    assert_eq!(rom[0xFFFD], 0xFF);
    {
        let mut memory = mem_rc.borrow_mut();
        memory.load_rom(rom, 0x0).unwrap();
    }
    let mut cpu = CPU::new(mem_rc.clone());
    cpu.boot_cycle();
    assert_eq!(cpu.program_counter.m_value, 0xFF00);
    return cpu;
}

#[test]
fn addr_setup_test() {
    address_mode_setup();
}

#[test]
fn addr_immiate_test() {
    // Setup
    let cpu = address_mode_setup();
    let pc = &cpu.program_counter;

    // Execute
    let address = cpu.fetch_address(&AddressingModes::Immediate).unwrap();
    let data = cpu.memory_rc.borrow()[address];

    // Verify
    assert_eq!(data, 0x01);
    assert_eq!(address, 0xFF01);
    assert_eq!(pc.m_value, 0xFF00);
}

#[test]
fn addr_absolute_test() {
    // Setup
    let cpu = address_mode_setup();
    let pc = &cpu.program_counter;

    // Execute
    let address = cpu.fetch_address(&AddressingModes::Absolute).unwrap();
    let data = cpu.memory_rc.borrow()[address];

    // Verify
    assert_eq!(pc.m_value, 0xFF00);
    assert_eq!(address, 0x0201);
    assert_eq!(data, 0xea);
}

#[test]
fn addr_absolute_x_test() {
    // Setup
    let cpu = address_mode_setup();

    // Execute
    cpu.x_cell.borrow_mut().m_value = 4;
    let address = cpu.fetch_address(&AddressingModes::AbsoluteXIndex).unwrap();
    let data = cpu.memory_rc.borrow()[address];

    // Verify
    assert_eq!(cpu.program_counter.m_value, 0xFF00);
    assert_eq!(address, 0x0205);
    assert_eq!(data, 0xea)
}

#[test]
fn addr_absolute_y_test() {
    // Setup
    let cpu = address_mode_setup();
    let pc = &cpu.program_counter;

    // Execute
    cpu.y_cell.borrow_mut().m_value = 7;
    let address = cpu.fetch_address(&AddressingModes::AbsoluteYIndex).unwrap();
    let data = cpu.memory_rc.borrow()[address];

    // Verify
    assert_eq!(pc.m_value, 0xFF00);
    assert_eq!(address, 0x0208);
    assert_eq!(data, 0xea)
}

#[test]
fn addr_indirect_test() {
    // Setup
    let cpu = address_mode_setup();
    let pc = &cpu.program_counter;

    // Execute
    let address = cpu.fetch_address(&AddressingModes::Indirect).unwrap();
    let data = cpu.memory_rc.borrow()[address];

    // Verify
    assert_eq!(pc.m_value, 0xFF00);
    assert_eq!(address, 0xEAEA);
    assert_eq!(data, 0xEA);
}

#[test]
fn addr_indirect_x_test() {
    // Setup
    let cpu = address_mode_setup();
    let pc = &cpu.program_counter;

    // Execute
    cpu.x_cell.borrow_mut().m_value = 5;
    let address = cpu
        .fetch_address(&AddressingModes::PreIndexIndirect)
        .unwrap();
    let data = cpu.memory_rc.borrow()[address];

    // Verify
    assert_eq!(pc.m_value, 0xFF00);
    assert_eq!(address, 0x0706); // pc + 1 = 0x1, x = 5. lookup addr = 0x0006, read word: 0x0706
    assert_eq!(data, 0xea); // almost all padding is 0xEA (nop instruction)
}

#[test]
fn addr_indirect_y_test() {
    // Setup
    let cpu = address_mode_setup();
    let pc = &cpu.program_counter;

    // Execute
    cpu.y_cell.borrow_mut().m_value = 10;
    let address = cpu
        .fetch_address(&AddressingModes::PostIndexIndirect)
        .unwrap();
    let data = cpu.memory_rc.borrow()[address];

    // Verify
    assert_eq!(pc.m_value, 0xFF00);
    assert_eq!(address, 0x0C0B); // pc + 1 = 0x1, lookup address = 0xB (0x0001 has 0x1) then + 10 (y) gives 0xB
    assert_eq!(data, 0xea); // almost all padding is 0xEA (nop instruction)
}

#[test]
fn addr_zeropage_test() {
    // Setup
    let cpu = address_mode_setup();
    let pc = &cpu.program_counter;

    // Execute
    let address = cpu.fetch_address(&AddressingModes::ZeroPage).unwrap();
    let data = cpu.memory_rc.borrow()[address];

    // Verify
    assert_eq!(pc.m_value, 0xFF00);
    assert_eq!(address, 0x0001);
    assert_eq!(data, 0x1);
}

#[test]
fn addr_zeropage_x_test() {
    // Setup
    let cpu = address_mode_setup();
    let pc = &cpu.program_counter;

    // Execute
    cpu.x_cell.borrow_mut().m_value = 7;
    let address = cpu.fetch_address(&AddressingModes::ZeroPageXIndex).unwrap();
    let data = cpu.memory_rc.borrow()[address];

    // Verify
    assert_eq!(pc.m_value, 0xFF00);
    assert_eq!(address, 0x0008);
    assert_eq!(data, 0x8);
}

#[test]
fn addr_zeropage_y_test() {
    // Setup
    let cpu = address_mode_setup();
    let pc = &cpu.program_counter;

    // Execute
    cpu.y_cell.borrow_mut().m_value = 10;
    let address = cpu.fetch_address(&AddressingModes::ZeroPageYIndex).unwrap();
    let data = cpu.memory_rc.borrow()[address];

    // Verify
    assert_eq!(pc.m_value, 0xFF00);
    assert_eq!(address, 0x000B);
    assert_eq!(data, 0xB);
}

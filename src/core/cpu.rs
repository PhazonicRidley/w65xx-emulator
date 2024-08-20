use std::cell::RefCell;
use std::rc::Rc;

use crate::peripherals::memory::VirtualMemory;

use super::{instructions::utils::AddressingModes, register::*};

#[derive(Debug)]
pub struct CPU {
    // IO

    // Registers
    pub accumulator_cell: Rc<RefCell<DataRegister>>,
    pub x_cell: Rc<RefCell<DataRegister>>,
    pub y_cell: Rc<RefCell<DataRegister>>,
    pub program_counter: ProgramCounter,
    pub stack_pointer: StackPointerRegister,
    pub processor_status_flags: StatusRegister,

    // Memory
    pub memory_rc: Rc<RefCell<VirtualMemory>>,
}

impl CPU {
    // TODO: Add configs if needed.
    pub fn new(memory_arc: Rc<RefCell<VirtualMemory>>) -> Self {
        let mem_arc = memory_arc.clone();
        return CPU {
            accumulator_cell: Rc::new(RefCell::new(DataRegister::new(String::from("A")))),
            x_cell: Rc::new(RefCell::new(DataRegister::new(String::from("X")))),
            y_cell: Rc::new(RefCell::new(DataRegister::new(String::from("Y")))),
            program_counter: ProgramCounter::from(0),
            stack_pointer: StackPointerRegister::new(0x01, 0xFF, memory_arc),
            processor_status_flags: StatusRegister::new(),
            memory_rc: mem_arc,
        };
    }

    pub fn boot_cycle(&mut self) {
        // TODO: 7 clock cycles
        self.program_counter.reset_register();
        let program_start_location: u16;
        {
            let memory = self.memory_rc.borrow();
            program_start_location = memory.read_word(0xFFFC);
        }
        self.program_counter.value = program_start_location;
    }

    pub fn fetch_address(&self, addressing_mode: &AddressingModes) -> Option<u16> {
        let memory = self.memory_rc.borrow();
        let pc = &self.program_counter;
        let x = self.x_cell.borrow();
        let y = self.y_cell.borrow();
        match addressing_mode {
            AddressingModes::Immediate | AddressingModes::Relative => Some(pc.value + 1),
            AddressingModes::Absolute => Some(memory.read_word(pc.value + 1)),
            AddressingModes::AbsoluteXIndex => {
                Some(memory.read_word(pc.value + 1) + (x.value as u16))
            }
            AddressingModes::AbsoluteYIndex => {
                Some(memory.read_word(pc.value + 1) + (y.value as u16))
            }
            AddressingModes::Indirect => {
                let lookup_addr = memory.read_word(pc.value + 1);
                Some(memory.read_word(lookup_addr))
            }
            AddressingModes::ZeroPage => Some(memory[pc.value + 1] as u16),
            AddressingModes::ZeroPageXIndex => Some((memory[pc.value + 1] + x.value) as u16),
            AddressingModes::ZeroPageYIndex => Some((memory[pc.value + 1] + y.value) as u16),

            AddressingModes::PreIndexIndirect => {
                let lookup_addr = (memory[pc.value + 1 + (x.value as u16)]) as u16;
                Some(memory.read_word(lookup_addr))
            }
            AddressingModes::PostIndexIndirect => {
                let lookup_addr = (memory[pc.value + 1] + y.value) as u16;
                Some(memory.read_word(lookup_addr))
            }

            _ => None,
        }
    }
}

use std::sync::{Arc, Mutex};

use crate::peripherals::memory::VirtualMemory;

use super::{instructions::utils::AddressingModes, register::*};

#[derive(Debug)]
pub struct CPU {
    // IO

    // Registers
    pub accumulator: Accumulator,
    pub x_register: IndexRegister,
    pub y_register: IndexRegister,
    pub program_counter: ProgramCounter,
    pub stack_pointer: StackPointerRegister,
    pub processor_status_flags: ProcessorStatusRegister,

    // Memory
    pub memory_arc: Arc<Mutex<VirtualMemory>>,
}

impl CPU {
    // TODO: Add configs if needed.
    pub fn new(memory_arc: Arc<Mutex<VirtualMemory>>) -> Self {
        let mem_arc = memory_arc.clone();
        return CPU {
            accumulator: Accumulator::new(),
            x_register: IndexRegister::new('x'),
            y_register: IndexRegister::new('y'),
            program_counter: ProgramCounter::from(0),
            stack_pointer: StackPointerRegister::new(0x01, 0xFF, memory_arc),
            processor_status_flags: ProcessorStatusRegister::new(),
            memory_arc: mem_arc,
        };
    }

    pub fn boot_cycle(&mut self) {
        // TODO: 7 clock cycles
        self.program_counter.reset_register();
        let program_start_location: u16;
        {
            let memory = self.memory_arc.try_lock().unwrap();
            program_start_location = memory.read_word(0xFFFC);
        }
        self.program_counter.load_data(program_start_location);
    }

    pub fn fetch_address(&self, addressing_mode: &AddressingModes) -> Option<u16> {
        let memory = self.memory_arc.try_lock().unwrap();
        let pc = &self.program_counter;
        let x = &self.x_register;
        let y = &self.y_register;
        match addressing_mode {
            AddressingModes::Immediate => Some(pc.get_data() + 1),
            AddressingModes::Absolute => Some(memory.read_word(pc.get_data() + 1)),
            AddressingModes::AbsoluteXIndex => {
                Some(memory.read_word(pc.get_data() + 1) + (x.get_data() as u16))
            }
            AddressingModes::AbsoluteYIndex => {
                Some(memory.read_word(pc.get_data() + 1) + (y.get_data() as u16))
            }
            AddressingModes::Indirect => {
                let lookup_addr = memory.read_word(pc.get_data() + 1);
                Some(memory.read_word(lookup_addr))
            }
            AddressingModes::ZeroPage => Some(memory[pc.get_data() + 1] as u16),
            AddressingModes::ZeroPageXIndex => {
                Some((memory[pc.get_data() + 1] + x.get_data()) as u16)
            }
            AddressingModes::ZeroPageYIndex => {
                Some((memory[pc.get_data() + 1] + y.get_data()) as u16)
            }

            AddressingModes::PreIndexIndirect => {
                let lookup_addr = (memory[pc.get_data() + 1 + (x.get_data() as u16)]) as u16;
                Some(memory.read_word(lookup_addr))
            }
            AddressingModes::PostIndexIndirect => {
                let lookup_addr = (memory[pc.get_data() + 1] + y.get_data()) as u16;
                Some(memory.read_word(lookup_addr))
            }
            _ => None,
        }
    }
}

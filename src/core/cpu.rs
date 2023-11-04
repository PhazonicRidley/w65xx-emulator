use std::sync::{Arc, Mutex};

use crate::peripherals::memory::VirtualMemory;

use super::{io::PinIO, register::*};

#[derive(Debug)]
pub struct CPU {
    // IO
    pins: PinIO,

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
    // From the prospective of the microprocessor
    pub fn read_data(&mut self, data: u8) {
        self.pins.data_buffer = data;
    }

    pub fn write_data(&self) -> u8 {
        return self.pins.data_buffer;
    }

    pub fn set_current_address(&mut self, address: u16) {
        self.pins.address_buffer = address;
    }

    pub fn get_current_address(&self) -> u16 {
        return self.pins.address_buffer;
    }

    pub fn boot_cycle(&mut self) {
        // TODO: 7 clock cycles
        self.program_counter.reset_register();
        let program_start_location: u16;
        {
            let memory = self.memory_arc.try_lock().unwrap();
            program_start_location = memory.read_word(0xFFFC);
        }
        self.set_current_address(program_start_location);
        self.program_counter.load_data(program_start_location);
    }
}

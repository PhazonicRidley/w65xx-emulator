use std::{cell::RefCell, rc::Rc};

use super::utils::AddressingModes;
use crate::core::{cpu::CPU, register::DataRegister};

// new LDA, LDX, LDY

impl CPU {
    // LDA, LDX, LDY
    pub fn load_instruction(
        &mut self,
        addressing_mode: AddressingModes,
        destination_reg_cell: Rc<RefCell<DataRegister>>,
    ) {
        let address: u16 = self.fetch_address(&addressing_mode).unwrap();
        let data;

        let memory = self.memory_rc.borrow_mut();
        data = memory[address];

        let mut register = destination_reg_cell.borrow_mut();
        register.value = data;
        self.processor_status_flags.update_nz_flags(register.value);
        self.program_counter
            .increment(addressing_mode.parameter_bytes());
    }

    // STA, STX, STY
    pub fn store_instruction(
        &mut self,
        addressing_mode: AddressingModes,
        source_reg_cell: Rc<RefCell<DataRegister>>,
    ) {
        let address = self.fetch_address(&addressing_mode).unwrap();
        {
            let mut memory = self.memory_rc.borrow_mut();
            memory[address] = source_reg_cell.borrow().value;
        }

        self.program_counter
            .increment(addressing_mode.parameter_bytes());
    }

    // TAX, TAY, TSX, TXA, TXS, TYA
    pub fn transfer_register(
        &mut self,
        source_register: Rc<RefCell<DataRegister>>,
        destination_register: Rc<RefCell<DataRegister>>,
    ) {
        let data = source_register.borrow().value;
        destination_register.borrow_mut().value = data;
        self.processor_status_flags
            .update_nz_flags(destination_register.borrow().value);
        self.program_counter.increment(0); // Only possible addressing mode is implied which has no parameters.
    }
}

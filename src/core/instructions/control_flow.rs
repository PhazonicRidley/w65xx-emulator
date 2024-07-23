use std::{cell::RefCell, rc::Rc};

use super::{alu, utils::AddressingModes, utils::BranchMode};
use crate::core::{cpu::CPU, register::DataRegister};

impl CPU {
    // CMP
    pub fn compare(
        &mut self,
        addressing_mode: &AddressingModes,
        reg_cell: Rc<RefCell<DataRegister>>,
    ) {
        let address = self.fetch_address(addressing_mode).unwrap();
        let mem_data = self.memory_rc.borrow()[address];
        alu::add_two_numbers(
            &mut self.processor_status_flags,
            reg_cell.borrow().m_value,
            !mem_data,
        );
    }

    // JMP, JSR
    pub fn jump(&mut self, addressing_mode: AddressingModes, is_subroutine: bool) {
        let pc_val = self.program_counter.m_value;
        let address = self.fetch_address(&addressing_mode).unwrap();
        let delta = self.memory_rc.borrow_mut()[address];
        let new_pc = pc_val.wrapping_add(delta as u16);
        if is_subroutine {
            // account for the current jump instruction. NOTE: this does not include the third byte of the JSR
            // instruction due to the way the actual hardware works. RTS will increment PC by 1 before setting PC.
            self.stack_pointer.push(self.program_counter.get_pch());
            self.stack_pointer.push(self.program_counter.get_pcl());
        }
        self.program_counter.m_value = new_pc; // Jump
    }

    // RTS
    pub fn subroutine_return(&mut self) {
        let new_pcl = self.stack_pointer.pop();
        let new_pch = self.stack_pointer.pop();

        self.program_counter.set_pch(new_pch);
        self.program_counter.set_pcl(new_pcl);
        self.program_counter.increment(0);
    }

    // BEQ, BNE, BMI, BCC, BCS, BVC, BVS, BPL
    pub fn branch_exec(&mut self, branch_mode: BranchMode) {
        if !branch_mode.verify(&self.processor_status_flags) {
            return;
        }
        let address = self.fetch_address(&AddressingModes::Relative).unwrap();
        let delta = self.memory_rc.borrow()[address];
        let old_pc = self.program_counter.m_value;
        let new_pc = old_pc.wrapping_add(delta as u16);
        self.program_counter.m_value = new_pc;
    }
}

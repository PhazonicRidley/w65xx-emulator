use super::{alu, utils::AddressingModes, utils::BranchMode};
use crate::core::{
    cpu::CPU,
    register::{ProcessorStatusRegister, Register},
};

// CMP
pub fn compare(processor_status_flags: &mut ProcessorStatusRegister, first: u8, second: u8) {
    alu::add_two_numbers(processor_status_flags, first, !second);
}

// JMP, JSR
pub fn jump(addressing_mode: AddressingModes, cpu: &mut CPU, is_subroutine: bool) {
    let mut pc_val = cpu.program_counter.get_data();
    let address = cpu.fetch_address(&addressing_mode).unwrap();
    let delta = cpu.memory_arc.lock().unwrap()[address];
    let new_pc = pc_val.wrapping_add(delta as u16);
    if is_subroutine {
        // account for the current jump instruction. NOTE: this does not include the third byte of the JSR
        // instruction due to the way the actual hardware works. RTS will increment PC by 1 before setting PC.
        pc_val = pc_val.wrapping_add(2);

        cpu.stack_pointer.push(cpu.program_counter.get_pcl());
        cpu.stack_pointer.push(cpu.program_counter.get_pch());
    }
    cpu.program_counter.load_data(new_pc) // Jump
}

// BEQ, BNE, BMI, BCC, BCS, BVC, BVS, BPL
pub fn branch_exec(cpu: &mut CPU, branch_mode: BranchMode, delta: u8) {
    if !branch_mode.verify(&cpu.processor_status_flags) {
        return;
    }
    let pc = &mut cpu.program_counter;
    let old_pc = pc.get_data();
    let new_pc = old_pc.wrapping_add(delta as u16);
    pc.load_data(new_pc);
}

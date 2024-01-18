use crate::core::{cpu::CPU, register::Register};

// PHA
pub fn push_accumulator(cpu: &mut CPU) {
    let acc_value = cpu.accumulator.get_data();
    cpu.stack_pointer.push(acc_value);
}

// PHP
pub fn push_status(cpu: &mut CPU) {
    let flags = cpu.processor_status_flags.get_flags();
    cpu.stack_pointer.push(flags);
}

// PLA
pub fn pop_accumulator(cpu: &mut CPU) {
    let acc_value = cpu.stack_pointer.pop();
    cpu.accumulator.load_data(acc_value);
}

// PLP
pub fn pop_status(cpu: &mut CPU) {
    let flags = cpu.stack_pointer.pop();
    cpu.processor_status_flags.set_mask(flags);
}

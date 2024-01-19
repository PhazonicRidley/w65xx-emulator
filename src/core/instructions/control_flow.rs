use crate::core::{
    cpu::CPU,
    register::{Register, StatusFlags},
};

use super::alu;
use super::utils::AddressingModes;

pub fn compare(addressing_mode: &AddressingModes, register: &impl Register<u8>, cpu: &mut CPU) {
    let register_operand = register.get_data();
    let address = cpu.fetch_address(addressing_mode).unwrap();
    let memory_operand = !cpu.memory_arc.lock().unwrap()[address] + 1; // negative of the memory operand
    alu::add_two_numbers(
        &mut cpu.processor_status_flags,
        register_operand,
        memory_operand,
    );
}

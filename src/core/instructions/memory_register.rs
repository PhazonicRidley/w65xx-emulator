use super::utils::AddressingModes;
use crate::core::{cpu::CPU, register::Register};

// LDA, LDX, LDY
pub fn load_instruction(
    addressing_mode: AddressingModes,
    register: &mut impl Register<u8>,
    cpu: &mut CPU,
) {
    let address: u16 = cpu.fetch_address(&addressing_mode).unwrap();
    let pc = &mut cpu.program_counter;
    let data;
    {
        let memory = cpu.memory_arc.try_lock().unwrap();
        data = memory[address];
    }
    register.load_data(data);
    cpu.processor_status_flags
        .update_nz_flags(register.get_data());
    pc.increment(addressing_mode.parameter_bytes());
}

// STA, STX, STY
pub fn store_instruction(
    addressing_mode: AddressingModes,
    register: &impl Register<u8>,
    cpu: &mut CPU,
) {
    let address = cpu.fetch_address(&addressing_mode).unwrap();
    let pc = &mut cpu.program_counter;
    {
        let mut memory = cpu.memory_arc.try_lock().unwrap();
        memory[address] = register.get_data();
    }

    pc.increment(addressing_mode.parameter_bytes());
}

// TAX, TAY, TSX, TXA, TXS, TYA
pub fn transfer_register(
    source_register: &impl Register<u8>,
    destination_register: &mut impl Register<u8>,
    cpu: &mut CPU,
) {
    let data = source_register.get_data();
    destination_register.load_data(data);
    cpu.processor_status_flags
        .update_nz_flags(destination_register.get_data());
    cpu.program_counter.increment(0); // Only possible addressing mode is implied which has no parameters.
}

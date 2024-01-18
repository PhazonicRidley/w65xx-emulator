use super::utils::AddressingModes;
use crate::core::{
    cpu::CPU,
    register::{ProcessorStatusRegister, Register, StatusFlags},
};

// Arithmetic functionality
fn add_two_numbers(status_flags: &mut ProcessorStatusRegister, first: u8, second: u8) -> u8 {
    let carry_flag = status_flags.check_flag(StatusFlags::Carry) as u8;
    let second_operand = second + carry_flag;
    let sum = first.wrapping_add(second_operand);
    status_flags.add_update_carry_flag(first, second_operand);
    status_flags.update_overflow_flag(first, second, sum);
    status_flags.update_nz_flags(sum);

    return sum;
}
// ADC
pub fn add_with_carry(addressing_mode: &AddressingModes, cpu: &mut CPU) {
    let address = cpu.fetch_address(&addressing_mode).unwrap(); // Should always return an address, this does not support an addressing mode that doesn't
    let memory_data = cpu.memory_arc.lock().unwrap()[address];
    let acc_data = cpu.accumulator.get_data();
    let sum = add_two_numbers(&mut cpu.processor_status_flags, acc_data, memory_data);
    cpu.accumulator.load_data(sum);
    cpu.program_counter
        .increment(addressing_mode.parameter_bytes());
}

// SBC
pub fn sub_with_carry(addressing_mode: &AddressingModes, cpu: &mut CPU) {
    let address = cpu.fetch_address(&addressing_mode).unwrap(); // Should always return an address, this does not support an addressing mode that doesn't
    let memory_data = !(cpu.memory_arc.lock().unwrap()[address]); // one's compliment of second operand
    let acc_data = cpu.accumulator.get_data();
    let sum = add_two_numbers(&mut cpu.processor_status_flags, acc_data, memory_data);
    cpu.accumulator.load_data(sum);
    cpu.program_counter
        .increment(addressing_mode.parameter_bytes());
}

// Bitwise logic functionality
fn bitwise_operations(
    addressing_mode: &AddressingModes,
    cpu: &mut CPU,
    operation: impl Fn(u8, u8) -> u8,
) {
    let address = cpu.fetch_address(&addressing_mode).unwrap();
    let memory_data = cpu.memory_arc.lock().unwrap()[address];
    let acc_data = cpu.accumulator.get_data();
    let op_result = operation(acc_data, memory_data);
    cpu.accumulator.load_data(op_result);
    cpu.processor_status_flags.update_nz_flags(op_result);
    cpu.program_counter
        .increment(addressing_mode.parameter_bytes());
}

// AND
pub fn bitwise_and(addressing_mode: &AddressingModes, cpu: &mut CPU) {
    bitwise_operations(addressing_mode, cpu, |x, y| x & y);
}

// ORA
pub fn bitwise_or(addressing_mode: &AddressingModes, cpu: &mut CPU) {
    bitwise_operations(addressing_mode, cpu, |x, y| x | y);
}

// EOR
pub fn bitwise_exclusive_or(addressing_mode: &AddressingModes, cpu: &mut CPU) {
    bitwise_operations(addressing_mode, cpu, |x, y| x ^ y);
}

// ASL, ROL
pub fn left_shift(addressing_mode: &AddressingModes, cpu: &mut CPU, rotate: bool) {
    let address_option = cpu.fetch_address(addressing_mode); // None means the addressing mode is the accumulator
    let mut data = match address_option {
        Some(addr) => cpu.memory_arc.lock().unwrap()[addr],
        None => cpu.accumulator.get_data(),
    };
    let old_carry = cpu.processor_status_flags.check_flag(StatusFlags::Carry) as u8;
    let new_carry = data & 0x80; // shift the MSB down to be a 1 if its set at all to be set in the carry.
    data <<= 1;

    if new_carry != 0 {
        cpu.processor_status_flags.set_flag(StatusFlags::Carry);
    } else {
        cpu.processor_status_flags.clear_flag(StatusFlags::Carry);
    }

    if rotate {
        data |= old_carry;
    }
    match address_option {
        Some(addr) => cpu.memory_arc.lock().unwrap()[addr] = data,
        None => cpu.accumulator.load_data(data),
    };
    cpu.processor_status_flags.update_nz_flags(data);
    cpu.program_counter
        .increment(addressing_mode.parameter_bytes());
}

// LSR, ROR
pub fn right_shift(addressing_mode: &AddressingModes, cpu: &mut CPU, rotate: bool) {
    let old_carry = cpu.processor_status_flags.check_flag(StatusFlags::Carry) as u8;
    let address_option = cpu.fetch_address(addressing_mode);
    let mut data = match address_option {
        Some(addr) => cpu.memory_arc.lock().unwrap()[addr],
        None => cpu.accumulator.get_data(),
    };
    let new_carry = data & 1;
    data >>= 1;
    if new_carry != 0 {
        cpu.processor_status_flags.set_flag(StatusFlags::Carry);
    } else {
        cpu.processor_status_flags.clear_flag(StatusFlags::Carry);
    }

    if rotate {
        data |= old_carry << 7;
    }

    match address_option {
        Some(addr) => cpu.memory_arc.lock().unwrap()[addr] = data,
        None => cpu.accumulator.load_data(data),
    };

    cpu.processor_status_flags.update_nz_flags(data);
    cpu.program_counter
        .increment(addressing_mode.parameter_bytes());
}

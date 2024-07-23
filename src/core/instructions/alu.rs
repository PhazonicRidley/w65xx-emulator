use std::{cell::RefCell, rc::Rc};

use super::utils::AddressingModes;
use crate::core::{
    cpu::CPU,
    register::{DataRegister, StatusFlags, StatusRegister},
};

// Arithmetic functionality
pub fn add_two_numbers(status_flags: &mut StatusRegister, first: u8, second: u8) -> u8 {
    let carry_flag = status_flags.check_flag(StatusFlags::Carry) as u8;
    let second_operand = second + carry_flag;
    let sum = first.wrapping_add(second_operand);
    status_flags.add_update_carry_flag(first, second_operand);
    status_flags.update_overflow_flag(first, second, sum);
    status_flags.update_nz_flags(sum);

    return sum;
}
// ADC, SBC
impl CPU {
    pub fn sum_with_carry(&mut self, addressing_mode: &AddressingModes, subtract: bool) {
        let address = self.fetch_address(&addressing_mode).unwrap(); // Should always return an address, this does not support an addressing mode that doesn't
        let mut memory_data = self.memory_rc.borrow_mut()[address];
        if subtract {
            memory_data = !memory_data;
        }
        let acc_data = self.accumulator_cell.borrow().m_value;
        let sum = add_two_numbers(&mut self.processor_status_flags, acc_data, memory_data);
        self.accumulator_cell.borrow_mut().m_value = sum;
        self.program_counter
            .increment(addressing_mode.parameter_bytes());
    }

    // Bitwise logic functionality
    fn bitwise_operations(
        &mut self,
        addressing_mode: &AddressingModes,
        operation: impl Fn(u8, u8) -> u8,
    ) {
        let address = self.fetch_address(&addressing_mode).unwrap();
        let memory_data = self.memory_rc.borrow()[address];
        let op_result: u8;
        {
            let mut accumulator = self.accumulator_cell.borrow_mut();
            let acc_data = accumulator.m_value;
            op_result = operation(acc_data, memory_data);
            accumulator.m_value = op_result;
        }

        self.processor_status_flags.update_nz_flags(op_result);
        self.program_counter
            .increment(addressing_mode.parameter_bytes());
    }

    // AND
    pub fn bitwise_and(&mut self, addressing_mode: &AddressingModes) {
        self.bitwise_operations(addressing_mode, |x, y| x & y);
    }

    // ORA
    pub fn bitwise_or(&mut self, addressing_mode: &AddressingModes) {
        self.bitwise_operations(addressing_mode, |x, y| x | y);
    }

    // EOR
    pub fn bitwise_exclusive_or(&mut self, addressing_mode: &AddressingModes) {
        self.bitwise_operations(addressing_mode, |x, y| x ^ y);
    }

    // ASL, ROL
    pub fn left_shift(&mut self, addressing_mode: &AddressingModes, rotate: bool) {
        let address_option = self.fetch_address(addressing_mode); // None means the addressing mode is the accumulator
        let mut data = match address_option {
            Some(addr) => self.memory_rc.borrow()[addr],
            None => self.accumulator_cell.borrow().m_value,
        };
        let old_carry = self.processor_status_flags.check_flag(StatusFlags::Carry) as u8;
        let new_carry = data & 0x80; // shift the MSB down to be a 1 if its set at all to be set in the carry.
        data <<= 1;

        if new_carry != 0 {
            self.processor_status_flags.set_flag(StatusFlags::Carry);
        } else {
            self.processor_status_flags.clear_flag(StatusFlags::Carry);
        }

        if rotate {
            data |= old_carry;
        }
        match address_option {
            Some(addr) => self.memory_rc.borrow_mut()[addr] = data,
            None => self.accumulator_cell.borrow_mut().m_value = data,
        };
        self.processor_status_flags.update_nz_flags(data);
        self.program_counter
            .increment(addressing_mode.parameter_bytes());
    }

    // LSR, ROR
    pub fn right_shift(&mut self, addressing_mode: &AddressingModes, rotate: bool) {
        let old_carry = self.processor_status_flags.check_flag(StatusFlags::Carry) as u8;
        let address_option = self.fetch_address(addressing_mode);
        let mut data = match address_option {
            Some(addr) => self.memory_rc.borrow()[addr],
            None => self.accumulator_cell.borrow().m_value,
        };
        let new_carry = data & 1;
        data >>= 1;
        if new_carry != 0 {
            self.processor_status_flags.set_flag(StatusFlags::Carry);
        } else {
            self.processor_status_flags.clear_flag(StatusFlags::Carry);
        }

        if rotate {
            data |= old_carry << 7;
        }

        match address_option {
            Some(addr) => self.memory_rc.borrow_mut()[addr] = data,
            None => self.accumulator_cell.borrow_mut().m_value = data,
        };

        self.processor_status_flags.update_nz_flags(data);
        self.program_counter
            .increment(addressing_mode.parameter_bytes());
    }

    // BIT
    pub fn bit_instruction(&mut self, addressing_mode: &AddressingModes) {
        let address = self.fetch_address(addressing_mode).unwrap();
        let mem_operand = self.memory_rc.borrow()[address];
        let result = mem_operand & self.accumulator_cell.borrow().m_value;

        // Set Flags
        if result == 0 {
            self.processor_status_flags.set_flag(StatusFlags::Zero);
        }

        if (result & 1 << 7) != 0 {
            self.processor_status_flags.set_flag(StatusFlags::Negative);
        } else {
            self.processor_status_flags
                .clear_flag(StatusFlags::Negative);
        }

        if (result & 1 << 6) != 0 {
            self.processor_status_flags.set_flag(StatusFlags::Overflow);
        } else {
            self.processor_status_flags
                .clear_flag(StatusFlags::Overflow);
        }
    }

    pub fn inc_dec_memory(&mut self, addressing_mode: &AddressingModes, dec: bool) {
        let address = self.fetch_address(addressing_mode).unwrap();
        let mut memory = self.memory_rc.borrow_mut();
        let value = memory[address];
        memory[address] = if dec {
            value.wrapping_sub(1)
        } else {
            value.wrapping_add(1)
        };

        self.program_counter
            .increment(addressing_mode.parameter_bytes());
    }

    pub fn inc_dec_register(&mut self, register_cell: Rc<RefCell<DataRegister>>, dec: bool) {
        let mut register = register_cell.borrow_mut();
        let value = register.m_value;
        register.m_value = if dec {
            value.wrapping_sub(1)
        } else {
            value.wrapping_add(1)
        };

        self.program_counter.increment(0);
    }
}

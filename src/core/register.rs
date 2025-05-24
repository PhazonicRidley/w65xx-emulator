use std::{cell::RefCell, error::Error, fmt::Display, rc::Rc};

use crate::peripherals::memory::VirtualMemory;
use strum::EnumIter;

#[derive(Debug)]
pub struct DataRegister {
    name: String,
    pub value: u8,
}
impl DataRegister {
    pub fn new(p_name: impl Into<String>) -> Self {
        return DataRegister {
            name: p_name.into(),
            value: 0,
        };
    }

    pub fn get_name(&self) -> &str {
        return self.name.as_str();
    }

    pub fn reset_register(&mut self) {
        self.value = 0;
    }
}

#[derive(Debug)]
pub struct ProgramCounter {
    pub value: u16, // Storing the two 8bit buffers in a single block
    default: u16,
}
impl From<u16> for ProgramCounter {
    fn from(p_default: u16) -> Self {
        return ProgramCounter {
            value: p_default,
            default: p_default,
        };
    }
}

impl ProgramCounter {
    fn fetch_byte(&self, low_byte: bool) -> u8 {
        if low_byte {
            return (self.value & 0xFF) as u8;
        } else {
            return ((self.value & 0xFF00) >> 8) as u8;
        }
    }

    fn set_byte(&mut self, byte: u8, low_byte: bool) {
        if low_byte {
            self.value &= 0xFF << 8; // clear low byte
            self.value |= byte as u16;
        } else {
            self.value &= 0xFF; // clear higher byte
            let zero_extended_byte = byte as u16;
            self.value |= (zero_extended_byte << 8) as u16;
        }
    }

    pub fn get_pcl(&self) -> u8 {
        return self.fetch_byte(true);
    }
    pub fn get_pch(&self) -> u8 {
        return self.fetch_byte(false);
    }

    pub fn set_pcl(&mut self, data: u8) {
        self.set_byte(data, true);
    }

    pub fn set_pch(&mut self, data: u8) {
        self.set_byte(data, false);
    }

    /// Increments program counter at least by 1. Adds how many parameters were used into the sum. 1 + number of parameters used
    pub fn increment(&mut self, num_params: u16) {
        self.value += num_params + 1;
    }

    pub fn reset_register(&mut self) {
        self.value = self.default;
    }
}

#[derive(Debug)]
// Register for the pointer to manage the 256 byte stack
pub struct StackPointerRegister {
    page: u8, // the high byte of the stack pointer address range
    pointer: u8,
    memory_rc: Rc<RefCell<VirtualMemory>>,
}

impl StackPointerRegister {
    pub fn new(p_page: u8, p_start_addr: u8, p_memory_rc: Rc<RefCell<VirtualMemory>>) -> Self {
        return StackPointerRegister {
            page: p_page,
            pointer: p_start_addr,
            memory_rc: p_memory_rc,
        };
    }

    pub fn push(&mut self, p_data: u8) {
        self.memory_rc.borrow_mut()[((self.page as u16) << 8) | self.pointer as u16] = p_data;
        self.pointer = self.pointer.wrapping_sub(1); // decrement stack pointer, allows for overflows
    }
    pub fn pop(&mut self) -> u8 {
        self.pointer = self.pointer.wrapping_add(1); // increment stack pointer, allows for overflows
        let byte = self.memory_rc.borrow()[((self.page as u16) << 8) | self.pointer as u16];
        return byte;
    }

    pub fn get_pointer(&self) -> u8 {
        return self.pointer;
    }

    pub fn reset_register(&mut self) {
        self.page = 0x01;
        self.pointer = 0xFF;
    }
}

#[derive(Debug, EnumIter, Clone)]
pub enum StatusFlags {
    Carry,
    Zero,
    InterruptDisable,
    Decimal,
    BRK,
    Overflow,
    Negative,
}

impl StatusFlags {
    pub fn get_mask(&self) -> u8 {
        return match self {
            Self::Carry => 1,
            Self::Zero => 1 << 1,
            Self::InterruptDisable => 1 << 2,
            Self::Decimal => 1 << 3,
            Self::BRK => 1 << 4,
            Self::Overflow => 1 << 6,
            Self::Negative => 1 << 7,
        };
    }
}

// The flag byte of the 6502 are Negative, Overflow, (padding bit), Break mark (BRK) command, decimal mode, Interupt Request, Zero, and Carry.
// Each are to be
#[derive(Debug)]
pub struct StatusRegister {
    flags: u8,
}

impl StatusRegister {
    pub fn new() -> Self {
        return StatusRegister { flags: 0b00100000 };
    }

    fn bit_manager(&mut self, flag: StatusFlags, to_set: bool) {
        let mask = flag.get_mask();
        if to_set {
            self.flags |= mask;
        } else {
            let inverse_mask = !mask;
            self.flags &= inverse_mask;
        }
    }

    pub fn set_flag(&mut self, flag: StatusFlags) {
        return self.bit_manager(flag, true);
    }

    pub fn clear_flag(&mut self, flag: StatusFlags) {
        return self.bit_manager(flag, false);
    }

    pub fn check_flag(&self, flag: StatusFlags) -> bool {
        let mask = flag.get_mask();
        let bit_state = mask & self.flags;
        return bit_state != 0;
    }

    pub fn check_mask(&self, mask: u8) -> bool {
        return self.flags & mask == mask;
    }

    pub fn set_mask(&mut self, mask: u8) {
        self.flags &= 0;
        self.flags |= mask | 32;
    }

    pub fn clear_mask(&mut self, mask: u8) {
        self.flags &= !mask | 32;
    }

    pub fn get_flags(&self) -> u8 {
        return self.flags;
    }

    pub fn add_update_carry_flag(&mut self, first_operand: u8, second_operand: u8) {
        // Tests by summing numbers together and detecting if it overflows or underflows
        // The inverse of the carry flag is the borrow flag. ~C = B
        let a = first_operand as u16;
        let b = second_operand as u16;
        let true_sum = a + b;
        let carry_flag = 0x100 & true_sum;

        if carry_flag != 0 {
            self.set_flag(StatusFlags::Carry);
        } else {
            self.clear_flag(StatusFlags::Carry);
        }
    }

    pub fn update_overflow_flag(&mut self, m: u8, n: u8, result: u8) {
        // Verifies the sign bit of the two operands in relation to their new resulting value. If the sign has flipped, the overflow flag is set.
        let first_test = (m ^ result) & (n ^ result) & 0x80;
        // let second_test = &0x80;

        if first_test != 0 {
            self.set_flag(StatusFlags::Overflow)
        } else {
            self.clear_flag(StatusFlags::Overflow)
        }
    }

    pub fn update_nz_flags(&mut self, data: u8) {
        let signed_data = data as i8;
        if signed_data < 0 {
            self.set_flag(StatusFlags::Negative);
            self.clear_flag(StatusFlags::Zero);
        } else if signed_data == 0 {
            self.clear_flag(StatusFlags::Negative);
            self.set_flag(StatusFlags::Zero);
        } else {
            self.clear_flag(StatusFlags::Negative);
            self.clear_flag(StatusFlags::Zero);
        }
    }
}

#[derive(Debug)]
pub struct RegisterError {
    err_msg: String,
}
impl From<&str> for RegisterError {
    fn from(value: &str) -> Self {
        return RegisterError {
            err_msg: value.to_owned(),
        };
    }
}
impl Display for RegisterError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.err_msg)
    }
}
impl Error for RegisterError {}

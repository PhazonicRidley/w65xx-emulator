use std::{
    error::Error,
    fmt::Display,
    sync::{Arc, Mutex},
};

use crate::peripherals::memory::VirtualMemory;
use num_traits::Unsigned;
use strum::EnumIter;

pub trait Register<T: Unsigned> {
    fn load_data(&mut self, data: T);
    fn get_data(&self) -> T;
    fn reset_register(&mut self);
}

#[derive(Debug)]
pub struct Accumulator {
    data: u8,
}

impl Accumulator {
    pub fn new() -> Self {
        return Accumulator { data: 0 };
    }
}

impl Register<u8> for Accumulator {
    fn load_data(&mut self, data: u8) {
        self.data = data
    }

    fn get_data(&self) -> u8 {
        return self.data;
    }

    fn reset_register(&mut self) {
        self.data = 0;
    }
}
#[derive(Debug)]
pub struct IndexRegister {
    name: char,
    data: u8,
}
impl IndexRegister {
    pub fn new(name: char) -> Self {
        return IndexRegister {
            name: name,
            data: 0,
        };
    }

    pub fn get_name(&self) -> char {
        return self.name;
    }
}

impl Register<u8> for IndexRegister {
    fn load_data(&mut self, data: u8) {
        self.data = data
    }

    fn get_data(&self) -> u8 {
        return self.data;
    }

    fn reset_register(&mut self) {
        self.data = 0;
    }
}

#[derive(Debug)]
pub struct ProgramCounter {
    data: u16, // Storing the two 8bit buffers in a single block
}
impl From<u16> for ProgramCounter {
    fn from(value: u16) -> Self {
        return ProgramCounter { data: value };
    }
}

impl ProgramCounter {
    fn fetch_byte(&self, low_byte: bool) -> u8 {
        if low_byte {
            return (self.data & 0xFF) as u8;
        } else {
            return ((self.data & 0xFF00) >> 8) as u8;
        }
    }

    fn set_byte(&mut self, byte: u8, low_byte: bool) {
        if low_byte {
            self.data &= 0xFF << 8; // clear low byte
            self.data |= byte as u16;
        } else {
            self.data &= 0xFF; // clear higher byte
            let zero_extended_byte = byte as u16;
            self.data |= (zero_extended_byte << 8) as u16;
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
        self.data += num_params + 1;
    }
}

impl Register<u16> for ProgramCounter {
    fn load_data(&mut self, data: u16) {
        self.data = data;
    }

    fn get_data(&self) -> u16 {
        return self.data;
    }

    fn reset_register(&mut self) {
        self.data = 0;
    }
}

#[derive(Debug)]
// Register for the pointer to manage the 256 byte stack
pub struct StackPointerRegister {
    address_range: u8, // the high byte of the stack pointer address range
    pointer: u8,
    memory_arc: Arc<Mutex<VirtualMemory>>,
}

impl StackPointerRegister {
    pub fn new(addr_range: u8, start_location: u8, memory: Arc<Mutex<VirtualMemory>>) -> Self {
        return StackPointerRegister {
            address_range: addr_range,
            pointer: start_location,
            memory_arc: memory,
        };
    }

    pub fn push(&mut self, data: u8) {
        self.memory_arc.lock().unwrap()[self.pointer as u16] = data;
        self.pointer = self.pointer.wrapping_sub(1); // decrement stack pointer, allows for overflows
    }
    pub fn pop(&mut self) -> u8 {
        self.pointer = self.pointer.wrapping_add(1); // increment stack pointer, allows for overflows
        let byte = self.memory_arc.lock().unwrap()[self.pointer as u16];
        return byte;
    }
}

impl Register<u8> for StackPointerRegister {
    fn load_data(&mut self, data: u8) {
        self.pointer = data
    }

    fn get_data(&self) -> u8 {
        return self.pointer;
    }

    fn reset_register(&mut self) {
        self.address_range = 0x01;
        self.pointer = 0xFF;
    }
}

// The flag byte of the 6502 are Negative, Overflow, (padding bit), Break mark (BRK) command, decimal mode, Interupt Request, Zero, and Carry.
// Each are to be
#[derive(Debug)]
pub struct ProcessorStatusRegister {
    flags: u8,
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
    fn get_mask(&self) -> u8 {
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

impl ProcessorStatusRegister {
    pub fn new() -> Self {
        return ProcessorStatusRegister { flags: 0b00100000 };
    }

    fn bit_manager(&mut self, flag: StatusFlags, to_set: bool) {
        let mask: u8 = flag.get_mask();
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
        let mask: u8 = flag.get_mask();
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
        println!("True sum: {}", true_sum);
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
            self.set_flag(StatusFlags::Zero);
            self.clear_flag(StatusFlags::Negative);
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

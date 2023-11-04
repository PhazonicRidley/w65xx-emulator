use std::{
    error::Error,
    fmt::Display,
    sync::{Arc, Mutex},
};

use crate::peripherals::memory::VirtualMemory;
use num_traits::Unsigned;
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

    pub fn increment(&mut self, num_params: u16) {
        self.data += num_params;
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
        self.memory_arc.try_lock().unwrap()[self.pointer as u16] = data;
        self.pointer = self.pointer.wrapping_sub(1); // decrement stack pointer, allows for overflows
    }
    pub fn pop(&mut self) -> u8 {
        self.pointer = self.pointer.wrapping_add(1); // increment stack pointer, allows for overflows
        let byte = self.memory_arc.try_lock().unwrap()[self.pointer as u16];
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
    labels: [char; 8],
}

impl ProcessorStatusRegister {
    pub fn new() -> Self {
        return ProcessorStatusRegister {
            flags: 0b00100000,
            labels: ['c', 'z', 'i', 'd', 'b', '1', 'v', 'n'],
        };
    }

    // Manages bits to set and clear them, called from wrapper functions
    fn verify_flag(&self, flag: char) -> Result<usize, RegisterError> {
        return match self.labels.iter().position(|&f| f == flag) {
            Some(bit) => Ok(bit),
            None => Err(RegisterError {
                err_msg: format!("Invalid flag: {} given", flag),
            }),
        };
    }
    fn bit_manager(&mut self, flag: char, to_set: bool) -> Result<(), RegisterError> {
        let bit_position = self.verify_flag(flag)?;
        let mask: u8 = 1 << bit_position;
        if to_set {
            self.flags |= mask;
        } else {
            let inverse_mask = !mask;
            self.flags &= inverse_mask;
        }

        return Ok(());
    }

    pub fn set_flag(&mut self, flag: char) -> Result<(), RegisterError> {
        return self.bit_manager(flag, true);
    }

    pub fn clear_flag(&mut self, flag: char) -> Result<(), RegisterError> {
        return self.bit_manager(flag, false);
    }

    pub fn check_flag(&self, flag: char) -> Result<bool, RegisterError> {
        let bit_position = self.verify_flag(flag)?;
        let mask: u8 = 1 << bit_position;
        let bit_state = mask & self.flags;
        return Ok(bit_state != 0);
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

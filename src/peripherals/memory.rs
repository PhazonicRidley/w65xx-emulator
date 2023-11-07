use std::{
    error::Error,
    fmt::Display,
    ops::{Index, IndexMut},
    vec::Vec,
};

pub fn test() {
    println!("Hello from memory");
}

// TODO: Maybe a safe mode that tracks what addresses are using what. (ROM, RAM, IO, Stack)
#[derive(Debug)]
pub struct VirtualMemory {
    buffer: [u8; 0xFFFF],
}

impl VirtualMemory {
    pub fn new() -> Self {
        let arr: [u8; 0xFFFF] = [0; 0xFFFF];
        return VirtualMemory { buffer: arr };
    }

    pub fn load_rom(
        &mut self,
        rom_data: Vec<u8>,
        starting_address: u16,
    ) -> Result<(), MemoryError> {
        if 0xFFFF - starting_address < rom_data.len() as u16 {
            return Err(MemoryError::new(
                "Not enough space to fit ROM at this memory address",
            ));
        }
        let mut rom_idx = 0;
        for i in starting_address..0xFFFF {
            if rom_idx >= rom_data.len() {
                break;
            }
            self.buffer[i as usize] = rom_data[(i - starting_address) as usize];
            rom_idx += 1;
        }
        return Ok(());
    }

    pub fn reinitialize(&mut self) {
        self.buffer = [0; 0xFFFF];
    }

    /// Reads a word in little endian order returns 0xHHLL, where 0xLL is the low byte and 0xHH is the highbyte
    pub fn read_word(&self, low_byte_addr: u16) -> u16 {
        let res =
            ((self[low_byte_addr.wrapping_add(1)] as u16) << 8) | (self[low_byte_addr] as u16);
        return res;
    }
}

impl Index<u16> for VirtualMemory {
    type Output = u8;
    fn index(&self, index: u16) -> &Self::Output {
        return &self.buffer[index as usize];
    }
}

impl IndexMut<u16> for VirtualMemory {
    fn index_mut(&mut self, index: u16) -> &mut Self::Output {
        return &mut self.buffer[index as usize];
    }
}

#[derive(Debug)]
pub struct MemoryError {
    error_msg: String, // TODO: change to an error code enum once standardized
}

impl MemoryError {
    pub fn new(err_str: &str) -> Self {
        return MemoryError {
            error_msg: String::from(err_str),
        };
    }

    pub fn get_message(&self) -> &String {
        return &self.error_msg;
    }
}
impl Display for MemoryError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "An error has occured, error: {}", self.error_msg)
    }
}
impl Error for MemoryError {}

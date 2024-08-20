// Handles buffers for 6502
// TODO: Figure out a use for
#[derive(Debug)]
pub struct PinIO {
    pub data_buffer: u8, // Represents D0-D7
    pub address_buffer: u16, // Represents A0-A15
                         // TODO: add more pins
}

impl PinIO {
    pub fn new() -> Self {
        return PinIO {
            data_buffer: 0,
            address_buffer: 0,
        };
    }
}

impl Default for PinIO {
    fn default() -> Self {
        return PinIO {
            data_buffer: 0,
            address_buffer: 0,
        };
    }
}

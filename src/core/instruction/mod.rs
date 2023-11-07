use std::fmt::Display;

use super::{
    cpu::CPU,
    register::{ProcessorStatusRegister, Register},
};

pub enum AddressingModes {
    // Param == Operand, considering all words with little endian as thats how they will appear in machine code.
    Accumulator,       // OPC A
    Absolute,          // OPC $LLHH where $LLHH is an address (2 params)
    AbsoluteXIndex, // OPC $LLHH, X, where X is the value in the X register and the indexed address is $LLHH + X with Carry
    AbsoluteYIndex, // OPC $LLHH, X, where X is the value in the Y register and the indexed address is $LLHH + Y with Carry
    Immediate,      // OPC #$BB where the parameter is just the value $BB
    Implied,        // OPC
    Indirect,       // OPC $LLHH, params are the bytes of the 16 bit address
    PreIndexIndirect, // OPC ($LL, X), adds X to the byte $LL to get effective address to work with. (Addition with no carry)
    PostIndexIndirect, // OPC ($LL, ) Y. Adds Y to looked up address given by the low byte $LL to get effective address. (Addition with no carry)
    Relative,          // OPC $BB. used for branching only, target location is PC + SIGNED $BB.
    ZeroPage,          // OPC $LL. Param is zeropage address, high byte is $00, low byte is $LL.
    ZeroPageXIndex, // OPC $LL, X. Param is a zeropage address offset by the number in the X register so $LL + X (addition without carry)
    ZeroPageYIndex, // OPC $LL, Y. Param is a zeropage address offset by the number in the Y register so $LL + X (addition without carry)
}

impl Display for AddressingModes {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let string = match self {
            Self::Accumulator => "Accumulator",
            Self::Absolute => "Absolute",
            Self::AbsoluteXIndex => "AbsoluteXIndex",
            Self::AbsoluteYIndex => "AbsoluteYIndex",
            Self::Immediate => "Immediate",
            Self::Implied => "Implied",
            Self::Indirect => "Indirect",
            Self::PreIndexIndirect => "PreIndexIndirect",
            Self::PostIndexIndirect => "PostIndexIndirect",
            Self::Relative => "Relative",
            Self::ZeroPage => "ZeroPage",
            Self::ZeroPageXIndex => "ZeroPageXIndex",
            Self::ZeroPageYIndex => "ZeroPageYIndex",
        };
        return write!(f, "{}", string);
    }
}

// https://www.masswerk.at/6502/6502_instruction_set.html
impl AddressingModes {
    pub fn parameter_bytes(&self) -> u16 {
        match self {
            Self::Accumulator => 0,
            Self::Absolute => 2,
            Self::AbsoluteXIndex => 2,
            Self::AbsoluteYIndex => 2,
            Self::Immediate => 1,
            Self::Implied => 0,
            Self::Indirect => 2,
            Self::PreIndexIndirect => 1,
            Self::PostIndexIndirect => 1,
            Self::Relative => 1,
            Self::ZeroPage => 1,
            Self::ZeroPageXIndex => 1,
            Self::ZeroPageYIndex => 1,
        }
    }
}

fn update_nz_flags(processor_flags: &mut ProcessorStatusRegister, register: &impl Register<u8>) {
    let signed_data = register.get_data() as i8;
    if signed_data < 0 {
        processor_flags.set_flag('n').unwrap();
        processor_flags.clear_flag('z').unwrap();
    } else if signed_data == 0 {
        processor_flags.set_flag('z').unwrap();
        processor_flags.clear_flag('n').unwrap();
    } else {
        processor_flags.clear_flag('n').unwrap();
        processor_flags.clear_flag('z').unwrap();
    }
}

// LDA, LDX, LDY
pub fn load_instruction(
    addressing_mode: AddressingModes,
    register: &mut impl Register<u8>,
    cpu: &mut CPU,
) {
    let address: u16 = cpu.fetch_address(&addressing_mode);
    let pc = &mut cpu.program_counter;
    let data;
    {
        let memory = cpu.memory_arc.try_lock().unwrap();
        data = memory[address];
    }
    register.load_data(data);
    update_nz_flags(&mut cpu.processor_status_flags, register);
    pc.increment(addressing_mode.parameter_bytes());
}

// STA, STX, STY
pub fn store_instruction(
    addressing_mode: AddressingModes,
    register: &impl Register<u8>,
    cpu: &mut CPU,
) {
    let address = cpu.fetch_address(&addressing_mode);
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
    update_nz_flags(&mut cpu.processor_status_flags, destination_register);
    cpu.program_counter.increment(0); // Only possible addressing mode is implied which has no parameters.
}

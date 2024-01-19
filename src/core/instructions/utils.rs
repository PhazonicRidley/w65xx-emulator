use std::fmt::Display;

use crate::core::register::{ProcessorStatusRegister, StatusFlags};

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

pub enum BranchMode {
    BMI,
    BNE,
    BPL,
    BVC,
    BVS,
    BCS,
    BCC,
    BEQ,
}

impl BranchMode {
    pub fn verify(&self, flag_reg: &ProcessorStatusRegister) -> bool {
        match self {
            Self::BEQ => flag_reg.check_flag(StatusFlags::Zero), // Z = 1
            Self::BCC => !flag_reg.check_flag(StatusFlags::Carry), // C = 0
            Self::BCS => flag_reg.check_flag(StatusFlags::Carry), // C = 1
            Self::BVC => !flag_reg.check_flag(StatusFlags::Overflow), // V = 0
            Self::BVS => flag_reg.check_flag(StatusFlags::Overflow), // V = 1
            Self::BPL => !flag_reg.check_flag(StatusFlags::Negative), // N = 0
            Self::BNE => !flag_reg.check_flag(StatusFlags::Zero), // Z = 0
            Self::BMI => flag_reg.check_flag(StatusFlags::Negative), // N = 1
        }
    }
}

// TODO: Move to register.rs
#[cfg(test)]
mod tests {
    use std::vec;

    use crate::core::register::{ProcessorStatusRegister, StatusFlags};

    #[test]
    fn overflow_test_addition() {
        // Setup
        let first_operands: Vec<u8> = vec![0x50, 0x50, 0x50, 0x50, 0xD0, 0xD0, 0xD0, 0xD0];
        let second_operands: Vec<u8> = vec![0x10, 0x50, 0x90, 0xD0, 0x10, 0x50, 0x90, 0xD0];
        let mut results: Vec<bool> = vec![];
        let expected_results = vec![false, true, false, false, false, false, true, false];
        let mut status_flags = ProcessorStatusRegister::new();
        assert_eq!(first_operands.len(), second_operands.len());
        // Execute
        for i in 0..first_operands.len() {
            let sum: u8 = first_operands[i].wrapping_add(second_operands[i]);
            status_flags.update_overflow_flag(first_operands[i], second_operands[i], sum);
            results.push(status_flags.check_flag(StatusFlags::Overflow));
        }

        // Verify
        for i in 0..expected_results.len() {
            assert_eq!(results[i], expected_results[i]);
        }
    }

    #[test]
    fn overflow_test_subtraction() {
        // Setup
        let first_operands: Vec<u8> = vec![0x50, 0x50, 0x50, 0x50, 0xD0, 0xD0, 0xD0, 0xD0];
        let second_operands: Vec<u8> = vec![!0xF0, !0xB0, !0x70, !0x30, !0xF0, !0xB0, !0x70, !0x30];
        let mut results: Vec<bool> = vec![];
        let expected_results = vec![false, true, false, false, false, false, true, false];
        let mut status_flags = ProcessorStatusRegister::new();
        assert_eq!(first_operands.len(), second_operands.len());
        // Execute
        for i in 0..first_operands.len() {
            let sum = first_operands[i].wrapping_add(second_operands[i]);
            status_flags.update_overflow_flag(first_operands[i], second_operands[i], sum);
            results.push(status_flags.check_flag(StatusFlags::Overflow));
        }
        // return;

        // Verify
        for i in 0..expected_results.len() {
            assert_eq!(results[i], expected_results[i]);
        }
    }

    #[test]
    fn carry_test() {
        let mut first: u8 = 0xff;
        let mut second: u8 = 0x2;
        let mut status_flags = ProcessorStatusRegister::new();

        status_flags.add_update_carry_flag(first, second);

        assert_eq!(status_flags.check_flag(StatusFlags::Carry), true);

        first = 0x80;
        second = 0x5;
        status_flags.add_update_carry_flag(first, second);
        assert_eq!(status_flags.check_flag(StatusFlags::Carry), false);
    }
}

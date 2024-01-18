use crate::core::{cpu::CPU, register::StatusFlags};

// Clear flags

// CLC
pub fn clear_carry_flag(cpu: &mut CPU) {
    cpu.processor_status_flags.clear_flag(StatusFlags::Carry);
}

// CLD
pub fn clear_decimal_flag(cpu: &mut CPU) {
    cpu.processor_status_flags.clear_flag(StatusFlags::Decimal);
}

// CLI
pub fn clear_interrupt_disable_flag(cpu: &mut CPU) {
    cpu.processor_status_flags
        .clear_flag(StatusFlags::InterruptDisable);
}

// CLV
pub fn clear_overflow_flag(cpu: &mut CPU) {
    cpu.processor_status_flags.clear_flag(StatusFlags::Overflow);
}

// Set flags

// SEC
pub fn set_carry_flag(cpu: &mut CPU) {
    cpu.processor_status_flags.set_flag(StatusFlags::Carry);
}

// SED
pub fn set_decimal_flag(cpu: &mut CPU) {
    cpu.processor_status_flags.set_flag(StatusFlags::Decimal);
}

// SEI
pub fn set_interrupt_disable_flag(cpu: &mut CPU) {
    cpu.processor_status_flags
        .set_flag(StatusFlags::InterruptDisable);
}

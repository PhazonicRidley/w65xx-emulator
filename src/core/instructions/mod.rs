use std::{thread, time};

pub mod alu;
pub mod control_flow;
pub mod memory_register;
pub mod stack;
pub mod status_flags;
pub mod utils;

pub fn no_operation() {
    thread::sleep(time::Duration::from_millis(1));
}

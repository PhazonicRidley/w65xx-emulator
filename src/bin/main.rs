#![allow(dead_code)]
#![allow(unused)]
#![deny(clippy::implicit_return)]

use std::{
    cell::RefCell,
    sync::{Arc, Mutex},
    thread,
}; // TODO: Use with stack pointer.

use w65xx_emulator::*;

fn main() {
    let a = 5;
    println!("a is {a}");
    println!("Hello, world!");
    lib_function();
}

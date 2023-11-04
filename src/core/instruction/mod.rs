use super::{cpu::CPU, register::Register};

// Test method for lda, to be refactored
pub fn load_accumulator(op_code: u8, cpu: &mut CPU) {
    let pc = &mut cpu.program_counter;
    let accumulator = &mut cpu.accumulator;
    let memory = cpu.memory_arc.try_lock().unwrap();
    let num_params: u16;
    match op_code {
        0xA9 => {
            accumulator.load_data(memory[pc.get_data() + 1]);
            num_params = 1;
        } // immediate
        0xA5 => {
            accumulator.load_data(memory[pc.get_data() + 1]);
            num_params = 1
        } // zp addressing
        0xB5 => {
            let zp_addr = (pc.get_data() + 1) + (cpu.x_register.get_data() as u16);
            accumulator.load_data(memory[zp_addr]);
            num_params = 1
        } // zp addressing
        0xAD => {
            let address = memory.read_word(pc.get_data() + 1);
            accumulator.load_data(memory[address]);
            num_params = 2;
        } // absolute addressing

        0xBD => {
            let address = memory.read_word(pc.get_data() + 1) + cpu.x_register.get_data() as u16;
            accumulator.load_data(memory[address]);
            num_params = 2;
        } // absolute x indexing
        0xB9 => {
            let address = memory.read_word(pc.get_data() + 1) + cpu.y_register.get_data() as u16;
            accumulator.load_data(memory[address]);
            num_params = 2;
        } // absolute y indexing
        0xA1 => {
            let lookup_addr = memory[pc.get_data() + 1] + cpu.x_register.get_data();
            let data = memory[lookup_addr as u16];
            accumulator.load_data(data);
            num_params = 1;
        } // indirect addressing with x (pre-indexed addressing)

        0xB1 => {
            let lookup_addr = memory[pc.get_data() + 1] as u16;
            let data = memory[lookup_addr + cpu.y_register.get_data() as u16];
            accumulator.load_data(data);
            num_params = 1;
        }

        _ => panic!("Invalid opcode"),
    }

    let signed_data = accumulator.get_data() as i8;
    let processor_status = &mut cpu.processor_status_flags;
    if signed_data < 0 {
        processor_status.set_flag('n').unwrap();
        processor_status.clear_flag('z').unwrap();
    } else if signed_data == 0 {
        processor_status.set_flag('z').unwrap();
        processor_status.clear_flag('n').unwrap();
    } else {
        processor_status.clear_flag('n').unwrap();
        processor_status.clear_flag('z').unwrap();
    }
    pc.increment(num_params);
}

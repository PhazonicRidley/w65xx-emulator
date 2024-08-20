use crate::core::cpu::CPU;

impl CPU {
    // PHA
    pub fn push_accumulator(&mut self) {
        let acc_value = self.accumulator_cell.borrow().value;
        self.stack_pointer.push(acc_value);
        self.program_counter.increment(0);
    }

    // PHP
    pub fn push_status(&mut self) {
        let flags = self.processor_status_flags.get_flags();
        self.stack_pointer.push(flags);
        self.program_counter.increment(0);
    }

    // PLA
    pub fn pop_accumulator(&mut self) {
        let acc_value = self.stack_pointer.pop();
        self.accumulator_cell.borrow_mut().value = acc_value;
        self.program_counter.increment(0);
    }

    // PLP
    pub fn pop_status(&mut self) {
        let flags = self.stack_pointer.pop();
        self.processor_status_flags.set_mask(flags);
        self.program_counter.increment(0);
    }
}

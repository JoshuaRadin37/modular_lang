use crate::instruction_set::Immediate;

pub const REGISTER_COUNT: usize = 8;

pub struct Registers {
    pub caller: [Option<Immediate>; REGISTER_COUNT],
    pub callee: [Option<Immediate>; REGISTER_COUNT],
    pub start_stack_pointer: usize,
    pub stack_pointer: usize,
    pub frame_pointer: usize,
}

impl Registers {
    pub fn new() -> Self {
        Registers {
            caller: [None; 8],
            callee: [None; 8],
            start_stack_pointer: 0,
            stack_pointer: 0,
            frame_pointer: 0,
        }
    }
}

use crate::instruction_set::Immediate;

pub const REGISTER_COUNT: usize = 8;

pub struct Registers {
    pub caller: [Option<Immediate>; REGISTER_COUNT],
    pub callee: [Option<Immediate>; REGISTER_COUNT],
}

impl Registers {
    pub fn new() -> Self {
        Registers {
            caller: [None, None, None, None, None, None, None, None],
            callee: [None, None, None, None, None, None, None, None],
        }
    }
}

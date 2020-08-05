use crate::instruction_set::Immediate;

pub const REGISTER_COUNT: usize = 8;

pub struct Registers {
    pub caller: [Immediate; REGISTER_COUNT],
    pub callee: [Immediate; REGISTER_COUNT],
}

impl Registers {
    pub fn new() -> Self {
        Registers {
            caller:
            [Immediate::USize(0), Immediate::USize(0),
                Immediate::USize(0), Immediate::USize(0),
                Immediate::USize(0), Immediate::USize(0),
                Immediate::USize(0), Immediate::USize(0)],
            callee: [Immediate::USize(0), Immediate::USize(0),
                Immediate::USize(0), Immediate::USize(0),
                Immediate::USize(0), Immediate::USize(0),
                Immediate::USize(0), Immediate::USize(0)],
        }
    }
}

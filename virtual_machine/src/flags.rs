pub struct Flags {
    pub carry: bool,
    pub parity: bool,
    pub zero: bool,
    pub sign: bool,
    pub trap: bool,
    pub interrupt_enable: bool,
    pub overflow: bool,
}

impl Flags {
    pub fn new() -> Self {
        Flags {
            carry: false,
            parity: false,
            zero: false,
            sign: false,
            trap: false,
            interrupt_enable: false,
            overflow: false,
        }
    }

    pub fn reset(&mut self) {
        *self = Flags::new();
    }
}

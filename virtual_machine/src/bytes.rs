use crate::instruction_set::{Immediate, Instruction};
use crate::bytes::machine_code_reader::InvalidInstructionError;
use crate::Family::{Second, Third, Fourth};

pub mod machine_code_reader;
pub mod machine_code_writer;


pub struct InstructionBytes<'a> {
    bytes: &'a [u8]
}

#[derive(Copy, Clone)]
pub enum Family {
    First = 0,
    Second = 1,
    Third = 2,
    Fourth = 3
}

impl Family {

    pub fn next(self) -> Result<Self, InvalidInstructionError> {
        match self {
            Family::First => {
                Ok(Second)
            },
            Family::Second => {
                Ok(Third)
            },
            Family::Third => {
                Ok(Fourth)
            },
            Family::Fourth => {
                Err(InvalidInstructionError)
            },
        }
    }
}

#[repr(u8)]
#[derive(Clone, Copy)]
pub enum RegisterDirection {
    Direct = 0,
    Indirect = 1
}

pub struct RegisterUsage {
    first: RegisterDirection,
    second: RegisterDirection
}

impl Into<u8> for &RegisterUsage {
    fn into(self) -> u8 {
        let first = self.first as u8;
        let second = self.second as u8;
        (first << 1) | second
    }
}

pub enum IndirectRegister {
    First,
    Second
}

impl RegisterUsage {
    pub fn both_direct() -> Self {
        RegisterUsage {
            first: RegisterDirection::Direct,
            second: RegisterDirection::Direct
        }
    }

    pub fn one_indirect(indirect_reg: IndirectRegister) -> Self {
        match indirect_reg {
            IndirectRegister::First => {
                RegisterUsage {
                    first: RegisterDirection::Indirect,
                    second: RegisterDirection::Direct
                }
            },
            IndirectRegister::Second => {
                RegisterUsage {
                    first: RegisterDirection::Direct,
                    second: RegisterDirection::Indirect
                }
            },
        }
    }
}

pub struct InstructionFields {
    family: Family,
    opcode: u8,
    opcode_modifiers: Option<u8>,
    register_usage: Option<RegisterUsage>,
    register1: Option<u8>,
    register2: Option<u8>,
    immediate: Option<u64>
}

impl InstructionFields {

    pub fn into_instruction(self) -> Result<Instruction, InvalidInstructionError> {
        unimplemented!()
    }
}

pub struct InstructionBytesBuilder {
    future_array: Vec<u8>,
    fields: InstructionFields
}

impl InstructionBytesBuilder {


    pub fn new() -> Self {
        Self {
            future_array: vec![],
            fields: InstructionFields {
                family: Family::First,
                opcode: 0,
                opcode_modifiers: Option::None,
                register_usage: Option::None,
                register1: Option::None,
                register2: Option::None,
                immediate: Option::None
            }
        }
    }
    pub fn build(&mut self) -> InstructionBytes {
        for _ in 0..(self.fields.family as u8) {
            self.future_array.push(0xFF)
        }
        self.future_array.push(self.fields.opcode);
        match self.fields.opcode_modifiers {
            None => {},
            Some(modifiers) => {
                self.future_array.push(modifiers)
            },
        }
        match &self.fields.register_usage {
            None => {},
            Some(modifiers) => {
                self.future_array.push(modifiers.into())
            },
        }
        InstructionBytes {
            bytes: & *self.future_array
        }
    }

    pub fn opcode(&mut self, opcode: u8) -> &mut Self {
        self.fields.opcode = opcode;
        self
    }
}




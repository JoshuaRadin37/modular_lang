use std::path::{Path};
use std::fs::File;
use byteorder::{ReadBytesExt, BigEndian};
use crate::instruction_set::{Instruction, Immediate, DataLocation, RegisterType, Size};
use crate::vm::POINTER_SIZE;
use crate::instruction_set::Instruction::{Push, PushVal};
use crate::vm::Fault::RegisterNotSet;
use std::error::Error;
use bitfield::fmt::Formatter;
use crate::{InstructionFields, Family};

pub struct Reader(Box<dyn std::io::Read>);
#[derive(Debug)]
pub struct InvalidInstructionError;

impl std::fmt::Display for InvalidInstructionError {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:?}", self)
    }
}

impl Error for InvalidInstructionError {

}



impl Reader {

    pub fn new(file: &Path) -> Self {
        let file = File::open(file);
        if file.is_err() {
            panic!("{:?}", file.err().unwrap());
        }
        let file = file.unwrap();
        Reader(Box::new(file))
    }

    fn get_next_byte(&mut self) -> Result<u8, std::io::Error> {
        self.0.read_u8()
    }

    fn get_next_immediate(&mut self) -> Result<Immediate, std::io::Error> {
        let mut buffer = [0u8; 8];
        self.0.read_exact(&mut buffer)?;
        Ok(Immediate::from(buffer))
    }

    fn get_next_usize(&mut self) -> Result<usize, std::io::Error> {
        let ret = if POINTER_SIZE == 8 {
            self.0.read_u64::<BigEndian>()? as usize
        } else {
            self.0.read_u32::<BigEndian>()? as usize
        };
        Ok(ret)
    }

    fn get_next_register(&mut self) -> Result<(RegisterType, usize), std::io::Error> {
        let byte = self.get_next_byte()?;
        let front = (byte >> 8) & 0xF;
        let back = byte & 0xF;
        let reg_type = match front {
            0b01 => RegisterType::Caller,
            0b10 => RegisterType::Callee,
            0b11 => RegisterType::Special,
            _ => {
                panic!("Invalid Opcode")
            }
        };
        Ok((reg_type, back as usize))
    }

    pub fn get_next_instruction(&mut self) -> Result<Result<Instruction, InvalidInstructionError>, std::io::Error> {
        let fields: InstructionFields = match self.get_instruction_fields()? {
            Ok(fields) => fields,
            Err(e) => {return Ok(Err(e)); }
        };
        Ok(fields.into_instruction())
    }

    fn get_instruction_fields(&mut self) -> Result<Result<InstructionFields, InvalidInstructionError>, std::io::Error> {
        let mut family = Family::First;
        let mut opcode: u8 = 0;
        loop {
            let byte = self.get_next_byte()?;
            if byte == 0xFF {
                family = match family.next() {
                    Ok(fam) => { fam },
                    Err(e) => {return Ok(Err(e))},
                }
            } else {
                opcode = byte;
            }
        }

        InstructionFields {
            family,
            opcode,
            opcode_modifiers: Option::None,
            register_usage: Option::None,
            register1: Option::None,
            register2: Option::None,
            immediate: Option::None
        }
    }
}
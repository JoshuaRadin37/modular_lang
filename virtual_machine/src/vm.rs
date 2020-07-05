use crate::flags::Flags;
use crate::instruction_set::Immediate::{Double, Float, U16, U32, U64, U8};
use crate::instruction_set::{Immediate, Instruction, RegisterType};
use crate::memory::Memory;
use crate::registers::{Registers, REGISTER_COUNT};
use crate::vm::Fault::InvalidReturn;
use byteorder::{BigEndian, ByteOrder};
use std::cmp::Ordering;

pub struct VirtualMachine {
    instructions: Vec<Instruction>,
    program_counter: usize,
    heaps: Vec<Memory>,
    stack: Memory,
    registers: Registers,
    flags: Flags,
}

pub static POINTER_SIZE: usize = std::mem::size_of::<usize>();

#[derive(Debug)]
pub enum Fault {
    InvalidReturn,
    PrimitiveTypeMismatch,
    SegmentationFault,
}

impl VirtualMachine {
    pub fn new() -> Self {
        Self {
            instructions: vec![],
            program_counter: !0,
            heaps: vec![],
            stack: Memory::new(),
            registers: Registers::new(),
            flags: Flags::new(),
        }
    }

    fn push(&mut self, val: Immediate) {
        let as_bytes: [u8; 8] = val.into();
        let mem = self.stack.get_at_mut(self.registers.stack_pointer);
        for i in 0..8 {
            mem[i] = as_bytes[i];
        }
        self.registers.stack_pointer += 8;
    }

    fn pop(&mut self, size: usize, is_float: bool) -> Result<Immediate, Fault> {
        if size <= self.registers.stack_pointer {
            return Err(Fault::SegmentationFault);
        }

        let buff = self.stack.get_at(self.registers.stack_pointer);

        let ret = match (size, is_float) {
            (1, false) => {
                let ret = U8(buff[0]);
                Ok(ret)
            }
            (2, false) => {
                let internal = BigEndian::read_u16(buff);
                Ok(U16(internal))
            }
            (4, false) => {
                let internal = BigEndian::read_u32(buff);
                Ok(U32(internal))
            }
            (8, false) => {
                let internal = BigEndian::read_u64(buff);
                Ok(U64(internal))
            }
            (4, true) => {
                let internal = BigEndian::read_f32(buff);
                Ok(Float(internal))
            }
            (8, true) => {
                let internal = BigEndian::read_f64(buff);
                Ok(Double(internal))
            }
            _ => return Err(Fault::PrimitiveTypeMismatch),
        };

        self.registers.stack_pointer -= size;

        ret
    }

    fn get_register_mut(&mut self, reg_type: RegisterType, reg: usize) -> &mut Option<Immediate> {
        if reg > REGISTER_COUNT {
            panic!("Illegal Register {:?} {}", reg_type, reg);
        }
        match reg_type {
            RegisterType::Caller => &mut self.registers.caller[reg],
            RegisterType::Callee => &mut self.registers.callee[reg],
        }
    }

    fn get_register(&self, reg_type: RegisterType, reg: usize) -> Option<Immediate> {
        if reg > REGISTER_COUNT {
            panic!("Illegal Register {:?} {}", reg_type, reg);
        }
        match reg_type {
            RegisterType::Caller => self.registers.caller[reg],
            RegisterType::Callee => self.registers.callee[reg],
        }
    }

    fn run_instruction(&mut self, instruction: &Instruction) -> Result<(), Fault> {
        let mut next_program_counter = self.program_counter + 1;
        match instruction {
            Instruction::PushVal(immediate) => self.push(*immediate),
            Instruction::Push(size) => {
                self.registers.stack_pointer += *size;
            }
            Instruction::Pop(size) => {
                self.registers.stack_pointer -= *size;
            }
            Instruction::Ret(option) => {
                let ret_location: Immediate = self.pop(POINTER_SIZE, false)?;
            }
            Instruction::Jump(counter) => {
                next_program_counter = *counter;
            }
            Instruction::Compare(comparison, size, is_float) => {}
            Instruction::PerformOperation(operation, size, is_float) => {}
            Instruction::AddressOf(offset) => {}
            Instruction::Dereference => {}
            Instruction::Alloc => {}
            Instruction::Call(_) => {}
            Instruction::Throw => {}
            Instruction::Catch => {}
        }

        Ok(())
    }
}

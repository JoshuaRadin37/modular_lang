use crate::flags::Flags;
use crate::instruction_set::Immediate::{Double, Float, U16, U32, U64, U8};
use crate::instruction_set::{Immediate, Instruction, JumpType, Literal, RegisterType};
use crate::memory::Memory;
use crate::registers::{Registers, REGISTER_COUNT};
use crate::vm::Fault::InvalidReturn;
use byteorder::{BigEndian, ByteOrder};
use std::cmp::Ordering;
use std::convert::TryInto;

pub struct VirtualMachine {
    instructions: Vec<Instruction>,
    program_counter: usize,
    pub(super) memory: Memory,
    pub(super) registers: Registers,
    flags: Flags,
    cont: bool,
}

pub static POINTER_SIZE: usize = std::mem::size_of::<usize>();

#[derive(Debug)]
pub enum Fault {
    InvalidReturn,
    PrimitiveTypeMismatch,
    SegmentationFault,
    InvalidRegister,
    InvalidMemorySize,
}

impl VirtualMachine {
    pub fn new() -> Self {
        Self {
            instructions: vec![],
            program_counter: !0,
            memory: Memory::new(),
            registers: Registers::new(),
            flags: Flags::new(),
            cont: true,
        }
    }

    fn push(&mut self, val: Immediate) {
        let as_bytes: [u8; 8] = val.into();
        let mut mem = self.memory.get_at_mut(self.registers.stack_pointer);
        for i in 0..8 {
            *mem[i] = as_bytes[i];
        }
        self.registers.stack_pointer -= 8;
    }

    fn pop(&mut self, size: usize, is_float: bool) -> Result<Immediate, Fault> {
        if size <= self.registers.stack_pointer {
            return Err(Fault::SegmentationFault);
        }

        let buff = &self.memory.get_at(self.registers.stack_pointer);

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

        self.registers.stack_pointer += size;

        ret
    }

    pub fn get_register_mut(&mut self, reg_type: RegisterType, reg: usize) -> Option<Vec<&mut u8>> {
        if reg > REGISTER_COUNT {
            panic!("Illegal Register {:?} {}", reg_type, reg);
        }
        let immediate = match reg_type {
            RegisterType::Caller => &mut self.registers.caller[reg],
            RegisterType::Callee => &mut self.registers.callee[reg],
        };
        immediate.as_mut().map(|imm| imm.into())
    }

    pub fn get_register(&self, reg_type: RegisterType, reg: usize) -> Option<Immediate> {
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
                self.registers.stack_pointer -= *size;
            }
            Instruction::Pop(size) => {
                self.registers.stack_pointer += *size;
            }
            Instruction::Ret(option) => {
                let ret_location: Immediate = self.pop(POINTER_SIZE, false)?;
                if let Immediate::Pointer(ret_pos_ptr) = ret_location.as_pointer() {
                    next_program_counter = ret_pos_ptr;
                } else {
                    return Err(Fault::InvalidReturn);
                }
            }
            Instruction::Jump(counter) => {
                next_program_counter = *counter;
            }
            Instruction::Compare(comparison, size, is_float) => {
                let val1: Immediate = self.pop(*size, *is_float)?;
                let val2: Immediate = self.pop(*size, *is_float)?;
                let returned_value: Immediate =
                    comparison.perform_op(&mut self.flags, val1, val2)?;
                self.push(returned_value);
            }
            Instruction::PerformOperation(operation, size, is_float) => {
                let val1: Immediate = self.pop(*size, *is_float)?;
                let val2: Immediate = self.pop(*size, *is_float)?;
                let returned_value: Immediate =
                    operation.perform_op(&mut self.flags, val1, val2)?;
                self.push(returned_value);
            }
            Instruction::AddressOf(location) => {}
            Instruction::Dereference(size) => {
                let val: Immediate = self.pop(POINTER_SIZE, false)?;
                let ptr: usize = val.as_pointer().try_into()?;
                let immediate =
                    Immediate::from(self.memory.get_at_of_size(ptr, *size)).to_size(*size as u8)?;
                self.push(immediate)
            }
            Instruction::Call(location) => {
                let program_counter = self.program_counter;
                let pc_imm = Immediate::from(program_counter);
                self.push(pc_imm);
                self.program_counter = *location;
            }
            Instruction::Throw => {}
            Instruction::Catch => {}
            Instruction::ConditionalJump(jump_type, location) => {
                let cond = match jump_type {
                    JumpType::Zero | JumpType::Equal => self.flags.zero,
                    JumpType::NotZero | JumpType::NotEqual => !self.flags.zero,
                    JumpType::Greater => self.flags.zero == self.flags.sign && !self.flags.zero,
                    JumpType::GreaterEqual => self.flags.zero == self.flags.sign || self.flags.zero,
                    JumpType::Above => !self.flags.carry && !self.flags.zero,
                    JumpType::AboveEqual => !self.flags.carry || self.flags.zero,
                    JumpType::Lesser => self.flags.sign != self.flags.zero,
                    JumpType::LessEqual => self.flags.sign != self.flags.zero || self.flags.zero,
                    JumpType::Below => self.flags.carry,
                    JumpType::BelowEqual => self.flags.carry || self.flags.zero,
                    JumpType::Overflow => self.flags.overflow,
                    JumpType::NotOverflow => !self.flags.overflow,
                    JumpType::Signed => self.flags.sign,
                    JumpType::NotSigned => !self.flags.sign,
                };
                if cond {
                    next_program_counter = *location;
                }
            }
            Instruction::Copy { src, size } => {
                let imm: Immediate = src.get_immediate(self, *size as usize, false)?;
                self.push(imm);
            }
            Instruction::Nop => {}
            Instruction::Halt => self.cont = false,
            Instruction::Move { dest, src, size } => {
                let src: Immediate = src.get_immediate(self, *size as usize, false)?;
                let destination: Vec<&mut u8> = dest.get_immediate_bytes(self, *size as usize)?;
                let src_as_u64: [u8; 8] = src.as_u64_no_coercion().into();
                for (index, byte) in destination.into_iter().enumerate() {
                    *byte = src_as_u64[index]
                }
            }
        }
        self.program_counter = next_program_counter;
        Ok(())
    }
}

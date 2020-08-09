use std::error::Error;
use std::fmt::{Display, Formatter};

use crate::flags::Flags;
use crate::instruction_set::Immediate::{Double, Float, U16, U32, U64, U8};
use crate::instruction_set::{Immediate, Instruction, JumpType, Literal, RegisterType};
use crate::memory::Memory;
use crate::registers::Registers;
use crate::vm::Fault::{PrimitiveTypeMismatch, SegmentationFault};

pub struct VirtualMachine {
    instructions: Vec<Instruction>,
    program_counter: usize,
    pub(super) memory: Memory,
    pub(super) registers: Registers,
    stack: Vec<Immediate>,
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
    InvalidAddressOfLocation(Literal),
    NotAVariable(String),
}

impl Display for Fault {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:?}", self)
    }
}

impl Error for Fault {}

impl VirtualMachine {
    pub fn new() -> Self {
        Self {
            instructions: vec![],
            program_counter: 0,
            memory: Memory::new(),
            registers: Registers::new(),
            stack: vec![],
            flags: Flags::new(),
            cont: true,
        }
    }

    fn push(&mut self, val: Immediate) {
        self.stack.push(val);
    }

    fn pop(&mut self) -> Result<Immediate, Fault> {
        self.stack.pop().ok_or(Fault::SegmentationFault)
    }

    pub fn get_register(&self, reg_type: RegisterType, reg: usize) -> Option<Immediate> {
        (match reg_type {
            RegisterType::Caller => self.registers.caller.get(reg),
            RegisterType::Callee => self.registers.callee.get(reg),
        })
        .map(|imm| imm.clone())
    }

    fn run_instruction(&mut self, instruction: &Instruction) -> Result<(), Fault> {
        let mut next_program_counter = self.program_counter + 1;
        match instruction {
            Instruction::PushVal(immediate) => self.push(immediate.clone()),
            Instruction::Pop => {
                let _ = self.pop()?;
            }
            Instruction::PopTo(dest) => {
                let imm = self.pop()?;
                let mut dest = dest.clone();
                let dest_imm = dest.get_immediate_mut(self)?;
                *dest_imm = imm;
            }
            Instruction::Ret(option) => {
                let ret_location: Immediate = self.pop()?;
                if let Immediate::USize(ret_pos_ptr) = ret_location {
                    next_program_counter = ret_pos_ptr;
                } else {
                    return Err(Fault::InvalidReturn);
                }
                if let Some(imm) = option {
                    self.push(imm.get_immediate(self)?);
                }
            }
            Instruction::Jump(counter) => {
                next_program_counter = *counter;
            }
            Instruction::Compare(comparison) => {
                let val1: Immediate = self.pop()?;
                let val2: Immediate = self.pop()?;
                let returned_value: Immediate =
                    comparison.perform_op(&mut self.flags, val1, val2)?;
                self.push(returned_value);
            }
            Instruction::PerformOperation(operation) => {
                let val1: Immediate = self.pop()?;
                let val2: Immediate = self.pop()?;
                let returned_value: Immediate =
                    operation.perform_op(&mut self.flags, val1, val2)?;
                self.push(returned_value);
            }
            Instruction::AddressOf(location) => match location {
                Literal::Variable(v) => {
                    let ret: &Immediate = self.memory.get_variable_ref(v)?;
                    let pointer = Immediate::Pointer(ret as *const Immediate as *mut Immediate);
                    self.push(pointer);
                }
                _ => {
                    return Err(Fault::InvalidAddressOfLocation(location.clone()));
                }
            },
            Instruction::Dereference => {
                let val: Immediate = self.pop()?;
                let immediate = match val {
                    Immediate::Pointer(ptr) => ptr,
                    Immediate::PointerConst(ptr) => ptr,
                    _ => {
                        return Err(PrimitiveTypeMismatch);
                    }
                };
                self.push(unsafe { (*immediate).clone() })
            }
            Instruction::Call(location) => {
                let program_counter = self.program_counter + 1;
                let pc_imm = Immediate::USize(program_counter);
                self.push(pc_imm);
                next_program_counter = *location;
            }
            Instruction::Throw(imm) => {
                unimplemented!("Throwing has not been implemented yet");
            }
            Instruction::Catch => {}
            Instruction::ConditionalJump(jump_type, location) => {
                let _ = self.pop()?;
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
            Instruction::Push { src } => {
                let imm = src.get_immediate(self)?;
                self.push(imm.clone());
            }
            Instruction::Nop => {}
            Instruction::Halt => self.cont = false,
            Instruction::Move { dest, src } => {
                let immediate = src.get_immediate(self)?;
                let imm: &mut Immediate = dest.get_immediate_mut_from_immutable(self)?;
                *imm = immediate;
            }
            Instruction::DeclareVar(name, scope) => {
                self.memory.declare_variable(name, scope);
            }
            Instruction::GetVar(name) => {
                self.push(self.memory.get_variable(name)?);
            }
            Instruction::SaveVar(name) => {
                let imm = self.pop()?;
                self.memory.set_variable(name, imm)?;
            }
            Instruction::Coerce { dest_type } => {
                let src: Immediate = self.pop()?;
                let imm = match dest_type {
                    U8(_) => src.into_u8(),
                    U16(_) => src.into_u16(),
                    U32(_) => src.into_u32(),
                    U64(_) => src.into_u64(),
                    Immediate::USize(_) => src.into_usize(),
                    Float(_) => src.into_float(),
                    Double(_) => src.into_double(),
                    Immediate::Char(_) => src.into_char(),
                    Immediate::Pointer(_) => {
                        if let Immediate::Pointer(_) = &src {
                            src
                        } else {
                            return Err(PrimitiveTypeMismatch);
                        }
                    }
                    Immediate::PointerConst(_) => {
                        if let Immediate::Pointer(_) = &src {
                            src
                        } else if let Immediate::PointerConst(_) = &src {
                            src
                        } else {
                            return Err(PrimitiveTypeMismatch);
                        }
                    }
                    _ => return Err(PrimitiveTypeMismatch),
                };
                self.push(imm);
            }
            Instruction::Enter => {
                self.memory.new_local_scope();
            }
            Instruction::Lower => {
                self.memory.new_lower_scope();
            }
            Instruction::Exit => {
                self.memory.exit_local_scope();
            }
            Instruction::CallFunction(_) => unimplemented!(),
        }
        self.program_counter = next_program_counter;
        Ok(())
    }

    pub fn execute(&mut self, instructions: Vec<Instruction>, start: usize) -> Result<u32, Fault> {
        self.flags.reset();
        self.program_counter = start;
        self.instructions = instructions;
        while self.cont {
            let instruction = self
                .instructions
                .get(self.program_counter)
                .ok_or_else(|| SegmentationFault)?
                .clone();
            self.run_instruction(&instruction)?;
        }
        match self.pop()? {
            U32(exit) => Ok(exit),
            _ => Err(Fault::PrimitiveTypeMismatch),
        }
    }

    pub fn headless_execute(instructions: Vec<Instruction>, start: usize) -> Result<u32, Fault> {
        let mut vm = VirtualMachine::new();
        vm.program_counter = start;
        vm.instructions = instructions;
        while vm.cont {
            let instruction = vm
                .instructions
                .get(vm.program_counter)
                .ok_or_else(|| SegmentationFault)?
                .clone();
            vm.run_instruction(&instruction)?;
        }
        match vm.pop()? {
            U32(exit) => Ok(exit),
            _ => Err(Fault::PrimitiveTypeMismatch),
        }
    }
}

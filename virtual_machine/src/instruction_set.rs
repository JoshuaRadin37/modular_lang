use crate::flags::Flags;
use crate::instruction_set::Immediate::*;
use crate::instruction_set::Offset::Advanced;
use crate::vm::{Fault, VirtualMachine, POINTER_SIZE};
use byteorder::{BigEndian, ByteOrder};
use std::cmp::Ordering;
use std::ops::{Add, BitAnd, BitOr, BitXor, Div, Mul, Rem, Sub};

mod immediate;
pub use immediate::*;

pub mod arithmetic;

#[derive(Debug, Copy, Clone)]
pub enum Operation {
    Add,
    Subtract,
    Multiply,
    Divide,
    Remainder,
    And,
    Or,
    Xor,
}

#[derive(Debug, Copy, Clone)]
pub enum ComparisonOperation {
    And,
    Or,
    LessThan,
    GreaterThan,
    LessThanEqual,
    GreaterThanEqual,
    /// Unsigned Operation
    Above,
    /// Unsigned Operation
    AboveEqual,
    Below,
    /// Unsigned Operation
    BelowEqual,
    Compare,
}

pub mod comparison;

#[derive(Debug, Copy, Clone)]
#[repr(u8)]
pub enum JumpType {
    Zero,
    NotZero,
    Equal,
    NotEqual,
    Greater,
    GreaterEqual,
    /// Unsigned Operation
    Above,
    /// Unsigned Operation
    AboveEqual,
    Lesser,
    LessEqual,
    /// Unsigned Operation
    Below,
    /// Unsigned Operation
    BelowEqual,
    Overflow,
    NotOverflow,
    Signed,
    NotSigned,
}

#[derive(Debug, Copy, Clone)]
#[repr(u8)]
pub enum RegisterType {
    Caller,
    Callee,
}

#[derive(Debug, Copy, Clone)]
pub enum Offset {
    Basic(isize),
    Advanced {
        steps: MemoryLocationInner,
        offset: MemoryLocationInner,
    },
}

#[derive(Debug, Copy, Clone)]
pub enum Literal {
    Location(MemoryLocationInner, Option<Offset>),
    Register(RegisterType, u8),
    Immediate(Immediate),
}

impl Literal {
    pub fn register(reg: RegisterType, num: u8) -> Literal {
        Literal::Register(reg, num)
    }

    pub fn immediate(imm: Immediate) -> Literal {
        Literal::Immediate(imm)
    }

    pub fn location(loc: MemoryLocationInner) -> Literal {
        Literal::Location(loc, None)
    }

    pub fn location_offset(loc: MemoryLocationInner, offset: isize) -> Literal {
        Literal::Location(loc, Some(Offset::Basic(offset)))
    }

    pub fn location_advanced_offset(
        loc: MemoryLocationInner,
        steps: MemoryLocationInner,
        offset: MemoryLocationInner,
    ) -> Literal {
        Literal::Location(loc, Some(Offset::Advanced { steps, offset }))
    }

    pub fn get_immediate(
        &self,
        virtual_machine: &VirtualMachine,
        size: usize,
        is_float: bool,
    ) -> Result<Immediate, Fault> {
        match self {
            Literal::Location(base, offset) => {
                let base = match base {
                    MemoryLocationInner::Location(loc) => {
                        let ptr: Immediate = virtual_machine
                            .memory
                            .get_at_of_size(*loc, POINTER_SIZE)
                            .into();
                        ptr.as_pointer()
                    }
                    MemoryLocationInner::Register(reg_type, num) => virtual_machine
                        .get_register(*reg_type, *num as usize)
                        .unwrap()
                        .as_pointer(),
                    MemoryLocationInner::Immediate(imm) => imm.as_pointer(),
                };
                if let Pointer(ptr) = base {
                    match offset {
                        None => {
                            let mut ret: Immediate =
                                virtual_machine.memory.get_at_of_size(ptr, size).into();
                            if is_float {
                                if size == 32 {
                                    ret = ret.as_float_no_coercion();
                                } else if size == 64 {
                                    ret = ret.as_double_no_coercion();
                                } else {
                                    return Err(Fault::PrimitiveTypeMismatch);
                                }
                            }
                            Ok(ret)
                        }
                        Some(offset) => match offset {
                            Offset::Basic(basic) => {
                                let offset: Immediate = (Immediate::from(*basic as usize)
                                    * Immediate::from(size))
                                .0?
                                .0;
                                let mod_ptr: Immediate = (base + offset).0?.0;
                                if let Pointer(mod_ptr) = mod_ptr.as_pointer() {
                                    Ok(Immediate::from(
                                        virtual_machine.memory.get_at_of_size(mod_ptr, size),
                                    ))
                                } else {
                                    unreachable!()
                                }
                            }
                            Offset::Advanced { steps, offset } => {
                                let steps: Immediate = match steps {
                                    MemoryLocationInner::Location(loc) => virtual_machine
                                        .memory
                                        .get_at_of_size(*loc, POINTER_SIZE)
                                        .into(),
                                    MemoryLocationInner::Register(reg_type, num) => virtual_machine
                                        .get_register(*reg_type, *num as usize)
                                        .unwrap()
                                        .as_pointer(),
                                    MemoryLocationInner::Immediate(imm) => *imm,
                                };
                                let step_size: Immediate = match offset {
                                    MemoryLocationInner::Location(loc) => virtual_machine
                                        .memory
                                        .get_at_of_size(*loc, POINTER_SIZE)
                                        .into(),
                                    MemoryLocationInner::Register(reg_type, num) => virtual_machine
                                        .get_register(*reg_type, *num as usize)
                                        .unwrap()
                                        .as_pointer(),
                                    MemoryLocationInner::Immediate(imm) => *imm,
                                };
                                let offset: Immediate = (step_size * steps).0?.0;
                                let mod_ptr: Immediate = (base + offset).0?.0;
                                if let Pointer(mod_ptr) = mod_ptr.as_pointer() {
                                    Ok(Immediate::from(
                                        virtual_machine.memory.get_at_of_size(mod_ptr, size),
                                    ))
                                } else {
                                    unreachable!()
                                }
                            }
                        },
                    }
                } else {
                    unreachable!()
                }
            }
            Literal::Register(reg_type, num) => virtual_machine
                .get_register(*reg_type, *num as usize)
                .ok_or(Fault::InvalidRegister),
            Literal::Immediate(immediate) => Ok(immediate.clone()),
        }
    }

    pub fn get_immediate_bytes<'a>(
        &self,
        virtual_machine: &'a mut VirtualMachine,
        size: usize,
    ) -> Result<Vec<&'a mut u8>, Fault> {
        match self {
            Literal::Location(base, offset) => {
                let base = match base {
                    MemoryLocationInner::Location(loc) => {
                        let ptr: Immediate = virtual_machine
                            .memory
                            .get_at_of_size(*loc, POINTER_SIZE)
                            .into();
                        ptr.as_pointer()
                    }
                    MemoryLocationInner::Register(reg_type, num) => virtual_machine
                        .get_register(*reg_type, *num as usize)
                        .unwrap()
                        .as_pointer(),
                    MemoryLocationInner::Immediate(imm) => imm.as_pointer(),
                };
                if let Pointer(ptr) = base {
                    match offset {
                        None => {
                            let mut ret = virtual_machine.memory.get_at_of_size_mut(ptr, size);
                            Ok(ret)
                        }
                        Some(offset) => match offset {
                            Offset::Basic(basic) => {
                                let offset: Immediate = (Immediate::from(*basic as usize)
                                    * Immediate::from(size))
                                .0?
                                .0;
                                let mod_ptr: Immediate = (base + offset).0?.0;
                                if let Pointer(mod_ptr) = mod_ptr.as_pointer() {
                                    Ok(virtual_machine.memory.get_at_of_size_mut(mod_ptr, size))
                                } else {
                                    unreachable!()
                                }
                            }
                            Offset::Advanced { steps, offset } => {
                                let steps: Immediate = match steps {
                                    MemoryLocationInner::Location(loc) => virtual_machine
                                        .memory
                                        .get_at_of_size(*loc, POINTER_SIZE)
                                        .into(),
                                    MemoryLocationInner::Register(reg_type, num) => virtual_machine
                                        .get_register(*reg_type, *num as usize)
                                        .unwrap()
                                        .as_pointer(),
                                    MemoryLocationInner::Immediate(imm) => *imm,
                                };
                                let step_size: Immediate = match offset {
                                    MemoryLocationInner::Location(loc) => virtual_machine
                                        .memory
                                        .get_at_of_size(*loc, POINTER_SIZE)
                                        .into(),
                                    MemoryLocationInner::Register(reg_type, num) => virtual_machine
                                        .get_register(*reg_type, *num as usize)
                                        .unwrap()
                                        .as_pointer(),
                                    MemoryLocationInner::Immediate(imm) => *imm,
                                };
                                let offset: Immediate = (step_size * steps).0?.0;
                                let mod_ptr: Immediate = (base + offset).0?.0;
                                if let Pointer(mod_ptr) = mod_ptr.as_pointer() {
                                    Ok(virtual_machine.memory.get_at_of_size_mut(mod_ptr, size))
                                } else {
                                    unreachable!()
                                }
                            }
                        },
                    }
                } else {
                    unreachable!()
                }
            }
            Literal::Register(reg_type, num) => virtual_machine
                .get_register_mut(*reg_type, *num as usize)
                .ok_or(Fault::InvalidRegister),
            Literal::Immediate(immediate) => panic!("Can't have a mutable immediate"),
        }
    }
}

#[derive(Debug, Copy, Clone)]
pub enum MemoryLocationInner {
    Location(usize),
    Register(RegisterType, u8),
    Immediate(Immediate),
}

pub struct Signed(Immediate);

impl ZeroComparable for Immediate {
    fn zero_compare(&self) -> Option<Ordering> {
        let zero: Immediate = match self {
            U8(_) => 0u8.into(),
            U16(_) => 0u16.into(),
            U32(_) => 0u32.into(),
            U64(_) => 0u64.into(),
            Float(_) => 0.0f32.into(),
            Double(_) => 0.0f64.into(),
            Char(_) => 0u8.into(),
            Pointer(_) => 0usize.into(),
        };
        self.partial_cmp(&zero)
    }
}

impl ZeroComparable for Signed {
    fn zero_compare(&self) -> Option<Ordering> {
        let zero: Signed = match self.0 {
            U8(_) => 0i8.into(),
            U16(_) => 0i16.into(),
            U32(_) => 0i32.into(),
            U64(_) => 0i64.into(),
            Float(_) => 0.0f32.into(),
            Double(_) => 0.0f64.into(),
            Char(_) => 0i8.into(),
            Pointer(_) => 0isize.into(),
        };
        self.partial_cmp(&zero)
    }
}

impl From<i8> for Signed {
    fn from(d: i8) -> Self {
        Signed(U8(d as u8))
    }
}

impl From<i16> for Signed {
    fn from(d: i16) -> Self {
        Signed(U16(d as u16))
    }
}

impl From<i32> for Signed {
    fn from(d: i32) -> Self {
        Signed(U32(d as u32))
    }
}

impl From<i64> for Signed {
    fn from(d: i64) -> Self {
        Signed(U64(d as u64))
    }
}

impl From<f32> for Signed {
    fn from(d: f32) -> Self {
        Signed(Float(d))
    }
}

impl From<f64> for Signed {
    fn from(d: f64) -> Self {
        Signed(Double(d))
    }
}

impl From<isize> for Signed {
    fn from(d: isize) -> Self {
        Signed(match POINTER_SIZE {
            4 => U32(d as u32),
            8 => U64(d as u64),
            _ => panic!("{:?}", Fault::PrimitiveTypeMismatch),
        })
    }
}

#[derive(Debug, Copy, Clone)]
pub enum Instruction {
    PushVal(Immediate),
    Push(usize),
    Pop(usize),
    Ret(Option<Literal>),
    Jump(usize),
    Compare(ComparisonOperation, usize, bool),
    PerformOperation(Operation, usize, bool),
    ConditionalJump(JumpType, usize),
    AddressOf(Literal),
    Dereference(usize),
    Call(usize),
    Throw,
    Catch,
    /// Copies a value to the top of the stack
    Copy {
        src: Literal,
        size: u8,
    },
    Move {
        dest: Literal,
        src: Literal,
        size: u8,
    },
    Nop,
    Halt,
}

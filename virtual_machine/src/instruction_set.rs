use std::cmp::Ordering;
use std::ops::{Add, BitAnd, BitOr, BitXor, Deref, Div, Mul, Rem, Sub};
use std::string::String;

pub use immediate::*;

use crate::flags::Flags;
use crate::instruction_set::Immediate::*;
use crate::intrinsics::known_types::MARKER_TRAITS;
use crate::memory::Scope;
use crate::resolution::functions::Function;
use crate::resolution::types::descriptor::Variant;
use crate::resolution::{Resolvable, FullIdentifier};
use crate::vm::Fault::{InvalidRegister, TypeMismatch};
use crate::vm::{Fault, VirtualMachine, POINTER_SIZE};

pub mod arithmetic;
mod immediate;

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

#[derive(Debug, Clone)]
pub enum Literal {
    /// The actual variable itself
    Variable(String),
    /// A register
    Register(RegisterType, u8),
    /// An immutable value
    Immediate(Immediate),
    /// pop the top of the stack
    Peak,
}

impl Literal {
    pub fn register(reg: RegisterType, num: u8) -> Literal {
        Literal::Register(reg, num)
    }

    pub fn immediate(imm: Immediate) -> Literal {
        Literal::Immediate(imm)
    }

    pub fn variable<'a, S: Deref<Target = &'a str>>(loc: S) -> Literal {
        Literal::Variable(loc.to_string())
    }

    pub fn get_immediate(&self, virtual_machine: &VirtualMachine) -> Result<Immediate, Fault> {
        match self {
            Literal::Variable(name) => virtual_machine.memory.get_variable(name),
            Literal::Register(reg, num) => match reg {
                RegisterType::Caller => virtual_machine
                    .registers
                    .caller
                    .get(*num as usize)
                    .map(|i| i.clone())
                    .ok_or(InvalidRegister),
                RegisterType::Callee => virtual_machine
                    .registers
                    .callee
                    .get(*num as usize)
                    .map(|i| i.clone())
                    .ok_or(InvalidRegister),
            },
            Literal::Immediate(im) => Ok(im.clone()),
            Literal::Peak => virtual_machine.peak().map(|imm| imm.clone()),
        }
    }

    pub fn get_immediate_ref<'a>(
        &'a self,
        virtual_machine: &'a VirtualMachine,
    ) -> Result<&'a Immediate, Fault> {
        match self {
            Literal::Variable(name) => virtual_machine.memory.get_variable_ref(name),
            Literal::Register(reg, num) => match reg {
                RegisterType::Caller => virtual_machine
                    .registers
                    .caller
                    .get(*num as usize)
                    .ok_or(InvalidRegister),
                RegisterType::Callee => virtual_machine
                    .registers
                    .callee
                    .get(*num as usize)
                    .ok_or(InvalidRegister),
            },
            Literal::Immediate(_) => Err(Fault::InvalidAddressOfLocation(self.clone())),
            Literal::Peak => virtual_machine.peak(),
        }
    }

    pub fn get_immediate_mut_from_immutable<'a>(
        &self,
        virtual_machine: &'a mut VirtualMachine,
    ) -> Result<&'a mut Immediate, Fault> {
        match self {
            Literal::Variable(name) => virtual_machine.memory.get_variable_mut(name),
            Literal::Register(reg, num) => match reg {
                RegisterType::Caller => virtual_machine
                    .registers
                    .caller
                    .get_mut(*num as usize)
                    .ok_or(InvalidRegister),
                RegisterType::Callee => virtual_machine
                    .registers
                    .callee
                    .get_mut(*num as usize)
                    .ok_or(InvalidRegister),
            },
            Literal::Immediate(_) => Err(Fault::InvalidAddressOfLocation(self.clone())),
            Literal::Peak => virtual_machine.peak_mut(),
        }
    }

    pub fn get_immediate_mut<'a>(
        &'a mut self,
        virtual_machine: &'a mut VirtualMachine,
    ) -> Result<&'a mut Immediate, Fault> {
        match self {
            Literal::Variable(name) => virtual_machine.memory.get_variable_mut(name),
            Literal::Register(reg, num) => match reg {
                RegisterType::Caller => virtual_machine
                    .registers
                    .caller
                    .get_mut(*num as usize)
                    .ok_or(InvalidRegister),
                RegisterType::Callee => virtual_machine
                    .registers
                    .callee
                    .get_mut(*num as usize)
                    .ok_or(InvalidRegister),
            },
            Literal::Immediate(im) => Ok(im),
            Literal::Peak => virtual_machine.peak_mut(),
        }
    }

    pub fn copy_immediate(
        &self,
        virtual_machine: &VirtualMachine,
        destination: &mut Immediate,
    ) -> Result<(), Fault> {
        let src: &Immediate = &self.get_immediate(virtual_machine)?;

        match (destination, src) {
            (U8(dest), U8(src)) => {
                *dest = src.clone();
            }
            (U16(dest), U16(src)) => {
                *dest = src.clone();
            }
            (U32(dest), U32(src)) => {
                *dest = src.clone();
            }
            (U64(dest), U64(src)) => {
                *dest = src.clone();
            }
            (USize(dest), USize(src)) => {
                *dest = src.clone();
            }
            (Float(dest), Float(src)) => {
                *dest = src.clone();
            }
            (Double(dest), Double(src)) => {
                *dest = src.clone();
            }
            (Char(dest), Char(src)) => {
                *dest = src.clone();
            }
            (Array(dest), Array(src)) => {
                *dest = src.clone();
            }
            (Pointer(dest), Pointer(src)) => {
                *dest = src.clone();
            }
            (PointerConst(dest), PointerConst(src)) => {
                *dest = src.clone();
            }
            (PointerConst(dest), Pointer(src)) => {
                *dest = src.clone();
            }
            (DetailedType(dest), DetailedType(src)) => {
                if !src
                    .get_descriptor()
                    .is_instance_of(dest.get_descriptor().get_identifier())
                {
                    return Err(TypeMismatch);
                }

                if dest
                    .get_descriptor()
                    .implements_trait(&MARKER_TRAITS[&"copy_trait".to_string()])
                {
                    *dest = src.clone();
                } else {
                    return Err(TypeMismatch);
                }
            }
            _ => {
                return Err(Fault::PrimitiveTypeMismatch);
            }
        }

        Ok(())
    }
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
            USize(_) => 0usize.into(),
            PointerConst(_) => 0usize.into(),
            _ => return None,
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
            USize(_) => 0isize.into(),
            PointerConst(_) => 0isize.into(),
            _ => return None,
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

#[derive(Debug, Clone)]
pub enum Instruction {
    PushVal(Immediate),
    Pop,
    PopTo(Literal),
    Ret(Option<Literal>),
    Jump(usize),
    Compare(ComparisonOperation),
    PerformOperation(Operation),
    ConditionalJump(JumpType, usize),
    AddressOf(Literal),
    Dereference,
    Call(usize),
    Throw(Immediate),
    Catch,
    /// Copies a value to the top of the stack
    Push {
        src: Literal,
    },
    Move {
        dest: Literal,
        src: Literal,
    },
    Nop,
    Halt,
    DeclareVar(String, Scope),
    GetVar(String),
    SaveVar(String),
    Coerce {
        dest_type: Immediate,
    },
    CallFunction(Function),
    GetField(Literal, FullIdentifier),
    GetMember(Literal, usize),
    /// Builds variants bottom up based on the variant in the destination type
    BuildVariant {
        dest_variant: Variant,
    },
    Enter,
    Lower,
    Exit,
    Heapify,
}

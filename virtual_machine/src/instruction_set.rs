use std::any::Any;
use crate::vm::Fault;

#[derive(Debug, Copy, Clone)]
pub enum Immediate {
    ValueUnsigned(usize),
    ValueSigned(isize),
    Float(f32),
    Double(f64),
    Char(u8),
    String(&'static str),
    Structure(Vec<Immediate>)
}

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

impl Operation {

    pub fn perform_op(&self, val1: Immediate, val2: Immediate) -> Result<Immediate, Fault> {

    }
}

#[derive(Debug, Copy, Clone)]
pub enum ComparisonOperation {
    And,
    Or,
    LessThan,
    GreaterThan,
    LessThanEqual,
    GreaterThanEqual
}

impl ComparisonOperation {
    pub fn perform_op(&self, val1: Immediate, val2: Immediate) -> Result<Immediate, Fault> {

    }
}

#[derive(Debug, Copy, Clone)]
pub enum Instruction {
    Push(Immediate),
    Pop,
    Ret,
    Jump(usize),
    Compare(ComparisonOperation),
    PerformOperation(Operation),
    ConditionalJump(usize),
    AddressOf,
    Dereference,
    Alloc,
    Call(usize),
    Throw,
    Catch,
    Pack
}
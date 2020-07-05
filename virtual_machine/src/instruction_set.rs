use crate::flags::Flags;
use crate::instruction_set::Immediate::*;
use crate::vm::{Fault, POINTER_SIZE};
use byteorder::{BigEndian, ByteOrder};
use std::cmp::Ordering;
use std::ops::{Add, BitAnd, BitOr, BitXor, Div, Mul, Rem, Sub};

#[derive(Debug, Copy, Clone)]
pub enum Immediate {
    U8(u8),
    U16(u16),
    U32(u32),
    U64(u64),
    Float(f32),
    Double(f64),
    Char(char),
    Pointer(usize),
}

macro_rules! as_other_primitive {
    ($input:expr, $dest_enum:path, $dest_type:ty) => {
        match $input {
            U8(d) => $dest_enum(d as $dest_type),
            U16(d) => $dest_enum(d as $dest_type),
            U32(d) => $dest_enum(d as $dest_type),
            U64(d) => $dest_enum(d as $dest_type),
            Float(d) => $dest_enum(d as $dest_type),
            Double(d) => $dest_enum(d as $dest_type),
            Char(d) => $dest_enum(d as $dest_type),
            Pointer(d) => $dest_enum(d as $dest_type),
        }
    };
}

trait ZeroComparable {

    fn zero_compare(&self) -> Option<Ordering>;
}

impl Immediate {
    pub fn as_u8(self) -> Self {
        as_other_primitive!(self, U8, u8)
    }

    pub fn as_u16(self) -> Self {
        as_other_primitive!(self, U16, u16)
    }

    pub fn as_u32(self) -> Self {
        as_other_primitive!(self, U32, u32)
    }

    pub fn as_u64(self) -> Self {
        as_other_primitive!(self, U64, u64)
    }
    pub fn as_float(self) -> Self {
        match self {
            U8(d) => Float(d as f32),
            U16(d) => Float(d as f32),
            U32(d) => Float(d as f32),
            U64(d) => Float(d as f32),
            Float(d) => Float(d as f32),
            Double(d) => Float(d as f32),
            Char(d) => Float(d as u8 as f32),
            Pointer(d) => Float(d as f32),
        }
    }
    pub fn as_double(self) -> Self {
        match self {
            U8(d) => Double(d as f64),
            U16(d) => Double(d as f64),
            U32(d) => Double(d as f64),
            U64(d) => Double(d as f64),
            Float(d) => Double(d as f64),
            Double(d) => Double(d as f64),
            Char(d) => Double(d as u8 as f64),
            Pointer(d) => Double(d as f64),
        }
    }
    pub fn as_char(self) -> Self {
        match self {
            U8(d) => Char(d as char),
            _ => {
                panic!("Can not convert a non-u8 primitive to a char");
            }
        }
    }
    pub fn as_pointer(self) -> Self {
        as_other_primitive!(self, Pointer, usize)
    }
    pub fn is_zero(&self) -> bool {
        match self {
            U8(d) => d == &0,
            U16(d) => d == &0,
            U32(d) => d == &0,
            U64(d) => d == &0,
            Float(d) => d == &0.0,
            Double(d) => d == &0.0,
            Char(d) => d == &'\0',
            Pointer(d) => d == &0,
        }
    }
    pub fn bool_equivalent(input: bool) -> Immediate {
        if input {
            U8(0)
        } else {
            U8(std::u8::MAX)
        }
    }
    /// Gets the most significant bit
    pub fn msb(&self) -> bool {
        match self {
            U8(d) => d >> 7 > 0,
            U16(d) => d >> 15 > 0,
            U32(d) => d >> 31 > 0,
            U64(d) => d >> 63 > 0,
            Char(d) => *d as u8 >> 7 > 0,
            Pointer(d) => d >> (POINTER_SIZE * 8 - 1) > 0,
            _ => {
                panic!("{:?}", Fault::PrimitiveTypeMismatch);
            }
        }
    }

    /// Gets the least significant bit
    pub fn lsb(&self) -> bool {
        match self {
            U8(d) => d & 0x1 > 0,
            U16(d) => d & 0x1 > 0,
            U32(d) => d & 0x1 > 0,
            U64(d) => d & 0x1 > 0,
            Char(d) => *d as u8 & 0x1 > 0,
            Pointer(d) => d & 0x1 > 0,
            _ => {
                panic!("{:?}", Fault::PrimitiveTypeMismatch);
            }
        }
    }
}

impl From<u8> for Immediate {
    fn from(d: u8) -> Self {
        U8(d)
    }
}

impl From<u16> for Immediate {
    fn from(d: u16) -> Self {
        U16(d)
    }
}

impl From<u32> for Immediate {
    fn from(d: u32) -> Self {
        U32(d)
    }
}

impl From<u64> for Immediate {
    fn from(d: u64) -> Self {
        U64(d)
    }
}

impl From<f32> for Immediate {
    fn from(d: f32) -> Self {
        Float(d)
    }
}

impl From<f64> for Immediate {
    fn from(d: f64) -> Self {
        Double(d)
    }
}

impl From<usize> for Immediate {
    fn from(d: usize) -> Self {
        match POINTER_SIZE {
            4 => {
                U32(d as u32)
            },
            8 => {
                U64(d as u64)
            },
            _ => {
                panic!("{:?}", Fault::PrimitiveTypeMismatch)
            }
        }
    }
}

impl <T> From<&T> for Immediate {
    fn from(d: &T) -> Self {
        let ptr = d as *const T;
        let as_usize = ptr as usize;
        Pointer(as_usize)
    }
}




pub type ImmediateResult = Result<Immediate, Fault>;

impl From<Immediate> for [u8; 8] {
    fn from(i: Immediate) -> Self {
        let mut out: [u8; 8] = [0; 8];
        match i {
            Immediate::U8(d) => {
                out[0] = d;
            }
            Immediate::U16(d) => {
                BigEndian::write_u16(&mut out, d);
            }
            Immediate::U32(d) => {
                BigEndian::write_u32(&mut out, d);
            }
            Immediate::U64(d) => {
                BigEndian::write_u64(&mut out, d);
            }
            Immediate::Float(f) => BigEndian::write_f32(&mut out, f),
            Immediate::Double(d) => BigEndian::write_f64(&mut out, d),
            Immediate::Char(c) => {
                c.encode_utf8(&mut out);
            }
            Immediate::Pointer(s) => {
                BigEndian::write_uint(&mut out, s as u64, std::mem::size_of::<usize>())
            }
        }
        out
    }
}

impl From<bool> for Immediate {
    fn from(b: bool) -> Self {
        Immediate::bool_equivalent(b)
    }
}

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
    Advanced { steps: usize, offset: MemoryType },
}

#[derive(Debug, Copy, Clone)]
pub enum MemoryType {
    Location(usize),
    Register(RegisterType, u8),
    Immediate(Immediate),
}

pub struct Signed(Immediate);

impl ZeroComparable for Immediate {
    fn zero_compare(&self) -> Option<Ordering> {
        let zero: Immediate = match self {
            U8(_) => {
                0u8.into()
            },
            U16(_) => {
                0u16.into()
            },
            U32(_) => {
                0u32.into()
            },
            U64(_) => {
                0u64.into()
            },
            Float(_) => {
                0.0f32.into()
            },
            Double(_) => {
                0.0f64.into()
            },
            Char(_) => {
                0u8.into()
            },
            Pointer(_) => {
                0usize.into()
            },
        };
        self.partial_cmp(&zero)
    }
}

impl ZeroComparable for Signed {
    fn zero_compare(&self) -> Option<Ordering> {
        let zero: Signed = match self.0 {
            U8(_) => {
                0i8.into()
            },
            U16(_) => {
                0i16.into()
            },
            U32(_) => {
                0i32.into()
            },
            U64(_) => {
                0i64.into()
            },
            Float(_) => {
                0.0f32.into()
            },
            Double(_) => {
                0.0f64.into()
            },
            Char(_) => {
                0i8.into()
            },
            Pointer(_) => {
                0isize.into()
            },
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
            4 => {
                U32(d as u32)
            },
            8 => {
                U64(d as u64)
            },
            _ => {
                panic!("{:?}", Fault::PrimitiveTypeMismatch)
            }
        })
    }
}

#[derive(Debug, Copy, Clone)]
pub enum Instruction {
    PushVal(Immediate),
    Push(usize),
    Pop(usize),
    Ret(Option<MemoryType>),
    Jump(usize),
    Compare(ComparisonOperation, usize, bool),
    PerformOperation(Operation, usize, bool),
    ConditionalJump(JumpType, usize),
    AddressOf(Offset),
    Dereference,
    Alloc,
    Call(usize),
    Throw,
    Catch,
    /// Copies a value to the top of the stack
    Copy(MemoryType),
    Nop,
    Halt,
}

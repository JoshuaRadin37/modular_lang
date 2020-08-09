use std::cmp::Ordering;
use std::convert::TryFrom;

use Immediate::*;

use crate::resolution::functions::Function;
use crate::resolution::types::descriptor::Variant;
use crate::vm::{Fault, POINTER_SIZE};
use byteorder::{BigEndian, ByteOrder};
use crate::resolution::types::TypedObject;

#[derive(Debug, Clone)]
pub enum Immediate {
    /// Represents the internal U8 type
    U8(u8),
    /// Represents the internal U16 type
    U16(u16),
    /// Represents the internal U32 type
    U32(u32),
    /// Represents the internal U64 type
    U64(u64),
    /// Represents the internal USize type
    USize(usize),
    /// Represents the internal f32 type
    Float(f32),
    /// Represents the internal f64 type
    Double(f64),
    /// Represents the internal char type
    Char(char),
    /// A mutable pointer to another immediate
    Pointer(*mut Immediate),
    /// An immutable pointer to another immediate
    PointerConst(*const Immediate),
    /// An array of the same type of Immediate
    ///
    /// # Safety
    ///
    /// These immediates should all be of the same type
    Array(Vec<Option<Immediate>>),
    /// Either a structure, a tuple, or an empty
    Variant(Variant),
    /// A type with more information stored in it
    DetailedType(TypedObject),
    Function(Function),
}

macro_rules! into_other_primitive {
    ($input:expr, $dest_enum:path, $dest_type:ty) => {
        match $input {
            U8(d) => $dest_enum(d as $dest_type),
            U16(d) => $dest_enum(d as $dest_type),
            U32(d) => $dest_enum(d as $dest_type),
            U64(d) => $dest_enum(d as $dest_type),
            USize(d) => $dest_enum(d as $dest_type),
            Float(d) => $dest_enum(d as $dest_type),
            Double(d) => $dest_enum(d as $dest_type),
            Char(d) => $dest_enum(d as $dest_type),
            Pointer(d) => $dest_enum(d as $dest_type),
            s => {
                panic!("{:?} can not be converted to a float", s);
            }
        }
    };
}

pub(crate) trait ZeroComparable {
    fn zero_compare(&self) -> Option<Ordering>;
}

impl Immediate {
    pub fn into_u8(self) -> Self {
        into_other_primitive!(self, U8, u8)
    }

    pub fn into_u16(self) -> Self {
        into_other_primitive!(self, U16, u16)
    }

    pub fn into_u32(self) -> Self {
        into_other_primitive!(self, U32, u32)
    }

    pub fn into_u64(self) -> Self {
        into_other_primitive!(self, U64, u64)
    }
    pub fn into_u64_no_coercion(self) -> Self {
        if let Double(buff) = self {
            let ptr = &buff as *const f64;
            let mod_ptr = ptr as *const u64;
            unsafe {
                let val = *mod_ptr;
                U64(val)
            }
        } else if let Float(_) = self {
            self.into_double().into_u64_no_coercion()
        } else {
            self.into_u64()
        }
    }
    pub fn into_usize(self) -> Self {
        into_other_primitive!(self, USize, usize)
    }

    pub fn into_float(self) -> Self {
        match self {
            U8(d) => Float(d as f32),
            U16(d) => Float(d as f32),
            U32(d) => Float(d as f32),
            U64(d) => Float(d as f32),
            USize(d) => Float(d as f32),
            Float(d) => Float(d as f32),
            Double(d) => Float(d as f32),
            Char(d) => Float(d as u8 as f32),
            s => {
                panic!("{:?} can not be converted to a float", s);
            }
        }
    }
    pub fn into_float_no_coercion(self) -> Self {
        if let U32(buff) = self {
            let ptr = &buff as *const u32;
            let mod_ptr = ptr as *const f32;
            unsafe {
                let val = *mod_ptr;
                Float(val)
            }
        } else if let Float(_) = self {
            self
        } else {
            self.into_u32().into_float_no_coercion()
        }
    }
    pub fn into_double_no_coercion(self) -> Self {
        if let U64(buff) = self {
            let ptr = &buff as *const u64;
            let mod_ptr = ptr as *const f64;
            unsafe {
                let val = *mod_ptr;
                Double(val)
            }
        } else if let Double(_) = self {
            self
        } else {
            self.into_u64().into_double_no_coercion()
        }
    }
    pub fn into_double(self) -> Self {
        match self {
            U8(d) => Double(d as f64),
            U16(d) => Double(d as f64),
            U32(d) => Double(d as f64),
            U64(d) => Double(d as f64),
            USize(d) => Double(d as f64),
            Float(d) => Double(d as f64),
            Double(d) => Double(d as f64),
            Char(d) => Double(d as u8 as f64),
            s => {
                panic!("{:?} can not be converted to a float", s);
            }
        }
    }
    pub fn into_char(self) -> Self {
        match self {
            U8(d) => Char(d as char),
            _ => {
                panic!("Can not convert a non-u8 primitive to a char");
            }
        }
    }
    pub fn into_size(self, size: u8) -> Result<Self, Fault> {
        match self {
            Float(_) | Double(_) | Char(_) | Pointer(_) => {
                return Err(Fault::PrimitiveTypeMismatch)
            }
            _ => {}
        }
        match size {
            1 => Ok(self.into_u8()),
            2 => Ok(self.into_u16()),
            4 => Ok(self.into_u32()),
            8 => Ok(self.into_u64()),
            _ => Err(Fault::InvalidMemorySize),
        }
    }
    pub fn is_zero(&self) -> bool {
        #[allow(clippy::float_cmp)]
        match self {
            U8(d) => d == &0,
            U16(d) => d == &0,
            U32(d) => d == &0,
            U64(d) => d == &0,
            USize(d) => d == &0,

            Float(d) => d == &0.0 ,
            Double(d) => d == &0.0,
            Char(d) => d == &'\0',
            Pointer(d) => d.is_null(),
            s => {
                panic!("{:?} can not be converted to a float", s);
            }
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
            USize(d) => d >> (if POINTER_SIZE == 4 { 31 } else { 63 }) > 0,
            Char(d) => *d as u8 >> 7 > 0,
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
            USize(d) => d & 0x1 > 0,
            Char(d) => *d as u8 & 0x1 > 0,
            _ => {
                panic!("{:?}", Fault::PrimitiveTypeMismatch);
            }
        }
    }
    pub fn set_bits(&self) -> u8 {
        let bytes: Vec<u8> = self.clone().into();
        let mut count = 0;
        for byte in &bytes {
            for i in 0..8 {
                let bit = (byte >> i) & 0b1;
                if bit == 1 {
                    count += 1;
                }
            }
        }
        count
    }

    pub fn can_copy(&self) -> bool {
        match self {
            Array(_) => false,
            Variant(_) => false,
            DetailedType(details) => {
                unimplemented!()
            },
            Function(_) => false,
            _ => true
        }
    }
}

impl From<Immediate> for Vec<u8> {
    fn from(i: Immediate) -> Self {
        let mut vec = vec![];

        match i {
            U8(d) => vec.push(d),
            U16(d) => {
                vec = vec![0; 2];
                BigEndian::write_u16(&mut *vec, d);
            }
            U32(d) => {
                vec = vec![0; 4];
                BigEndian::write_u32(&mut *vec, d);
            }
            U64(d) => {
                vec = vec![0; 8];
                BigEndian::write_u64(&mut *vec, d);
            }
            _ => {
                panic!("{:?}", Fault::PrimitiveTypeMismatch);
            }
        }

        vec
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
            4 => U32(d as u32),
            8 => U64(d as u64),
            _ => panic!("{:?}", Fault::PrimitiveTypeMismatch),
        }
    }
}

impl From<&Immediate> for Immediate {
    fn from(d: &Immediate) -> Self {
        PointerConst(d)
    }
}

impl From<&mut Immediate> for Immediate {
    fn from(d: &mut Immediate) -> Self {
        Pointer(d)
    }
}

impl TryFrom<Immediate> for u8 {
    type Error = Fault;

    fn try_from(value: Immediate) -> Result<Self, Self::Error> {
        if let U8(ret) = value.into_u8() {
            Ok(ret)
        } else {
            Err(Fault::PrimitiveTypeMismatch)
        }
    }
}

impl TryFrom<Immediate> for u16 {
    type Error = Fault;

    fn try_from(value: Immediate) -> Result<Self, Self::Error> {
        if let U16(ret) = value.into_u16() {
            Ok(ret)
        } else {
            Err(Fault::PrimitiveTypeMismatch)
        }
    }
}

impl TryFrom<Immediate> for u32 {
    type Error = Fault;

    fn try_from(value: Immediate) -> Result<Self, Self::Error> {
        if let U32(ret) = value.into_u32() {
            Ok(ret)
        } else {
            Err(Fault::PrimitiveTypeMismatch)
        }
    }
}

impl TryFrom<Immediate> for u64 {
    type Error = Fault;

    fn try_from(value: Immediate) -> Result<Self, Self::Error> {
        if let U64(ret) = value.into_u64() {
            Ok(ret)
        } else {
            Err(Fault::PrimitiveTypeMismatch)
        }
    }
}

impl TryFrom<Immediate> for f32 {
    type Error = Fault;

    fn try_from(value: Immediate) -> Result<Self, Self::Error> {
        if let Float(ret) = value.into_float() {
            Ok(ret)
        } else {
            Err(Fault::PrimitiveTypeMismatch)
        }
    }
}

impl TryFrom<Immediate> for f64 {
    type Error = Fault;

    fn try_from(value: Immediate) -> Result<Self, Self::Error> {
        if let Double(ret) = value.into_double() {
            Ok(ret)
        } else {
            Err(Fault::PrimitiveTypeMismatch)
        }
    }
}

impl TryFrom<Immediate> for char {
    type Error = Fault;

    fn try_from(value: Immediate) -> Result<Self, Self::Error> {
        if let Char(ret) = value {
            Ok(ret)
        } else {
            Err(Fault::PrimitiveTypeMismatch)
        }
    }
}

pub type ImmediateResult = Result<Immediate, Fault>;

impl From<bool> for Immediate {
    fn from(b: bool) -> Self {
        Immediate::bool_equivalent(b)
    }
}

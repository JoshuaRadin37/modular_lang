use std::cmp::Ordering;

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

use crate::vm::{Fault, POINTER_SIZE};
use Immediate::*;

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

pub(crate) trait ZeroComparable {
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
    pub fn as_u64_no_coercion(self) -> Self {
        if let Double(buff) = self {
            let ptr = &buff as *const f64;
            let mod_ptr = ptr as *const u64;
            unsafe {
                let val = *mod_ptr;
                U64(val)
            }
        } else if let Float(_) = self {
            self.as_double().as_u64_no_coercion()
        } else {
            self.as_u64()
        }
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
    pub fn as_float_no_coercion(self) -> Self {
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
            self.as_u32().as_float_no_coercion()
        }
    }
    pub fn as_double_no_coercion(self) -> Self {
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
            self.as_u64().as_double_no_coercion()
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
    pub fn to_size(self, size: u8) -> Result<Self, Fault> {
        match self {
            Float(_) | Double(_) | Char(_) | Pointer(_) => {
                return Err(Fault::PrimitiveTypeMismatch)
            }
            _ => {}
        }
        match size {
            1 => Ok(self.as_u8()),
            2 => Ok(self.as_u16()),
            4 => Ok(self.as_u32()),
            8 => Ok(self.as_u64()),
            _ => Err(Fault::InvalidMemorySize),
        }
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
    pub fn set_bits(&self) -> u8 {
        let bytes: [u8; 8] = self.clone().into();
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

impl<T> From<&T> for Immediate {
    fn from(d: &T) -> Self {
        let ptr = d as *const T;
        let as_usize = ptr as usize;
        Pointer(as_usize)
    }
}

impl TryFrom<Immediate> for u8 {
    type Error = Fault;

    fn try_from(value: Immediate) -> Result<Self, Self::Error> {
        if let U8(ret) = value.as_u8() {
            Ok(ret)
        } else {
            Err(Fault::PrimitiveTypeMismatch)
        }
    }
}

impl TryFrom<Immediate> for u16 {
    type Error = Fault;

    fn try_from(value: Immediate) -> Result<Self, Self::Error> {
        if let U16(ret) = value.as_u16() {
            Ok(ret)
        } else {
            Err(Fault::PrimitiveTypeMismatch)
        }
    }
}

impl TryFrom<Immediate> for u32 {
    type Error = Fault;

    fn try_from(value: Immediate) -> Result<Self, Self::Error> {
        if let U32(ret) = value.as_u32() {
            Ok(ret)
        } else {
            Err(Fault::PrimitiveTypeMismatch)
        }
    }
}

impl TryFrom<Immediate> for u64 {
    type Error = Fault;

    fn try_from(value: Immediate) -> Result<Self, Self::Error> {
        if let U64(ret) = value.as_u64() {
            Ok(ret)
        } else {
            Err(Fault::PrimitiveTypeMismatch)
        }
    }
}

impl TryFrom<Immediate> for usize {
    type Error = Fault;

    fn try_from(value: Immediate) -> Result<Self, Self::Error> {
        if let Pointer(ret) = value.as_pointer() {
            Ok(ret)
        } else {
            Err(Fault::PrimitiveTypeMismatch)
        }
    }
}

impl TryFrom<Immediate> for f32 {
    type Error = Fault;

    fn try_from(value: Immediate) -> Result<Self, Self::Error> {
        if let Float(ret) = value.as_float() {
            Ok(ret)
        } else {
            Err(Fault::PrimitiveTypeMismatch)
        }
    }
}

impl TryFrom<Immediate> for f64 {
    type Error = Fault;

    fn try_from(value: Immediate) -> Result<Self, Self::Error> {
        if let Double(ret) = value.as_double() {
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

use byteorder::{BigEndian, ByteOrder};
use std::convert::TryFrom;

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

impl From<[u8; 8]> for Immediate {
    fn from(arr: [u8; 8]) -> Self {
        BigEndian::read_u64(&arr).into()
    }
}

impl Into<Immediate> for &[u8] {
    fn into(self) -> Immediate {
        let len = self.len();
        let mut buffer: u64 = 0;
        for i in 0..len {
            if i > 1 {
                buffer = buffer << 8;
            }
        }
        let imm = Immediate::from(buffer);
        imm.to_size(len as u8).unwrap()
    }
}

impl From<bool> for Immediate {
    fn from(b: bool) -> Self {
        Immediate::bool_equivalent(b)
    }
}

impl<'a> From<&'a mut Immediate> for Vec<&'a mut u8> {
    fn from(imm: &'a mut Immediate) -> Self {
        unsafe {
            match imm {
                U8(d) => vec![d],
                U16(d) => {
                    let ptr = d as *mut u16 as *mut u8;
                    vec![&mut *(ptr.add(1)), &mut *ptr]
                }
                U32(d) => {
                    let ptr = d as *mut u32 as *mut u8;
                    let mut ret = vec![];
                    for i in (0..4).rev() {
                        ret.push(&mut *(ptr.add(i)))
                    }
                    ret
                }
                U64(d) => {
                    let ptr = d as *mut u64 as *mut u8;
                    let mut ret = vec![];
                    for i in (0..8).rev() {
                        ret.push(&mut *(ptr.add(i)))
                    }
                    ret
                }
                Float(d) => {
                    let ptr = d as *mut f32 as *mut u8;
                    let mut ret = vec![];
                    for i in (0..4).rev() {
                        ret.push(&mut *(ptr.add(i)))
                    }
                    ret
                }
                Double(d) => {
                    let ptr = d as *mut f64 as *mut u8;
                    let mut ret = vec![];
                    for i in (0..8).rev() {
                        ret.push(&mut *(ptr.add(i)))
                    }
                    ret
                }
                Char(d) => {
                    let mut buff = [0; 2];
                    let mut buff = d.encode_utf16(&mut buff);
                    let mut ret = vec![];
                    let ptr = &mut buff[0] as *mut u16 as *mut u8;
                    for i in (0..2).rev() {
                        ret.push(&mut *(ptr.add(i)))
                    }
                    let ptr = &mut buff[1] as *mut u16 as *mut u8;
                    for i in (0..2).rev() {
                        ret.push(&mut *(ptr.add(i)))
                    }
                    ret
                }
                Pointer(d) => {
                    let ptr = d as *mut usize as *mut u8;
                    let mut ret = vec![];
                    for i in (0..POINTER_SIZE).rev() {
                        ret.push(&mut *(ptr.add(i)))
                    }
                    ret
                }
            }
        }
    }
}

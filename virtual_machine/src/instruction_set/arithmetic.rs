use super::*;

pub struct OverflowingResult(pub Result<(Immediate, bool), Fault>);

impl From<(u8, bool)> for OverflowingResult {
    fn from(t: (u8, bool)) -> Self {
        OverflowingResult(Ok((U8(t.0), t.1)))
    }
}

impl From<(u16, bool)> for OverflowingResult {
    fn from(t: (u16, bool)) -> Self {
        OverflowingResult(Ok((U16(t.0), t.1)))
    }
}

impl From<(u32, bool)> for OverflowingResult {
    fn from(t: (u32, bool)) -> Self {
        OverflowingResult(Ok((U32(t.0), t.1)))
    }
}

impl From<(u64, bool)> for OverflowingResult {
    fn from(t: (u64, bool)) -> Self {
        OverflowingResult(Ok((U64(t.0), t.1)))
    }
}

impl Add<Immediate> for Immediate {
    type Output = OverflowingResult;

    fn add(self, rhs: Immediate) -> Self::Output {
        match (self, rhs) {
            (U8(v1), U8(v2)) => v1.overflowing_add(v2).into(),
            (U16(v1), U16(v2)) => v1.overflowing_add(v2).into(),
            (U32(v1), U32(v2)) => v1.overflowing_add(v2).into(),
            (U64(v1), U64(v2)) => v1.overflowing_add(v2).into(),
            (Float(v1), Float(v2)) => OverflowingResult(Ok((Float(v1 + v2), false))),
            (Double(v1), Double(v2)) => OverflowingResult(Ok((Double(v1 + v2), true))),
            _ => OverflowingResult(Err(Fault::PrimitiveTypeMismatch)),
        }
    }
}

impl Sub<Immediate> for Immediate {
    type Output = OverflowingResult;

    fn sub(self, rhs: Immediate) -> Self::Output {
        match (self, rhs) {
            (U8(v1), U8(v2)) => v1.overflowing_sub(v2).into(),
            (U16(v1), U16(v2)) => v1.overflowing_sub(v2).into(),
            (U32(v1), U32(v2)) => v1.overflowing_sub(v2).into(),
            (U64(v1), U64(v2)) => v1.overflowing_sub(v2).into(),
            (Float(v1), Float(v2)) => OverflowingResult(Ok((Float(v1 - v2), false))),
            (Double(v1), Double(v2)) => OverflowingResult(Ok((Double(v1 - v2), true))),
            _ => OverflowingResult(Err(Fault::PrimitiveTypeMismatch)),
        }
    }
}

impl Mul<Immediate> for Immediate {
    type Output = OverflowingResult;

    fn mul(self, rhs: Immediate) -> Self::Output {
        match (self, rhs) {
            (U8(v1), U8(v2)) => v1.overflowing_mul(v2).into(),
            (U16(v1), U16(v2)) => v1.overflowing_mul(v2).into(),
            (U32(v1), U32(v2)) => v1.overflowing_mul(v2).into(),
            (U64(v1), U64(v2)) => v1.overflowing_mul(v2).into(),
            (Float(v1), Float(v2)) => OverflowingResult(Ok((Float(v1 * v2), false))),
            (Double(v1), Double(v2)) => OverflowingResult(Ok((Double(v1 * v2), true))),
            _ => OverflowingResult(Err(Fault::PrimitiveTypeMismatch)),
        }
    }
}

impl Div<Immediate> for Immediate {
    type Output = OverflowingResult;

    fn div(self, rhs: Immediate) -> Self::Output {
        match (self, rhs) {
            (U8(v1), U8(v2)) => v1.overflowing_div(v2).into(),
            (U16(v1), U16(v2)) => v1.overflowing_div(v2).into(),
            (U32(v1), U32(v2)) => v1.overflowing_div(v2).into(),
            (U64(v1), U64(v2)) => v1.overflowing_div(v2).into(),
            (Float(v1), Float(v2)) => OverflowingResult(Ok((Float(v1 / v2), false))),
            (Double(v1), Double(v2)) => OverflowingResult(Ok((Double(v1 / v2), true))),
            _ => OverflowingResult(Err(Fault::PrimitiveTypeMismatch)),
        }
    }
}

impl Rem<Immediate> for Immediate {
    type Output = OverflowingResult;

    fn rem(self, rhs: Immediate) -> Self::Output {
        match (self, rhs) {
            (U8(v1), U8(v2)) => v1.overflowing_rem(v2).into(),
            (U16(v1), U16(v2)) => v1.overflowing_rem(v2).into(),
            (U32(v1), U32(v2)) => v1.overflowing_rem(v2).into(),
            (U64(v1), U64(v2)) => v1.overflowing_rem(v2).into(),
            (Float(v1), Float(v2)) => OverflowingResult(Ok((Float(v1 % v2), false))),
            (Double(v1), Double(v2)) => OverflowingResult(Ok((Double(v1 % v2), true))),
            _ => OverflowingResult(Err(Fault::PrimitiveTypeMismatch)),
        }
    }
}

impl BitAnd<Immediate> for Immediate {
    type Output = ImmediateResult;

    fn bitand(self, rhs: Immediate) -> Self::Output {
        match (self, rhs) {
            (U8(v1), U8(v2)) => Ok(U8(v1 & v2)),
            (U16(v1), U16(v2)) => Ok(U16(v1 & v2)),
            (U32(v1), U32(v2)) => Ok(U32(v1 & v2)),
            (U64(v1), U64(v2)) => Ok(U64(v1 & v2)),
            _ => Err(Fault::PrimitiveTypeMismatch),
        }
    }
}

impl BitOr<Immediate> for Immediate {
    type Output = ImmediateResult;

    fn bitor(self, rhs: Immediate) -> Self::Output {
        match (self, rhs) {
            (U8(v1), U8(v2)) => Ok(U8(v1 | v2)),
            (U16(v1), U16(v2)) => Ok(U16(v1 | v2)),
            (U32(v1), U32(v2)) => Ok(U32(v1 | v2)),
            (U64(v1), U64(v2)) => Ok(U64(v1 | v2)),
            _ => Err(Fault::PrimitiveTypeMismatch),
        }
    }
}

impl BitXor<Immediate> for Immediate {
    type Output = ImmediateResult;

    fn bitxor(self, rhs: Immediate) -> Self::Output {
        match (self, rhs) {
            (U8(v1), U8(v2)) => Ok(U8(v1 ^ v2)),
            (U16(v1), U16(v2)) => Ok(U16(v1 ^ v2)),
            (U32(v1), U32(v2)) => Ok(U32(v1 ^ v2)),
            (U64(v1), U64(v2)) => Ok(U64(v1 ^ v2)),
            _ => Err(Fault::PrimitiveTypeMismatch),
        }
    }
}

impl Operation {
    pub fn perform_op(
        &self,
        flags: &mut Flags,
        val1: Immediate,
        val2: Immediate,
    ) -> Result<Immediate, Fault> {
        use Ordering::*;
        let ret = match self {
            Operation::Add => {
                let cmp1 = val1.zero_compare();
                let cmp2 = val2.zero_compare();
                let (ret, overflow): (Immediate, bool) = (val1 + val2).0?;
                flags.carry = overflow;
                flags.overflow =
                    match (cmp1, cmp2, ret.zero_compare()) {
                        (Some(Greater), Some(Greater), Some(Less))
                        | (Some(Greater), Some(Greater), Some(Equal)) => true,
                        (Some(Less), Some(Less), Some(Greater))
                        | (Some(Less), Some(Less), Some(Equal)) => true,
                        _ => false,
                    };
                flags.sign = !ret.msb();

                ret
            }
            Operation::Subtract => {
                let (ret, overflow): (Immediate, bool) = (val1 - val2).0?;
                flags.carry = overflow;
                flags.sign = !ret.msb();
                ret
            }
            Operation::Multiply => {
                let (ret, overflow): (Immediate, bool) = (val1 * val2).0?;
                flags.carry = overflow;
                flags.sign = !ret.msb();
                ret
            }
            Operation::Divide => {
                let (ret, overflow): (Immediate, bool) = (val1 / val2).0?;
                flags.carry = overflow;
                flags.sign = !ret.msb();
                ret
            }
            Operation::Remainder => {
                let (ret, overflow): (Immediate, bool) = (val1 % val2).0?;
                flags.carry = overflow;
                flags.sign = !ret.msb();
                ret
            }
            Operation::And => {
                let ret: Immediate = (val1 & val2)?;

                ret
            }
            Operation::Or => {
                let ret: Immediate = (val1 | val2)?;

                ret
            }
            Operation::Xor => {
                let ret: Immediate = (val1 ^ val2)?;

                ret
            }
        };
        flags.zero = ret.is_zero();
        Ok(ret)
    }
}

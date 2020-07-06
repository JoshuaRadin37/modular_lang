use super::*;
impl PartialEq for Immediate {
    fn eq(&self, other: &Self) -> bool {
        match (self, other) {
            (U8(v1), U8(v2)) => v1 == v2,
            (U16(v1), U16(v2)) => v1 == v2,
            (U32(v1), U32(v2)) => v1 == v2,
            (U64(v1), U64(v2)) => v1 == v2,
            (Float(v1), Float(v2)) => v1 == v2,
            (Double(v1), Double(v2)) => v1 == v2,
            (Pointer(v1), Pointer(v2)) => v1 == v2,
            _ => {
                panic!("{:?}", Fault::PrimitiveTypeMismatch);
            }
        }
    }
}

impl PartialOrd for Immediate {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        match (self, other) {
            (U8(v1), U8(v2)) => v1.partial_cmp(v2),
            (U16(v1), U16(v2)) => v1.partial_cmp(v2),
            (U32(v1), U32(v2)) => v1.partial_cmp(v2),
            (U64(v1), U64(v2)) => v1.partial_cmp(v2),
            (Float(v1), Float(v2)) => v1.partial_cmp(v2),
            (Double(v1), Double(v2)) => v1.partial_cmp(v2),
            (Pointer(v1), Pointer(v2)) => v1.partial_cmp(v2),
            _ => None,
        }
    }
}

impl PartialEq for Signed {
    fn eq(&self, other: &Self) -> bool {
        let Signed(this) = self;
        let Signed(other) = other;
        match (this, other) {
            (U8(v1), U8(v2)) => (*v1 as i8).eq(&(*v2 as i8)),
            (U16(v1), U16(v2)) => (*v1 as i16).eq(&(*v2 as i16)),
            (U32(v1), U32(v2)) => (*v1 as i32).eq(&(*v2 as i32)),
            (U64(v1), U64(v2)) => (*v1 as i64).eq(&(*v2 as i64)),
            (Float(v1), Float(v2)) => v1.eq(v2),
            (Double(v1), Double(v2)) => v1.eq(v2),
            (Pointer(v1), Pointer(v2)) => v1.eq(v2),
            _ => {
                panic!("{:?}", Fault::PrimitiveTypeMismatch);
            }
        }
    }
}

impl PartialOrd for Signed {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        let Signed(this) = self;
        let Signed(other) = other;
        match (this, other) {
            (U8(v1), U8(v2)) => (*v1 as i8).partial_cmp(&(*v2 as i8)),
            (U16(v1), U16(v2)) => (*v1 as i16).partial_cmp(&(*v2 as i16)),
            (U32(v1), U32(v2)) => (*v1 as i32).partial_cmp(&(*v2 as i32)),
            (U64(v1), U64(v2)) => (*v1 as i64).partial_cmp(&(*v2 as i64)),
            (Float(v1), Float(v2)) => v1.partial_cmp(v2),
            (Double(v1), Double(v2)) => v1.partial_cmp(v2),
            (Pointer(v1), Pointer(v2)) => v1.partial_cmp(v2),
            _ => None,
        }
    }
}

impl ComparisonOperation {
    pub fn perform_op(
        &self,
        flags: &mut Flags,
        val1: Immediate,
        val2: Immediate,
    ) -> Result<Immediate, Fault> {
        let ret = match self {
            ComparisonOperation::And => {
                let compare = !(val1.is_zero() || val2.is_zero());
                Ok(compare.into())
            }
            ComparisonOperation::Or => {
                let compare = !(val1.is_zero() && val2.is_zero());
                Ok(compare.into())
            }
            ComparisonOperation::LessThan => {
                let val1 = Signed(val1);
                let val2 = Signed(val2);
                let compare = val1.partial_cmp(&val2);
                Ok((if let Some(Ordering::Less) = compare {
                    true
                } else {
                    false
                })
                .into())
            }
            ComparisonOperation::GreaterThan => {
                let val1 = Signed(val1);
                let val2 = Signed(val2);
                let compare = val1.partial_cmp(&val2);
                Ok((if let Some(Ordering::Greater) = compare {
                    true
                } else {
                    false
                })
                .into())
            }
            ComparisonOperation::LessThanEqual => {
                let val1 = Signed(val1);
                let val2 = Signed(val2);
                let compare = val1.partial_cmp(&val2);
                Ok((if let Some(Ordering::Less) = compare {
                    true
                } else if let Some(Ordering::Equal) = compare {
                    true
                } else {
                    false
                })
                .into())
            }
            ComparisonOperation::GreaterThanEqual => {
                let val1 = Signed(val1);
                let val2 = Signed(val2);
                let compare = val1.partial_cmp(&val2);
                Ok((if let Some(Ordering::Greater) = compare {
                    true
                } else if let Some(Ordering::Equal) = compare {
                    true
                } else {
                    false
                })
                .into())
            }
            ComparisonOperation::Above => {
                let compare = val1.partial_cmp(&val2);
                Ok((if let Some(Ordering::Greater) = compare {
                    true
                } else {
                    false
                })
                .into())
            }
            ComparisonOperation::AboveEqual => {
                let compare = val1.partial_cmp(&val2);
                Ok((if let Some(Ordering::Greater) = compare {
                    true
                } else if let Some(Ordering::Equal) = compare {
                    true
                } else {
                    false
                })
                .into())
            }
            ComparisonOperation::Below => {
                let compare = val1.partial_cmp(&val2);
                Ok((if let Some(Ordering::Less) = compare {
                    true
                } else {
                    false
                })
                .into())
            }
            ComparisonOperation::BelowEqual => {
                let compare = val1.partial_cmp(&val2);
                Ok((if let Some(Ordering::Less) = compare {
                    true
                } else if let Some(Ordering::Equal) = compare {
                    true
                } else {
                    false
                })
                .into())
            }
            ComparisonOperation::Compare => {
                let sub: Immediate = (val1 - val2).0?.0;
                Ok(sub)
            }
        };
        if let Ok(imm) = ret {
            flags.zero = imm.is_zero();
            flags.sign = !imm.msb();
            flags.parity = imm.set_bits() % 2 == 0;
        }
        ret
    }
}

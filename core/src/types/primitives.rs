use crate::types::Type;

trait Primitive : PartialEq {
    fn size(&self) -> usize;
}

impl <R : Primitive> Type for R {
    fn get_size(&self) -> usize {
        self.size()
    }

    fn can_call(&self) -> bool {
        false
    }

    fn has_fields(&self) -> bool {
        false
    }

    fn has_variants(&self) -> bool {
        false
    }

    fn is_invariant<T: Type>(&self, other: &T) -> bool {
        self == other
    }

    fn is_covariant<T: Type>(&self, other: &T) -> bool {
        unimplemented!()
    }

    fn is_contravariant<T: Type>(&self, other: &T) -> bool {
        unimplemented!()
    }

    fn can_cast_to<T: Type>(&self, other: &T) -> bool {
        unimplemented!()
    }
}


#[derive(PartialEq)]
struct Char;
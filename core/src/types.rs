pub mod primitives;

pub trait Type {
    fn get_size(&self) -> usize;
    fn can_call(&self) -> bool;
    fn has_fields(&self) -> bool;
    fn has_variants(&self) -> bool;

    fn is_invariant<T: Type>(&self, other: &T) -> bool;
    fn is_covariant<T: Type>(&self, other: &T) -> bool;
    fn is_contravariant<T: Type>(&self, other: &T) -> bool;

    fn can_cast_to<T: Type>(&self, other: &T) -> bool;
}

use crate::resolution::{FullIdentifier, Resolvable, Identifier};
use crate::instruction_set::Immediate;
use std::collections::HashMap;
use crate::resolution::functions::Function;

#[derive(Clone, Debug)]
pub enum Variant {
    Tuple(Vec<Immediate>),
    Structure(HashMap<Identifier, Immediate>),
    Empty
}

pub enum MemberFunction {
    Owner(Function),
    Unowned,
    Super(FullIdentifier)
}

#[derive(Clone, Debug)]
pub struct TypeDescriptor {
    identifier: FullIdentifier,
    v_tables: Vec<HashMap<FullIdentifier, Vec<Function>>>,
    fields: HashMap<FullIdentifier, * mut Immediate>,
    variants: HashMap<Identifier, Variant>,
}

impl TypeDescriptor {



}

impl Resolvable for TypeDescriptor {
    fn get_identifier(&self) -> &FullIdentifier {
        &self.identifier
    }
}
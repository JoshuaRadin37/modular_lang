use std::collections::HashMap;
use std::sync::{Weak, Arc};

use crate::instruction_set::Immediate;
use crate::resolution::{FullIdentifier, Identifier, Resolvable};
use crate::resolution::functions::Function;

#[derive(Debug)]
pub enum StorageType {
    Variants(HashMap<Identifier, Variant>),
    Single(Variant),
    None
}

#[derive(Debug)]
pub struct TypeDescriptor {
    pub identifier: FullIdentifier,
    pub is_trait: bool,
    pub is_struct: bool,
    pub is_enum: bool,
    pub is_call: bool,
    pub v_tables: Vec<HashMap<FullIdentifier, Vec<Function>>>,
    pub parents: Vec<Weak<TypeDescriptor>>,
    pub parent_data: HashMap<FullIdentifier, Variant>,
    pub variants: StorageType,
}

impl TypeDescriptor {

    pub fn implements_trait(&self, name: &FullIdentifier) -> bool {
        if self.is_trait && self.get_identifier() == name {
            return true;
        }
        for parent in self.parents {
            let strong: Arc<TypeDescriptor> = parent.upgrade().unwrap();

            if strong.implements_trait(name) {
                return true;
            }
        }

        false
    }

    pub fn is_instance_of(&self, name: &FullIdentifier) -> bool {
        if self.get_identifier() == name {
            return true;
        }

        for parent in self.parents {
            let strong: Arc<TypeDescriptor> = parent.upgrade().unwrap();

            if strong.is_instance_of(name) {
                return true;
            }
        }

        false
    }
}


impl Resolvable for TypeDescriptor {
    fn get_identifier(&self) -> &FullIdentifier {
        &self.identifier
    }
}

#[derive(Clone, Debug)]
pub enum Variant {
    Tuple(Vec<Immediate>),
    Structure { order: Vec<Identifier>, fields: HashMap<Identifier, Immediate> },
    Empty,
}

pub enum MemberFunction {
    Owner(Function),
    Unowned,
    Super(FullIdentifier),
}


use std::collections::HashMap;
use std::sync::{Weak, Arc};

use crate::instruction_set::Immediate;
use crate::intrinsics::simplification::{Simplifier, TupleMember};
use crate::resolution::types::descriptor::{TypeDescriptor, Variant};
use crate::resolution::{FullIdentifier, Resolvable};
use crate::vm::Fault;

pub mod descriptor;

#[derive(Clone, Debug)]
pub struct TypedObject {
    self_variant: Variant,
    parent_variants: HashMap<FullIdentifier, Variant>,
    descriptor: Weak<TypeDescriptor>,
}

impl TypedObject {
    pub fn new(
        self_variant: Variant,
        parent_variants: HashMap<FullIdentifier, Variant>,
        descriptor: Weak<TypeDescriptor>,
    ) -> Self {
        TypedObject {
            self_variant,
            parent_variants,
            descriptor,
        }
    }

    pub fn get_self_variant(&self) -> &Variant {
        &self.self_variant
    }

    pub fn get_self_variant_mut(&mut self) -> &mut Variant {
        &mut self.self_variant
    }

    pub fn get_parent_variant(&self, parent: &FullIdentifier) -> &Variant {
        &self.parent_variants[parent]
    }

    pub fn get_parent_variant_mut(&mut self, parent: &FullIdentifier) -> &mut Variant {
        self.parent_variants.get_mut(parent).unwrap()
    }

    pub fn get_descriptor(&self) -> Arc<TypeDescriptor> {
        self.descriptor.upgrade().unwrap()
    }

    pub fn get_field(&self, identifier: &FullIdentifier) -> Result<&Immediate, Fault> {
        if identifier == self.get_descriptor().get_identifier() {
            let desc = self.get_descriptor();
            let field_name = desc.get_identifier().get_name();
            return self.get_field(&field_name.clone().into());
        }
        match identifier.clone() {
            FullIdentifier::Name(inner_field) => match &self.self_variant {
                Variant::Tuple(tuple) => {
                    let member = TupleMember.unsimplified(inner_field);
                    tuple.get(member).ok_or(Fault::InvalidField)
                }
                Variant::Structure { order: _, fields } => {
                    fields.get(&inner_field).ok_or(Fault::InvalidField)
                }
                Variant::Empty => Err(Fault::InvalidField),
            },
            full => {
                let key: &FullIdentifier = self
                    .parent_variants
                    .keys()
                    .find(|other_namespace| other_namespace.is_sub_identifier_of(&full))
                    .ok_or(Fault::InvalidField)?;

                unimplemented!()
            }
        }
    }

    pub fn get_field_mut(&mut self, identifier: &FullIdentifier) -> Result<&mut Immediate, Fault> {
        if identifier == self.get_descriptor().get_identifier() {
            let desc = self.get_descriptor();
            let field_name = desc.get_identifier().get_name();
            return self.get_field_mut(&field_name.clone().into());
        }
        match identifier.clone() {
            FullIdentifier::Name(inner_field) => match &mut self.self_variant {
                Variant::Tuple(tuple) => {
                    let member = TupleMember.unsimplified(inner_field);
                    tuple.get_mut(member).ok_or(Fault::InvalidField)
                }
                Variant::Structure { order: _, fields } => {
                    fields.get_mut(&inner_field).ok_or(Fault::InvalidField)
                }
                Variant::Empty => Err(Fault::InvalidField),
            },
            full => {
                let key: &FullIdentifier = self
                    .parent_variants
                    .keys()
                    .find(|other_namespace| other_namespace.is_sub_identifier_of(&full))
                    .ok_or(Fault::InvalidField)?;

                unimplemented!()
            }
        }
    }
}

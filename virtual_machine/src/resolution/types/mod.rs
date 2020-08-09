use crate::resolution::types::descriptor::{Variant, TypeDescriptor};
use std::collections::HashMap;
use crate::resolution::FullIdentifier;
use std::sync::Weak;

pub mod descriptor;

#[derive(Clone, Debug)]
pub struct TypedObject {
    self_variant: Variant,
    parent_variants: HashMap<FullIdentifier, Variant>,
    descriptor: Weak<TypeDescriptor>
}


impl TypedObject {

    pub fn new(self_variant: Variant, parent_variants: HashMap<FullIdentifier, Variant>, descriptor: Weak<TypeDescriptor>) -> Self {
        TypedObject { self_variant, parent_variants, descriptor }
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
        &mut self.parent_variants[parent]
    }

    pub fn get_descriptor(&self) -> &TypeDescriptor {
        let upgrade = self.descriptor.upgrade().unwrap();
        &*upgrade
    }


}

use crate::resolution::FullIdentifier;
use std::collections::HashMap;

lazy_static! {
    pub static ref MARKER_TRAITS: HashMap<String, FullIdentifier> = HashMap::new();
}

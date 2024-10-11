use indexmap::IndexMap;

use crate::validators::{identifier::NenyrIdentifierValidator, property::NenyrPropertyValidator};

pub struct NenyrAliases {
    values: IndexMap<String, String>,
}

impl NenyrIdentifierValidator for NenyrAliases {}
impl NenyrPropertyValidator for NenyrAliases {}

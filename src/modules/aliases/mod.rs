use indexmap::IndexMap;

use crate::{
    converters::property::NenyrPropertyConverter, validators::identifier::NenyrIdentifierValidator,
};

#[derive(Debug, PartialEq, Clone)]
pub struct NenyrAliases {
    values: IndexMap<String, String>,
}

impl NenyrIdentifierValidator for NenyrAliases {}
impl NenyrPropertyConverter for NenyrAliases {}

impl NenyrAliases {
    pub fn new() -> Self {
        Self {
            values: IndexMap::new(),
        }
    }

    pub fn process_aliases(&self) {}
}

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

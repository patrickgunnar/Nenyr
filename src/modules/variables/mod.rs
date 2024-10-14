use indexmap::IndexMap;

use crate::validators::{
    identifier::NenyrIdentifierValidator, variable_value::NenyrVariableValueValidator,
};

#[derive(Debug, PartialEq, Clone)]
pub struct NenyrVariables {
    pub values: IndexMap<String, String>,
}

impl NenyrIdentifierValidator for NenyrVariables {}
impl NenyrVariableValueValidator for NenyrVariables {}

impl NenyrVariables {
    pub fn new() -> Self {
        Self {
            values: IndexMap::new(),
        }
    }

    pub fn process_variables(&self) {}
}

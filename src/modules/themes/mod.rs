use indexmap::IndexMap;

use crate::validators::{
    identifier::NenyrIdentifierValidator, variable_value::NenyrVariableValueValidator,
};

#[derive(Debug, PartialEq, Clone)]
pub struct NenyrThemes {
    light_schema: Option<IndexMap<String, String>>,
    dark_schema: Option<IndexMap<String, String>>,
}

impl NenyrIdentifierValidator for NenyrThemes {}
impl NenyrVariableValueValidator for NenyrThemes {}

impl NenyrThemes {
    pub fn new() -> Self {
        Self {
            light_schema: None,
            dark_schema: None,
        }
    }

    pub fn process_themes(&self) {}
}

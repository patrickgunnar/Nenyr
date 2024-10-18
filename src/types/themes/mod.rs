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

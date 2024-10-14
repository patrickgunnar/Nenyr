use indexmap::IndexMap;

use crate::validators::{identifier::NenyrIdentifierValidator, typeface::NenyrTypefaceValidator};

#[derive(Debug, PartialEq, Clone)]
pub struct NenyrTypefaces {
    values: IndexMap<String, String>,
}

impl NenyrIdentifierValidator for NenyrTypefaces {}
impl NenyrTypefaceValidator for NenyrTypefaces {}

impl NenyrTypefaces {
    pub fn new() -> Self {
        Self {
            values: IndexMap::new(),
        }
    }

    pub fn process_typefaces(&self) {}
}

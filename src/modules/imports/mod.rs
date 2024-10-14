use std::collections::HashSet;

use crate::validators::import::NenyrImportValidator;

#[derive(Debug, PartialEq, Clone)]
pub struct NenyrImports {
    values: HashSet<String>,
}

impl NenyrImportValidator for NenyrImports {}

impl NenyrImports {
    pub fn new() -> Self {
        Self {
            values: HashSet::new(),
        }
    }

    pub fn process_imports(&self) {}
}

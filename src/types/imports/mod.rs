use std::collections::HashSet;

use crate::validators::import::NenyrImportValidator;

#[derive(Debug, PartialEq, Clone)]
pub struct NenyrImports {
    values: HashSet<String>,
}

impl NenyrImportValidator for NenyrImports {}

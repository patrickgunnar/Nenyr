use std::collections::HashSet;

use crate::validators::import::NenyrImportValidator;

pub struct NenyrImports {
    values: HashSet<String>,
}

impl NenyrImportValidator for NenyrImports {}

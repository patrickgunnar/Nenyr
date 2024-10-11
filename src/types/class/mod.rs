use indexmap::IndexMap;

use crate::validators::{
    identifier::NenyrIdentifierValidator, property::NenyrPropertyValidator,
    style_pattern::NenyrStylePatternValidator, style_syntax::NenyrStyleSyntaxValidator,
};

#[derive(Debug, PartialEq, Clone)]
pub struct NenyrStyleClass {
    class_name: String,
    deriving_from: Option<String>,
    is_important: Option<bool>,

    style_patterns: Option<IndexMap<String, IndexMap<String, String>>>,
    responsive_patterns: Option<IndexMap<String, IndexMap<String, IndexMap<String, String>>>>,
}

impl NenyrIdentifierValidator for NenyrStyleClass {}
impl NenyrPropertyValidator for NenyrStyleClass {}
impl NenyrStyleSyntaxValidator for NenyrStyleClass {}
impl NenyrStylePatternValidator for NenyrStyleClass {}

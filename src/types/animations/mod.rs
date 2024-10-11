use indexmap::IndexMap;

use crate::validators::{
    i64_vector::NenyrF64Validator, identifier::NenyrIdentifierValidator,
    property::NenyrPropertyValidator, style_syntax::NenyrStyleSyntaxValidator,
};

#[derive(Debug, PartialEq, Clone)]
pub struct NenyrKeyframeFraction {
    fraction_stops: Vec<i64>,
    fraction_properties: Option<IndexMap<String, String>>,
}

#[derive(Debug, PartialEq, Clone)]
pub struct NenyrAnimation {
    animation_name: String,
    progressive_count: Option<i64>,

    fraction_keyframe: Option<Vec<NenyrKeyframeFraction>>,
    progressive_keyframe: Option<Vec<IndexMap<String, String>>>,

    from_keyframe: Option<IndexMap<String, String>>,
    halfway_keyframe: Option<IndexMap<String, String>>,
    to_keyframe: Option<IndexMap<String, String>>,
}

impl NenyrIdentifierValidator for NenyrAnimation {}
impl NenyrPropertyValidator for NenyrAnimation {}
impl NenyrStyleSyntaxValidator for NenyrAnimation {}

impl NenyrPropertyValidator for NenyrKeyframeFraction {}
impl NenyrStyleSyntaxValidator for NenyrKeyframeFraction {}
impl NenyrF64Validator for NenyrKeyframeFraction {}

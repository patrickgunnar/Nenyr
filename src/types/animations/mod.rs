use indexmap::IndexMap;

use crate::validators::{
    i64_vector::NenyrI64Validator, identifier::NenyrIdentifierValidator,
    property::NenyrPropertyValidator, style_syntax::NenyrStyleSyntaxValidator,
};

pub struct NenyrKeyframeFraction {
    fraction_stops: Vec<i64>,
    fraction_properties: Option<IndexMap<String, String>>,
}

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
impl NenyrI64Validator for NenyrKeyframeFraction {}

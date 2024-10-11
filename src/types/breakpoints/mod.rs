use indexmap::IndexMap;

use crate::validators::{
    breakpoint::NenyrBreakpointValidator, identifier::NenyrIdentifierValidator,
};

pub struct NenyrBreakpoints {
    mobile_first: Option<IndexMap<String, String>>,
    desktop_first: Option<IndexMap<String, String>>,
}

impl NenyrIdentifierValidator for NenyrBreakpoints {}
impl NenyrBreakpointValidator for NenyrBreakpoints {}

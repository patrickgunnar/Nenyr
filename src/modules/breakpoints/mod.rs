use indexmap::IndexMap;

use crate::validators::{
    breakpoint::NenyrBreakpointValidator, identifier::NenyrIdentifierValidator,
};

#[derive(Debug, PartialEq, Clone)]
pub struct NenyrBreakpoints {
    mobile_first: Option<IndexMap<String, String>>,
    desktop_first: Option<IndexMap<String, String>>,
}

impl NenyrIdentifierValidator for NenyrBreakpoints {}
impl NenyrBreakpointValidator for NenyrBreakpoints {}

impl NenyrBreakpoints {
    pub fn new() -> Self {
        Self {
            mobile_first: None,
            desktop_first: None,
        }
    }

    pub fn process_breakpoints(&self) {}
}

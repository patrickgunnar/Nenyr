use indexmap::IndexMap;

use crate::creators::{
    aliases::NenyrAliasesCreator, animation::NenyrAnimationCreator,
    breakpoints::NenyrBreakpointsCreator, class::NenyrClassCreator, imports::NenyrImportsCreator,
    themes::NenyrThemesCreator, typefaces::NenyrTypefacesCreator, variables::NenyrVariablesCreator,
};

use super::{
    aliases::NenyrAliases, animations::NenyrAnimation, breakpoints::NenyrBreakpoints,
    class::NenyrStyleClass, imports::NenyrImports, themes::NenyrThemes, typefaces::NenyrTypefaces,
    variables::NenyrVariables,
};

#[derive(Debug, PartialEq, Clone)]
pub struct CentralContext {
    imports: Option<NenyrImports>,
    typefaces: Option<NenyrTypefaces>,
    breakpoints: Option<NenyrBreakpoints>,
    aliases: Option<NenyrAliases>,
    variables: Option<NenyrVariables>,
    themes: Option<NenyrThemes>,
    animations: Option<IndexMap<String, NenyrAnimation>>,
    classes: Option<IndexMap<String, NenyrStyleClass>>,
}

impl NenyrAliasesCreator for CentralContext {}
impl NenyrVariablesCreator for CentralContext {}
impl NenyrAnimationCreator for CentralContext {}
impl NenyrClassCreator for CentralContext {}
impl NenyrThemesCreator for CentralContext {}
impl NenyrImportsCreator for CentralContext {}
impl NenyrTypefacesCreator for CentralContext {}
impl NenyrBreakpointsCreator for CentralContext {}

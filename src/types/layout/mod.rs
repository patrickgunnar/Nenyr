use indexmap::IndexMap;

use crate::{
    creators::{
        aliases::NenyrAliasesCreator, animation::NenyrAnimationCreator, class::NenyrClassCreator,
        themes::NenyrThemesCreator, variables::NenyrVariablesCreator,
    },
    validators::identifier::NenyrIdentifierValidator,
};

use super::{
    aliases::NenyrAliases, animations::NenyrAnimation, class::NenyrStyleClass, themes::NenyrThemes,
    variables::NenyrVariables,
};

#[derive(Debug, PartialEq, Clone)]
pub struct LayoutContext {
    layout_name: String,
    aliases: Option<NenyrAliases>,
    variables: Option<NenyrVariables>,
    themes: Option<NenyrThemes>,
    animations: Option<IndexMap<String, NenyrAnimation>>,
    classes: Option<IndexMap<String, NenyrStyleClass>>,
}

impl NenyrIdentifierValidator for LayoutContext {}

impl NenyrAliasesCreator for LayoutContext {}
impl NenyrVariablesCreator for LayoutContext {}
impl NenyrAnimationCreator for LayoutContext {}
impl NenyrClassCreator for LayoutContext {}
impl NenyrThemesCreator for LayoutContext {}

impl LayoutContext {
    pub fn new(layout_name: String) -> Self {
        Self {
            layout_name,
            aliases: None,
            variables: None,
            themes: None,
            animations: None,
            classes: None,
        }
    }
}

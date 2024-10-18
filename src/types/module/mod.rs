use indexmap::IndexMap;

use crate::{
    creators::{
        aliases::NenyrAliasesCreator, animation::NenyrAnimationCreator, class::NenyrClassCreator,
        variables::NenyrVariablesCreator,
    },
    validators::identifier::NenyrIdentifierValidator,
};

use super::{
    aliases::NenyrAliases, animations::NenyrAnimation, class::NenyrStyleClass,
    variables::NenyrVariables,
};

#[derive(Debug, PartialEq, Clone)]
pub struct ModuleContext {
    module_name: String,
    extending_from: Option<String>,
    aliases: Option<NenyrAliases>,
    variables: Option<NenyrVariables>,
    animations: Option<IndexMap<String, NenyrAnimation>>,
    classes: Option<IndexMap<String, NenyrStyleClass>>,
}

impl NenyrIdentifierValidator for ModuleContext {}

impl NenyrAliasesCreator for ModuleContext {}
impl NenyrVariablesCreator for ModuleContext {}
impl NenyrAnimationCreator for ModuleContext {}
impl NenyrClassCreator for ModuleContext {}

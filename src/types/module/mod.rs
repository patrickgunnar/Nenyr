use indexmap::IndexMap;

use super::{
    aliases::NenyrAliases, animations::NenyrAnimation, class::NenyrStyleClass,
    variables::NenyrVariables,
};

#[derive(Debug, PartialEq, Clone)]
pub struct ModuleContext {
    pub module_name: String,
    pub extending_from: Option<String>,
    pub aliases: Option<NenyrAliases>,
    pub variables: Option<NenyrVariables>,
    pub animations: Option<IndexMap<String, NenyrAnimation>>,
    pub classes: Option<IndexMap<String, NenyrStyleClass>>,
}

impl ModuleContext {
    pub fn new(module_name: String, extending_from: Option<String>) -> Self {
        Self {
            module_name,
            extending_from,
            aliases: None,
            variables: None,
            animations: None,
            classes: None,
        }
    }

    pub(crate) fn add_style_class_to_context(
        &mut self,
        class_name: String,
        style_class: NenyrStyleClass,
    ) {
        if self.classes == None {
            self.classes = Some(IndexMap::new());
        }

        if let Some(classes) = &mut self.classes {
            classes.insert(class_name, style_class);
        }
    }

    pub(crate) fn add_animation_to_context(
        &mut self,
        animation_name: String,
        animation: NenyrAnimation,
    ) {
        if self.animations == None {
            self.animations = Some(IndexMap::new());
        }

        if let Some(animations) = &mut self.animations {
            animations.insert(animation_name, animation);
        }
    }

    pub(crate) fn add_variables_to_context(&mut self, variables: NenyrVariables) {
        self.variables = Some(variables);
    }

    pub(crate) fn add_aliases_to_context(&mut self, aliases: NenyrAliases) {
        self.aliases = Some(aliases);
    }
}

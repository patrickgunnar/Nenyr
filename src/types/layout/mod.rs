use indexmap::IndexMap;

use super::{
    aliases::NenyrAliases, animations::NenyrAnimation, class::NenyrStyleClass, themes::NenyrThemes,
    variables::NenyrVariables,
};

#[derive(Debug, PartialEq, Clone)]
pub struct LayoutContext {
    pub layout_name: String,
    pub aliases: Option<NenyrAliases>,
    pub variables: Option<NenyrVariables>,
    pub themes: Option<NenyrThemes>,
    pub animations: Option<IndexMap<String, NenyrAnimation>>,
    pub classes: Option<IndexMap<String, NenyrStyleClass>>,
}

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

use indexmap::IndexMap;

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

impl CentralContext {
    pub fn new() -> Self {
        Self {
            imports: None,
            typefaces: None,
            breakpoints: None,
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
}

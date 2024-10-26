use indexmap::IndexMap;

use super::{
    aliases::NenyrAliases, animations::NenyrAnimation, class::NenyrStyleClass,
    variables::NenyrVariables,
};

/// Represents the context for a module within the Nenyr framework.
///
/// The `ModuleContext` struct encapsulates all the styling elements relevant to a specific module,
/// including aliases, variables, animations, and style classes. It also allows for the
/// specification of a module that this module extends, supporting a modular and reusable
/// design within the Nenyr framework.
///
/// # Fields
/// - `module_name`: A `String` representing the name of the module.
/// - `extending_from`: An optional `String` that indicates the name of another module that
///   this module extends. This allows for inheritance of styles and properties from the
///   specified module.
/// - `aliases`: An optional collection of style aliases specific to this module context.
/// - `variables`: An optional collection of style variables specific to this module context.
/// - `animations`: An optional `IndexMap` that maps animation names to `NenyrAnimation` instances.
/// - `classes`: An optional `IndexMap` that maps class names to `NenyrStyleClass` instances.
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
    /// Creates a new `ModuleContext` with the specified module name and optional extending module.
    ///
    /// # Parameters
    /// - `module_name`: A `String` that defines the name of the module.
    /// - `extending_from`: An optional `String` that specifies the name of the module being extended.
    ///
    /// # Returns
    /// A new instance of `ModuleContext` initialized with the given module name, extending module,
    /// and all other fields set to `None`.
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

    /// Adds a style class to the module context.
    ///
    /// This method will insert a `NenyrStyleClass` into the `classes` field,
    /// creating the `IndexMap` if it does not already exist.
    ///
    /// # Parameters
    /// - `class_name`: A `String` representing the name of the style class to be added.
    /// - `style_class`: A `NenyrStyleClass` instance that defines the styles for the class.
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

    /// Adds an animation to the module context.
    ///
    /// This method will insert a `NenyrAnimation` into the `animations` field,
    /// creating the `IndexMap` if it does not already exist.
    ///
    /// # Parameters
    /// - `animation_name`: A `String` representing the name of the animation to be added.
    /// - `animation`: A `NenyrAnimation` instance that defines the animation properties.
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

    /// Adds a collection of variables to the module context.
    ///
    /// This method will set the `variables` field to the provided `NenyrVariables`.
    ///
    /// # Parameters
    /// - `variables`: A `NenyrVariables` instance representing the style variables to be added.
    pub(crate) fn add_variables_to_context(&mut self, variables: NenyrVariables) {
        self.variables = Some(variables);
    }

    /// Adds a collection of aliases to the module context.
    ///
    /// This method will set the `aliases` field to the provided `NenyrAliases`.
    ///
    /// # Parameters
    /// - `aliases`: A `NenyrAliases` instance representing the style aliases to be added.
    pub(crate) fn add_aliases_to_context(&mut self, aliases: NenyrAliases) {
        self.aliases = Some(aliases);
    }
}

#[cfg(test)]
mod tests {
    use crate::types::{
        aliases::NenyrAliases, animations::NenyrAnimation, class::NenyrStyleClass,
        module::ModuleContext, variables::NenyrVariables,
    };

    #[test]
    fn test_new_module_context() {
        let module_name = String::from("TestModule");
        let extending_from = Some(String::from("BaseModule"));
        let context = ModuleContext::new(module_name.clone(), extending_from.clone());

        assert_eq!(context.module_name, module_name);
        assert_eq!(context.extending_from, extending_from);
        assert!(context.aliases.is_none());
        assert!(context.variables.is_none());
        assert!(context.animations.is_none());
        assert!(context.classes.is_none());
    }

    #[test]
    fn test_add_style_class_to_context() {
        let mut context = ModuleContext::new(String::from("TestModule"), None);
        let style_class = NenyrStyleClass::new("myClass".to_string(), None);

        context.add_style_class_to_context(String::from("myClass"), style_class.clone());

        assert!(context.classes.is_some());
        let classes = context.classes.as_ref().unwrap();
        assert_eq!(classes.len(), 1);
        assert!(classes.contains_key("myClass"));
        assert_eq!(classes.get("myClass"), Some(&style_class));
    }

    #[test]
    fn test_add_animation_to_context() {
        let mut context = ModuleContext::new(String::from("TestModule"), None);
        let animation = NenyrAnimation::new("fadeIn".to_string());

        context.add_animation_to_context(String::from("fadeIn"), animation.clone());

        assert!(context.animations.is_some());
        let animations = context.animations.as_ref().unwrap();
        assert_eq!(animations.len(), 1);
        assert!(animations.contains_key("fadeIn"));
        assert_eq!(animations.get("fadeIn"), Some(&animation));
    }

    #[test]
    fn test_add_variables_to_context() {
        let mut context = ModuleContext::new(String::from("TestModule"), None);
        let variables = NenyrVariables::new();

        context.add_variables_to_context(variables.clone());

        assert!(context.variables.is_some());
        assert_eq!(context.variables.as_ref().unwrap(), &variables);
    }

    #[test]
    fn test_add_aliases_to_context() {
        let mut context = ModuleContext::new(String::from("TestModule"), None);
        let aliases = NenyrAliases::new();

        context.add_aliases_to_context(aliases.clone());

        assert!(context.aliases.is_some());
        assert_eq!(context.aliases.as_ref().unwrap(), &aliases);
    }
}

use indexmap::IndexMap;

use super::{
    aliases::NenyrAliases, animations::NenyrAnimation, breakpoints::NenyrBreakpoints,
    class::NenyrStyleClass, imports::NenyrImports, themes::NenyrThemes, typefaces::NenyrTypefaces,
    variables::NenyrVariables,
};

/// Represents the central context for the Nenyr styling system.
///
/// The `CentralContext` struct aggregates various elements essential for
/// styling in the Nenyr framework. It encompasses imports, typefaces,
/// breakpoints, aliases, variables, themes, animations, and style classes.
///
/// Each field within the struct is optional, indicating that the context
/// can be incrementally built up as needed during the styling process.
#[derive(Debug, PartialEq, Clone)]
pub struct CentralContext {
    pub imports: Option<NenyrImports>,
    pub typefaces: Option<NenyrTypefaces>,
    pub breakpoints: Option<NenyrBreakpoints>,
    pub aliases: Option<NenyrAliases>,
    pub variables: Option<NenyrVariables>,
    pub themes: Option<NenyrThemes>,
    pub animations: Option<IndexMap<String, NenyrAnimation>>,
    pub classes: Option<IndexMap<String, NenyrStyleClass>>,
}

impl CentralContext {
    /// Creates a new instance of `CentralContext`.
    ///
    /// This constructor initializes the struct with all fields set to `None`,
    /// allowing for a fresh start in defining a styling context.
    ///
    /// # Returns
    ///
    /// A `CentralContext` with all fields initialized to `None`.
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

    /// Adds a style class to the context.
    ///
    /// This method inserts a new `NenyrStyleClass` into the `classes`
    /// map within the `CentralContext`. If the `classes` map is not
    /// initialized, it creates a new `IndexMap` before inserting the
    /// style class.
    ///
    /// # Parameters
    ///
    /// - `class_name`: A `String` representing the name of the style class.
    /// - `style_class`: The `NenyrStyleClass` instance to be added to the context.
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

    /// Adds an animation to the context.
    ///
    /// This method inserts a new `NenyrAnimation` into the `animations`
    /// map within the `CentralContext`. If the `animations` map is not
    /// initialized, it creates a new `IndexMap` before adding the animation.
    ///
    /// # Parameters
    ///
    /// - `animation_name`: A `String` representing the name of the animation.
    /// - `animation`: The `NenyrAnimation` instance to be added to the context.
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

    /// Adds variables to the context.
    ///
    /// This method sets the `variables` field in the `CentralContext`
    /// with the provided `NenyrVariables`.
    ///
    /// # Parameters
    ///
    /// - `variables`: The `NenyrVariables` instance to be added to the context.
    pub(crate) fn add_variables_to_context(&mut self, variables: NenyrVariables) {
        self.variables = Some(variables);
    }

    /// Adds aliases to the context.
    ///
    /// This method sets the `aliases` field in the `CentralContext`
    /// with the provided `NenyrAliases`.
    ///
    /// # Parameters
    ///
    /// - `aliases`: The `NenyrAliases` instance to be added to the context.
    pub(crate) fn add_aliases_to_context(&mut self, aliases: NenyrAliases) {
        self.aliases = Some(aliases);
    }

    /// Adds typefaces to the context.
    ///
    /// This method sets the `typefaces` field in the `CentralContext`
    /// with the provided `NenyrTypefaces`.
    ///
    /// # Parameters
    ///
    /// - `typefaces`: The `NenyrTypefaces` instance to be added to the context.
    pub(crate) fn add_typefaces_to_context(&mut self, typefaces: NenyrTypefaces) {
        self.typefaces = Some(typefaces);
    }

    /// Adds imports to the context.
    ///
    /// This method sets the `imports` field in the `CentralContext`
    /// with the provided `NenyrImports`.
    ///
    /// # Parameters
    ///
    /// - `imports`: The `NenyrImports` instance to be added to the context.
    pub(crate) fn add_imports_to_context(&mut self, imports: NenyrImports) {
        self.imports = Some(imports);
    }

    /// Adds themes to the context.
    ///
    /// This method sets the `themes` field in the `CentralContext`
    /// with the provided `NenyrThemes`.
    ///
    /// # Parameters
    ///
    /// - `themes`: The `NenyrThemes` instance to be added to the context.
    pub(crate) fn add_themes_to_context(&mut self, themes: NenyrThemes) {
        self.themes = Some(themes);
    }

    /// Adds breakpoints to the context.
    ///
    /// This method sets the `breakpoints` field in the `CentralContext`
    /// with the provided `NenyrBreakpoints`.
    ///
    /// # Parameters
    ///
    /// - `breakpoints`: The `NenyrBreakpoints` instance to be added to the context.
    pub(crate) fn add_breakpoints_to_context(&mut self, breakpoints: NenyrBreakpoints) {
        self.breakpoints = Some(breakpoints);
    }
}

#[cfg(test)]
mod tests {
    use crate::types::{
        aliases::NenyrAliases, animations::NenyrAnimation, breakpoints::NenyrBreakpoints,
        central::CentralContext, class::NenyrStyleClass, imports::NenyrImports,
        typefaces::NenyrTypefaces, variables::NenyrVariables,
    };

    #[test]
    fn test_new_central_context() {
        let context = CentralContext::new();
        assert!(context.imports.is_none());
        assert!(context.typefaces.is_none());
        assert!(context.breakpoints.is_none());
        assert!(context.aliases.is_none());
        assert!(context.variables.is_none());
        assert!(context.themes.is_none());
        assert!(context.animations.is_none());
        assert!(context.classes.is_none());
    }

    #[test]
    fn test_add_style_class_to_context() {
        let mut context = CentralContext::new();
        let style_class = NenyrStyleClass::new("myClass".to_string(), None);

        context.add_style_class_to_context("myClass".to_string(), style_class.clone());

        assert!(context.classes.is_some());
        let classes = context.classes.as_ref().unwrap();

        assert_eq!(classes.len(), 1);
        assert!(classes.contains_key("myClass"));
        assert_eq!(classes["myClass"], style_class);
    }

    #[test]
    fn test_add_animation_to_context() {
        let mut context = CentralContext::new();
        let animation = NenyrAnimation::new("fadeIn".to_string());

        context.add_animation_to_context("fadeIn".to_string(), animation.clone());

        assert!(context.animations.is_some());
        let animations = context.animations.as_ref().unwrap();

        assert_eq!(animations.len(), 1);
        assert!(animations.contains_key("fadeIn"));
        assert_eq!(animations["fadeIn"], animation);
    }

    #[test]
    fn test_add_variables_to_context() {
        let mut context = CentralContext::new();
        let variables = NenyrVariables::new();

        context.add_variables_to_context(variables.clone());

        assert!(context.variables.is_some());
        assert_eq!(context.variables.as_ref().unwrap(), &variables);
    }

    #[test]
    fn test_add_aliases_to_context() {
        let mut context = CentralContext::new();
        let aliases = NenyrAliases::new();

        context.add_aliases_to_context(aliases.clone());

        assert!(context.aliases.is_some());
        assert_eq!(context.aliases.as_ref().unwrap(), &aliases);
    }

    #[test]
    fn test_add_typefaces_to_context() {
        let mut context = CentralContext::new();
        let typefaces = NenyrTypefaces::new();

        context.add_typefaces_to_context(typefaces.clone());

        assert!(context.typefaces.is_some());
        assert_eq!(context.typefaces.as_ref().unwrap(), &typefaces);
    }

    #[test]
    fn test_add_imports_to_context() {
        let mut context = CentralContext::new();
        let imports = NenyrImports::new();

        context.add_imports_to_context(imports.clone());

        assert!(context.imports.is_some());
        assert_eq!(context.imports.as_ref().unwrap(), &imports);
    }

    #[test]
    fn test_add_breakpoints_to_context() {
        let mut context = CentralContext::new();
        let breakpoints = NenyrBreakpoints::new();

        context.add_breakpoints_to_context(breakpoints.clone());

        assert!(context.breakpoints.is_some());
        assert_eq!(context.breakpoints.as_ref().unwrap(), &breakpoints);
    }
}

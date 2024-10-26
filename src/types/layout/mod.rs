use indexmap::IndexMap;

use super::{
    aliases::NenyrAliases, animations::NenyrAnimation, class::NenyrStyleClass, themes::NenyrThemes,
    variables::NenyrVariables,
};

/// Represents the context for a layout within the Nenyr framework.
///
/// The `LayoutContext` struct encapsulates all the styling elements relevant to a specific layout,
/// such as aliases, variables, themes, animations, and style classes. It allows for the dynamic
/// addition of styles and related properties as needed.
///
/// # Fields
/// - `layout_name`: A `String` representing the name of the layout.
/// - `aliases`: An optional collection of style aliases specific to this layout context.
/// - `variables`: An optional collection of style variables specific to this layout context.
/// - `themes`: An optional collection of themes associated with this layout context.
/// - `animations`: An optional `IndexMap` that maps animation names to `NenyrAnimation` instances.
/// - `classes`: An optional `IndexMap` that maps class names to `NenyrStyleClass` instances.
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
    /// Creates a new `LayoutContext` with the specified layout name.
    ///
    /// # Parameters
    /// - `layout_name`: A `String` that defines the name of the layout.
    ///
    /// # Returns
    /// A new instance of `LayoutContext` initialized with the given layout name and
    /// all other fields set to `None`.
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

    /// Adds a style class to the layout context.
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

    /// Adds an animation to the layout context.
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

    /// Adds a collection of variables to the layout context.
    ///
    /// This method will set the `variables` field to the provided `NenyrVariables`.
    ///
    /// # Parameters
    /// - `variables`: A `NenyrVariables` instance representing the style variables to be added.
    pub(crate) fn add_variables_to_context(&mut self, variables: NenyrVariables) {
        self.variables = Some(variables);
    }

    /// Adds a collection of aliases to the layout context.
    ///
    /// This method will set the `aliases` field to the provided `NenyrAliases`.
    ///
    /// # Parameters
    /// - `aliases`: A `NenyrAliases` instance representing the style aliases to be added.
    pub(crate) fn add_aliases_to_context(&mut self, aliases: NenyrAliases) {
        self.aliases = Some(aliases);
    }

    /// Adds a collection of themes to the layout context.
    ///
    /// This method will set the `themes` field to the provided `NenyrThemes`.
    ///
    /// # Parameters
    /// - `themes`: A `NenyrThemes` instance representing the themes to be added.
    pub(crate) fn add_themes_to_context(&mut self, themes: NenyrThemes) {
        self.themes = Some(themes);
    }
}

#[cfg(test)]
mod tests {
    use crate::types::{
        aliases::NenyrAliases, animations::NenyrAnimation, class::NenyrStyleClass,
        layout::LayoutContext, themes::NenyrThemes, variables::NenyrVariables,
    };

    #[test]
    fn test_layout_context_creation() {
        let layout_context = LayoutContext::new("TestLayout".to_string());
        assert_eq!(layout_context.layout_name, "TestLayout");
        assert!(layout_context.aliases.is_none());
        assert!(layout_context.variables.is_none());
        assert!(layout_context.themes.is_none());
        assert!(layout_context.animations.is_none());
        assert!(layout_context.classes.is_none());
    }

    #[test]
    fn test_add_style_class_to_context() {
        let mut layout_context = LayoutContext::new("TestLayout".to_string());
        let style_class = NenyrStyleClass::new("myClass".to_string(), None);

        layout_context.add_style_class_to_context("myClass".to_string(), style_class.clone());
        assert!(layout_context.classes.is_some());

        let classes = layout_context.classes.as_ref().unwrap();

        assert_eq!(classes.len(), 1);
        assert_eq!(classes.get("myClass"), Some(&style_class));
    }

    #[test]
    fn test_add_animation_to_context() {
        let mut layout_context = LayoutContext::new("TestLayout".to_string());
        let animation = NenyrAnimation::new("fade".to_string());

        layout_context.add_animation_to_context("fade".to_string(), animation.clone());
        assert!(layout_context.animations.is_some());

        let animations = layout_context.animations.as_ref().unwrap();

        assert_eq!(animations.len(), 1);
        assert_eq!(animations.get("fade"), Some(&animation));
    }

    #[test]
    fn test_add_variables_to_context() {
        let mut layout_context = LayoutContext::new("TestLayout".to_string());
        let variables = NenyrVariables::new();

        layout_context.add_variables_to_context(variables.clone());

        assert!(layout_context.variables.is_some());
        assert_eq!(layout_context.variables.as_ref().unwrap(), &variables);
    }

    #[test]
    fn test_add_aliases_to_context() {
        let mut layout_context = LayoutContext::new("TestLayout".to_string());
        let aliases = NenyrAliases::new();

        layout_context.add_aliases_to_context(aliases.clone());

        assert!(layout_context.aliases.is_some());
        assert_eq!(layout_context.aliases.as_ref().unwrap(), &aliases);
    }

    #[test]
    fn test_add_themes_to_context() {
        let mut layout_context = LayoutContext::new("TestLayout".to_string());
        let themes = NenyrThemes::new();

        layout_context.add_themes_to_context(themes.clone());

        assert!(layout_context.themes.is_some());
        assert_eq!(layout_context.themes.as_ref().unwrap(), &themes);
    }
}

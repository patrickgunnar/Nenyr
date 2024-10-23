use indexmap::IndexMap;

/// Represents a style class in the Nenyr DSL.
///
/// The `NenyrStyleClass` struct encapsulates the styling information associated with a specific class
/// in the Nenyr DSL, allowing for both standard and responsive style patterns. It supports
/// inheritance through the `deriving_from` field and can indicate the importance of styles via the
/// `is_important` field.
///
/// # Fields
///
/// - `class_name`: The name of the style class.
/// - `deriving_from`: An optional field representing the class this style class derives from.
/// - `is_important`: An optional boolean indicating whether the styles in this class are marked as important.
/// - `style_patterns`: An optional map of style patterns associated with this class.
/// - `responsive_patterns`: An optional map of responsive style patterns, organized by panoramic names.
#[derive(Debug, PartialEq, Clone)]
pub struct NenyrStyleClass {
    class_name: String,
    deriving_from: Option<String>,
    is_important: Option<bool>,

    style_patterns: Option<IndexMap<String, IndexMap<String, String>>>,
    responsive_patterns: Option<IndexMap<String, IndexMap<String, IndexMap<String, String>>>>,
}

impl NenyrStyleClass {
    /// Creates a new `NenyrStyleClass`.
    ///
    /// # Parameters
    ///
    /// - `class_name`: The name of the style class to be created.
    /// - `deriving_from`: An optional name of the class this style class derives from.
    ///
    /// # Returns
    ///
    /// A new instance of `NenyrStyleClass`.
    pub fn new(class_name: String, deriving_from: Option<String>) -> Self {
        Self {
            class_name,
            deriving_from,
            is_important: None,
            style_patterns: None,
            responsive_patterns: None,
        }
    }

    /// Sets the importance of the style class.
    ///
    /// This method sets the `is_important` field to indicate whether the styles within this class
    /// should take precedence over other styles.
    ///
    /// # Parameters
    ///
    /// - `is_important`: A boolean value indicating the importance of the style class.
    pub(crate) fn set_importance(&mut self, is_important: bool) {
        self.is_important = Some(is_important);
    }

    /// Resets a pattern node for the specified pattern name.
    ///
    /// This method initializes or resets the style patterns for a given pattern name, preparing
    /// it for new style rules.
    ///
    /// # Parameters
    ///
    /// - `pattern_name`: The name of the pattern to reset.
    pub(crate) fn reset_pattern_node(&mut self, pattern_name: &str) {
        if self.style_patterns == None {
            self.style_patterns = Some(IndexMap::new());
        }

        if let Some(style_pattern) = &mut self.style_patterns {
            style_pattern.insert(pattern_name.to_string(), IndexMap::new());
        }
    }

    /// Adds a style rule to a specified pattern.
    ///
    /// This method adds a property-value pair to the specified pattern's style rules. If the
    /// pattern does not exist, it will be created.
    ///
    /// # Parameters
    ///
    /// - `pattern_name`: The name of the pattern to which the style rule is to be added.
    /// - `property`: The property name to be set.
    /// - `value`: The value of the property.
    pub(crate) fn add_style_rule(&mut self, pattern_name: String, property: String, value: String) {
        if self.style_patterns == None {
            self.style_patterns = Some(IndexMap::new());
        }

        if let Some(style_pattern) = &mut self.style_patterns {
            if let Some(existing_pattern) = style_pattern.get_mut(&pattern_name) {
                existing_pattern.insert(property, value);
            } else {
                let property_value = IndexMap::from([(property, value)]);

                style_pattern.insert(pattern_name, property_value);
            }
        }
    }

    /// Resets a panoramic node for the specified panoramic name.
    ///
    /// This method initializes or resets the responsive patterns for a given panoramic name,
    /// preparing it for new responsive style rules.
    ///
    /// # Parameters
    ///
    /// - `breakpoint_name`: The name of the panoramic to reset.
    pub(crate) fn reset_panoramic_node(&mut self, breakpoint_name: &str) {
        if self.responsive_patterns == None {
            self.responsive_patterns = Some(IndexMap::new());
        }

        if let Some(responsive_patterns) = &mut self.responsive_patterns {
            responsive_patterns.insert(breakpoint_name.to_string(), IndexMap::new());
        }
    }

    /// Resets a pattern node within a specified panoramic node.
    ///
    /// This method initializes or resets the pattern node for a specified pattern name within a
    /// given panoramic context.
    ///
    /// # Parameters
    ///
    /// - `breakpoint_name`: The name of the panoramic to contain the pattern.
    /// - `pattern_name`: The name of the pattern to reset.
    pub(crate) fn reset_pattern_node_on_panoramic_node(
        &mut self,
        breakpoint_name: &str,
        pattern_name: &str,
    ) {
        if self.responsive_patterns == None {
            self.responsive_patterns = Some(IndexMap::new());
        }

        if let Some(responsive_patterns) = &mut self.responsive_patterns {
            if let Some(panoramic_patterns) = responsive_patterns.get_mut(breakpoint_name) {
                panoramic_patterns.insert(pattern_name.to_string(), IndexMap::new());
            }
        }
    }

    /// Adds a responsive style rule to a specified panoramic pattern.
    ///
    /// This method adds a property-value pair to the specified pattern's responsive style rules.
    /// If the panoramic pattern does not exist, it will be created.
    ///
    /// # Parameters
    ///
    /// - `breakpoint_name`: The name of the panoramic context for which the rule is to be added.
    /// - `pattern_name`: The name of the pattern to which the responsive style rule is to be added.
    /// - `property`: The property name to be set.
    /// - `value`: The value of the property.
    pub(crate) fn add_responsive_style_rule(
        &mut self,
        breakpoint_name: String,
        pattern_name: String,
        property: String,
        value: String,
    ) {
        if self.responsive_patterns == None {
            self.responsive_patterns = Some(IndexMap::new());
        }

        if let Some(responsive_patterns) = &mut self.responsive_patterns {
            if let Some(panoramic_patterns) = responsive_patterns.get_mut(&breakpoint_name) {
                if let Some(existing_pattern) = panoramic_patterns.get_mut(&pattern_name) {
                    existing_pattern.insert(property, value);
                } else {
                    let property_value = IndexMap::from([(property, value)]);

                    panoramic_patterns.insert(pattern_name, property_value);
                }
            } else {
                let property_value = IndexMap::from([(property, value)]);
                let style_pattern = IndexMap::from([(pattern_name, property_value)]);

                responsive_patterns.insert(breakpoint_name, style_pattern);
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use indexmap::IndexMap;

    #[test]
    fn test_new_nenyr_style_class() {
        let class = NenyrStyleClass::new("test-class".to_string(), None);

        assert_eq!(class.class_name, "test-class");
        assert_eq!(class.deriving_from, None);
        assert_eq!(class.is_important, None);
        assert!(class.style_patterns.is_none());
        assert!(class.responsive_patterns.is_none());
    }

    #[test]
    fn test_set_importance() {
        let mut class = NenyrStyleClass::new("test-class".to_string(), None);

        class.set_importance(true);
        assert_eq!(class.is_important, Some(true));
    }

    #[test]
    fn test_reset_pattern_node() {
        let mut class = NenyrStyleClass::new("test-class".to_string(), None);
        class.reset_pattern_node("base-pattern");

        let mut expected_patterns = IndexMap::new();
        expected_patterns.insert("base-pattern".to_string(), IndexMap::new());

        assert_eq!(class.style_patterns, Some(expected_patterns));
    }

    #[test]
    fn test_add_style_rule() {
        let mut class = NenyrStyleClass::new("test-class".to_string(), None);
        class.add_style_rule(
            "base-pattern".to_string(),
            "color".to_string(),
            "red".to_string(),
        );

        let mut expected_property = IndexMap::new();
        expected_property.insert("color".to_string(), "red".to_string());

        let mut expected_patterns = IndexMap::new();
        expected_patterns.insert("base-pattern".to_string(), expected_property);

        assert_eq!(class.style_patterns, Some(expected_patterns));
    }

    #[test]
    fn test_reset_panoramic_node() {
        let mut class = NenyrStyleClass::new("test-class".to_string(), None);
        class.reset_panoramic_node("lg");

        let mut expected_responsive_patterns = IndexMap::new();
        expected_responsive_patterns.insert("lg".to_string(), IndexMap::new());

        assert_eq!(
            class.responsive_patterns,
            Some(expected_responsive_patterns)
        );
    }

    #[test]
    fn test_add_responsive_style_rule() {
        let mut class = NenyrStyleClass::new("test-class".to_string(), None);
        
        class.add_responsive_style_rule(
            "lg".to_string(),
            "base-pattern".to_string(),
            "width".to_string(),
            "100%".to_string(),
        );

        let mut expected_property = IndexMap::new();
        expected_property.insert("width".to_string(), "100%".to_string());

        let mut expected_pattern = IndexMap::new();
        expected_pattern.insert("base-pattern".to_string(), expected_property);

        let mut expected_responsive_patterns = IndexMap::new();
        expected_responsive_patterns.insert("lg".to_string(), expected_pattern);

        assert_eq!(
            class.responsive_patterns,
            Some(expected_responsive_patterns)
        );
    }
}

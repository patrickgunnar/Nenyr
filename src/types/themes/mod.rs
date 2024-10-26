use super::variables::NenyrVariables;

/// Enum representing different theme kinds in the Nenyr styling framework.
///
/// `NenyrThemesKind` allows specifying whether a theme is light or dark,
/// which can then be associated with different styling configurations in
/// the `NenyrThemes` struct.
///
/// # Variants
/// - `Light`: Represents a light theme configuration.
/// - `Dark`: Represents a dark theme configuration.
#[derive(Debug, PartialEq, Clone)]
pub enum NenyrThemesKind {
    Light,
    Dark,
}

/// Holds the theme schemas (variables) for light and dark themes in the Nenyr framework.
///
/// The `NenyrThemes` struct is designed to store and manage theme-specific
/// variables, with separate schemas for light and dark themes. Each schema,
/// represented by `NenyrVariables`, holds the styling variables unique to that
/// theme, allowing for distinct configurations based on theme type.
///
/// # Fields
/// - `light_schema`: An optional `NenyrVariables` struct containing variables
///   specific to the light theme. Defaults to `None` until set.
/// - `dark_schema`: An optional `NenyrVariables` struct containing variables
///   specific to the dark theme. Defaults to `None` until set.
#[derive(Debug, PartialEq, Clone)]
pub struct NenyrThemes {
    pub light_schema: Option<NenyrVariables>,
    pub dark_schema: Option<NenyrVariables>,
}

impl NenyrThemes {
    /// Creates a new instance of `NenyrThemes`.
    ///
    /// This function initializes a `NenyrThemes` struct with both `light_schema`
    /// and `dark_schema` set to `None`, providing a clean starting point for
    /// defining theme-specific variables.
    ///
    /// # Returns
    /// A new `NenyrThemes` instance with unset theme schemas.
    pub fn new() -> Self {
        Self {
            light_schema: None,
            dark_schema: None,
        }
    }

    /// Adds theme-specific variables to the appropriate theme schema.
    ///
    /// This function assigns a set of `NenyrVariables` to either the `light_schema`
    /// or `dark_schema` field based on the specified `schema_kind`. This allows
    /// for flexible theme-based styling, enabling each theme to maintain its own
    /// set of variables independently.
    ///
    /// # Parameters
    /// - `schema_kind`: A reference to `NenyrThemesKind` indicating whether the
    ///   variables apply to the light or dark theme.
    /// - `variables`: A `NenyrVariables` instance containing the variables to be
    ///   added to the specified theme schema.
    pub(crate) fn add_variables(
        &mut self,
        schema_kind: &NenyrThemesKind,
        variables: NenyrVariables,
    ) {
        match schema_kind {
            NenyrThemesKind::Light => {
                self.light_schema = Some(variables);
            }
            NenyrThemesKind::Dark => {
                self.dark_schema = Some(variables);
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_nenyr_themes_new() {
        let themes = NenyrThemes::new();

        // Assert that both schemas are None when a new instance is created
        assert_eq!(themes.light_schema, None);
        assert_eq!(themes.dark_schema, None);
    }

    #[test]
    fn test_add_variables_to_light_theme() {
        let mut themes = NenyrThemes::new();
        let mut variables = NenyrVariables::new();

        variables.add_variable("lightVar".to_string(), "#000000".to_string());

        // Add variables to the light theme
        themes.add_variables(&NenyrThemesKind::Light, variables.clone());

        // Assert that light_schema is set and dark_schema remains None
        assert_eq!(themes.light_schema, Some(variables));
        assert_eq!(themes.dark_schema, None);
    }

    #[test]
    fn test_add_variables_to_dark_theme() {
        let mut themes = NenyrThemes::new();
        let mut variables = NenyrVariables::new();

        variables.add_variable("lightVar".to_string(), "#000000".to_string());

        // Add variables to the dark theme
        themes.add_variables(&NenyrThemesKind::Dark, variables.clone());

        // Assert that dark_schema is set and light_schema remains None
        assert_eq!(themes.dark_schema, Some(variables));
        assert_eq!(themes.light_schema, None);
    }

    #[test]
    fn test_add_variables_to_both_themes() {
        let mut themes = NenyrThemes::new();
        let mut light_variables = NenyrVariables::new();
        let mut dark_variables = NenyrVariables::new();

        light_variables.add_variable("lightVar".to_string(), "#000000".to_string());
        dark_variables.add_variable("darkVar".to_string(), "#000000".to_string());

        // Add variables to both light and dark themes
        themes.add_variables(&NenyrThemesKind::Light, light_variables.clone());
        themes.add_variables(&NenyrThemesKind::Dark, dark_variables.clone());

        // Assert that both schemas are correctly set
        assert_eq!(themes.light_schema, Some(light_variables));
        assert_eq!(themes.dark_schema, Some(dark_variables));
    }

    #[test]
    fn test_nenyr_themes_kind_enum() {
        // Ensure that NenyrThemesKind variants can be matched and compared
        assert_eq!(NenyrThemesKind::Light, NenyrThemesKind::Light);
        assert_eq!(NenyrThemesKind::Dark, NenyrThemesKind::Dark);
    }
}

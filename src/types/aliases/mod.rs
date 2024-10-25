use indexmap::IndexMap;

/// `NenyrAliases` is a struct designed to store alias mappings for commonly used Nenyr properties.
/// It functions as a collection of key-value pairs where each key represents an alias identifier,
/// and each value is the CSS property string that the alias refers to. This struct is used within
/// the Nenyr context of Galadriel CSS to simplify referencing complex or frequently used properties,
/// promoting consistency and reducing redundancy in style declarations.
#[derive(Debug, PartialEq, Clone)]
pub struct NenyrAliases {
    /// A map that stores alias identifiers and their corresponding Nenyr properties, maintaining insertion order.
    pub values: IndexMap<String, String>,
}

impl NenyrAliases {
    /// Creates a new, empty `NenyrAliases` instance.
    ///
    /// This method initializes the `values` map to store alias entries for Nenyr properties,
    /// ready to hold alias mappings as key-value pairs.
    ///
    /// # Returns
    /// - A new instance of `NenyrAliases` with an empty `values` map.
    pub fn new() -> Self {
        Self {
            values: IndexMap::new(),
        }
    }

    /// Adds or updates an alias in the `NenyrAliases` map. If the alias identifier already exists,
    /// its value is updated with the new one provided.
    ///
    /// # Parameters
    /// - `identifier`: A `String` that uniquely represents the alias name.
    /// - `value`: A `String` that contains the CSS property this alias refers to.
    pub(crate) fn add_alias(&mut self, identifier: String, value: String) {
        self.values.insert(identifier, value);
    }
}

#[cfg(test)]
mod tests {
    use crate::types::aliases::NenyrAliases;

    #[test]
    fn test_new_creates_empty_nenyr_aliases() {
        let aliases = NenyrAliases::new();

        assert!(aliases.values.is_empty(), "Expected an empty values map");
    }

    #[test]
    fn test_add_alias_inserts_new_alias() {
        let mut aliases = NenyrAliases::new();

        aliases.add_alias("bgd".to_string(), "background".to_string());
        assert_eq!(aliases.values.get("bgd"), Some(&"background".to_string()));

        aliases.add_alias("dp".to_string(), "display".to_string());
        assert_eq!(aliases.values.get("dp"), Some(&"display".to_string()));
    }
}

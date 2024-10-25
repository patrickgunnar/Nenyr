use indexmap::IndexMap;

/// `NenyrVariables` represents a collection of key-value pairs where each key is a variable identifier,
/// and each value is the associated variable's string representation. This struct is utilized within the
/// Nenyr context of Galadriel Nenyr to store and manage Nenyr variables, offering efficient retrieval and
/// modification of stored values.
#[derive(Debug, PartialEq, Clone)]
pub struct NenyrVariables {
    /// Holds the mapping of variable identifiers to their values, preserving insertion order.
    pub values: IndexMap<String, String>,
}

impl NenyrVariables {
    /// Creates a new, empty `NenyrVariables` instance.
    ///
    /// This method initializes the `values` map, ready to store Nenyr variable pairs.
    ///
    /// # Returns
    /// - A new instance of `NenyrVariables` with an empty `values` map.
    pub fn new() -> Self {
        Self {
            values: IndexMap::new(),
        }
    }

    /// Adds a variable to the `NenyrVariables` map or updates an existing variable if the identifier already exists.
    ///
    /// # Parameters
    /// - `identifier`: A `String` that uniquely represents the variable name.
    /// - `value`: A `String` containing the value to be assigned to the variable.
    pub(crate) fn add_variable(&mut self, identifier: String, value: String) {
        self.values.insert(identifier, value);
    }
}

#[cfg(test)]
mod tests {
    use crate::types::variables::NenyrVariables;

    #[test]
    fn test_new_creates_empty_instance() {
        let variables = NenyrVariables::new();

        assert!(variables.values.is_empty());
    }

    #[test]
    fn test_add_variable_inserts_new_variable() {
        let mut variables = NenyrVariables::new();

        variables.add_variable("var1".to_string(), "value1".to_string());
        assert_eq!(variables.values.get("var1"), Some(&"value1".to_string()));
    }

    #[test]
    fn test_add_variable_updates_existing_variable() {
        let mut variables = NenyrVariables::new();

        variables.add_variable("var1".to_string(), "initial_value".to_string());
        variables.add_variable("var1".to_string(), "updated_value".to_string());
        assert_eq!(
            variables.values.get("var1"),
            Some(&"updated_value".to_string())
        );
    }

    #[test]
    fn test_add_variable_with_special_characters() {
        let mut variables = NenyrVariables::new();

        variables.add_variable("spécial_ñame".to_string(), "valüe_@_1".to_string());
        assert_eq!(
            variables.values.get("spécial_ñame"),
            Some(&"valüe_@_1".to_string())
        );
    }
}

use indexmap::IndexMap;

/// `NenyrImports` is a struct for managing external CSS imports within the Galadriel CSS framework.
/// This struct stores a collection of unique import statements that reference external stylesheets, fonts,
/// or other external CSS resources necessary for the styling of an application.
/// By using a `IndexMap`, `NenyrImports` ensures that each import is unique, preventing duplicate entries.
#[derive(Debug, PartialEq, Clone)]
pub struct NenyrImports {
    /// A collection of unique import statements used within an application. This `IndexMap` holds
    /// external resource URLs or paths, ensuring each import is stored only once.
    pub values: IndexMap<String, ()>,
}

impl NenyrImports {
    /// Creates a new, empty `NenyrImports` instance.
    ///
    /// This method initializes the `values` set to hold import strings, ready to be populated with
    /// external CSS resource URLs or paths.
    ///
    /// # Returns
    /// - A new instance of `NenyrImports` with an empty `values` set.
    pub fn new() -> Self {
        Self {
            values: IndexMap::new(),
        }
    }

    /// Adds a new import to the `NenyrImports` set if it doesn't already exist, ensuring only unique imports.
    ///
    /// # Parameters
    /// - `value`: A `String` representing the external CSS resource URL or path to be added to the import set.
    pub(crate) fn add_import(&mut self, value: String) {
        self.values.insert(value, ());
    }
}

#[cfg(test)]
mod tests {
    #[cfg(test)]
    mod tests {
        use crate::types::imports::NenyrImports;

        #[test]
        fn test_new_creates_empty_nenyr_imports() {
            let imports = NenyrImports::new();

            assert!(imports.values.is_empty(), "Expected an empty values map");
        }

        #[test]
        fn test_add_imports_inserts_new_typeface() {
            let mut imports = NenyrImports::new();

            imports.add_import("../../mocks/imports/another_external.css".to_string());
            assert_eq!(
                imports
                    .values
                    .contains_key("../../mocks/imports/another_external.css"),
                true
            );

            imports.add_import(
                "https://fonts.googleapis.com/css2?family=Matemasie&display=swap".to_string(),
            );
            assert_eq!(
                imports.values.contains_key(
                    "https://fonts.googleapis.com/css2?family=Matemasie&display=swap"
                ),
                true
            );
        }
    }
}

use indexmap::IndexMap;

/// `NenyrTypefaces` is a struct dedicated to storing mappings of custom typefaces used within the Galadriel CSS framework.
/// Each entry in `NenyrTypefaces` consists of a key-value pair, where the key is an identifier for a typeface
/// and the value is the corresponding font-family or typeface definition. This struct is utilized within
/// the Nenyr context of Galadriel CSS to easily reference specific fonts, facilitating efficient styling and
/// consistent typography across the application.
#[derive(Debug, PartialEq, Clone)]
pub struct NenyrTypefaces {
    /// A mapping of typeface identifiers to their respective font-family definitions, maintaining
    /// the order of insertion.
    pub values: IndexMap<String, String>,
}

impl NenyrTypefaces {
    /// Creates a new, empty `NenyrTypefaces` instance.
    ///
    /// This method initializes the `values` map, preparing it to store typeface entries as key-value pairs.
    ///
    /// # Returns
    /// - A new instance of `NenyrTypefaces` with an empty `values` map.
    pub fn new() -> Self {
        Self {
            values: IndexMap::new(),
        }
    }

    /// Adds a typeface entry to the `NenyrTypefaces` map or updates an existing entry if the identifier already exists.
    ///
    /// # Parameters
    /// - `identifier`: A `String` that uniquely represents the typeface identifier.
    /// - `value`: A `String` containing the font-family or typeface definition to be associated with the identifier.
    pub(crate) fn add_typeface(&mut self, identifier: String, value: String) {
        self.values.insert(identifier, value);
    }
}

#[cfg(test)]
mod tests {
    #[cfg(test)]
    mod tests {
        use crate::types::typefaces::NenyrTypefaces;

        #[test]
        fn test_new_creates_empty_nenyr_typefaces() {
            let typefaces = NenyrTypefaces::new();

            assert!(typefaces.values.is_empty(), "Expected an empty values map");
        }

        #[test]
        fn test_add_typefaces_inserts_new_typeface() {
            let mut typefaces = NenyrTypefaces::new();

            typefaces.add_typeface(
                "myFont".to_string(),
                "../../../mocks/typefaces/showa-source-curry.regular-webfont.eot".to_string(),
            );
            assert_eq!(
                typefaces.values.get("myFont"),
                Some(
                    &"../../../mocks/typefaces/showa-source-curry.regular-webfont.eot".to_string()
                )
            );

            typefaces.add_typeface(
                "myFont2".to_string(),
                "../../../mocks/typefaces/rosemartin.regular.otf".to_string(),
            );
            assert_eq!(
                typefaces.values.get("myFont2"),
                Some(&"../../../mocks/typefaces/rosemartin.regular.otf".to_string())
            );
        }
    }
}

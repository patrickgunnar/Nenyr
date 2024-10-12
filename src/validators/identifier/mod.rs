/// The `NenyrIdentifierValidator` trait provides a method for validating string identifiers.
///
/// This trait defines the logic to validate whether an identifier string is considered valid
/// according to a set of rules commonly used in programming conventions like camelCase and PascalCase.
///
/// ## Identifier Validation Rules:
///
/// 1. **Non-empty**: The identifier must not be an empty string.
/// 2. **Starting Character**: The first character must be an ASCII alphabetic character (a-z or A-Z).
/// 3. **Alphanumeric Characters**: All subsequent characters must be ASCII alphanumeric (a-z, A-Z, 0-9).
///
/// This allows identifiers like `camelCase` and `PascalCase` but disallows strings that start with numbers
/// or include non-alphanumeric characters like symbols (`@`, `#`, `_`).
///
/// ## Method
///
/// ### `is_valid_identifier`
///
/// Validates whether the given identifier is valid.
///
/// - **Parameters**:
///     - `identifier`: A reference to a string slice (`&str`) representing the identifier to be validated.
///
/// - **Returns**:
///     - `true` if the identifier is valid according to the rules above.
///     - `false` if the identifier is empty, starts with a non-alphabetic character, or contains invalid characters.
pub trait NenyrIdentifierValidator {
    /// Validates if the given identifier follows the specified rules.
    ///
    /// - The identifier must not be empty.
    /// - The first character must be an ASCII alphabetic character (a-z, A-Z).
    /// - All subsequent characters must be ASCII alphanumeric (a-z, A-Z, 0-9).
    ///
    /// # Parameters
    ///
    /// - `identifier`: A string slice reference (`&str`) representing the identifier to validate.
    ///
    /// # Returns
    ///
    /// - `true` if the identifier is valid.
    /// - `false` if the identifier is empty, starts with a non-alphabetic character, or contains invalid characters.
    fn is_valid_identifier(&self, identifier: &str) -> bool {
        if identifier.is_empty() {
            return false;
        }

        let mut chars = identifier.chars();

        if let Some(first_char) = chars.next() {
            if !first_char.is_ascii_alphabetic() {
                return false;
            }
        }

        chars.all(|c| c.is_ascii_alphanumeric())
    }
}

#[cfg(test)]
mod tests {
    use super::NenyrIdentifierValidator;

    struct Identifier {}

    impl Identifier {
        pub fn new() -> Self {
            Self {}
        }
    }

    impl NenyrIdentifierValidator for Identifier {}

    #[test]
    fn all_identifier_are_valid() {
        let identifier = Identifier::new();
        let valid_identifiers = vec![
            "thisIsAValidIdentifier",
            "anotherValidIdentifier",
            "TryingAnotherValidIdentifier",
            "ValidPascalCaseIdentifier",
            "validCamelCaseIdentifier",
            "camelCase",
            "PascalCase",
            "someRandomIdentifier",
            "AnotherRandomIdentifier",
            "AReallyLongIdentifierJustToBeTested",
            "anIdentifierWith788Numbers",
            "AnotherNumbered4545Identifier",
            "It89Works55With55Numbers45",
            "camelCaseSample",
            "PascalCaseIdentifier",
            "mixedCaseList",
            "CamelCaseExample",
            "pascalCaseMix",
            "exampleOfCamelCase",
            "AnotherPascalCaseExample",
            "simpleCamelCase",
            "AnotherMixedCaseExample",
            "longCamelCaseIdentifier",
            "SamplePascalCase",
            "lowerCamelCaseIdentifier",
            "FinalPascalCaseExample",
            "camelCaseIdentifier",
            "SuperPascalCaseTest",
        ];

        for valid_identifier in valid_identifiers {
            assert!(identifier.is_valid_identifier(valid_identifier));
        }
    }

    #[test]
    fn all_identifier_are_not_valid() {
        let identifier = Identifier::new();
        let valid_identifiers = vec![
            "1thisIsAValidIdentifier",
            "2anotherValidIdentifier",
            "3TryingAnotherValidIdentifier",
            "4ValidPascalCaseIdentifier",
            "5validCamelCaseIdentifier",
            "6camelCase",
            "7PascalCase",
            "8someRandomIdentifier",
            "9AnotherRandomIdentifier",
            "10_AReallyLongIdentifierJustToBeTested",
            "11_anIdentifierWith788Numbers",
            "12_AnotherNumbered4545Identifier",
            "@It89Works55With55Numbers45",
            "@camelCaseSample",
            "@PascalCaseIdentifier",
            "#mixedCaseList",
            "#CamelCaseExample",
            "#pascalCaseMix",
            "&exampleOfCamelCase",
            "&AnotherPascalCaseExample",
            "&simpleCamelCase",
            "_AnotherMixedCaseExample",
            "_longCamelCaseIdentifier",
            "_SamplePascalCase",
            "_lowerCamelCaseIdentifier",
            "_FinalPascalCaseExample",
            "_camelCaseIdentifier",
            "_SuperPascalCaseTest",
            "nenyrTest_ToAnotherInvalid#Identifier",
            "Another_Invalid_Identifier",
            "This-Identifier_should.not-be=valid",
            "",
        ];

        for valid_identifier in valid_identifiers {
            assert!(!identifier.is_valid_identifier(valid_identifier));
        }
    }

    #[test]
    fn performance_test_large_identifier_valid_vector() {
        let identifier = Identifier::new();
        let valid_identifiers: Vec<String> = (0..1000).map(|i| format!("nenyrTest{}", i)).collect();

        for valid_identifier in valid_identifiers {
            assert!(identifier.is_valid_identifier(&valid_identifier));
        }
    }

    #[test]
    fn performance_test_large_identifier_not_valid_vector() {
        let identifier = Identifier::new();
        let valid_identifiers: Vec<String> = (0..1000).map(|i| format!("{}nenyrTest", i)).collect();

        for valid_identifier in valid_identifiers {
            assert!(!identifier.is_valid_identifier(&valid_identifier));
        }
    }
}

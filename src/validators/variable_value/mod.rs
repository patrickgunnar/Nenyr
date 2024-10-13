use regex::Regex;

/// Trait for validating variable values used in the Nenyr DSL.
///
/// This trait provides a default method `is_valid_variable_value` that checks if a given string
/// (representing a variable value) adheres to a set of rules designed to ensure the validity of
/// the value within the context of Nenyr. The method returns `true` if the value is valid and `false` otherwise.
///
/// # Validation Rules
///
/// The following criteria are used to validate the variable value:
///
/// - **Invalid Characters**: Ensures that the value does not contain characters like `{`, `}`, `@`, `$`, `!`, `;`, and `:`.
/// - **Incomplete Functions**: Detects incomplete functions (e.g., `rgb(`, `rgba(`, `hsl(`, `calc(`, `url(`, etc.) that are missing a closing parenthesis.
/// - **Unbalanced Quotes**: Identifies values with an odd number of single (`'`) or double (`"`) quotes, which indicates unbalanced quotes.
/// - **Comments**: Matches and invalidates values containing comments (e.g., `/* comment */`).
/// - **Invalid Keywords**: Flags values that are known to be invalid, such as `undefined`, `invalid`, or `NaN`.
/// - **Invalid URL Format**: Invalidates `url()` if it does not contain any content inside the parentheses.
/// - **Invalid Property Format**: Detects and invalidates property-like values such as `property: value` where both the property and value are just words without proper context.
///
/// # Future Improvements
///
/// - This validator will be enhanced to cover additional invalid patterns and edge cases
///   as the framework evolves. Future versions will focus on improving the regex patterns
///   and potentially introducing more comprehensive validation rules.
pub trait NenyrVariableValueValidator {
    /// Validates the provided variable value against defined invalid patterns.
    ///
    /// Returns `true` if the variable value is valid, and `false` otherwise.
    ///
    /// # Parameters
    ///
    /// - `variable_value`: A string slice representing the variable value to validate.
    fn is_valid_variable_value(&self, variable_value: &str) -> bool {
        // Regular expression to match invalid characters such as `{`, `}`, `@`, `$`, `!`, `;`, and `:`.
        let invalid_char_regex = Regex::new(r"[{}@$!;:]").unwrap();

        // Regular expression to match incomplete functions, such as `rgb(`, `rgba(`, `hsl(`, `calc(`, or `url(`
        // that do not have closing parentheses.
        let incomplete_function_regex =
            Regex::new(r"(rgb|rgba|hsl|calc|url|linear-gradient)\([^)]*$").unwrap();

        // Regular expression to match unbalanced quotes. This regex matches values with an odd number of quotes.
        let unbalanced_quotes_regex = Regex::new(r#"(?:(?:[^"]*"){1,})|(?:[^']*'){1,}"#).unwrap();

        // Regular expression to match comments (e.g., `/* comment */`).
        let comments_regex = Regex::new(r"/\*.*\*/").unwrap();

        // Regular expression to match known invalid keywords such as `undefined`, `invalid`, and `NaN`.
        let invalid_keyword_regex = Regex::new(r"^(undefined|invalid|NaN)$").unwrap();

        // Regular expression to match invalid URL format, specifically `url()` with no content inside.
        let invalid_url_regex = Regex::new(r#"url\(\s*\)"#).unwrap();

        // Regular expression to match invalid property format, such as `property: value` where both are words.
        let invalid_property_regex = Regex::new(r"^\w+\s*:\s*\w+$").unwrap();

        // TODO: Improve this group later.
        //let number_without_unit_regex = Regex::new(r"^\d+$").unwrap();
        //let invalid_number_unit_regex = Regex::new(r"^\d+\s+[a-zA-Z]+$").unwrap();
        //let invalid_unit_regex = Regex::new(r"^\d+[a-zA-Z]+$").unwrap();
        //let invalid_negative_value_regex = Regex::new(r"^-?\d+(\.\d+)?[a-zA-Z]+$").unwrap();

        // Validate the value by ensuring it does not match any of the invalid patterns.
        !invalid_char_regex.is_match(variable_value)
            && !incomplete_function_regex.is_match(variable_value)
            && !unbalanced_quotes_regex.is_match(variable_value)
            && !comments_regex.is_match(variable_value)
            && !invalid_keyword_regex.is_match(variable_value)
            //&& !invalid_color_name_regex.is_match(variable_value)
            && !invalid_url_regex.is_match(variable_value)
            && !invalid_property_regex.is_match(variable_value)
    }
}

#[cfg(test)]
mod tests {
    use super::NenyrVariableValueValidator;

    struct Variable {}

    impl Variable {
        pub fn new() -> Self {
            Self {}
        }
    }

    impl NenyrVariableValueValidator for Variable {}

    #[test]
    fn all_variables_are_valid() {
        let variable = Variable::new();
        let valid_values = vec![
            "#000000",
            "red",
            "calc(2 + 2)",
            "linear-gradient(to right top, #051937, #004d7a, #008793, #00bf72, #a8eb12)",
            "flex",
            "100%",
            "80vh",
            "70vw",
            "300px",
            "150rem",
            "1px solid rgba(245, 40, 145, 0.8)",
            "rgb(245, 40, 145)",
            "hsl(205 100% 50%)",
        ];

        for val in valid_values {
            assert!(variable.is_valid_variable_value(val));
        }
    }

    #[test]
    fn all_variables_are_not_valid() {
        let variable = Variable::new();
        let invalid_values = vec![
            "#000000;",
            "red;",
            "calc(2 : 2)",
            "linear-gradient(to right top, #051937, #004d7a, #008793, #00bf72, #a8eb12",
            "flex@",
            "100%!",
            "80v$h",
            "7{0vw}",
            "undefined",
            "150:rem",
            "1px solid rgba(245, 40, 145, 0.8",
            "rgb(245, 40, 145",
            "hsl(205 100% 50%",
        ];

        for val in invalid_values {
            assert!(!variable.is_valid_variable_value(val));
        }
    }

    #[test]
    fn performance_test_large_variables_valid_vector() {
        let variable = Variable::new();
        let valid_variables: Vec<String> = (0..1000)
            .map(|i| format!("1px solid rgba(245, 40, 145, 0.{})", i))
            .collect();

        for valid_variable in valid_variables {
            assert!(variable.is_valid_variable_value(&valid_variable));
        }
    }

    #[test]
    fn performance_test_large_variables_not_valid_vector() {
        let variable = Variable::new();
        let valid_variables: Vec<String> =
            (0..1000).map(|i| format!("hsl(205 100% {}%", i)).collect();

        for valid_variable in valid_variables {
            assert!(!variable.is_valid_variable_value(&valid_variable));
        }
    }
}

use regex::Regex;

/// A trait responsible for validating the syntax of style rules.
///
/// This trait provides a method to check if a given style rule
/// adheres to the defined syntax standards. Currently, it only
/// validates that the rule does not contain any invalid characters.
/// The invalid characters defined include curly braces ( special
/// symbols such as `@`, `!`, and the delimiters `;` and `:`.
/// This validation helps ensure that style rules
/// are formatted correctly before being processed further.
///
/// In future iterations of the implementation, more complex
/// validation rules will be added to handle additional invalid
/// cases and improve the robustness of the syntax checking.
/// The focus on other functionalities will precede the expansion
/// of this validation logic.
///
/// # Note
///
/// Future developments may introduce new invalid cases, enhancing
/// the accuracy and utility of the syntax validation process.
pub trait NenyrStyleSyntaxValidator {
    /// Validates the syntax of a given style rule.
    ///
    /// This method checks if the provided style rule contains any
    /// invalid characters. If it does, the method returns `false`;
    /// otherwise, it returns `true`.
    ///
    /// # Parameters
    /// - `rule`: A string slice that represents the style rule to validate.
    ///
    /// # Returns
    /// - `true` if the syntax of the style rule is valid (i.e., does
    ///   not contain any invalid characters).
    /// - `false` if the syntax is invalid (i.e., contains one or
    ///   more of the invalid characters defined).
    fn is_valid_style_syntax(&self, rule: &str) -> bool {
        let invalid_chars = Regex::new(r"[@!;:]").unwrap();

        !invalid_chars.is_match(rule)
    }
}

#[cfg(test)]
mod tests {
    use super::NenyrStyleSyntaxValidator;

    struct StyleSyntax {}

    impl StyleSyntax {
        pub fn new() -> Self {
            Self {}
        }
    }

    impl NenyrStyleSyntaxValidator for StyleSyntax {}

    #[test]
    fn all_style_syntax_are_valid() {
        let styles_syntax = StyleSyntax::new();
        let valid_syntax = vec![
            "calc(20px + 20px)",
            "blue",
            "#000000",
            "#FFFFFF",
            "20px",
            "1px solid blue",
            "85%",
            "50vh",
            "70vw",
            "rgba(255, 99, 71, 0.7)",
            "inset 0px 4px 10px rgba(0, 0, 0, 0.2)",
            "rotate(45deg)",
            "scale(1.2)",
            "translateX(30px)",
            "skewY(10deg)",
            "5em",
            "clamp(10px, 5vw, 20px)",
            "repeating-linear-gradient(45deg, red, yellow 10%, blue 20%)",
            "repeat(3, 1fr)",
            "minmax(100px, auto)",
            "3s ease-in-out",
            "none",
            "url('image.jpg')",
            "border-box",
            "space-around",
            "flex-start",
            "italic",
            "uppercase",
            "center",
            "hidden",
        ];

        for rule in valid_syntax {
            assert!(styles_syntax.is_valid_style_syntax(rule));
        }
    }

    #[test]
    fn all_style_syntax_are_not_valid() {
        let styles_syntax = StyleSyntax::new();
        let valid_syntax = vec![
            "calc(20px : 20px)",
            "blue;",
            "@000000",
            "$;FFFFFF",
            "20!px",
            "{1px@ solid blue",
            "85:%",
            "50;",
            "70:vw",
        ];

        for rule in valid_syntax {
            assert!(!styles_syntax.is_valid_style_syntax(rule));
        }
    }
}

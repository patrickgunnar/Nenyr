/// `NenyrErrorKind` is an enumeration that categorizes errors that can occur
/// within the Nenyr framework. This enum provides a structured way to identify
/// and handle different types of errors that may arise during parsing,
/// validation, or execution of Nenyr code.
///
/// By categorizing errors into distinct types, developers can implement
/// more precise error handling strategies, allowing for tailored responses
/// to specific error conditions.
///
/// # Variants
///
/// - `SyntaxError`: Indicates that there is a syntax-related issue within
///   the Nenyr code. This can include missing delimiters, incorrect
///   formatting, or any other structural problems that prevent the code
///   from being parsed correctly.
///
/// - `ValidationError`: Represents an error that occurs during the
///   validation phase. This variant is used when the Nenyr code fails to
///   meet certain semantic rules or constraints defined within the framework.
///   This might involve checks for correct data types, invalid values, or
///   other logical inconsistencies.
///
/// - `MissingContext`: Signals that a required context for processing the
///   Nenyr code is missing. This may occur if necessary contextual
///   information has not been provided or is not accessible at the time
///   of execution.
///
/// - `Other`: A catch-all variant for errors that do not fit into the
///   above categories. This can be used for any unexpected errors or
///   conditions that do not have a specific type assigned to them.
#[derive(Debug, PartialEq, Clone)]
pub enum NenyrErrorKind {
    SyntaxError,
    ValidationError,
    MissingContext,
    Other,
}

/// `NenyrError` is a structure that encapsulates detailed information about errors
/// that occur during the processing of Nenyr code. This struct is designed to provide
/// comprehensive error reporting, ensuring that users receive the most relevant
/// information about errors encountered in their code.
///
/// By including contextual information about the error, such as surrounding lines
/// of code and suggestions for resolution, `NenyrError` enhances the developer
/// experience by making it easier to identify and fix issues.
///
/// # Fields
///
/// - `suggestion`: An optional field that provides personalized suggestions
///   on how to fix the error. This can help users quickly resolve issues
///   by offering actionable advice.
///
/// - `line_before`: An optional string that contains the line of code
///   preceding the line where the error occurred. This provides context
///   for understanding the error in relation to surrounding code.
///
/// - `line_after`: An optional string that contains the line of code
///   following the line where the error occurred. Similar to `line_before`,
///   this helps in providing context for the error.
///
/// - `context_name`: An optional string that indicates the name of the
///   context in which the error occurred. This can help users understand
///   where in their codebase the issue is located.
///
/// - `context_path`: A string that specifies the file path of the
///   `.nyr` file where the error occurred. This is crucial for locating
///   the source of the error within the project structure.
///
/// - `error_line`: A string representation of the actual line of code
///   where the error occurred. This provides the exact context of the error,
///   allowing users to see the problematic code directly.
///
/// - `error_message`: A string that provides a personalized message
///   explaining the reason for the error. This is intended to help users
///   understand what went wrong and why.
///
/// - `error_kind`: An instance of `NenyrErrorKind` that categorizes the
///   error. This enum helps in identifying the type of error that occurred,
///   enabling more specific error handling.
///
/// - `error_on_line`: A `usize` indicating the line number where the error
///   occurred. This can be particularly useful for debugging and error
///   reporting in user interfaces.
///
/// - `error_on_col`: A `usize` indicating the column number where the
///   error occurred. This level of detail helps users pinpoint the exact
///   location of the error within the line.
///
/// - `error_on_pos`: A `usize` indicating the position (or index) in the
///   content file where the error occurred. This provides the most granular
///   detail about the error's location.
#[derive(Debug, PartialEq, Clone)]
pub struct NenyrError {
    pub suggestion: Option<String>,
    pub line_before: Option<String>,
    pub line_after: Option<String>,
    pub context_name: Option<String>,
    pub context_path: String,
    pub error_line: String,
    pub error_message: String,
    pub error_kind: NenyrErrorKind,
    pub error_on_line: usize,
    pub error_on_col: usize,
    pub error_on_pos: usize,
}

impl NenyrError {
    pub(crate) fn new(
        suggestion: Option<String>,
        line_before: Option<String>,
        line_after: Option<String>,
        context_name: Option<String>,
        context_path: String,
        error_line: String,
        error_message: String,
        error_kind: NenyrErrorKind,
        error_on_line: usize,
        error_on_col: usize,
        error_on_pos: usize,
    ) -> Self {
        Self {
            suggestion,
            line_before,
            line_after,
            context_name,
            context_path,
            error_line,
            error_message,
            error_kind,
            error_on_line,
            error_on_col,
            error_on_pos,
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::error::{NenyrError, NenyrErrorKind};

    fn create_all_fields_error() -> NenyrError {
        NenyrError::new(
            Some("suggestion".to_string()),
            Some("line before".to_string()),
            Some("line after".to_string()),
            Some("context name".to_string()),
            "context path".to_string(),
            "error line".to_string(),
            "error message".to_string(),
            NenyrErrorKind::SyntaxError,
            10,
            5,
            20,
        )
    }

    fn create_none_fields_error() -> NenyrError {
        NenyrError::new(
            None,
            None,
            None,
            None,
            "context path".to_string(),
            "error line".to_string(),
            "error message".to_string(),
            NenyrErrorKind::SyntaxError,
            10,
            5,
            20,
        )
    }

    #[test]
    fn creating_nenyr_error_with_all_fields() {
        let error = create_all_fields_error();

        assert_eq!(error.suggestion, Some("suggestion".to_string()));
        assert_eq!(error.line_before, Some("line before".to_string()));
        assert_eq!(error.line_after, Some("line after".to_string()));
        assert_eq!(error.context_name, Some("context name".to_string()));
        assert_eq!(error.context_path, "context path".to_string());
        assert_eq!(error.error_line, "error line".to_string());
        assert_eq!(error.error_message, "error message".to_string());
        assert_eq!(error.error_kind, NenyrErrorKind::SyntaxError);
        assert_eq!(error.error_on_line, 10);
        assert_eq!(error.error_on_col, 5);
        assert_eq!(error.error_on_pos, 20);
    }

    #[test]
    fn creating_nenyr_error_with_none_fields() {
        let error = create_none_fields_error();

        assert_eq!(error.suggestion, None);
        assert_eq!(error.line_before, None);
        assert_eq!(error.line_after, None);
        assert_eq!(error.context_name, None);
        assert_eq!(error.context_path, "context path".to_string());
        assert_eq!(error.error_line, "error line".to_string());
        assert_eq!(error.error_message, "error message".to_string());
        assert_eq!(error.error_kind, NenyrErrorKind::SyntaxError);
        assert_eq!(error.error_on_line, 10);
        assert_eq!(error.error_on_col, 5);
        assert_eq!(error.error_on_pos, 20);
    }

    #[test]
    fn nenyr_error_kind_variants() {
        let syntax_error = NenyrErrorKind::SyntaxError;
        let validation_error = NenyrErrorKind::ValidationError;
        let missing_context = NenyrErrorKind::MissingContext;
        let other_error = NenyrErrorKind::Other;

        assert_eq!(syntax_error, NenyrErrorKind::SyntaxError);
        assert_eq!(validation_error, NenyrErrorKind::ValidationError);
        assert_eq!(missing_context, NenyrErrorKind::MissingContext);
        assert_eq!(other_error, NenyrErrorKind::Other);
    }

    #[test]
    fn test_nenyr_error_clone() {
        let error = create_none_fields_error();
        let error_cloned = error.clone();

        assert_eq!(error, error_cloned);

        let error = create_all_fields_error();
        let error_cloned = error.clone();

        assert_eq!(error, error_cloned);
    }

    #[test]
    fn test_nenyr_error_partial_eq() {
        let none_fields_error = create_none_fields_error();
        let all_fields_error = create_all_fields_error();

        assert_ne!(none_fields_error, all_fields_error);
    }

    #[test]
    fn test_nenyr_error_debug() {
        let printed_error = r#"NenyrError { suggestion: Some("suggestion"), line_before: Some("line before"), line_after: Some("line after"), context_name: Some("context name"), context_path: "context path", error_line: "error line", error_message: "error message", error_kind: SyntaxError, error_on_line: 10, error_on_col: 5, error_on_pos: 20 }"#;
        let all_fields_error = create_all_fields_error();

        assert_eq!(printed_error.to_string(), format!("{:?}", all_fields_error));
    }
}

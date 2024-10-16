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

/// Represents detailed error tracing information within a Nenyr document.
///
/// This struct captures the context of an error in the parsing or processing of Nenyr DSL code,
/// providing line and column information along with the surrounding lines for enhanced debugging.
///
/// It helps pinpoint the exact position of an error by including the line content before and after
/// the error, as well as the error line itself. This contextual data allows for precise reporting
/// and tracing during error handling.
///
/// # Fields
///
/// - `line_before`: The content of the line directly preceding the error line (if available), useful for context.
/// - `line_after`: The content of the line directly following the error line (if available), useful for context.
/// - `error_line`: The content of the line where the error occurred (if available).
/// - `error_on_line`: The 1-based line number where the error occurred.
/// - `error_on_col`: The 1-based column number where the error occurred.
/// - `error_on_pos`: The byte offset from the start of the file where the error occurred.
#[derive(Debug, PartialEq, Clone)]
pub struct NenyrErrorTracing {
    pub line_before: Option<String>,
    pub line_after: Option<String>,
    pub error_line: Option<String>,
    pub error_on_line: usize,
    pub error_on_col: usize,
    pub error_on_pos: usize,
}

impl NenyrErrorTracing {
    /// Constructs a new `NenyrErrorTracing` instance.
    ///
    /// This constructor allows setting all the error context information, including the lines
    /// before and after the error, the specific error line, and the exact position of the error in terms of
    /// line number, column number, and byte position.
    ///
    /// # Parameters
    ///
    /// - `line_before`: The line content before the error (optional).
    /// - `line_after`: The line content after the error (optional).
    /// - `error_line`: The line content where the error occurred (optional).
    /// - `error_on_line`: The 1-based line number where the error occurred.
    /// - `error_on_col`: The 1-based column number where the error occurred.
    /// - `error_on_pos`: The byte offset from the start of the file where the error occurred.
    ///
    /// # Returns
    ///
    /// Returns a `NenyrErrorTracing` struct populated with the provided context information.
    pub fn new(
        line_before: Option<String>,
        line_after: Option<String>,
        error_line: Option<String>,
        error_on_line: usize,
        error_on_col: usize,
        error_on_pos: usize,
    ) -> Self {
        Self {
            line_before,
            line_after,
            error_line,
            error_on_line,
            error_on_col,
            error_on_pos,
        }
    }
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
    pub context_name: Option<String>,
    pub context_path: String,
    pub error_message: String,
    pub error_kind: NenyrErrorKind,
    pub error_tracing: NenyrErrorTracing,
}

impl NenyrError {
    pub(crate) fn new(
        suggestion: Option<String>,
        context_name: Option<String>,
        context_path: String,
        error_message: String,
        error_kind: NenyrErrorKind,
        error_tracing: NenyrErrorTracing,
    ) -> Self {
        Self {
            suggestion,
            context_name,
            context_path,
            error_message,
            error_kind,
            error_tracing,
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::error::{NenyrError, NenyrErrorKind};

    use super::NenyrErrorTracing;

    fn create_all_fields_error() -> NenyrError {
        NenyrError::new(
            Some("suggestion".to_string()),
            Some("context name".to_string()),
            "context path".to_string(),
            "error message".to_string(),
            NenyrErrorKind::SyntaxError,
            NenyrErrorTracing::new(
                Some("line before".to_string()),
                Some("line after".to_string()),
                Some("error line".to_string()),
                10,
                5,
                20,
            ),
        )
    }

    fn create_none_fields_error() -> NenyrError {
        NenyrError::new(
            None,
            None,
            "context path".to_string(),
            "error message".to_string(),
            NenyrErrorKind::SyntaxError,
            NenyrErrorTracing::new(None, None, None, 10, 5, 20),
        )
    }

    #[test]
    fn creating_nenyr_error_with_all_fields() {
        let error = create_all_fields_error();

        assert_eq!(error.suggestion, Some("suggestion".to_string()));
        assert_eq!(error.context_name, Some("context name".to_string()));
        assert_eq!(error.context_path, "context path".to_string());
        assert_eq!(error.error_message, "error message".to_string());
        assert_eq!(error.error_kind, NenyrErrorKind::SyntaxError);
        assert_eq!(
            error.error_tracing.line_before,
            Some("line before".to_string())
        );
        assert_eq!(
            error.error_tracing.line_after,
            Some("line after".to_string())
        );
        assert_eq!(
            error.error_tracing.error_line,
            Some("error line".to_string())
        );
        assert_eq!(error.error_tracing.error_on_line, 10);
        assert_eq!(error.error_tracing.error_on_col, 5);
        assert_eq!(error.error_tracing.error_on_pos, 20);
    }

    #[test]
    fn creating_nenyr_error_with_none_fields() {
        let error = create_none_fields_error();

        assert_eq!(error.suggestion, None);
        assert_eq!(error.context_name, None);
        assert_eq!(error.context_path, "context path".to_string());
        assert_eq!(error.error_message, "error message".to_string());
        assert_eq!(error.error_kind, NenyrErrorKind::SyntaxError);
        assert_eq!(error.error_tracing.error_line, None);
        assert_eq!(error.error_tracing.line_before, None);
        assert_eq!(error.error_tracing.line_after, None);
        assert_eq!(error.error_tracing.error_on_line, 10);
        assert_eq!(error.error_tracing.error_on_col, 5);
        assert_eq!(error.error_tracing.error_on_pos, 20);
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
        let printed_error = r#"NenyrError { suggestion: Some("suggestion"), context_name: Some("context name"), context_path: "context path", error_message: "error message", error_kind: SyntaxError, error_tracing: NenyrErrorTracing { line_before: Some("line before"), line_after: Some("line after"), error_line: Some("error line"), error_on_line: 10, error_on_col: 5, error_on_pos: 20 } }"#;
        let all_fields_error = create_all_fields_error();

        assert_eq!(printed_error.to_string(), format!("{:?}", all_fields_error));
    }
}

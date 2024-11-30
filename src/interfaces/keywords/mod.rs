use crate::{
    error::{NenyrError, NenyrErrorKind},
    tokens::NenyrTokens,
    NenyrParser, NenyrResult,
};

/// # `NenyrParser` Keyword Parsing Methods
///
/// This implementation of the `NenyrParser` provides utility methods to parse specific
/// Nenyr language keywords such as `Construct` and `Declare`. These methods help ensure
/// that the input tokens adhere to the expected syntax rules for these keywords.
/// The methods return results based on the correctness of the tokens being parsed,
/// or they generate informative error messages if the syntax does not match expectations.
impl NenyrParser {
    /// Parses the `Construct` keyword in the Nenyr document.
    ///
    /// # Parameters
    ///
    /// - `suggestion`: An `Option<String>` that provides an optional suggestion for the error message.
    /// - `error_message`: A `&str` that contains the error message to be used if the keyword is not found.
    /// - `parse_fn`: A closure that defines further parsing logic if the `Construct` keyword is successfully parsed.
    ///
    /// # Returns
    ///
    /// - `NenyrResult<T>`: If the `Construct` keyword is found, the `parse_fn` closure is executed,
    ///   and its result is returned. Otherwise, a `NenyrError` is returned indicating the syntax error.
    ///
    /// # Errors
    ///
    /// Returns a `NenyrError` of kind `SyntaxError` if the current token is not `Construct`. The error
    /// message includes optional suggestions for fixing the issue and context about where the error occurred.
    pub(crate) fn parse_construct_keyword<F, T>(
        &mut self,
        suggestion: Option<String>,
        error_message: &str,
        parse_fn: F,
    ) -> NenyrResult<T>
    where
        F: Fn(&mut Self) -> NenyrResult<T>,
    {
        if let NenyrTokens::Construct = self.current_token {
            self.process_next_token()?;

            return parse_fn(self);
        }

        Err(NenyrError::new(
            suggestion,
            None,
            self.context_path.to_string(),
            self.add_nenyr_token_to_error(error_message),
            NenyrErrorKind::SyntaxError,
            self.get_tracing(),
        ))
    }

    /// Parses the `Declare` keyword in the Nenyr document.
    ///
    /// # Parameters
    ///
    /// - `suggestion`: An `Option<String>` that provides an optional suggestion for the error message.
    /// - `error_message`: A `&str` containing the error message used if the keyword is not found.
    ///
    /// # Returns
    ///
    /// - `NenyrResult<()>`: Returns `Ok(())` if the `Declare` keyword is found and parsed successfully.
    ///   If the token does not match, a `NenyrError` is returned indicating the syntax error.
    ///
    /// # Errors
    ///
    /// Returns a `NenyrError` of kind `SyntaxError` if the current token is not `Declare`.
    /// The error message includes suggestions for fixing the issue and contextual information.
    pub(crate) fn parse_declare_keyword(
        &mut self,
        suggestion: Option<String>,
        error_message: &str,
    ) -> NenyrResult<()> {
        if let NenyrTokens::Declare = self.current_token {
            self.process_next_token()?;

            return Ok(());
        }

        Err(NenyrError::new(
            suggestion,
            self.context_name.clone(),
            self.context_path.to_string(),
            self.add_nenyr_token_to_error(error_message),
            NenyrErrorKind::SyntaxError,
            self.get_tracing(),
        ))
    }
}

#[cfg(test)]
mod tests {
    use crate::NenyrParser;

    #[test]
    fn construct_keyword_is_valid() {
        let raw_nenyr = "Construct";
        let mut parser = NenyrParser::new();
        parser.setup_dependencies(raw_nenyr.to_string(), "".to_string());

        let _ = parser.process_next_token();
        assert_eq!(parser.parse_construct_keyword(None, "", |_| Ok(())), Ok(()));
    }

    #[test]
    fn construct_keyword_is_not_valid() {
        let raw_nenyr = "Construct0";
        let mut parser = NenyrParser::new();
        parser.setup_dependencies(raw_nenyr.to_string(), "".to_string());

        let _ = parser.process_next_token();
        assert_ne!(parser.parse_construct_keyword(None, "", |_| Ok(())), Ok(()));
    }

    #[test]
    fn declare_keyword_is_valid() {
        let raw_nenyr = "Declare";
        let mut parser = NenyrParser::new();
        parser.setup_dependencies(raw_nenyr.to_string(), "".to_string());

        let _ = parser.process_next_token();
        assert_eq!(parser.parse_declare_keyword(None, ""), Ok(()))
    }

    #[test]
    fn declare_keyword_is_not_valid() {
        let raw_nenyr = "Declare0";
        let mut parser = NenyrParser::new();
        parser.setup_dependencies(raw_nenyr.to_string(), "".to_string());

        let _ = parser.process_next_token();
        assert_ne!(parser.parse_declare_keyword(None, ""), Ok(()))
    }
}

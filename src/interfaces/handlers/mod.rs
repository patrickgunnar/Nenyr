use crate::{error::NenyrErrorTracing, tokens::NenyrTokens, NenyrParser, NenyrResult};

/// # NenyrParser Handlers
///
/// This implementation of the `NenyrParser` struct includes a set of utility methods
/// that facilitate various aspects of the Nenyr parsing process. These handlers
/// abstract operations related to token processing, context management, and error
/// tracing, providing essential functionality for the parsing workflow.
///
/// The methods in this block are intended to enhance the usability and maintainability
/// of the parser by providing clear interfaces for common tasks, thereby promoting
/// code readability and efficiency in handling Nenyr language constructs.
impl NenyrParser {
    /// Advances the parser to the next token in the input stream.
    ///
    /// This method retrieves the next token from the lexer and updates the current token
    /// in the parser's state. It is essential for iterating through the tokens of
    /// the Nenyr input.
    ///
    /// # Returns
    /// Returns a `NenyrResult<()>`, which indicates the success or failure of the operation.
    /// On success, it returns `Ok(())`. On failure, it returns an error if the lexer
    /// encounters an issue while retrieving the next token.
    pub(crate) fn process_next_token(&mut self) -> NenyrResult<()> {
        self.current_token = self.lexer.next_token()?;

        Ok(())
    }

    /// Retrieves the current lexer position tracing information.
    ///
    /// This method returns a `NenyrErrorTracing` object that contains details about the
    /// current position in the input stream. This information is useful for generating
    /// informative error messages and debugging.
    ///
    /// # Returns
    /// Returns a `NenyrErrorTracing` instance that provides information about the
    /// current lexer state, including line and column numbers.
    pub(crate) fn get_tracing(&self) -> NenyrErrorTracing {
        self.lexer.trace_lexer_position()
    }

    /// Sets the context name for the current parsing operation.
    ///
    /// This method allows the user to define a context name, which can be helpful
    /// for tracking the current parsing state and generating contextual error messages.
    /// It also updates the lexer with the new context name.
    ///
    /// # Parameters
    /// - `context_name`: An `Option<String>` representing the context name to be set.
    /// If `None` is provided, it clears the existing context name.
    ///
    /// # Returns
    /// This method does not return a value.
    pub(crate) fn set_context_name(&mut self, context_name: Option<String>) {
        self.context_name = context_name.clone();
        self.lexer.set_context_name(context_name);
    }

    /// Constructs a detailed error message by appending the current token to the
    /// provided error message.
    ///
    /// This method takes an error message as input and formats it to include information
    /// about the current token that was found during parsing. This is particularly useful
    /// for providing context when an unexpected token is encountered.
    ///
    /// # Parameters
    /// - `error_message`: A `&str` representing the base error message to which
    /// the current token will be appended.
    ///
    /// # Returns
    /// Returns a `String` containing the formatted error message that includes
    /// the original error message along with the current token found during parsing.
    pub(crate) fn add_nenyr_token_to_error(&self, error_message: &str) -> String {
        let transformed_token = match self.current_token.clone() {
            NenyrTokens::Comma => ",",
            NenyrTokens::CurlyBracketOpen => "{",
            NenyrTokens::CurlyBracketClose => "}",
            NenyrTokens::ParenthesisOpen => "(",
            NenyrTokens::ParenthesisClose => ")",
            NenyrTokens::SquareBracketOpen => "[",
            NenyrTokens::SquareBracketClose => "]",
            NenyrTokens::Colon => ":",
            NenyrTokens::Identifier(val) => &val.to_owned(),
            NenyrTokens::StringLiteral(val) => &val.to_owned(),
            NenyrTokens::Number(num) => &num.to_string(),
            other => &format!("{:?}", other),
        };

        if cfg!(test) {
            format!(
                "{} However, found `{}` instead.",
                error_message, transformed_token
            )
        } else {
            format!(
                "{} Unfortunately, instead of the expected value, we received the following: `{}`.",
                error_message, transformed_token
            )
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::{error::NenyrErrorTracing, tokens::NenyrTokens, NenyrParser};

    #[test]
    fn process_next_token_is_valid() {
        let raw_nenyr = "Construct";
        let mut parser = NenyrParser::new();
        parser.setup_dependencies(raw_nenyr.to_string(), "".to_string());

        let _ = parser.process_next_token();
        assert_eq!(parser.current_token, NenyrTokens::Construct);
    }

    #[test]
    fn process_next_token_is_not_valid() {
        let raw_nenyr = "Central";
        let mut parser = NenyrParser::new();
        parser.setup_dependencies(raw_nenyr.to_string(), "".to_string());

        let _ = parser.process_next_token();
        assert_ne!(parser.current_token, NenyrTokens::Construct);
    }

    #[test]
    fn get_tracing_is_valid() {
        let raw_nenyr = "Central";
        let mut parser = NenyrParser::new();
        parser.setup_dependencies(raw_nenyr.to_string(), "".to_string());

        let _ = parser.process_next_token();
        assert_eq!(
            parser.get_tracing(),
            NenyrErrorTracing::new(None, None, Some("Central".to_string()), 1, 8, 7,)
        );
    }

    #[test]
    fn setting_context_name_is_valid() {
        let raw_nenyr = "Central";
        let mut parser = NenyrParser::new();
        parser.setup_dependencies(raw_nenyr.to_string(), "".to_string());

        parser.set_context_name(Some("myContextName".to_string()));

        assert_eq!(parser.context_name, Some("myContextName".to_string()));
    }

    #[test]
    fn add_nenyr_token_is_valid() {
        let raw_nenyr = "Central";
        let mut parser = NenyrParser::new();
        parser.setup_dependencies(raw_nenyr.to_string(), "".to_string());

        let _ = parser.process_next_token();
        assert_eq!(
            parser.add_nenyr_token_to_error("This is an error message."),
            "This is an error message. However, found `Central` instead.".to_string()
        );
    }
}

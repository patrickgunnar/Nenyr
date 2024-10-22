use crate::{
    error::{NenyrError, NenyrErrorKind},
    tokens::NenyrTokens,
    NenyrParser, NenyrResult,
};

/// # NenyrParser Literal Parsing Methods
///
/// This implementation of the `NenyrParser` provides methods to parse various literals
/// used in the Nenyr DSL, such as string literals, numeric literals, and other literal types.
/// These methods ensure that tokens representing literals are correctly processed according
/// to the syntax rules of the Nenyr language.
///
/// The literal parsing methods return the parsed values upon successful validation, or
/// they generate detailed error messages if the tokens do not match the expected syntax.
/// In addition, these methods often provide suggestions for how the input could be corrected
/// when syntax errors are detected.
///
/// Each literal parser is designed to handle specific edge cases, such as missing tokens,
/// invalid formatting, or incomplete expressions, ensuring that the parsing process remains
/// robust and informative throughout.
impl<'a> NenyrParser<'a> {
    /// Parses a string literal from the token stream and handles its validation.
    ///
    /// # Parameters
    /// - `suggestion`: An optional suggestion string that is added to the error message
    ///   when the parsing of a string literal fails.
    /// - `error_message`: A string slice representing the error message to be displayed
    ///   when a string literal is expected but not found.
    /// - `with_next_move`: A boolean flag to indicate if the parser should move to the
    ///   next token after successfully parsing the string literal. If `true`, the
    ///   parser will advance to the next token.
    ///
    /// # Returns
    /// - `NenyrResult<String>`: Returns the extracted string literal if the current token
    ///   is a valid string literal. If the literal is successfully parsed, the result is
    ///   returned; otherwise, a `NenyrError` is returned.
    ///
    /// # Errors
    /// This method returns a `NenyrError` in the following situations:
    /// - **Missing or Invalid String Literal**: If the current token is not a string literal,
    ///   or the literal is empty, an error is generated using the provided `suggestion` and
    ///   `error_message`.
    pub(crate) fn parse_string_literal(
        &mut self,
        suggestion: Option<String>,
        error_message: &str,
        with_next_move: bool,
    ) -> NenyrResult<String> {
        // Check if the current token is a string literal
        if let NenyrTokens::StringLiteral(val) = self.current_token.clone() {
            // Ensure the string is not empty
            if !val.is_empty() {
                // Move to the next token if requested
                if with_next_move {
                    self.process_next_token()?;
                }

                // Return the valid string literal
                return Ok(val);
            }
        }

        // Return an error if the string literal is missing or invalid
        Err(NenyrError::new(
            suggestion,
            self.context_name.clone(),
            self.context_path.to_string(),
            self.add_nenyr_token_to_error(error_message),
            NenyrErrorKind::SyntaxError,
            self.get_tracing(),
        ))
    }

    /// Parses a boolean literal (`true` or `false`) from the current token and optionally
    /// moves to the next token if `with_next_move` is true.
    ///
    /// # Parameters
    /// - `suggestion`: An optional string containing suggestions for fixing syntax errors
    ///   if the parsing fails. This suggestion will be passed along to any error generated.
    /// - `error_message`: A string describing the error that occurred, used to provide context
    ///   when the expected boolean literal is not found. This message is displayed in the error.
    /// - `with_next_move`: A boolean flag indicating whether to automatically advance to the
    ///   next token after parsing the current boolean literal. If `true`, the parser moves to
    ///   the next token after a successful parse.
    ///
    /// # Returns
    /// - `Ok(true)` if the current token is a `true` literal.
    /// - `Ok(false)` if the current token is a `false` literal.
    /// - `Err(NenyrError)` if the token is not a valid boolean literal, or if no token is present.
    ///
    /// # Errors
    /// If the current token is neither `true` nor `false`, an error of type `SyntaxError`
    /// is returned, containing the `error_message` and any optional `suggestion` provided.
    pub(crate) fn parse_boolean_literal(
        &mut self,
        suggestion: Option<String>,
        error_message: &str,
        with_next_move: bool,
    ) -> NenyrResult<bool> {
        // Check if the current token is a boolean `true` literal
        if let NenyrTokens::True = self.current_token {
            // Move to the next token if requested
            if with_next_move {
                self.process_next_token()?;
            }

            return Ok(true);
        }

        // Check if the current token is a boolean `false` literal
        if let NenyrTokens::False = self.current_token {
            // Move to the next token if requested
            if with_next_move {
                self.process_next_token()?;
            }

            return Ok(false);
        }

        // Return an error if the boolean literal is missing or invalid
        Err(NenyrError::new(
            suggestion,
            self.context_name.clone(),
            self.context_path.to_string(),
            self.add_nenyr_token_to_error(error_message),
            NenyrErrorKind::SyntaxError,
            self.get_tracing(),
        ))
    }

    /// Parses an identifier literal from the current token.
    ///
    /// This method checks whether the current token is an identifier and returns it as a `String` if valid.
    /// If the token is valid and the `with_next_move` flag is set to true, the parser will advance to the next token
    /// after processing the current one.
    ///
    /// # Parameters
    ///
    /// - `suggestion`: An optional string providing suggestions for valid identifiers if parsing fails.
    /// - `error_message`: A message describing the error if the identifier literal is missing or invalid.
    /// - `with_next_move`: A boolean flag indicating whether to advance the parser to the next token after successful parsing.
    ///
    /// # Returns
    ///
    /// Returns a `NenyrResult<String>` which is:
    /// - `Ok(val)`: The parsed identifier literal as a `String` if successful.
    /// - `Err(NenyrError)`: A `NenyrError` indicating the parsing failure, including context details such as
    ///   the current context name, context path, and the provided error message.
    ///
    /// # Errors
    ///
    /// This method will return an error if:
    /// - The current token is not an identifier.
    /// - The identifier is empty.
    /// - There are issues related to the parsing context, resulting in a syntax error.
    pub(crate) fn parse_identifier_literal(
        &mut self,
        suggestion: Option<String>,
        error_message: &str,
        with_next_move: bool,
    ) -> NenyrResult<String> {
        if let NenyrTokens::Identifier(val) = self.current_token.clone() {
            // Ensure the identifier is not empty
            if !val.is_empty() {
                // Move to the next token if requested
                if with_next_move {
                    self.process_next_token()?;
                }

                // Return the valid identifier literal
                return Ok(val);
            }
        }

        // Return an error if the identifier literal is missing or invalid
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
    fn string_is_valid() {
        let raw_nenyr = r#""This is a valid string""#;
        let mut parser = NenyrParser::new(raw_nenyr, "");

        let _ = parser.process_next_token();
        assert_eq!(
            parser.parse_string_literal(None, "", false),
            Ok("This is a valid string".to_string())
        );
    }

    #[test]
    fn empty_string_is_not_valid() {
        let raw_nenyr = r#""#;
        let mut parser = NenyrParser::new(raw_nenyr, "");

        let _ = parser.process_next_token();
        assert_ne!(
            parser.parse_string_literal(None, "", false),
            Ok("".to_string())
        );
    }

    #[test]
    fn string_is_not_valid() {
        let raw_nenyr = r#""This is na invalid string"#;
        let mut parser = NenyrParser::new(raw_nenyr, "");

        let _ = parser.process_next_token();
        assert_ne!(
            parser.parse_string_literal(None, "", false),
            Ok("This is na invalid string".to_string())
        );
    }

    #[test]
    fn true_boolean_is_valid() {
        let raw_nenyr = r#"true"#;
        let mut parser = NenyrParser::new(raw_nenyr, "");

        let _ = parser.process_next_token();
        assert_eq!(parser.parse_boolean_literal(None, "", false), Ok(true));
    }

    #[test]
    fn false_boolean_is_valid() {
        let raw_nenyr = r#"false"#;
        let mut parser = NenyrParser::new(raw_nenyr, "");

        let _ = parser.process_next_token();
        assert_eq!(parser.parse_boolean_literal(None, "", false), Ok(false));
    }

    #[test]
    fn boolean_is_not_valid() {
        let raw_nenyr = r#"Central"#;
        let mut parser = NenyrParser::new(raw_nenyr, "");

        let _ = parser.process_next_token();
        assert_ne!(parser.parse_boolean_literal(None, "", false), Ok(false));
    }

    #[test]
    fn identifier_is_valid() {
        let raw_nenyr = r#"thisIsAValidIdentifier"#;
        let mut parser = NenyrParser::new(raw_nenyr, "");

        let _ = parser.process_next_token();
        assert_eq!(
            parser.parse_identifier_literal(None, "", false),
            Ok("thisIsAValidIdentifier".to_string())
        );
    }

    #[test]
    fn identifier_is_not_valid() {
        let raw_nenyr = r#"this_is_an_invalid_identifier"#;
        let mut parser = NenyrParser::new(raw_nenyr, "");

        let _ = parser.process_next_token();
        assert_ne!(
            parser.parse_identifier_literal(None, "", false),
            Ok("this_is_an_invalid_identifier".to_string())
        );
    }

    #[test]
    fn empty_identifier_is_not_valid() {
        let raw_nenyr = r#""#;
        let mut parser = NenyrParser::new(raw_nenyr, "");

        let _ = parser.process_next_token();
        assert_ne!(
            parser.parse_identifier_literal(None, "", false),
            Ok("".to_string())
        );
    }
}

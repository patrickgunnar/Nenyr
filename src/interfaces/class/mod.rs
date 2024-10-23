use crate::{
    error::{NenyrError, NenyrErrorKind},
    loop_while_not,
    tokens::NenyrTokens,
    types::class::NenyrStyleClass,
    validators::identifier::NenyrIdentifierValidator,
    NenyrParser, NenyrResult,
};

impl<'a> NenyrParser<'a> {
    /// Parses a `Class` declaration in the Nenyr syntax.
    ///
    /// This function processes the following syntax:
    ///
    /// ```nenyr
    /// Class('className') { ... }
    /// Class('className') Deriving('parentClass') { ... }
    /// ```
    ///
    /// It validates the class name and derives the class structure, then proceeds to handle
    /// the class body enclosed within curly brackets. If a deriving statement is present,
    /// it ensures the proper class derivation chain is followed.
    ///
    /// # Errors
    ///
    /// If the syntax is invalid, it returns detailed `NenyrError` for the following scenarios:
    ///
    /// - Missing or misplaced parentheses around the class name or deriving name.
    /// - Invalid or empty class or deriving name.
    /// - Missing curly brackets for the class block.
    ///
    /// # Returns
    ///
    /// On success, it returns a tuple containing the class name and the corresponding
    /// `NenyrStyleClass` object, which encapsulates the styles and derived properties
    /// for the class.
    pub(crate) fn process_class_method(&mut self) -> NenyrResult<(String, NenyrStyleClass)> {
        self.process_next_token()?;

        let class_name = self.retrieve_class_or_deriving_name(
            Some("Ensure that an opening parenthesis `(` is placed after the keyword `Class` to properly define the class name. The correct syntax is: `Class('className') { ... }`.".to_string()),
            "The declaration block of `Class` was expecting an open parenthesis `(` after the keyword `Class`, but none was found.",
            Some("Ensure that the class name in the `Class` declaration is properly closed with a parenthesis `)`. The correct syntax is: `Class('className') { ... }`.".to_string()),
            "The `Class` declaration is missing a closing parenthesis `)` after the class name.",
            Some("All `Class` declarations must have a non-empty string as a name. The name should contain only alphanumeric characters, with the first character being a letter. The correct syntax is: `Class('className') { ... }`.".to_string()),
            "The `Class` declaration must receive a name that is a non-empty string, but no class name was found.",
            Some("A valid class name should contain only alphanumeric characters, with the first character being an alphabetic letter. Examples: `'myClassName01'`, `'className01'`, etc.".to_string()),
            "The validation of the class name failed. The provided name does not meet the required format.",
        )?;

        self.process_next_token()?;

        let deriving_from = self.retrieve_deriving_from(&class_name)?;

        self.parse_curly_bracketed_delimiter(
            Some(format!("Ensure that the `{}` class or deriving name declaration is followed by an opening curly bracket `{{` to properly define the class block. The correct syntax is: `Declare Class('{}') {{ ... }}` or `Declare Class('{}') Deriving('layoutName') {{ ... }}`.", &class_name, &class_name, &class_name)),
            &format!("An opening curly bracket `{{` was expected after the `{}` class or deriving name declaration to start the class block, but it was not found.", &class_name),
            Some(format!("Ensure that each class definition block is properly closed with a corresponding closing curly bracket `}}`. Example: `Declare Class('{}') {{ ... }}` or `Declare Class('{}') Deriving('layoutName') {{ ... }}`.", &class_name, &class_name)),
            &format!("A closing curly bracket `}}` was expected to terminate the `{}` class definition block, but it was not found.", &class_name),
            |parser| parser.retrieve_class_block(&class_name, &deriving_from),
        )
    }

    /// Retrieves the name of the parent class from which the current class derives.
    ///
    /// This function checks for the presence of the `Deriving` keyword and processes
    /// the deriving name. It ensures the proper parent class is referenced in the class
    /// declaration.
    ///
    /// # Parameters
    /// - `class_name`: The name of the class being processed.
    ///
    /// # Returns
    ///
    /// Returns `Some(String)` containing the parent class name if a deriving statement
    /// is found. If no deriving statement is present, returns `None`.
    ///
    /// # Errors
    ///
    /// An error is returned if the deriving name syntax is invalid, such as missing
    /// parentheses or an empty name.
    fn retrieve_deriving_from(&mut self, class_name: &str) -> NenyrResult<Option<String>> {
        if let NenyrTokens::Deriving = self.current_token {
            self.process_next_token()?;

            let deriving_from = self.retrieve_class_or_deriving_name(
                Some(format!("Ensure that an opening parenthesis `(` is placed after the keyword `Deriving` to properly define the deriving name. The correct syntax is: `Class('{}') Deriving('layoutName') {{ ... }}`.", class_name)),
                "The statement of `Deriving` was expecting an open parenthesis `(` after the keyword `Deriving`, but none was found.",
                Some(format!("Ensure that the deriving name in the `Deriving` statement is properly closed with a parenthesis `)`. The correct syntax is: `Class('{}') Deriving('layoutName') {{ ... }}`.", class_name)),
                "The `Deriving` statement is missing a closing parenthesis `)` after the deriving name.",
                Some(format!("All `Deriving` statements must have a non-empty string as a name. The name should contain only alphanumeric characters, with the first character being a letter. The correct syntax is: `Class('{}') Deriving('layoutName') {{ ... }}`.", class_name)),
                "The `Deriving` statement must receive a name that is a non-empty string, but no deriving name was found.",
                Some("A valid deriving name should contain only alphanumeric characters, with the first character being an alphabetic letter. Examples: `'myLayoutName01'`, `'layoutName01'`, etc.".to_string()),
                "The validation of the deriving name failed. The provided name does not meet the required format.",
            )?;

            self.process_next_token()?;

            return Ok(Some(deriving_from));
        }

        Ok(None)
    }

    /// Retrieves the class or deriving name based on the current token.
    ///
    /// This method processes a parenthesized string literal that follows either
    /// `Class` or `Deriving` keywords. It ensures the name is properly enclosed
    /// in parentheses and follows naming conventions (non-empty string with
    /// alphanumeric characters, starting with a letter).
    ///
    /// # Parameters
    /// - `suggestion_on_open`: Optional suggestion message for missing open parenthesis.
    /// - `error_message_on_open`: Error message if the open parenthesis is missing.
    /// - `suggestion_on_close`: Optional suggestion message for missing close parenthesis.
    /// - `error_message_on_close`: Error message if the close parenthesis is missing.
    /// - `suggestion_on_parse_literal`: Optional suggestion for parsing the literal.
    /// - `error_message_on_parse_literal`: Error message for invalid literal parsing.
    /// - `suggestion_on_invalid`: Optional suggestion for invalid name formats.
    /// - `error_message_on_invalid`: Error message if the name is invalid.
    ///
    /// # Returns
    ///
    /// Returns the parsed class or deriving name as a `String` if valid.
    ///
    /// # Errors
    ///
    /// Returns an error if the syntax of the name does not match the required format,
    /// or if parentheses are missing.
    fn retrieve_class_or_deriving_name(
        &mut self,
        suggestion_on_open: Option<String>,
        error_message_on_open: &str,
        suggestion_on_close: Option<String>,
        error_message_on_close: &str,
        suggestion_on_parse_literal: Option<String>,
        error_message_on_parse_literal: &str,
        suggestion_on_invalid: Option<String>,
        error_message_on_invalid: &str,
    ) -> NenyrResult<String> {
        let current_name = self.parse_parenthesized_delimiter(
            suggestion_on_open,
            error_message_on_open,
            suggestion_on_close,
            error_message_on_close,
            |parser| {
                parser.parse_string_literal(
                    suggestion_on_parse_literal.clone(),
                    error_message_on_parse_literal,
                    true,
                )
            },
        )?;

        if !self.is_valid_identifier(&current_name) {
            return Err(NenyrError::new(
                suggestion_on_invalid,
                self.context_name.clone(),
                self.context_path.to_string(),
                self.add_nenyr_token_to_error(error_message_on_invalid),
                NenyrErrorKind::SyntaxError,
                self.get_tracing(),
            ));
        }

        Ok(current_name)
    }

    /// Retrieves the class block defined within curly brackets `{}`.
    ///
    /// This method processes the inner styles and declarations for a class
    /// by iterating through its block. It handles validation of delimiters
    /// (commas between block patterns) and ensures correct syntax formatting.
    ///
    /// # Parameters
    /// - `class_name`: The name of the class being processed.
    /// - `deriving_from`: Optionally, the parent class from which the current class derives.
    ///
    /// # Returns
    ///
    /// Returns a tuple containing the class name and the `NenyrStyleClass` object
    /// representing the class's styles and properties.
    ///
    /// # Errors
    ///
    /// An error is returned if there are issues with the block structure, such as
    /// missing commas or unclosed blocks.
    fn retrieve_class_block(
        &mut self,
        class_name: &str,
        deriving_from: &Option<String>,
    ) -> NenyrResult<(String, NenyrStyleClass)> {
        let mut style_class = NenyrStyleClass::new(class_name.to_string(), deriving_from.clone());

        loop_while_not!(
            self,
            Some(format!("Remove any duplicated commas from the `{}` class inner block to ensure proper syntax. The parser expects every pattern block to follow valid delimiters. Example: `Declare Class('{}') {{ Stylesheet({{ ... }}), PanoramicViewer({{ ... }}), ... }}`.", class_name, class_name)),
            &format!("A duplicated comma was found inside the `{}` class block. The parser expected to find a new pattern block, but it was not found.", class_name),
            Some(format!("Ensure that a comma is placed after each block definition inside the `{}` class to separate elements correctly. Proper syntax is required for the parser to process the context. Example: `Declare Class('{}') {{ Stylesheet({{ ... }}), PanoramicViewer({{ ... }}), ... }}`.", class_name, class_name)),
            &format!("All patterns inside the `{}` class block must be separated by commas. A comma is missing after the pattern block definition. The parser expected a comma to separate elements but did not find one.", class_name),
            || self.processing_state.is_block_active(),
            |is_active| self.processing_state.set_block_active(is_active),
            {
                self.process_patterns_methods(class_name, &mut style_class, false, &None)?;
            }
        );

        self.processing_state.set_block_active(false);

        Ok((class_name.to_string(), style_class))
    }
}

#[cfg(test)]
mod tests {
    use crate::{types::class::NenyrStyleClass, NenyrParser};

    fn mock_class() -> NenyrStyleClass {
        let mut cls = NenyrStyleClass::new("myClassName".to_string(), None);

        cls.add_style_rule(
            "_stylesheet".to_string(),
            "background-color".to_string(),
            "blue".to_string(),
        );
        cls.add_style_rule(
            "_stylesheet".to_string(),
            "border".to_string(),
            "10px solid red".to_string(),
        );
        cls.add_style_rule(
            "_stylesheet".to_string(),
            "height".to_string(),
            "100px".to_string(),
        );
        cls.add_style_rule(
            "_stylesheet".to_string(),
            "width".to_string(),
            "200px".to_string(),
        );

        cls.set_importance(true);

        cls.add_responsive_style_rule(
            "myBreakpoint".to_string(),
            "_stylesheet".to_string(),
            "background-color".to_string(),
            "blue".to_string(),
        );
        cls.add_responsive_style_rule(
            "myBreakpoint".to_string(),
            "_stylesheet".to_string(),
            "border".to_string(),
            "10px solid red".to_string(),
        );
        cls.add_responsive_style_rule(
            "myBreakpoint".to_string(),
            "_stylesheet".to_string(),
            "height".to_string(),
            "100px".to_string(),
        );
        cls.add_responsive_style_rule(
            "myBreakpoint".to_string(),
            "_stylesheet".to_string(),
            "width".to_string(),
            "200px".to_string(),
        );

        cls
    }

    #[test]
    fn mock_test_is_valid() {
        let raw_nenyr = "
        ('myClassName') {
        Important(true),
        Stylesheet({
            backgroundColor: 'blue',
            border: '10px solid red',
            height: '100px',
            width: '200px'
        }),
        PanoramicViewer({
            myBreakpoint({
                Stylesheet({
                    backgroundColor: 'blue',
                    border: '10px solid red',
                    height: '100px',
                    width: '200px'
                })
            })
        })
    }";
        let mut parser = NenyrParser::new(raw_nenyr, "");

        assert_eq!(
            parser.process_class_method(),
            Ok(("myClassName".to_string(), mock_class()))
        )
    }

    #[test]
    fn simple_class_is_valid() {
        let raw_nenyr = "('myTestingClass') Deriving('discreteAudio') {
        PanoramicViewer({
            onMobTablet({}),
            onDeskDesktop({})
        }),
        Stylesheet({
            backgroundColor: '${accentColorVar}',
            backgroundColor: '#0000FF',
            background: '#00FF00',
            padding: '${m15px21}',
            bdr: '5px'
        }),
    },";
        let mut parser = NenyrParser::new(raw_nenyr, "");

        assert_eq!(format!("{:?}", parser.process_class_method()), "Ok((\"myTestingClass\", NenyrStyleClass { class_name: \"myTestingClass\", deriving_from: Some(\"discreteAudio\"), is_important: None, style_patterns: Some({\"_stylesheet\": {\"background-color\": \"#0000FF\", \"background\": \"#00FF00\", \"padding\": \"${m15px21}\", \"bdr\": \"5px\"}}), responsive_patterns: Some({\"onMobTablet\": {}, \"onDeskDesktop\": {}}) }))".to_string());
    }

    #[test]
    fn simple_class_is_not_valid() {
        let raw_nenyr = "('myTestingClass') Deriving('discreteAudio') {
        PanoramicViewer({
            onMobTablet({}),
            onDeskDesktop({})
        }),
        Stylesheet({
            backgroundColor: '${accentColorVar}',
            backgroundColor: '#0000FF',
            background: '#00FF00',
            padding: '${m15px21}',
            bdr: '5px'
        }),
    ,";
        let mut parser = NenyrParser::new(raw_nenyr, "");

        assert_eq!(format!("{:?}", parser.process_class_method()), "Err(NenyrError { suggestion: Some(\"Remove any duplicated commas from the `myTestingClass` class inner block to ensure proper syntax. The parser expects every pattern block to follow valid delimiters. Example: `Declare Class('myTestingClass') { Stylesheet({ ... }), PanoramicViewer({ ... }), ... }`.\"), context_name: None, context_path: \"\", error_message: \"A duplicated comma was found inside the `myTestingClass` class block. The parser expected to find a new pattern block, but it was not found. However, found `,` instead.\", error_kind: SyntaxError, error_tracing: NenyrErrorTracing { line_before: Some(\"        }),\"), line_after: None, error_line: Some(\"    ,\"), error_on_line: 13, error_on_col: 6, error_on_pos: 365 } })".to_string());
    }

    #[test]
    fn long_class_is_valid() {
        let raw_nenyr = "('miniatureTrogon') Deriving('discreteAudio') {
        Important(true),
        Stylesheet({
            backgroundColor: '${accentColorVar}',
            backgroundColor: '#0000FF',
            background: '#00FF00',
            padding: '${m15px21}',
            bdr: '5px'
        }),
        Hover({
            background: '${secondaryColor}',
            padding: '${m15px21}',
            bdr: '5px'
        }),
        PanoramicViewer({
            onMobTablet({
                Stylesheet({
                    // Este é um comentário de linha.
                    display: 'block', // Este é um comentário de linha.
                })
            }),
            onDeskDesktop({
                Hover({
                    bgd: '${secondaryColor}', // Este é um comentário de linha.
                    pdg: '${m15px}'
                })
            })
        })
    },";
        let mut parser = NenyrParser::new(raw_nenyr, "");

        assert_eq!(format!("{:?}", parser.process_class_method()), "Ok((\"miniatureTrogon\", NenyrStyleClass { class_name: \"miniatureTrogon\", deriving_from: Some(\"discreteAudio\"), is_important: Some(true), style_patterns: Some({\"_stylesheet\": {\"background-color\": \"#0000FF\", \"background\": \"#00FF00\", \"padding\": \"${m15px21}\", \"bdr\": \"5px\"}, \":hover\": {\"background\": \"${secondaryColor}\", \"padding\": \"${m15px21}\", \"bdr\": \"5px\"}}), responsive_patterns: Some({\"onMobTablet\": {\"_stylesheet\": {\"display\": \"block\"}}, \"onDeskDesktop\": {\":hover\": {\"bgd\": \"${secondaryColor}\", \"pdg\": \"${m15px}\"}}}) }))".to_string());
    }

    #[test]
    fn long_class_is_not_valid() {
        let raw_nenyr = "'miniatureTrogon') Deriving('discreteAudio') {
        Important(true),
        Stylesheet({
            backgroundColor: '${accentColorVar}',
            backgroundColor: '#0000FF',
            background: '#00FF00',
            padding: '${m15px21}',
            bdr: '5px'
        }),
        Hover({
            background: '${secondaryColor}',
            padding: '${m15px21}',
            bdr: '5px'
        }),
        PanoramicViewer({
            onMobTablet({
                Stylesheet({
                    // Este é um comentário de linha.
                    display: 'block', // Este é um comentário de linha.
                })
            }),
            onDeskDesktop({
                Hover({
                    bgd: '${secondaryColor}', // Este é um comentário de linha.
                    pdg: '${m15px}'
                })
            })
        })
    },";
        let mut parser = NenyrParser::new(raw_nenyr, "");

        assert_eq!(format!("{:?}", parser.process_class_method()), "Err(NenyrError { suggestion: Some(\"Ensure that an opening parenthesis `(` is placed after the keyword `Class` to properly define the class name. The correct syntax is: `Class('className') { ... }`.\"), context_name: None, context_path: \"\", error_message: \"The declaration block of `Class` was expecting an open parenthesis `(` after the keyword `Class`, but none was found. However, found `StringLiteral(\\\"miniatureTrogon\\\")` instead.\", error_kind: SyntaxError, error_tracing: NenyrErrorTracing { line_before: None, line_after: Some(\"        Important(true),\"), error_line: Some(\"'miniatureTrogon') Deriving('discreteAudio') {\"), error_on_line: 1, error_on_col: 18, error_on_pos: 17 } })".to_string());
    }

    #[test]
    fn empty_class_is_not_valid() {
        let raw_nenyr = "";
        let mut parser = NenyrParser::new(raw_nenyr, "");

        assert_eq!(format!("{:?}", parser.process_class_method()), "Err(NenyrError { suggestion: Some(\"Ensure that an opening parenthesis `(` is placed after the keyword `Class` to properly define the class name. The correct syntax is: `Class('className') { ... }`.\"), context_name: None, context_path: \"\", error_message: \"The declaration block of `Class` was expecting an open parenthesis `(` after the keyword `Class`, but none was found. However, found `EndOfLine` instead.\", error_kind: SyntaxError, error_tracing: NenyrErrorTracing { line_before: None, line_after: None, error_line: None, error_on_line: 1, error_on_col: 1, error_on_pos: 0 } })".to_string());
    }
}

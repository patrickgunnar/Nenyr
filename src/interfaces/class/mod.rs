use crate::{
    error::{NenyrError, NenyrErrorKind},
    loop_while_not,
    tokens::NenyrTokens,
    types::class::NenyrStyleClass,
    validators::identifier::NenyrIdentifierValidator,
    NenyrParser, NenyrResult,
};

impl<'a> NenyrParser<'a> {
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

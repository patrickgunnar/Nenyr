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
            Some("".to_string()),
            "",
            Some("".to_string()),
            "",
            Some("".to_string()),
            "",
            Some("".to_string()),
            "",
        )?;

        self.process_next_token()?;

        let deriving_from = self.retrieve_deriving_from()?;

        self.parse_curly_bracketed_delimiter(
            Some("".to_string()),
            "",
            Some("".to_string()),
            "",
            |parser| parser.retrieve_class_block(&class_name, &deriving_from),
        )
    }

    fn retrieve_deriving_from(&mut self) -> NenyrResult<Option<String>> {
        if let NenyrTokens::Deriving = self.current_token {
            self.process_next_token()?;

            let deriving_from = self.retrieve_class_or_deriving_name(
                Some("".to_string()),
                "",
                Some("".to_string()),
                "",
                Some("".to_string()),
                "",
                Some("".to_string()),
                "",
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
            Some("".to_string()),
            "",
            Some("".to_string()),
            "",
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

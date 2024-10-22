use crate::{
    error::{NenyrError, NenyrErrorKind},
    loop_while_not,
    tokens::NenyrTokens,
    types::class::NenyrStyleClass,
    validators::identifier::NenyrIdentifierValidator,
    NenyrParser, NenyrResult,
};

impl<'a> NenyrParser<'a> {
    pub(crate) fn process_panoramic_pattern(
        &mut self,
        class_name: &str,
        style_class: &mut NenyrStyleClass,
    ) -> NenyrResult<()> {
        self.process_next_token()?;

        // First, parse the expression within the parentheses.
        self.parse_parenthesized_delimiter(
            Some("".to_string()),
            "",
            Some("".to_string()),
            "",
            |parser| {
                // Once inside the parentheses, parse the expression within the curly brackets.
                parser.parse_curly_bracketed_delimiter(
                    Some("".to_string()),
                    "",
                    Some("".to_string()),
                    "",
                    |parser| parser.process_panoramic_block(class_name, style_class),
                )?;

                // Processes the next token
                parser.process_next_token()
            },
        )
    }

    fn process_panoramic_block(
        &mut self,
        class_name: &str,
        style_class: &mut NenyrStyleClass,
    ) -> NenyrResult<()> {
        loop_while_not!(
            self,
            Some("".to_string()),
            "",
            Some("".to_string()),
            "",
            || self.processing_state.is_internal_block_active(),
            |is_active| self.processing_state.set_internal_block_active(is_active),
            {
                self.retrieve_panoramic_identifier(class_name, style_class)?;
            }
        );

        self.processing_state.set_internal_block_active(false);

        Ok(())
    }

    fn retrieve_panoramic_identifier(
        &mut self,
        class_name: &str,
        style_class: &mut NenyrStyleClass,
    ) -> NenyrResult<()> {
        let panoramic_name = self.parse_identifier_literal(Some("".to_string()), "", true)?;

        if !self.is_valid_identifier(&panoramic_name) {
            return Err(NenyrError::new(
                Some("".to_string()),
                self.context_name.clone(),
                self.context_path.to_string(),
                self.add_nenyr_token_to_error(""),
                NenyrErrorKind::SyntaxError,
                self.get_tracing(),
            ));
        }

        self.processing_state.set_internal_block_active(true);
        style_class.reset_panoramic_node(&panoramic_name);

        // First, parse the expression within the parentheses.
        self.parse_parenthesized_delimiter(
            Some("".to_string()),
            "",
            Some("".to_string()),
            "",
            |parser| {
                // Once inside the parentheses, parse the expression within the curly brackets.
                parser.parse_curly_bracketed_delimiter(
                    Some("".to_string()),
                    "",
                    Some("".to_string()),
                    "",
                    |parser| {
                        parser.process_panoramic_children(class_name, &panoramic_name, style_class)
                    },
                )?;

                parser.process_next_token()
            },
        )
    }

    fn process_panoramic_children(
        &mut self,
        class_name: &str,
        panoramic_name: &str,
        style_class: &mut NenyrStyleClass,
    ) -> NenyrResult<()> {
        match self.current_token {
            NenyrTokens::CurlyBracketClose => Ok(()),
            _ => {
                self.process_patterns_methods(
                    class_name,
                    style_class,
                    true,
                    &Some(panoramic_name.to_string()),
                )?;

                // Processes the next token
                self.process_next_token()
            }
        }
    }
}

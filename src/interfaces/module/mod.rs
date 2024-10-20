use crate::{
    error::{NenyrError, NenyrErrorKind},
    loop_while_not,
    tokens::NenyrTokens,
    types::module::ModuleContext,
    validators::identifier::NenyrIdentifierValidator,
    NenyrParser, NenyrResult,
};

impl<'a> NenyrParser<'a> {
    pub(crate) fn process_module_context(&mut self) -> NenyrResult<ModuleContext> {
        self.process_next_token()?;

        let module_name = self.retrieve_module_or_extending_name(
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
        self.set_context_name(Some(module_name.clone()));

        let extending_from = self.retrieve_extending_from()?;

        self.parse_curly_bracketed_delimiter(
            Some("".to_string()),
            "",
            Some("".to_string()),
            "",
            |parser| parser.retrieve_module_context_block(&module_name, &extending_from),
        )
    }

    fn retrieve_extending_from(&mut self) -> NenyrResult<Option<String>> {
        if let NenyrTokens::Extending = self.current_token {
            self.process_next_token()?;

            let extending_from = self.retrieve_module_or_extending_name(
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

            return Ok(Some(extending_from));
        }

        Ok(None)
    }

    fn retrieve_module_or_extending_name(
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
        let module_name = self.parse_parenthesized_delimiter(
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

        if !self.is_valid_identifier(&module_name) {
            return Err(NenyrError::new(
                suggestion_on_invalid,
                self.context_name.clone(),
                self.context_path.to_string(),
                self.add_nenyr_token_to_error(error_message_on_invalid),
                NenyrErrorKind::SyntaxError,
                self.get_tracing(),
            ));
        }

        Ok(module_name)
    }

    fn retrieve_module_context_block(
        &mut self,
        module_name: &str,
        extending_from: &Option<String>,
    ) -> NenyrResult<ModuleContext> {
        let mut module_context =
            ModuleContext::new(module_name.to_string(), extending_from.clone());

        loop_while_not!(
            self,
            Some("".to_string()),
            "",
            Some("".to_string()),
            "",
            || self.processing_state.is_context_active(),
            |is_active| self.processing_state.set_context_active(is_active),
            {
                self.handle_module_declare_keyword(&mut module_context)?;
            }
        );

        self.processing_state.set_context_active(false);

        Ok(module_context)
    }

    fn handle_module_declare_keyword(
        &mut self,
        module_context: &mut ModuleContext,
    ) -> NenyrResult<()> {
        self.parse_declare_keyword(Some("".to_string()), "")?;

        self.process_module_methods(module_context)
    }

    fn process_module_methods(&mut self, _module_context: &mut ModuleContext) -> NenyrResult<()> {
        self.processing_state.set_context_active(true);

        match self.current_token {
            NenyrTokens::Aliases => {}
            NenyrTokens::Variables => {}
            NenyrTokens::Animation => {}
            NenyrTokens::Class => {}
            _ => {
                return Err(NenyrError::new(
                    Some("".to_string()),
                    self.context_name.clone(),
                    self.context_path.to_string(),
                    self.add_nenyr_token_to_error(""),
                    NenyrErrorKind::SyntaxError,
                    self.get_tracing(),
                ));
            }
        }

        Ok(())
    }
}

use crate::{
    error::{NenyrError, NenyrErrorKind},
    loop_while_not,
    tokens::NenyrTokens,
    types::layout::LayoutContext,
    validators::identifier::NenyrIdentifierValidator,
    NenyrParser, NenyrResult,
};

impl<'a> NenyrParser<'a> {
    pub(crate) fn process_layout_context(&mut self) -> NenyrResult<LayoutContext> {
        self.process_next_token()?;

        let layout_name = self.retrieve_layout_name()?;

        self.set_context_name(Some(layout_name.clone()));
        self.process_next_token()?;
        self.parse_curly_bracketed_delimiter(
            Some("".to_string()),
            "",
            Some("".to_string()),
            "",
            |parser| parser.retrieve_layout_context_block(&layout_name),
        )
    }

    fn retrieve_layout_name(&mut self) -> NenyrResult<String> {
        let layout_name = self.parse_parenthesized_delimiter(
            Some("".to_string()),
            "",
            Some("".to_string()),
            "",
            |parser| parser.parse_string_literal(Some("".to_string()), "", true),
        )?;

        if !self.is_valid_identifier(&layout_name) {
            return Err(NenyrError::new(
                Some("".to_string()),
                self.context_name.clone(),
                self.context_path.to_string(),
                self.add_nenyr_token_to_error(""),
                NenyrErrorKind::SyntaxError,
                self.get_tracing(),
            ));
        }

        Ok(layout_name)
    }

    fn retrieve_layout_context_block(&mut self, layout_name: &str) -> NenyrResult<LayoutContext> {
        let mut layout_context = LayoutContext::new(layout_name.to_string());

        loop_while_not!(
            self,
            Some("".to_string()),
            "",
            Some("".to_string()),
            "",
            || self.processing_state.is_context_active(),
            |is_active| self.processing_state.set_context_active(is_active),
            {
                self.handle_layout_declare_keyword(&mut layout_context)?;
            }
        );

        self.processing_state.set_context_active(false);

        Ok(layout_context)
    }

    fn handle_layout_declare_keyword(
        &mut self,
        layout_context: &mut LayoutContext,
    ) -> NenyrResult<()> {
        self.parse_declare_keyword(Some("".to_string()), "")?;

        self.process_layout_methods(layout_context)
    }

    fn process_layout_methods(&mut self, _layout_context: &mut LayoutContext) -> NenyrResult<()> {
        self.processing_state.set_context_active(true);

        match self.current_token {
            NenyrTokens::Aliases => {}
            NenyrTokens::Variables => {}
            NenyrTokens::Themes => {}
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

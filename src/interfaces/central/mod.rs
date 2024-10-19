use crate::{
    error::{NenyrError, NenyrErrorKind},
    tokens::NenyrTokens,
    types::central::CentralContext,
    NenyrParser, NenyrResult,
};

impl<'a> NenyrParser<'a> {
    pub(crate) fn process_central_context(&mut self) -> NenyrResult<CentralContext> {
        self.parse_central_keyword(
            Some("Ensure the correct syntax is used, beginning with `Construct Central { ... }` to properly define the context.".to_string()),
            "Expected the `Central` keyword to follow the `Construct` keyword, but it was not found.".to_string(),
            Self::handle_central_context_delimiter,
        )
    }

    fn handle_central_context_delimiter(&mut self) -> NenyrResult<CentralContext> {
        self.set_context_name(Some("Central".to_string()));

        self.parse_curly_bracketed_delimiter(
            Some("".to_string()),
            "".to_string(),
            Some("".to_string()),
            "".to_string(),
            Self::retrieve_central_context_block,
        )
    }

    fn retrieve_central_context_block(&mut self) -> NenyrResult<CentralContext> {
        let mut central_context = CentralContext::new();

        // TODO: Create a macro to handle the loop, make it receive self and a block of code. In the block of code I pass the next handler to be called.
        while self.current_token != NenyrTokens::CurlyBracketClose {
            if let NenyrTokens::Comma = self.current_token {
                if self.processing_state.is_context_active() {
                    self.processing_state.set_context_active(false);
                    self.process_next_token()?;

                    continue;
                } else {
                    return Err(NenyrError::new(
                        Some("".to_string()),
                        self.context_name.clone(),
                        self.context_path.to_string(),
                        self.add_nenyr_token_to_error("".to_string()),
                        NenyrErrorKind::SyntaxError,
                        self.get_tracing(),
                    ));
                }
            }

            if self.processing_state.is_context_active() {
                return Err(NenyrError::new(
                    Some("".to_string()),
                    self.context_name.clone(),
                    self.context_path.to_string(),
                    self.add_nenyr_token_to_error("".to_string()),
                    NenyrErrorKind::SyntaxError,
                    self.get_tracing(),
                ));
            }

            self.handle_central_declare_keyword(&mut central_context)?;
            self.process_next_token()?;
        }

        self.processing_state.set_context_active(false);

        Ok(central_context)
    }

    fn handle_central_declare_keyword(
        &mut self,
        central_context: &mut CentralContext,
    ) -> NenyrResult<()> {
        self.parse_declare_keyword(Some("".to_string()), "".to_string())?;

        self.process_central_methods(central_context)
    }

    fn process_central_methods(&mut self, central_context: &mut CentralContext) -> NenyrResult<()> {
        self.processing_state.set_context_active(true);

        match self.current_token {
            NenyrTokens::Imports => {}
            NenyrTokens::Typefaces => {}
            NenyrTokens::Breakpoints => {}
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
                    self.add_nenyr_token_to_error("".to_string()),
                    NenyrErrorKind::SyntaxError,
                    self.get_tracing(),
                ));
            }
        }

        Ok(())
    }
}

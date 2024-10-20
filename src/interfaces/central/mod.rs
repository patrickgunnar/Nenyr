use crate::{
    error::{NenyrError, NenyrErrorKind},
    loop_while_not,
    tokens::NenyrTokens,
    types::central::CentralContext,
    NenyrParser, NenyrResult,
};

impl<'a> NenyrParser<'a> {
    pub(crate) fn process_central_context(&mut self) -> NenyrResult<CentralContext> {
        self.process_next_token()?;
        self.set_context_name(Some("Central".to_string()));

        self.parse_curly_bracketed_delimiter(
            Some("Ensure the construction of the central context includes an opening curly bracket `{` to define the start of the context block. The correct syntax is `Construct Central { ... }`.".to_string()),
            "Expected an opening curly bracket `{` after the `Central` keyword to start the central context block, but none was found.",
            Some("Ensure the central context block is properly closed with a curly bracket `}`. The correct syntax is `Construct Central { ... }`.".to_string()),
            "Expected a closing curly bracket `}` to end the central context block, but none was found.",
            Self::retrieve_central_context_block,
        )
    }

    fn retrieve_central_context_block(&mut self) -> NenyrResult<CentralContext> {
        let mut central_context = CentralContext::new();

        loop_while_not!(
            self,
            Some("Remove any duplicated commas from the Nenyr central context to ensure proper syntax. The parser expects the `Declare` keyword to follow valid delimiters.".to_string()),
            "A duplicated comma was found inside the Nenyr central context. The parser expected to find the `Declare` keyword but did not.",
            Some("Ensure that a comma is placed after each block definition to separate elements correctly. Proper syntax is required for the parser to process the context. Example: `Construct Central { Declare Typefaces({ ... }), Declare Imports([ ... ]), ... }`.".to_string()),
            "All methods inside the central context block must be separated by commas. A comma is missing after the block definition. The parser expected a comma to separate elements but did not find one.",
            || self.processing_state.is_context_active(),
            |is_active| self.processing_state.set_context_active(is_active),
            {
                self.handle_central_declare_keyword(&mut central_context)?;
            }
        );

        self.processing_state.set_context_active(false);

        Ok(central_context)
    }

    fn handle_central_declare_keyword(
        &mut self,
        central_context: &mut CentralContext,
    ) -> NenyrResult<()> {
        self.parse_declare_keyword(
            Some("Ensure that each method inside the central context is correctly declared using the `Declare` keyword. Example: `Construct Central { Declare Breakpoints({ ... }), Declare Classes([ ... ]), ... }`.".to_string()),
            "All methods declaration inside the central context must begin with the `Declare` keyword, but one of the methods is missing this declaration."
        )?;

        self.process_central_methods(central_context)
    }

    fn process_central_methods(
        &mut self,
        _central_context: &mut CentralContext,
    ) -> NenyrResult<()> {
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
                    Some("Ensure that only valid methods supported by the central context are declared. Review the documentation for methods allowed within `Construct Central { ... }`.".to_string()),
                    self.context_name.clone(),
                    self.context_path.to_string(),
                    self.add_nenyr_token_to_error("One of the methods in the central context is either not a valid Nenyr method or is not supported within the central context."),
                    NenyrErrorKind::SyntaxError,
                    self.get_tracing(),
                ));
            }
        }

        Ok(())
    }
}

use crate::{
    error::{NenyrError, NenyrErrorKind},
    tokens::NenyrTokens,
    NenyrParser, NenyrResult,
};

impl<'a> NenyrParser<'a> {
    pub(crate) fn parse_curly_bracketed_delimiter<F, T>(
        &mut self,
        suggestion_on_open: Option<String>,
        error_message_on_open: String,
        suggestion_on_close: Option<String>,
        error_message_on_close: String,
        parse_fn: F,
    ) -> NenyrResult<T>
    where
        F: Fn(&mut Self) -> NenyrResult<T>,
    {
        if let NenyrTokens::CurlyBracketOpen = self.current_token {
            self.process_next_token()?;

            let parsed_value = parse_fn(self)?;

            if let NenyrTokens::CurlyBracketClose = self.current_token {
                return Ok(parsed_value);
            }

            return Err(NenyrError::new(
                suggestion_on_close,
                self.context_name.clone(),
                self.context_path.to_string(),
                self.add_nenyr_token_to_error(error_message_on_close),
                NenyrErrorKind::SyntaxError,
                self.get_tracing(),
            ));
        }

        Err(NenyrError::new(
            suggestion_on_open,
            self.context_name.clone(),
            self.context_path.to_string(),
            self.add_nenyr_token_to_error(error_message_on_open),
            NenyrErrorKind::SyntaxError,
            self.get_tracing(),
        ))
    }
}

use crate::{
    error::{NenyrError, NenyrErrorKind},
    tokens::NenyrTokens,
    NenyrParser, NenyrResult,
};

impl<'a> NenyrParser<'a> {
    pub(crate) fn parse_construct_keyword<F, T>(
        &mut self,
        suggestion: Option<String>,
        error_message: String,
        parse_fn: F,
    ) -> NenyrResult<T>
    where
        F: Fn(&mut Self) -> NenyrResult<T>,
    {
        if let NenyrTokens::Construct = self.current_token {
            self.process_next_token()?;

            return parse_fn(self);
        }

        Err(NenyrError::new(
            suggestion,
            None,
            self.context_path.to_string(),
            self.add_nenyr_token_to_error(error_message),
            NenyrErrorKind::SyntaxError,
            self.get_tracing(),
        ))
    }

    pub(crate) fn parse_central_keyword<F, T>(
        &mut self,
        suggestion: Option<String>,
        error_message: String,
        parse_fn: F,
    ) -> NenyrResult<T>
    where
        F: Fn(&mut Self) -> NenyrResult<T>,
    {
        if let NenyrTokens::Central = self.current_token {
            self.process_next_token()?;

            return parse_fn(self);
        }

        Err(NenyrError::new(
            suggestion,
            Some("Central".to_string()),
            self.context_path.to_string(),
            self.add_nenyr_token_to_error(error_message),
            NenyrErrorKind::SyntaxError,
            self.get_tracing(),
        ))
    }

    pub(crate) fn parse_declare_keyword(
        &mut self,
        suggestion: Option<String>,
        error_message: String,
    ) -> NenyrResult<()> {
        if let NenyrTokens::Declare = self.current_token {
            self.process_next_token()?;

            return Ok(());
        }

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

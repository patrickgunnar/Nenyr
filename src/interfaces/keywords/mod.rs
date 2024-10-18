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

        let error_message = format!(
            "{} However, found `{:?}` instead.",
            error_message, self.current_token
        );

        Err(NenyrError::new(
            suggestion,
            None,
            self.context_path.to_string(),
            error_message,
            NenyrErrorKind::SyntaxError,
            self.get_tracing(),
        ))
    }
}

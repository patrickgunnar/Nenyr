use crate::{error::NenyrErrorTracing, NenyrParser, NenyrResult};

impl<'a> NenyrParser<'a> {
    pub(crate) fn process_next_token(&mut self) -> NenyrResult<()> {
        self.current_token = self.lexer.next_token()?;

        Ok(())
    }

    pub(crate) fn get_tracing(&self) -> NenyrErrorTracing {
        self.lexer.trace_lexer_position()
    }

    pub(crate) fn set_context_name(&mut self, context_name: Option<String>) {
        self.lexer.set_context_name(context_name);
    }
}

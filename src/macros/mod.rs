/// # Macro: `loop_while_not!`
///
/// This macro provides a structured way to loop over tokens in a block and validates the
/// presence or absence of commas while ensuring proper parsing states in the context
/// of Nenyr tokens. The macro ensures the following:
/// - **Handling duplicated commas**: Detects and provides feedback if a duplicated comma
///   is found when the state does not expect one.
/// - **Handling missing commas**: Detects if a comma is missing where it is expected,
///   triggering an error.
///
/// The macro processes each token until a closing curly bracket (`}`) is encountered.
/// It checks for both missing and duplicated commas and manages the parsing state using
/// the provided functions and error messages.
///
/// ## Parameters
/// - `$self`: The parser instance, used to access the current token and manage the parser's state.
/// - `$duplicated_comma_suggestion`: A string slice representing the suggestion message for
///   resolving duplicated commas.
/// - `$duplicated_comma_error_message`: A string slice representing the error message when
///   a duplicated comma is encountered.
/// - `$missing_comma_suggestion`: A string slice representing the suggestion message for
///   resolving missing commas.
/// - `$missing_comma_error_message`: A string slice representing the error message for a missing comma.
/// - `$is_active_fn`: A function or closure that returns a `bool`, indicating whether the current
///   parsing state is active (i.e., expecting a comma).
/// - `$set_active_state`: A function or closure that sets the active state of the parsing process.
///   Typically, this would toggle the state when a comma is processed.
/// - `$body`: A block of code that represents the core logic to be executed within each iteration
///   of the loop for each token.
///
/// ## Behavior
/// - The macro iterates over the tokens within a block, processing each one and checking for
///   proper syntax regarding commas.
/// - If a **comma** is encountered, the macro checks if the parser is in a state where a comma
///   is allowed (`$is_active_fn`). If it is allowed, the state is deactivated and the next token
///   is processed. If a comma is not allowed, it triggers an error using the provided
///   `$duplicated_comma_suggestion` and `$duplicated_comma_error_message`.
/// - If the state is active but no comma is encountered where expected, the macro generates
///   an error using the `$missing_comma_suggestion` and `$missing_comma_error_message`.
///
/// ## Error Handling
/// This macro returns an error in two cases:
/// 1. **Duplicated Comma**: If a comma appears in a context where it is not allowed, an error
///    is raised with the duplicated comma error message.
/// 2. **Missing Comma**: If the parsing state expects a comma, but none is found, an error
///    is raised with the missing comma error message.
#[macro_export]
macro_rules! loop_while_not {
    (
        $self:expr,
        $duplicated_comma_suggestion:expr,
        $duplicated_comma_error_message:expr,
        $missing_comma_suggestion:expr,
        $missing_comma_error_message:expr,
        $is_active_fn:expr,
        $set_active_state:expr,
        $body:block
    ) => {{
        // Loop through the tokens until a closing curly bracket (`}`) is found.
        while $self.current_token != NenyrTokens::CurlyBracketClose
            && $self.current_token != NenyrTokens::SquareBracketClose
        {
            // If the current token is a comma, handle it based on the active state.
            if let NenyrTokens::Comma = $self.current_token {
                // If the state allows a comma, deactivate the state and process the next token.
                if $is_active_fn() {
                    $set_active_state(false);
                    $self.process_next_token()?;

                    continue;

                // If the state does not allow a comma, return an error for duplicated comma.
                } else {
                    return Err(NenyrError::new(
                        $duplicated_comma_suggestion,
                        $self.context_name.clone(),
                        $self.context_path.to_string(),
                        $self.add_nenyr_token_to_error($duplicated_comma_error_message),
                        NenyrErrorKind::SyntaxError,
                        $self.get_tracing(),
                    ));
                }
            }

            // If the state is active (i.e., a comma is expected), but no comma is found,
            // return an error indicating a missing comma.
            if $is_active_fn() {
                return Err(NenyrError::new(
                    $missing_comma_suggestion,
                    $self.context_name.clone(),
                    $self.context_path.to_string(),
                    $self.add_nenyr_token_to_error($missing_comma_error_message),
                    NenyrErrorKind::SyntaxError,
                    $self.get_tracing(),
                ));
            }

            // Process the body of the macro, which defines the custom logic.
            {
                $body
            }

            // Move to the next token for further processing.
            $self.process_next_token()?;
        }
    }};
}

use crate::{
    error::{NenyrError, NenyrErrorKind},
    loop_while_not,
    tokens::NenyrTokens,
    types::central::CentralContext,
    NenyrParser, NenyrResult,
};

/// # NenyrParser Central Context Parsing Methods
///
/// This implementation of the `NenyrParser` provides methods specifically designed
/// to parse blocks of code that define the central context within the Nenyr language.
/// The central context is crucial for the overall structure and function of the Nenyr
/// program, and these methods ensure that the syntax adheres to the expected rules
/// and conventions for central context declarations.
///
/// The methods in this group validate the syntax, process specific tokens, and
/// generate meaningful error messages if the input does not conform to the expected
/// structure. These methods work together to build a complete representation of
/// the central context, encapsulated in the `CentralContext` type.
impl<'a> NenyrParser<'a> {
    /// Processes the central context declaration.
    ///
    /// This method initiates the parsing of a central context block by moving to the
    /// next token after the `Central` keyword. It expects an opening curly bracket
    /// `{` to signify the start of the block. If the expected tokens are not found,
    /// it generates appropriate error messages to guide the user in correcting the syntax.
    ///
    /// The method then calls `retrieve_central_context_block` to handle the contents
    /// of the block. Upon successful parsing, it returns an instance of `CentralContext`.
    ///
    /// # Errors
    /// Returns a `NenyrError` if the expected opening or closing curly brackets are
    /// not found, or if there are syntax issues within the central context block.
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

    /// Retrieves the contents of the central context block.
    ///
    /// This method processes the individual declarations within the central context block
    /// and validates them using the `loop_while_not!` macro to ensure correct syntax.
    /// It handles multiple declarations that are separated by commas, and it specifically
    /// looks for the `Declare` keyword as part of the declarations.
    ///
    /// If a syntax error, such as a missing comma or a duplicated comma, is detected,
    /// an appropriate error message is generated to inform the user of the issue.
    ///
    /// # Errors
    /// Returns a `NenyrError` if the expected syntax is not followed within the block,
    /// including issues like duplicated commas or missing declarations.
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

    /// Handles the parsing of declarations within the central context.
    ///
    /// This method specifically looks for the `Declare` keyword and validates that
    /// each declaration adheres to the expected syntax rules. If the `Declare` keyword
    /// is missing or malformed, an error is raised to guide the user in correcting the
    /// syntax.
    ///
    /// The method then calls `process_central_methods` to handle the actual methods
    /// declared within the context.
    ///
    /// # Parameters
    /// - `central_context`: A mutable reference to the `CentralContext` where parsed
    ///   declarations will be stored.
    ///
    /// # Errors
    /// Returns a `NenyrError` if the declaration does not start with the `Declare`
    /// keyword or if any other syntax issues are encountered.
    fn handle_central_declare_keyword(
        &mut self,
        central_context: &mut CentralContext,
    ) -> NenyrResult<()> {
        self.parse_declare_keyword(
            Some("Ensure that each method inside the central context is correctly declared using the `Declare` keyword. Example: `Construct Central { Declare Breakpoints({ ... }), Declare Class({ ... }), ... }`.".to_string()),
            "All methods declaration inside the central context must begin with the `Declare` keyword, but one of the methods is missing this declaration."
        )?;

        self.process_central_methods(central_context)
    }

    /// Processes individual method declarations within the central context.
    ///
    /// This method validates the current token to ensure it corresponds to one of the
    /// valid methods that can be declared within the central context. It checks for
    /// specific tokens like `Imports`, `Typefaces`, `Breakpoints`, and others.
    ///
    /// If an invalid method is encountered, an error is generated with a suggestion
    /// for correcting the issue based on the expected methods allowed in the central
    /// context.
    ///
    /// # Parameters
    /// - `central_context`: A mutable reference to the `CentralContext` instance to
    ///   which methods will be added.
    ///
    /// # Errors
    /// Returns a `NenyrError` if an unsupported method is declared or if the current
    /// token does not match any valid method.
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
            NenyrTokens::Class => {
                let (class_name, style_class) = self.process_class_method()?;

                central_context.add_style_class_to_context(class_name, style_class);
            }
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

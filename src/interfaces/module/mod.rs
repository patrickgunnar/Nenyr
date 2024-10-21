use crate::{
    error::{NenyrError, NenyrErrorKind},
    loop_while_not,
    tokens::NenyrTokens,
    types::module::ModuleContext,
    validators::identifier::NenyrIdentifierValidator,
    NenyrParser, NenyrResult,
};

/// # NenyrParser Module Context Parsing Methods
///
/// This implementation of the `NenyrParser` provides methods specifically designed
/// to parse blocks of code that define the module context within the Nenyr language.
/// The module context is crucial for the overall structure and function of the Nenyr
/// program, and these methods ensure that the syntax adheres to the expected rules
/// and conventions for module context declarations.
///
/// The methods in this group validate the syntax, process specific tokens, and
/// generate meaningful error messages if the input does not conform to the expected
/// structure. These methods work together to build a complete representation of
/// the module context, encapsulated in the `ModuleContext` type.
impl<'a> NenyrParser<'a> {
    /// Processes a module context within the Nenyr DSL.
    ///
    /// This method initiates the parsing of a module context, expecting a `Module` keyword followed
    /// by an open parenthesis `(`, a module name, and an optional extending name. It also ensures that
    /// the module context is properly defined within curly brackets `{}`.
    ///
    /// # Returns
    /// - `NenyrResult<ModuleContext>`: A result containing the successfully parsed `ModuleContext`
    ///   or an error if the parsing fails.
    ///
    /// # Errors
    /// - Returns an error if the `Module` keyword is not followed by an open parenthesis `(`, if the
    ///   module name is invalid, or if the curly brackets are not properly balanced.
    pub(crate) fn process_module_context(&mut self) -> NenyrResult<ModuleContext> {
        self.process_next_token()?;

        let module_name = self.retrieve_module_or_extending_name(
            Some("Ensure that the `Module` keyword is followed by an open parenthesis `(` for proper declaration. Example: `Construct Module('moduleName') { ... }`.".to_string()),
            "Expected an open parenthesis `(` after the `Module` keyword to declare the module name, but it was not found.",
            Some("Ensure that the module name in the module declaration is properly closed with a parenthesis `)`. Example: `Construct Module('moduleName') { ... }`.".to_string()),
            "Expected a closing parenthesis `)` after the module name in the module declaration, but it was not found.",
            Some("Ensure that the module context name declaration includes a valid name. It should consist only of letters and numbers, with the first character being a letter. Example: `Construct Module('moduleName01') { ... }`.".to_string()),
            "The module context name declaration must receive a non-empty string as the module name, but none was found.",
            Some("A valid module context name should contain only alphanumeric characters, with the first character being an alphabetic letter. Examples: `'myModuleContext01'`, `'module01'`, etc.".to_string()),
            "The validation of the module context name failed. The provided name does not meet the required format.",
        )?;

        self.process_next_token()?;
        self.set_context_name(Some(module_name.clone()));

        let extending_from = self.retrieve_extending_from()?;

        self.parse_curly_bracketed_delimiter(
            Some("Ensure that the module context or extending name declaration is followed by an opening curly bracket `{` to define the module context. Example: `Construct Module('moduleName') { ... } or Construct Module('moduleName') Extending('layoutName') { ... }`.".to_string()),
            "Expected an opening curly bracket `{` after the module context or extending name declaration to start the module context block, but it was not found.",
            Some("Ensure that each opened curly bracket `{` is properly closed with a corresponding closing curly bracket `}`. Example: `Construct Module('moduleName') { ... }`.".to_string()),
            "Expected a closing curly bracket `}` to terminate the module context block declaration, but it was not found.",
            |parser| parser.retrieve_module_context_block(&module_name, &extending_from),
        )
    }

    /// Retrieves the name of the context being extended from, if applicable.
    ///
    /// This method checks if the current token indicates an `Extending` declaration and processes
    /// it accordingly. If an extending name is found, it validates the name and returns it.
    ///
    /// # Returns
    /// - `NenyrResult<Option<String>>`: An optional result containing the extending name if present,
    ///   or `None` if there is no extending context.
    ///
    /// # Errors
    /// - Returns an error if the `Extending` keyword is not followed by an open parenthesis `(`, if
    ///   the extending name is invalid, or if the closing parenthesis `)` is missing.
    fn retrieve_extending_from(&mut self) -> NenyrResult<Option<String>> {
        if let NenyrTokens::Extending = self.current_token {
            self.process_next_token()?;

            let extending_from = self.retrieve_module_or_extending_name(
                Some("Ensure that the `Extending` keyword is followed by an open parenthesis '(' for proper declaration. Example: `Construct Module('moduleName') Extending('layoutName') { ... }`.".to_string()),
                "Expected an open parenthesis '(' after the `Extending` keyword to declare the extending name, but it was not found.",
                Some("Ensure that the extending name in the module declaration is properly closed with a parenthesis ')'. Example: `Construct Module('moduleName') Extending('layoutName') { ... }`.".to_string()),
                "Expected a closing parenthesis ')' after the extending name in the module declaration, but it was not found.",
                Some("Ensure that the extending name in the module context declaration includes a valid name. It should consist only of letters and numbers, with the first character being a letter. Example: `Construct Module('moduleName01') Extending('layoutName') { ... }`.".to_string()),
                "The extending name in the module context declaration must receive a non-empty string, but none was found.",
                Some("A valid extending name in the module context declaration should contain only alphanumeric characters, with the first character being an alphabetic letter. Examples: 'myLayoutContext01', 'layout01', etc.".to_string()),
                "The validation of the extending name in the module context declaration failed. The provided name does not meet the required format.",
            )?;

            self.process_next_token()?;

            return Ok(Some(extending_from));
        }

        Ok(None)
    }

    /// Retrieves the name of the module or extending context.
    ///
    /// This method handles the parsing of the name declared in either the module or extending context.
    /// It ensures that the name is enclosed in parentheses, validates it against the naming rules, and
    /// returns the valid name.
    ///
    /// # Parameters
    /// - `suggestion_on_open`: An optional string providing a suggestion for correctly opening the declaration.
    /// - `error_message_on_open`: A message to display if an open parenthesis is expected but not found.
    /// - `suggestion_on_close`: An optional string providing a suggestion for correctly closing the declaration.
    /// - `error_message_on_close`: A message to display if a close parenthesis is expected but not found.
    /// - `suggestion_on_parse_literal`: An optional string providing a suggestion for parsing a literal.
    /// - `error_message_on_parse_literal`: A message to display if there is an error parsing the literal.
    /// - `suggestion_on_invalid`: An optional string providing a suggestion if the identifier is invalid.
    /// - `error_message_on_invalid`: A message to display if the identifier does not meet the required format.
    ///
    /// # Returns
    /// - `NenyrResult<String>`: A result containing the valid module or extending name.
    ///
    /// # Errors
    /// - Returns an error if the name is not enclosed in parentheses or does not adhere to naming conventions.
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

    /// Processes the block of the current module context.
    ///
    /// This method initializes a new `ModuleContext` and enters a loop that
    /// continues processing while the context has children. It expects to handle
    /// declarations of various methods inside the module context.
    ///
    /// # Parameters
    ///
    /// - `module_name`: The name of the module being declared.
    /// - `extending_from`: An optional string that represents the name of the
    ///   module being extended.
    ///
    /// # Returns
    ///
    /// Returns a `NenyrResult<ModuleContext>` which contains the processed
    /// `ModuleContext` on success.
    ///
    /// # Errors
    ///
    /// This function may return a `NenyrError` if:
    /// - Duplicate commas are found in the module context.
    /// - The `Declare` keyword is missing after a valid delimiter.
    /// - A method inside the module context is missing a comma separator.
    fn retrieve_module_context_block(
        &mut self,
        module_name: &str,
        extending_from: &Option<String>,
    ) -> NenyrResult<ModuleContext> {
        let mut module_context =
            ModuleContext::new(module_name.to_string(), extending_from.clone());

        loop_while_not!(
            self,
            Some("Remove any duplicated commas from the module context to ensure proper syntax. The parser expects the `Declare` keyword to follow valid delimiters. Example: `Construct Module('moduleName') { Declare Class({ ... }), Declare Animation({ ... }), ... }`.".to_string()),
            "A duplicated comma was found inside the module context. The parser expected to find the `Declare` keyword, but it was not found.",
            Some("Ensure that a comma is placed after each block definition to separate elements correctly. Proper syntax is required for the parser to process the context. Example: `Construct Module('moduleName') { Declare Variables({ ... }), Declare Aliases({ ... }), ... }`.".to_string()),
            "All methods inside the module context block must be separated by commas. A comma is missing after the block definition. The parser expected a comma to separate elements but did not find one.",
            || self.processing_state.is_context_active(),
            |is_active| self.processing_state.set_context_active(is_active),
            {
                self.handle_module_declare_keyword(&mut module_context)?;
            }
        );

        self.processing_state.set_context_active(false);

        Ok(module_context)
    }

    /// Handles the `Declare` keyword within the module context.
    ///
    /// This method checks that each method inside the module context is properly
    /// declared using the `Declare` keyword. It also processes the methods
    /// declared within the context.
    ///
    /// # Parameters
    ///
    /// - `module_context`: A mutable reference to the `ModuleContext` that is
    ///   being populated with methods.
    ///
    /// # Returns
    ///
    /// Returns a `NenyrResult<()>` indicating success or failure.
    ///
    /// # Errors
    ///
    /// This function may return a `NenyrError` if:
    /// - A method declaration inside the module context does not start with the
    ///   `Declare` keyword.
    fn handle_module_declare_keyword(
        &mut self,
        module_context: &mut ModuleContext,
    ) -> NenyrResult<()> {
        self.parse_declare_keyword(
            Some("Ensure that each method inside the module context is correctly declared using the `Declare` keyword. Example: `Construct Module('moduleName') { Declare Animation({ ... }), Declare Class({ ... }), ... }`.".to_string()),
            "All method declarations inside the module context must begin with the `Declare` keyword, but one of the methods is missing this declaration."
        )?;

        self.process_module_methods(module_context)
    }

    /// Processes the methods declared within the module context.
    ///
    /// This method evaluates the current token to determine the type of
    /// method being declared. If the token matches any valid method types,
    /// it allows for processing to continue; otherwise, it raises an error.
    ///
    /// # Parameters
    ///
    /// - `module_context`: A mutable reference to the `ModuleContext` to
    ///   which methods are being added.
    ///
    /// # Returns
    ///
    /// Returns a `NenyrResult<()>` indicating success or failure.
    ///
    /// # Errors
    ///
    /// This function may return a `NenyrError` if:
    /// - An invalid method type is encountered that is not supported within
    ///   the module context.
    fn process_module_methods(&mut self, _module_context: &mut ModuleContext) -> NenyrResult<()> {
        self.processing_state.set_context_active(true);

        match self.current_token {
            NenyrTokens::Aliases => {}
            NenyrTokens::Variables => {}
            NenyrTokens::Animation => {}
            NenyrTokens::Class => {}
            _ => {
                return Err(NenyrError::new(
                    Some("Ensure that only valid methods supported by the module context are declared. Review the documentation for methods allowed within `Construct Module('moduleName') { ... }`.".to_string()),
                    self.context_name.clone(),
                    self.context_path.to_string(),
                    self.add_nenyr_token_to_error("One of the methods in the module context is either not a valid Nenyr method or is not supported within the module context."),
                    NenyrErrorKind::SyntaxError,
                    self.get_tracing(),
                ));
            }
        }

        Ok(())
    }
}

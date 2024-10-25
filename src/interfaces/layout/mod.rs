use crate::{
    error::{NenyrError, NenyrErrorKind},
    loop_while_not,
    tokens::NenyrTokens,
    types::layout::LayoutContext,
    validators::identifier::NenyrIdentifierValidator,
    NenyrParser, NenyrResult,
};

/// # NenyrParser Layout Context Parsing Methods
///
/// This implementation of the `NenyrParser` provides methods specifically designed
/// to parse blocks of code that define the layout context within the Nenyr language.
/// The layout context is crucial for the overall structure and function of the Nenyr
/// program, and these methods ensure that the syntax adheres to the expected rules
/// and conventions for layout context declarations.
///
/// The methods in this group validate the syntax, process specific tokens, and
/// generate meaningful error messages if the input does not conform to the expected
/// structure. These methods work together to build a complete representation of
/// the layout context, encapsulated in the `LayoutContext` type.
impl<'a> NenyrParser<'a> {
    /// Processes a layout context based on the current token stream.
    ///
    /// This method first retrieves the layout name, sets the current context, and
    /// then expects an opening curly bracket `{` to initiate the layout context block.
    /// It ensures that the block is properly delimited with matching braces. The layout
    /// context is built by invoking `retrieve_layout_context_block`, which handles
    /// the parsing of methods within the block.
    ///
    /// # Returns
    /// - `NenyrResult<LayoutContext>`: The resulting layout context, encapsulating
    ///   the defined layout and its associated methods.
    ///
    /// # Errors
    /// - Will return an error if the layout name is invalid or if the expected
    ///   delimiters are not found.
    pub(crate) fn process_layout_context(&mut self) -> NenyrResult<LayoutContext> {
        self.process_next_token()?;

        let layout_name = self.retrieve_layout_name()?;

        self.set_context_name(Some(layout_name.clone()));
        self.process_next_token()?;
        self.parse_curly_bracketed_delimiter(
            Some("Ensure that the layout context name declaration is followed by an opening curly bracket `{` to define the layout context. Example: `Construct Layout('layoutName') { ... }`.".to_string()),
            "Expected an opening curly bracket `{` after the layout context name declaration to start the layout context block, but it was not found.",
            Some("Ensure that each opened curly bracket `{` is properly closed with a corresponding closing curly bracket `}`. Example: `Construct Layout('layoutName') { ... }`.".to_string()),
            "Expected a closing curly bracket `}` to terminate the layout context block declaration, but it was not found.",
            |parser| parser.retrieve_layout_context_block(&layout_name),
        )
    }

    /// Retrieves and validates the layout name from the layout context declaration.
    ///
    /// This method parses the expected parenthetical format of the layout declaration,
    /// ensuring it follows the structure `Construct Layout('layoutName')`. It validates
    /// that the layout name is a proper identifier according to the rules defined in
    /// the Nenyr DSL.
    ///
    /// # Returns
    /// - `NenyrResult<String>`: The valid layout name as a string.
    ///
    /// # Errors
    /// - Will return an error if the layout name is missing or does not adhere to the
    ///   valid identifier rules.
    fn retrieve_layout_name(&mut self) -> NenyrResult<String> {
        let layout_name = self.parse_parenthesized_delimiter(
            Some("Ensure that the `Layout` keyword is followed by an open parenthesis `(` for proper declaration. Example: `Construct Layout('layoutName') { ... }`.".to_string()),
            "Expected an open parenthesis `(` after the `Layout` keyword to declare the layout name, but it was not found.",
            Some("Ensure that the layout name in the layout declaration is properly closed with a parenthesis `)`. Example: `Construct Layout('layoutName') { ... }`.".to_string()),
            "Expected a closing parenthesis `)` after the layout name in the layout declaration, but it was not found.",
            |parser| parser.parse_string_literal(
                Some("Ensure that the layout context name declaration includes a valid name. It should consist only of letters and numbers, with the first character being a letter. Example: `Construct Layout('layoutName01') { ... }`.".to_string()),
                "The layout context name declaration must receive a non-empty string as the layout name, but none was found.",
                true
            ),
        )?;

        if !self.is_valid_identifier(&layout_name) {
            return Err(NenyrError::new(
                Some("A valid layout context name should contain only alphanumeric characters, with the first character being an alphabetic letter. Examples: `'myLayoutContext01'`, `'layout01'`, etc.".to_string()),
                self.context_name.clone(),
                self.context_path.to_string(),
                self.add_nenyr_token_to_error("The validation of the layout context name failed. The provided name does not meet the required format."),
                NenyrErrorKind::SyntaxError,
                self.get_tracing(),
            ));
        }

        Ok(layout_name)
    }

    /// Parses the current layout context block.
    ///
    /// This method processes the contents of the layout context block, identifying
    /// and validating the methods declared within it. It ensures proper syntax,
    /// including the presence of commas to separate method declarations. It uses a
    /// loop to continuously retrieve and process valid method declarations until
    /// the end of the block is reached.
    ///
    /// # Parameters
    /// - `layout_name`: A string slice representing the name of the layout context.
    ///
    /// # Returns
    /// - `NenyrResult<LayoutContext>`: The constructed layout context populated with
    ///   the defined methods.
    ///
    /// # Errors
    /// - Returns an error if the context contains invalid syntax or if unexpected
    ///   tokens are found.
    fn retrieve_layout_context_block(&mut self, layout_name: &str) -> NenyrResult<LayoutContext> {
        let mut layout_context = LayoutContext::new(layout_name.to_string());

        loop_while_not!(
            self,
            Some("Remove any duplicated commas from the layout context to ensure proper syntax. The parser expects the `Declare` keyword to follow valid delimiters. Example: `Construct Layout('layoutName') { Declare Class({ ... }), Declare Animation({ ... }), ... }`.".to_string()),
            "A duplicated comma was found inside the layout context. The parser expected to find the `Declare` keyword, but it was not found.",
            Some("Ensure that a comma is placed after each block definition to separate elements correctly. Proper syntax is required for the parser to process the context. Example: `Construct Layout('layoutName') { Declare Variables({ ... }), Declare Aliases({ ... }), ... }`.".to_string()),
            "All methods inside the layout context block must be separated by commas. A comma is missing after the block definition. The parser expected a comma to separate elements but did not find one.",
            || self.processing_state.is_context_active(),
            |is_active| self.processing_state.set_context_active(is_active),
            {
                self.handle_layout_declare_keyword(&mut layout_context)?;
            }
        );

        self.processing_state.set_context_active(false);

        Ok(layout_context)
    }

    /// Handles the parsing of methods declared within the layout context, ensuring
    /// that each method begins with the `Declare` keyword.
    ///
    /// This method validates the declaration of methods inside the layout context block.
    /// It expects each method to start with the `Declare` keyword and delegates the
    /// processing of the methods to `process_layout_methods`.
    ///
    /// # Parameters
    /// - `layout_context`: A mutable reference to the `LayoutContext` being populated.
    ///
    /// # Returns
    /// - `NenyrResult<()>`: An empty result indicating successful parsing.
    ///
    /// # Errors
    /// - Returns an error if any method does not start with the `Declare` keyword.
    fn handle_layout_declare_keyword(
        &mut self,
        layout_context: &mut LayoutContext,
    ) -> NenyrResult<()> {
        self.parse_declare_keyword(
            Some("Ensure that each method inside the layout context is correctly declared using the `Declare` keyword. Example: `Construct Layout('layoutName') { Declare Animation({ ... }), Declare Class({ ... }), ... }`.".to_string()),
            "All method declarations inside the layout context must begin with the `Declare` keyword, but one of the methods is missing this declaration."
        )?;

        self.process_layout_methods(layout_context)
    }

    /// Processes the valid methods that can be declared within the layout context.
    ///
    /// This method checks for the presence of valid method tokens (e.g., `Aliases`,
    /// `Variables`, `Themes`, `Animation`, `Class`) and ensures that only allowed
    /// methods are declared within the layout context. It returns an error if any
    /// invalid methods are encountered.
    ///
    /// # Parameters
    /// - `layout_context`: A mutable reference to the `LayoutContext` being populated.
    ///
    /// # Returns
    /// - `NenyrResult<()>`: An empty result indicating successful processing of the
    ///   methods.
    ///
    /// # Errors
    /// - Returns an error if an unrecognized method is declared within the layout context.
    fn process_layout_methods(&mut self, layout_context: &mut LayoutContext) -> NenyrResult<()> {
        self.processing_state.set_context_active(true);

        match self.current_token {
            NenyrTokens::Aliases => {
                let aliases = self.process_aliases_method()?;

                layout_context.add_aliases_to_context(aliases);
            }
            NenyrTokens::Variables => {
                let variables = self.process_variables_method(false)?;

                layout_context.add_variables_to_context(variables);
            }
            NenyrTokens::Themes => {}
            NenyrTokens::Animation => {
                let (animation_name, animation) = self.process_animation_method()?;

                layout_context.add_animation_to_context(animation_name, animation);
            }
            NenyrTokens::Class => {
                let (class_name, style_class) = self.process_class_method()?;

                layout_context.add_style_class_to_context(class_name, style_class);
            }
            _ => {
                return Err(NenyrError::new(
                    Some("Ensure that only valid methods supported by the layout context are declared. Review the documentation for methods allowed within `Construct Layout('layoutName') { ... }`.".to_string()),
                    self.context_name.clone(),
                    self.context_path.to_string(),
                    self.add_nenyr_token_to_error("One of the methods in the layout context is either not a valid Nenyr method or is not supported within the layout context."),
                    NenyrErrorKind::SyntaxError,
                    self.get_tracing(),
                ));
            }
        }

        Ok(())
    }
}

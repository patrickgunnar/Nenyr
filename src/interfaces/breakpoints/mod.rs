use indexmap::IndexMap;

use crate::{
    error::{NenyrError, NenyrErrorKind},
    loop_while_not,
    tokens::NenyrTokens,
    types::breakpoints::{NenyrBreakpointKind, NenyrBreakpoints},
    validators::breakpoint::NenyrBreakpointValidator,
    NenyrParser, NenyrResult,
};

impl<'a> NenyrParser<'a> {
    /// Processes the `Breakpoints` declaration method.
    ///
    /// This method is responsible for parsing the `Breakpoints` declaration within the Nenyr syntax.
    /// It expects the declaration to be enclosed in parentheses, and the internal structure to contain
    /// valid patterns defined by curly brackets.
    ///
    /// # Errors
    ///
    /// Returns an error if:
    /// - The `Breakpoints` declaration does not start with an opening parenthesis `(`.
    /// - The declaration does not end with a closing parenthesis `)`.
    /// - The internal block does not begin with an opening curly bracket `{`.
    /// - The block does not properly close with a closing curly bracket `}`.
    /// - Any patterns defined are not valid.
    pub(crate) fn process_breakpoints_method(&mut self) -> NenyrResult<NenyrBreakpoints> {
        self.process_next_token()?;

        self.parse_parenthesized_delimiter(
            Some("Ensure that the `Breakpoints` declaration block is enclosed with both an opening and a closing parenthesis. Correct syntax example: `Declare Breakpoints({ ... })`.".to_string()),
            "The `Breakpoints` block is missing an opening parenthesis `(` after the `Breakpoints` keyword. The parser expected an opening parenthesis to begin the breakpoint declarations.",
            Some("Ensure that the `Breakpoints` block includes both an opening and a closing parenthesis. The syntax should follow the correct format: `Declare Breakpoints({ ... })`.".to_string()),
            "A closing parenthesis `)` is missing for the `Breakpoints` declaration block. The parser expected a closing parenthesis to properly end the breakpoint declarations.",
            |parser| {
                let breakpoints = parser.parse_curly_bracketed_delimiter(
                    Some("After the opening parenthesis, an opening curly bracket `{` is required to properly define the patterns block in the `Breakpoints` declaration. Ensure the pattern follows correct Nenyr syntax, like `Declare Breakpoints({ ... })`.".to_string()),
                    "The `Breakpoints` declaration block was expected to receive an object as a value, but an opening curly bracket `{` was not found after the opening parenthesis.",
                    Some("Ensure that the patterns block within the `Breakpoints` declaration is properly closed with a closing curly bracket `}`. The correct syntax should look like: `Declare Breakpoints({ ... })`.".to_string()),
                    "The `Breakpoints` declaration block is missing a closing curly bracket `}` to properly close the patterns block.",
                    Self::process_breakpoints_children,
                )?;

                parser.process_next_token()?;

                Ok(breakpoints)
            },
        )
    }

    /// Processes the children patterns within the `Breakpoints` declaration.
    ///
    /// This method iterates through the patterns defined in the `Breakpoints` block. It ensures that
    /// each pattern follows the correct syntax and structure. If duplicated commas or incorrect syntax
    /// are found, appropriate errors will be returned.
    ///
    /// # Errors
    ///
    /// Returns an error if:
    /// - Duplicated commas are found in the patterns block.
    /// - A pattern is missing a required comma for separation.
    fn process_breakpoints_children(&mut self) -> NenyrResult<NenyrBreakpoints> {
        let mut breakpoints = NenyrBreakpoints::new();

        loop_while_not!(
            self,
            Some("Remove any duplicated commas from the patterns block in the `Breakpoints` declaration. Ensure proper syntax by following valid delimiters. Example: `Declare Breakpoints({ MobileFirst({ ... }), DesktopFirst({ ... }) })`.".to_string()),
            "A duplicated comma was found in the patterns block of the `Breakpoints` declarations. The parser expected to find a new pattern statement but none was found.",
            Some("Ensure that a comma is placed after each pattern definition inside the `Breakpoints` declaration to separate elements correctly. Proper syntax is required for the parser to process the context. Example: `Declare Breakpoints({ MobileFirst({ ... }), DesktopFirst({ ... }) })`.".to_string()),
            "The patterns in the `Breakpoints` declaration must be separated by commas. A comma is missing between the patterns in the `Breakpoints` declaration. The parser expected a comma to separate elements but did not find one.",
            || self.processing_state.is_block_active(),
            |is_active| self.processing_state.set_block_active(is_active),
            {
                self.process_breakpoints_pattern(&mut breakpoints)?;
            }
        );

        self.processing_state.set_block_active(false);

        Ok(breakpoints)
    }

    /// Processes an individual pattern within the `Breakpoints` declaration.
    ///
    /// This method checks the current token to determine if it matches valid breakpoint patterns (e.g.,
    /// `MobileFirst`, `DesktopFirst`). If the token does not match a valid pattern, an error is returned
    /// indicating the invalid declaration.
    ///
    /// # Parameters
    ///
    /// - `breakpoints`: A mutable reference to `NenyrBreakpoints` where the processed pattern will be added.
    ///
    /// # Errors
    ///
    /// Returns an error if:
    /// - The current token does not match a valid breakpoint pattern.
    fn process_breakpoints_pattern(
        &mut self,
        breakpoints: &mut NenyrBreakpoints,
    ) -> NenyrResult<()> {
        self.processing_state.set_block_active(true);

        match self.current_token {
            NenyrTokens::MobileFirst => self
                .process_breakpoint_pattern_block(&NenyrBreakpointKind::MobileFirst, breakpoints),
            NenyrTokens::DesktopFirst => self
                .process_breakpoint_pattern_block(&NenyrBreakpointKind::DesktopFirst, breakpoints),
            _ => {
                return Err(NenyrError::new(
                    Some("Fix or remove the invalid pattern declaration. Only valid and permitted patterns are allowed within the `Breakpoints` declaration. Please refer to the documentation to verify which patterns are permitted inside `Breakpoints`.".to_string()),
                    self.context_name.clone(),
                    self.context_path.to_string(),
                    self.add_nenyr_token_to_error("The `Breakpoints` declaration contains an invalid pattern statement. Please ensure that all methods within `Breakpoints` are correctly defined and formatted."),
                    NenyrErrorKind::SyntaxError,
                    self.get_tracing(),
                ));
            }
        }
    }

    /// Processes the block of a specific breakpoint pattern.
    ///
    /// This method expects the pattern to be defined within parentheses and then
    /// looks for the curly brackets that define the properties of the pattern. It validates
    /// the syntax and structure of the properties block.
    ///
    /// # Parameters
    ///
    /// - `breakpoint_kind`: A reference to the kind of breakpoint being processed (e.g., `MobileFirst`, `DesktopFirst`).
    /// - `breakpoints`: A mutable reference to `NenyrBreakpoints` where the processed properties will be added.
    ///
    /// # Errors
    ///
    /// Returns an error if:
    /// - The pattern block does not open and close with parentheses.
    /// - The properties block does not open and close with curly brackets.
    fn process_breakpoint_pattern_block(
        &mut self,
        breakpoint_kind: &NenyrBreakpointKind,
        breakpoints: &mut NenyrBreakpoints,
    ) -> NenyrResult<()> {
        self.process_next_token()?;

        self.parse_parenthesized_delimiter(
            Some("Ensure that all patterns inside the `Breakpoints` block declaration are enclosed with both an opening and a closing parenthesis. Correct syntax example: `Declare Breakpoints({ MobileFirst({ ... }), DesktopFirst({ ... }) })`.".to_string()),
            "One of the patterns in the `Breakpoints` declaration is missing an open parenthesis `(` after the pattern keyword. The parser expected a parenthesis to begin the pattern definition.",
            Some("Ensure that all patterns within the `Breakpoints` block have both an opening and a closing parenthesis. The syntax should follow the correct format, such as `Declare Breakpoints({ MobileFirst({ ... }), DesktopFirst({ ... }) })`.".to_string()),
            "A closing parenthesis `)` is missing for one of the patterns in the `Breakpoints` declaration. The parser expected a closing parenthesis to properly end the pattern declaration.",
            |parser| {
                parser.parse_curly_bracketed_delimiter(
                    Some("After the opening parenthesis, an opening curly bracket `{` is required to properly define the patterns block in `Breakpoints` declaration. Ensure the pattern follows the correct Nenyr syntax, such as `Declare Breakpoints({ MobileFirst({ ... }), DesktopFirst({ ... }) })`.".to_string()),
                    "One of the patterns in the `Breakpoints` declaration was expected to receive an object as a value, but an opening curly bracket `{` was not found after the opening parenthesis.",
                    Some("Ensure that the patterns block within the pattern in `Breakpoints` declaration is properly closed with a closing curly bracket `}`. The correct syntax should look like: `Declare Breakpoints({ MobileFirst({ ... }), DesktopFirst({ ... }) })`.".to_string()),
                    "One of the patterns in the `Breakpoints` declaration is missing a closing curly bracket `}` to properly close the patterns block.",
                    |parser| parser.handle_breakpoints_variables(breakpoint_kind, breakpoints),
                )?;

                parser.process_next_token()
            },
        )
    }

    /// Handles the parsing of breakpoint variables defined in a Nenyr context.
    ///
    /// This function processes the `Breakpoints` declaration by extracting and validating
    /// properties defined within it. It ensures that no duplicated commas are present, and
    /// each property is correctly formatted and separated.
    ///
    /// # Parameters
    /// - `breakpoint_kind`: A reference to the kind of breakpoint being processed. This determines
    ///   the context in which breakpoints are added.
    /// - `breakpoints`: A mutable reference to a `NenyrBreakpoints` struct where the parsed
    ///   breakpoints will be stored.
    ///
    /// # Errors
    /// This function will return a `NenyrError` if:
    /// - Duplicated commas are found in the properties block.
    /// - A property definition is malformed or incorrectly separated.
    /// - The overall syntax of the `Breakpoints` declaration does not follow the expected structure.
    fn handle_breakpoints_variables(
        &mut self,
        breakpoint_kind: &NenyrBreakpointKind,
        breakpoints: &mut NenyrBreakpoints,
    ) -> NenyrResult<()> {
        let mut properties: IndexMap<String, String> = IndexMap::new();

        loop_while_not!(
            self,
            Some("Remove any duplicated commas from the properties block in the `Breakpoints` declaration. Ensure proper syntax by following valid delimiters. Example: `Declare Breakpoints({ MobileFirst({ key: 'value', anotherKey: 'value' }), ... })`.".to_string()),
            "A duplicated comma was found in the properties block of the `Breakpoints` declarations. The parser expected to find a new property statement but none was found.",
            Some("Ensure that a comma is placed after each property definition inside the `Breakpoints` declaration to separate elements correctly. Proper syntax is required for the parser to process the context. Example: `Declare Breakpoints({ DesktopFirst({ key: 'value', anotherKey: 'value' }), ... })`.".to_string()),
            "The properties in the `Breakpoints` declaration must be separated by commas. A comma is missing between the properties in the `Breakpoints` declaration. The parser expected a comma to separate elements but did not find one.",
            || self.processing_state.is_nested_block_active(),
            |is_active| self.processing_state.set_nested_block_active(is_active),
            {
                self.process_breakpoints_property(&mut properties)?;
            }
        );

        breakpoints.add_breakpoints(breakpoint_kind, properties);
        self.processing_state.set_nested_block_active(false);

        Ok(())
    }

    /// Processes a single breakpoint property within a `Breakpoints` declaration.
    ///
    /// This function retrieves the identifier for a breakpoint and validates it. If valid,
    /// it will proceed to extract the corresponding value and add it to the properties map.
    ///
    /// # Parameters
    /// - `properties`: A mutable reference to an `IndexMap<String, String>` where valid
    ///   breakpoint properties are stored.
    ///
    /// # Errors
    /// Returns a `NenyrError` if:
    /// - The current token is not a valid identifier for a breakpoint.
    /// - The identifier fails to meet the naming conventions (must be alphanumeric and
    ///   start with a letter).
    fn process_breakpoints_property(
        &mut self,
        properties: &mut IndexMap<String, String>,
    ) -> NenyrResult<()> {
        self.processing_state.set_nested_block_active(true);

        if let NenyrTokens::Identifier(identifier) = self.current_token.clone() {
            return self.process_breakpoints_value(identifier, properties);
        }

        Err(NenyrError::new(
            Some("Specify a valid identifier for the breakpoint that consists only of alphanumeric characters, with the first character being a letter. For example: 'myBreakpoint1', 'exampleBreakpoint', etc.".to_string()),
            self.context_name.clone(),
            self.context_path.to_string(),
            self.add_nenyr_token_to_error("The `Breakpoints` declaration contains an invalid identifier for the breakpoint name. Please ensure the identifier follows the required format."),
            NenyrErrorKind::SyntaxError,
            self.get_tracing(),
        ))
    }

    /// Parses and validates the value associated with a specific breakpoint.
    ///
    /// This function expects the identifier of the breakpoint and retrieves the corresponding
    /// value. It checks that the value is a valid non-empty string and follows the required syntax.
    ///
    /// # Parameters
    /// - `identifier`: The name of the breakpoint for which the value is being processed.
    /// - `properties`: A mutable reference to an `IndexMap<String, String>` where the
    ///   identifier-value pair will be stored if valid.
    ///
    /// # Errors
    /// This function will return a `NenyrError` if:
    /// - The expected colon delimiter following the breakpoint identifier is missing.
    /// - The value for the breakpoint is either missing or empty.
    /// - The value does not meet the defined semantic validation criteria.
    fn process_breakpoints_value(
        &mut self,
        identifier: String,
        properties: &mut IndexMap<String, String>,
    ) -> NenyrResult<()> {
        self.process_next_token()?;

        self.parse_colon_delimiter(
            Some(format!("Ensure that each breakpoint is defined with a colon after it. The correct syntax is: `Breakpoints({{ MobileFirst({{ {}: 'breakpoint value', ... }}), ... }})`.", identifier)),
            &format!("The `{}` breakpoint in the `Breakpoints` declaration is missing a colon after the breakpoint name definition.", identifier),
            true
        )?;

        let value = self.parse_string_literal(
            Some(format!("Ensure that all breakpoints are assigned non-empty string values. You can either remove the breakpoint or specify a non-empty string value for it: `Breakpoints({{ DesktopFirst({{ {}: 'breakpoint value', ... }}), ... }})`.", identifier)), 
            &format!("The `{}` breakpoint in the `Breakpoints` declaration should receive a non-empty string as a value, but none was found.", identifier),
            false
        )?;

        if self.is_valid_breakpoint(&value) {
            properties.insert(identifier, value);

            return Ok(());
        }

        Err(NenyrError::new(
            Some("Ensure that all breakpoint values are semantically correct to be validated. Please refer to the documentation to verify the correct way to define breakpoint values.".to_string()),
            self.context_name.clone(),
            self.context_path.to_string(),
            self.add_nenyr_token_to_error(&format!("The `{}` breakpoint in the `Breakpoint` declaration contains an invalid value and could not be validated.", identifier)),
            NenyrErrorKind::SyntaxError,
            self.get_tracing(),
        ))
    }
}

#[cfg(test)]
mod tests {
    use crate::NenyrParser;

    #[test]
    fn themes_are_valid() {
        let raw_nenyr = "Breakpoints({
        MobileFirst({
            onMobTablet: '780px',
            onMobDesktop: '1240px',
            onMobXl: '1440px',
            onMobXXl: '2240px'
        }),
        DesktopFirst({
            onDeskTablet: '780px',
            onDeskDesktop: '1240px',
            onDeskXl: '1440px',
            onDeskXXl: '2240px'
        })
    })";
        let mut parser = NenyrParser::new(raw_nenyr, "");

        let _ = parser.process_next_token();
        assert_eq!(
            format!("{:?}", parser.process_breakpoints_method()),
            "Ok(NenyrBreakpoints { mobile_first: Some({\"onMobTablet\": \"780px\", \"onMobDesktop\": \"1240px\", \"onMobXl\": \"1440px\", \"onMobXXl\": \"2240px\"}), desktop_first: Some({\"onDeskTablet\": \"780px\", \"onDeskDesktop\": \"1240px\", \"onDeskXl\": \"1440px\", \"onDeskXXl\": \"2240px\"}) })".to_string()
        );
    }

    #[test]
    fn themes_are_not_valid() {
        let raw_nenyr = "Breakpoints({
        MobileFirst({
            onMobTablet: '780px',
            onMobDesktop: '1240px',
            onMobXl: '1440px',
            onMobXXl: '2240px'
        }),
        DesktopFirst{
            onDeskTablet: '780px',
            onDeskDesktop: '1240px',
            onDeskXl: '1440px',
            onDeskXXl: '2240px'
        })
    })";
        let mut parser = NenyrParser::new(raw_nenyr, "");

        let _ = parser.process_next_token();
        assert_eq!(
            format!("{:?}", parser.process_breakpoints_method()),
            "Err(NenyrError { suggestion: Some(\"Ensure that all patterns inside the `Breakpoints` block declaration are enclosed with both an opening and a closing parenthesis. Correct syntax example: `Declare Breakpoints({ MobileFirst({ ... }), DesktopFirst({ ... }) })`.\"), context_name: None, context_path: \"\", error_message: \"One of the patterns in the `Breakpoints` declaration is missing an open parenthesis `(` after the pattern keyword. The parser expected a parenthesis to begin the pattern definition. However, found `{` instead.\", error_kind: SyntaxError, error_tracing: NenyrErrorTracing { line_before: Some(\"        }),\"), line_after: Some(\"            onDeskTablet: '780px',\"), error_line: Some(\"        DesktopFirst{\"), error_on_line: 8, error_on_col: 22, error_on_pos: 201 } })".to_string()
        );
    }

    #[test]
    fn empty_themes_are_valid() {
        let raw_nenyr = "Breakpoints({ })
    })";
        let mut parser = NenyrParser::new(raw_nenyr, "");

        let _ = parser.process_next_token();
        assert_eq!(
            format!("{:?}", parser.process_breakpoints_method()),
            "Ok(NenyrBreakpoints { mobile_first: None, desktop_first: None })".to_string()
        );
    }

    #[test]
    fn only_mobile_first_themes_are_valid() {
        let raw_nenyr = "Breakpoints({
        MobileFirst({
            onMobTablet: '780px',
            onMobDesktop: '1240px',
            onMobXl: '1440px',
            onMobXXl: '2240px'
        })
    })";
        let mut parser = NenyrParser::new(raw_nenyr, "");

        let _ = parser.process_next_token();
        assert_eq!(
            format!("{:?}", parser.process_breakpoints_method()),
            "Ok(NenyrBreakpoints { mobile_first: Some({\"onMobTablet\": \"780px\", \"onMobDesktop\": \"1240px\", \"onMobXl\": \"1440px\", \"onMobXXl\": \"2240px\"}), desktop_first: None })".to_string()
        );
    }

    #[test]
    fn only_desktop_first_themes_are_valid() {
        let raw_nenyr = "Breakpoints({
        DesktopFirst({
            onDeskTablet: '780px',
            onDeskDesktop: '1240px',
            onDeskXl: '1440px',
            onDeskXXl: '2240px'
        })
    })";
        let mut parser = NenyrParser::new(raw_nenyr, "");

        let _ = parser.process_next_token();
        assert_eq!(
            format!("{:?}", parser.process_breakpoints_method()),
            "Ok(NenyrBreakpoints { mobile_first: None, desktop_first: Some({\"onDeskTablet\": \"780px\", \"onDeskDesktop\": \"1240px\", \"onDeskXl\": \"1440px\", \"onDeskXXl\": \"2240px\"}) })".to_string()
        );
    }

    #[test]
    fn empty_mobile_and_desktop_themes_are_valid() {
        let raw_nenyr = "Breakpoints({
        MobileFirst({ }),
        DesktopFirst({ })
    })";
        let mut parser = NenyrParser::new(raw_nenyr, "");

        let _ = parser.process_next_token();
        assert_eq!(
            format!("{:?}", parser.process_breakpoints_method()),
            "Ok(NenyrBreakpoints { mobile_first: Some({}), desktop_first: Some({}) })".to_string()
        );
    }
}

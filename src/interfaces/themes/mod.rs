use crate::{
    error::{NenyrError, NenyrErrorKind},
    loop_while_not,
    tokens::NenyrTokens,
    types::themes::{NenyrThemes, NenyrThemesKind},
    NenyrParser, NenyrResult,
};

impl NenyrParser {
    /// Processes the `Themes` declaration method.
    ///
    /// This function expects to find a `Themes` keyword followed by an opening parenthesis.
    /// It verifies that the declaration is correctly formatted, capturing the themes defined within
    /// curly brackets. The method parses and returns a `NenyrThemes` object, containing the
    /// parsed theme data.
    ///
    /// # Errors
    /// This method returns a `NenyrResult<NenyrThemes>` which will be an error if:
    /// - The `Themes` block is missing an opening or closing parenthesis.
    /// - The curly brackets defining the themes are improperly formatted.
    /// - Invalid theme patterns are encountered.
    pub(crate) fn process_themes_method(&mut self) -> NenyrResult<NenyrThemes> {
        self.process_next_token()?;

        self.parse_parenthesized_delimiter(
            Some("Ensure that the `Themes` declaration block is enclosed with both an opening and a closing parenthesis. Correct syntax example: `Declare Themes({ ... })`.".to_string()),
            "The `Themes` block is missing an opening parenthesis `(` after the `Themes` keyword. The parser expected an opening parenthesis to begin the theme declarations.",
            Some("Ensure that the `Themes` block includes both an opening and a closing parenthesis. The syntax should follow the correct format: `Declare Themes({ ... })`.".to_string()),
            "A closing parenthesis `)` is missing for the `Themes` declaration block. The parser expected a closing parenthesis to properly end the theme declarations.",
            |parser| {
                let themes = parser.parse_curly_bracketed_delimiter(
                    Some("After the opening parenthesis, an opening curly bracket `{` is required to properly define the patterns block in the `Themes` declaration. Ensure the pattern follows correct Nenyr syntax, like `Declare Themes({ ... })`.".to_string()),
                    "The `Themes` declaration block was expected to receive an object as a value, but an opening curly bracket `{` was not found after the opening parenthesis.",
                    Some("Ensure that the patterns block within the `Themes` declaration is properly closed with a closing curly bracket `}`. The correct syntax should look like: `Declare Themes({ ... })`.".to_string()),
                    "The `Themes` declaration block is missing a closing curly bracket `}` to properly close the patterns block.",
                    Self::process_themes_children,
                )?;

                parser.process_next_token()?;

                Ok(themes)
            },
        )
    }

    /// Processes the child patterns defined within the `Themes` declaration.
    ///
    /// This method iterates through each pattern within the `Themes` block, ensuring that they are
    /// correctly formatted and separated by commas. It captures any theme-specific variables
    /// associated with the patterns.
    ///
    /// # Errors
    /// Returns a `NenyrResult<NenyrThemes>` which will be an error if:
    /// - Duplicate commas are found in the patterns block.
    /// - A new pattern statement is expected but not found.
    /// - Missing commas between patterns result in syntax errors.
    fn process_themes_children(&mut self) -> NenyrResult<NenyrThemes> {
        let mut themes = NenyrThemes::new();

        loop_while_not!(
            self,
            Some("Remove any duplicated commas from the patterns block in the `Themes` declaration. Ensure proper syntax by following valid delimiters. Example: `Declare Themes({ Light({ ... }), Dark({ ... }) })`.".to_string()),
            "A duplicated comma was found in the patterns block of the `Themes` declarations. The parser expected to find a new pattern statement but none was found.",
            Some("Ensure that a comma is placed after each pattern definition inside the `Themes` declaration to separate elements correctly. Proper syntax is required for the parser to process the context. Example: `Declare Themes({ Light({ ... }), Dark({ ... }) })`.".to_string()),
            "The patterns in the `Themes` declaration must be separated by commas. A comma is missing between the patterns in the `Themes` declaration. The parser expected a comma to separate elements but did not find one.",
            || self.processing_state.is_extra_block_active(),
            |is_active| self.processing_state.set_extra_block_active(is_active),
            {
                self.process_themes_pattern(&mut themes)?;
            }
        );

        self.processing_state.set_extra_block_active(false);

        Ok(themes)
    }

    /// Processes an individual theme pattern in the `Themes` declaration.
    ///
    /// The method identifies the current pattern as either `Light` or `Dark`, and invokes the
    /// respective handler to process its contents. If an invalid pattern is encountered,
    /// an error is returned indicating the issue.
    ///
    /// # Errors
    /// Returns a `NenyrResult<()>` which will be an error if:
    /// - An invalid pattern declaration is detected within the `Themes`.
    fn process_themes_pattern(&mut self, themes: &mut NenyrThemes) -> NenyrResult<()> {
        self.processing_state.set_extra_block_active(true);

        match self.current_token {
            NenyrTokens::Light => self.process_inner_pattern_block(&NenyrThemesKind::Light, themes),
            NenyrTokens::Dark => self.process_inner_pattern_block(&NenyrThemesKind::Dark, themes),
            _ => {
                return Err(NenyrError::new(
                    Some("Fix or remove the invalid pattern declaration. Only valid and permitted patterns are allowed within the `Themes` declaration. Please refer to the documentation to verify which patterns are permitted inside `Themes`.".to_string()),
                    self.context_name.clone(),
                    self.context_path.to_string(),
                    self.add_nenyr_token_to_error("The `Themes` declaration contains an invalid pattern statement. Please ensure that all methods within `Themes` are correctly defined and formatted."),
                    NenyrErrorKind::SyntaxError,
                    self.get_tracing(),
                ));
            }
        }
    }

    /// Processes the inner block of a specific theme pattern.
    ///
    /// This method ensures that each pattern block (`Light` or `Dark`) is enclosed within
    /// parentheses and that it contains a properly formatted curly-bracketed object. It captures
    /// any associated variables if present, allowing for flexible theme definitions.
    ///
    /// # Errors
    /// Returns a `NenyrResult<()>` which will be an error if:
    /// - Missing opening or closing parentheses are detected in the pattern definition.
    /// - Curly brackets defining the theme properties are not found or improperly formatted.
    fn process_inner_pattern_block(
        &mut self,
        schema_kind: &NenyrThemesKind,
        themes: &mut NenyrThemes,
    ) -> NenyrResult<()> {
        self.process_next_token()?;

        self.parse_parenthesized_delimiter(
            Some("Ensure that all patterns inside the `Themes` block declaration are enclosed with both an opening and a closing parenthesis. Correct syntax example: `Declare Themes({ Light({ ... }), Dark({ ... }) })`".to_string()),
            "One of the patterns in the `Themes` declaration is missing an open parenthesis `(` after the pattern keyword. The parser expected a parenthesis to begin the pattern definition.",
            Some("Ensure that all patterns within the `Themes` block have both an opening and a closing parenthesis. The syntax should follow the correct format, such as `Declare Themes({ Light({ ... }), Dark({ ... }) })`.".to_string()),
            "A closing parenthesis `)` is missing for one of the patterns in the `Themes` declaration. The parser expected a closing parenthesis to properly end the pattern declaration.",
            |parser| {
                parser.parse_curly_bracketed_delimiter(
                    Some("After the opening parenthesis, an opening curly bracket `{` is required to properly define the patterns block in `Themes` declaration. Ensure the pattern follows the correct Nenyr syntax, such as `Declare Themes({ Light({ ... }), Dark({ ... }) })`.".to_string()),
                    "One of the patterns in the `Themes` declaration was expected to receive an object as a value, but an opening curly bracket `{` was not found after the opening parenthesis.",
                    Some("Ensure that the patterns block within the pattern in `Themes` declaration is properly closed with a closing curly bracket `}`. The correct syntax should look like: `Declare Themes({ Light({ ... }), Dark({ ... }) })`.".to_string()),
                    "One of the patterns in the `Themes` declaration is missing a closing curly bracket `}` to properly close the patterns block.",
                    |parser| parser.handle_themes_variables(schema_kind, themes),
                )?;

                parser.process_next_token()
            },
        )
    }

    /// Handles the variables associated with a theme pattern.
    ///
    /// This function processes any `Variables` keyword present within the current pattern block,
    /// adding them to the specified theme. If the current token is not a recognized pattern or
    /// variable, it raises a syntax error indicating an invalid declaration.
    ///
    /// # Errors
    /// Returns a `NenyrResult<()>` which will be an error if:
    /// - An unsupported pattern declaration is found.
    /// - Variables are improperly defined within the theme pattern.
    fn handle_themes_variables(
        &mut self,
        schema_kind: &NenyrThemesKind,
        themes: &mut NenyrThemes,
    ) -> NenyrResult<()> {
        match self.current_token {
            NenyrTokens::CurlyBracketClose => {}
            NenyrTokens::Variables => {
                let variables = self.process_variables_method(true)?;

                themes.add_variables(schema_kind, variables);
                self.process_next_token()?;

                if let NenyrTokens::Comma = self.current_token {
                    self.process_next_token()?;
                }
            }
            _ => {
                return Err(NenyrError::new(
                    Some("Please fix or remove the unsupported pattern declaration from `Themes`. Refer to the documentation to verify how to properly define a `Themes` method.".to_string()),
                    self.context_name.clone(),
                    self.context_path.to_string(),
                    self.add_nenyr_token_to_error("The `Themes` method contains an invalid pattern statement. Only the `MobileFirst` or `DesktopFirst` patterns are allowed within `Themes`, and each can receive only a `Variables` declaration."),
                    NenyrErrorKind::SyntaxError,
                    self.get_tracing(),
                ));
            }
        }

        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use crate::NenyrParser;

    #[test]
    fn themes_are_valid() {
        let raw_nenyr = "Themes({
        Light({
            Variables({
                primaryColor: '#FFFFFF',
                secondaryColor: '#CCCCCC',
                accentColorVar: '#FF5733'
            })
        }),
        Dark({
            Variables({
                primaryColor: '#333333',
                secondaryColor: '#666666',
                accentColorVar: '#FF5733'
            })
        })
    })";
        let mut parser = NenyrParser::new();
        parser.setup_dependencies(raw_nenyr.to_string(), "".to_string());

        let _ = parser.process_next_token();
        assert_eq!(
            format!("{:?}", parser.process_themes_method()),
            "Ok(NenyrThemes { light_schema: Some(NenyrVariables { values: {\"primaryColor\": \"#FFFFFF\", \"secondaryColor\": \"#CCCCCC\", \"accentColorVar\": \"#FF5733\"} }), dark_schema: Some(NenyrVariables { values: {\"primaryColor\": \"#333333\", \"secondaryColor\": \"#666666\", \"accentColorVar\": \"#FF5733\"} }) })".to_string()
        );
    }

    #[test]
    fn themes_are_not_valid() {
        let raw_nenyr = "Themes({
        Light(
            Variables({
                primaryColor: '#FFFFFF',
                secondaryColor: '#CCCCCC',
                accentColorVar: '#FF5733'
            })
        }),
        Dark({
            Variables({
                primaryColor: '#333333',
                secondaryColor: '#666666',
                accentColorVar: '#FF5733'
            })
        })
    })";
        let mut parser = NenyrParser::new();
        parser.setup_dependencies(raw_nenyr.to_string(), "".to_string());

        let _ = parser.process_next_token();
        assert_eq!(
            format!("{:?}", parser.process_themes_method()),
            "Err(NenyrError { suggestion: Some(\"After the opening parenthesis, an opening curly bracket `{` is required to properly define the patterns block in `Themes` declaration. Ensure the pattern follows the correct Nenyr syntax, such as `Declare Themes({ Light({ ... }), Dark({ ... }) })`.\"), context_name: None, context_path: \"\", error_message: \"One of the patterns in the `Themes` declaration was expected to receive an object as a value, but an opening curly bracket `{` was not found after the opening parenthesis. However, found `Variables` instead.\", error_kind: SyntaxError, error_tracing: NenyrErrorTracing { line_before: Some(\"        Light(\"), line_after: Some(\"                primaryColor: '#FFFFFF',\"), error_line: Some(\"            Variables({\"), error_on_line: 3, error_on_col: 22, error_on_pos: 45 } })".to_string()
        );
    }

    #[test]
    fn empty_themes_are_valid() {
        let raw_nenyr = "Themes({ })";
        let mut parser = NenyrParser::new();
        parser.setup_dependencies(raw_nenyr.to_string(), "".to_string());

        let _ = parser.process_next_token();
        assert_eq!(
            format!("{:?}", parser.process_themes_method()),
            "Ok(NenyrThemes { light_schema: None, dark_schema: None })".to_string()
        );
    }

    #[test]
    fn only_light_themes_are_valid() {
        let raw_nenyr = "Themes({
        Light({
            Variables({
                primaryColor: '#FFFFFF',
                secondaryColor: '#CCCCCC',
                accentColorVar: '#FF5733'
            })
        })
    })";
        let mut parser = NenyrParser::new();
        parser.setup_dependencies(raw_nenyr.to_string(), "".to_string());

        let _ = parser.process_next_token();
        assert_eq!(
            format!("{:?}", parser.process_themes_method()),
            "Ok(NenyrThemes { light_schema: Some(NenyrVariables { values: {\"primaryColor\": \"#FFFFFF\", \"secondaryColor\": \"#CCCCCC\", \"accentColorVar\": \"#FF5733\"} }), dark_schema: None })".to_string()
        );
    }

    #[test]
    fn only_dark_themes_are_valid() {
        let raw_nenyr = "Themes({
        Dark({
            Variables({
                primaryColor: '#333333',
                secondaryColor: '#666666',
                accentColorVar: '#FF5733'
            })
        })
    })";
        let mut parser = NenyrParser::new();
        parser.setup_dependencies(raw_nenyr.to_string(), "".to_string());

        let _ = parser.process_next_token();
        assert_eq!(
            format!("{:?}", parser.process_themes_method()),
            "Ok(NenyrThemes { light_schema: None, dark_schema: Some(NenyrVariables { values: {\"primaryColor\": \"#333333\", \"secondaryColor\": \"#666666\", \"accentColorVar\": \"#FF5733\"} }) })".to_string()
        );
    }

    #[test]
    fn empty_dark_and_valid_themes_are_valid() {
        let raw_nenyr = "Themes({
        Light({ }),
        Dark({ })
    })";
        let mut parser = NenyrParser::new();
        parser.setup_dependencies(raw_nenyr.to_string(), "".to_string());

        let _ = parser.process_next_token();
        assert_eq!(
            format!("{:?}", parser.process_themes_method()),
            "Ok(NenyrThemes { light_schema: None, dark_schema: None })".to_string()
        );
    }
}

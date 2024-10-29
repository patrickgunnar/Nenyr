use crate::{
    converters::property::NenyrPropertyConverter,
    error::{NenyrError, NenyrErrorKind},
    loop_while_not,
    tokens::NenyrTokens,
    types::aliases::NenyrAliases,
    NenyrParser, NenyrResult,
};

impl<'a> NenyrParser<'a> {
    /// Processes the `Aliases` declaration block in Nenyr syntax.
    ///
    /// This method expects the `Aliases` keyword to be followed by a set of parenthesis, within which
    /// a curly-bracketed object defines alias properties. It validates the entire structure, ensuring
    /// proper syntax is followed.
    ///
    /// # Errors
    ///
    /// This method returns a `NenyrError` if:
    /// - The `Aliases` block does not begin with an opening parenthesis `(`.
    /// - The `Aliases` block does not include a closing parenthesis `)`.
    /// - The properties block does not start with an opening curly bracket `{`.
    /// - The properties block does not end with a closing curly bracket `}`.
    /// - There are any syntax issues within the properties block, such as duplicated commas.
    ///
    /// # Returns
    ///
    /// Returns a `NenyrAliases` instance containing the parsed alias definitions.
    pub(crate) fn process_aliases_method(&mut self) -> NenyrResult<NenyrAliases> {
        self.process_next_token()?;

        self.parse_parenthesized_delimiter(
            Some("Ensure that the `Aliases` declaration block is enclosed with both an opening and a closing parenthesis. Correct syntax example: `Declare Aliases({ ... })`.".to_string()),
            "The `Aliases` block is missing an opening parenthesis `(` after the `Aliases` keyword. The parser expected an opening parenthesis to begin the alias declarations.",
            Some("Ensure that the `Aliases` block includes both an opening and a closing parenthesis. The syntax should follow the correct format: `Declare Aliases({ ... })`.".to_string()),
            "A closing parenthesis `)` is missing for the `Aliases` declaration block. The parser expected a closing parenthesis to properly end the alias declarations.",
            |parser| {
                let aliases = parser.parse_curly_bracketed_delimiter(
                    Some("After the opening parenthesis, an opening curly bracket `{` is required to properly define the properties block in the `Aliases` declaration. Ensure the pattern follows correct Nenyr syntax, like `Declare Aliases({ key: 'value', ... })`.".to_string()),
                    "The `Aliases` declaration block was expected to receive an object as a value, but an opening curly bracket `{` was not found after the opening parenthesis.",
                    Some("Ensure that the properties block within the `Aliases` declaration is properly closed with a closing curly bracket `}`. The correct syntax should look like: `Declare Aliases({ key: 'value', ... })`.".to_string()),
                    "The `Aliases` declaration block is missing a closing curly bracket `}` to properly close the properties block.",
                    Self::process_aliases_children,
                )?;

                parser.process_next_token()?;

                Ok(aliases)
            },
        )
    }

    /// Processes the children of the `Aliases` declaration block.
    ///
    /// This method iteratively processes alias identifiers and their corresponding values, ensuring
    /// that the properties are correctly defined and separated by commas. It handles syntax validation
    /// for the alias properties.
    ///
    /// # Errors
    ///
    /// Returns a `NenyrError` if:
    /// - Duplicated commas are found within the properties block.
    /// - A new property statement is expected but not found.
    /// - Commas are missing between properties.
    ///
    /// # Returns
    ///
    /// Returns a `NenyrAliases` instance containing all parsed aliases from the properties block.
    fn process_aliases_children(&mut self) -> NenyrResult<NenyrAliases> {
        let mut aliases = NenyrAliases::new();

        loop_while_not!(
            self,
            Some("Remove any duplicated commas from the properties block in the `Aliases` declaration. Ensure proper syntax by following valid delimiters. Example: `Declare Aliases({ key: 'value', anotherKey: 'anotherValue', ... })`.".to_string()),
            "A duplicated comma was found in the properties block of the `Aliases` declarations. The parser expected to find a new property statement but none was found.",
            Some("Ensure that a comma is placed after each property definition inside the `Aliases` declaration to separate elements correctly. Proper syntax is required for the parser to process the context. Example: `Declare Aliases({ key: 'value', anotherKey: 'anotherValue', ... })`.".to_string()),
            "The properties in the `Aliases` declaration must be separated by commas. A comma is missing between the properties in the `Aliases` declaration. The parser expected a comma to separate elements but did not find one.",
            || self.processing_state.is_block_active(),
            |is_active| self.processing_state.set_block_active(is_active),
            {
                self.process_alias_identifier(&mut aliases)?;
            }
        );

        self.processing_state.set_block_active(false);

        Ok(aliases)
    }

    /// Processes an individual alias identifier within the `Aliases` declaration.
    ///
    /// This method validates the alias identifier to ensure it adheres to the required naming conventions
    /// and retrieves its corresponding value. It raises errors if the identifier is invalid or if it fails to
    /// meet the criteria for alias names.
    ///
    /// # Parameters
    ///
    /// - `aliases`: A mutable reference to the `NenyrAliases` instance to which the alias will be added.
    ///
    /// # Errors
    ///
    /// Returns a `NenyrError` if:
    /// - The identifier is not a valid identifier (must be alphanumeric and start with a letter).
    fn process_alias_identifier(&mut self, aliases: &mut NenyrAliases) -> NenyrResult<()> {
        self.processing_state.set_block_active(true);

        if let NenyrTokens::Identifier(identifier) = self.current_token.clone() {
            return self.process_alias_value(identifier, aliases);
        }

        Err(NenyrError::new(
            Some("Specify a valid identifier for the alias that consists only of alphanumeric characters, with the first character being a letter. For example: 'myAlias1', 'exampleAlias', etc.".to_string()),
            self.context_name.clone(),
            self.context_path.to_string(),
            self.add_nenyr_token_to_error("The `Aliases` declaration contains an invalid identifier for the alias name. Please ensure the identifier follows the required format."),
            NenyrErrorKind::SyntaxError,
            self.get_tracing(),
        ))
    }

    /// Processes the value associated with an alias identifier.
    ///
    /// This method expects a valid alias identifier followed by a colon, and then retrieves the corresponding
    /// property value. It validates that the property is a valid Nenyr property and adds it to the alias collection.
    ///
    /// # Parameters
    ///
    /// - `identifier`: A string representing the alias identifier.
    /// - `aliases`: A mutable reference to the `NenyrAliases` instance to which the alias will be added.
    ///
    /// # Errors
    ///
    /// Returns a `NenyrError` if:
    /// - The property value is not a valid Nenyr property.
    fn process_alias_value(
        &mut self,
        identifier: String,
        aliases: &mut NenyrAliases,
    ) -> NenyrResult<()> {
        self.process_next_token()?;
        self.parse_colon_delimiter(
            Some(format!("Ensure that each alias is defined with a colon after it. The correct syntax is: `Aliases({{ {}: 'alias value', ... }})`.", identifier)),
            &format!("The `{}` alias in the `Aliases` declaration is missing a colon after the alias name definition.", identifier),
            true
        )?;

        if let Some(property) = self.convert_nenyr_property_to_css_property(&self.current_token) {
            aliases.add_alias(identifier, property);

            return Ok(());
        }

        Err(NenyrError::new(
            Some("Ensure that only valid Nenyr properties are used as values for aliases. Please refer to the documentation to verify the available Nenyr properties.".to_string()),
            self.context_name.clone(),
            self.context_path.to_string(),
            self.add_nenyr_token_to_error(&format!("The `{}` alias contains an invalid property, which is not a valid Nenyr property as a value.", identifier)),
            NenyrErrorKind::SyntaxError,
            self.get_tracing(),
        ))
    }
}

#[cfg(test)]
mod tests {
    use crate::NenyrParser;

    #[test]
    fn aliases_are_valid() {
        let raw_nenyr = "Aliases({
        bgd: background,
        bgd: backgroundColor,
        pdg: padding,
        dp: display,
        wd: width,
        hgt: height
    })";
        let mut parser = NenyrParser::new(raw_nenyr, "");

        let _ = parser.process_next_token();
        assert_eq!(
            format!("{:?}", parser.process_aliases_method()),
            "Ok(NenyrAliases { values: {\"bgd\": \"background-color\", \"pdg\": \"padding\", \"dp\": \"display\", \"wd\": \"width\", \"hgt\": \"height\"} })".to_string()
        );
    }

    #[test]
    fn aliases_are_not_valid() {
        let raw_nenyr = "Aliases(
        bgd: background,
        bgd: backgroundColor,
        pdg: padding,
        dp: display,
        wd: width,
        hgt: height
    })";
        let mut parser = NenyrParser::new(raw_nenyr, "");

        let _ = parser.process_next_token();
        assert_eq!(
            format!("{:?}", parser.process_aliases_method()),
            "Err(NenyrError { suggestion: Some(\"After the opening parenthesis, an opening curly bracket `{` is required to properly define the properties block in the `Aliases` declaration. Ensure the pattern follows correct Nenyr syntax, like `Declare Aliases({ key: 'value', ... })`.\"), context_name: None, context_path: \"\", error_message: \"The `Aliases` declaration block was expected to receive an object as a value, but an opening curly bracket `{` was not found after the opening parenthesis. However, found `bgd` instead.\", error_kind: SyntaxError, error_tracing: NenyrErrorTracing { line_before: Some(\"Aliases(\"), line_after: Some(\"        bgd: backgroundColor,\"), error_line: Some(\"        bgd: background,\"), error_on_line: 2, error_on_col: 12, error_on_pos: 20 } })".to_string()
        );
    }

    #[test]
    fn empty_aliases_are_valid() {
        let raw_nenyr = "Aliases({ })";
        let mut parser = NenyrParser::new(raw_nenyr, "");

        let _ = parser.process_next_token();
        assert_eq!(
            format!("{:?}", parser.process_aliases_method()),
            "Ok(NenyrAliases { values: {} })".to_string()
        );
    }
}

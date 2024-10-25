use crate::{
    error::{NenyrError, NenyrErrorKind},
    loop_while_not,
    tokens::NenyrTokens,
    types::typefaces::NenyrTypefaces,
    validators::{identifier::NenyrIdentifierValidator, typeface::NenyrTypefaceValidator},
    NenyrParser, NenyrResult,
};

impl<'a> NenyrParser<'a> {
    /// Processes the `Typefaces` method declaration.
    ///
    /// This method expects to find a `Typefaces` keyword followed by an opening parenthesis,
    /// a curly-bracketed block defining the typefaces, and a closing parenthesis.
    ///
    /// # Errors
    ///
    /// Returns an error if:
    /// - The `Typefaces` declaration block is not properly enclosed with parentheses.
    /// - The properties block is missing an opening or closing curly bracket.
    /// - The properties block contains syntax errors, such as duplicated commas or missing commas.
    pub(crate) fn process_typefaces_method(&mut self) -> NenyrResult<NenyrTypefaces> {
        self.process_next_token()?;

        self.parse_parenthesized_delimiter(
            Some("Ensure that the `Typefaces` declaration block is enclosed with both an opening and a closing parenthesis. Correct syntax example: `Declare Typefaces({ ... })`.".to_string()),
            "The `Typefaces` block is missing an opening parenthesis `(` after the `Typefaces` keyword. The parser expected an opening parenthesis to begin the typeface declarations.",
            Some("Ensure that the `Typefaces` block includes both an opening and a closing parenthesis. The syntax should follow the correct format: `Declare Typefaces({ ... })`.".to_string()),
            "A closing parenthesis `)` is missing for the `Typefaces` declaration block. The parser expected a closing parenthesis to properly end the typeface declarations.",
            |parser| {
                let typefaces = parser.parse_curly_bracketed_delimiter(
                    Some("After the opening parenthesis, an opening curly bracket `{` is required to properly define the properties block in the `Typefaces` declaration. Ensure the pattern follows correct Nenyr syntax, like `Declare Typefaces({ key: 'value', ... })`.".to_string()),
                    "The `Typefaces` declaration block was expected to receive an object as a value, but an opening curly bracket `{` was not found after the opening parenthesis.",
                    Some("Ensure that the properties block within the `Typefaces` declaration is properly closed with a closing curly bracket `}`. The correct syntax should look like: `Declare Typefaces({ key: 'value', ... })`.".to_string()),
                    "The `Typefaces` declaration block is missing a closing curly bracket `}` to properly close the properties block.",
                    Self::process_typefaces_children,
                )?;

                parser.process_next_token()?;

                Ok(typefaces)
            },
        )
    }

    /// Processes the children of the `Typefaces` declaration block.
    ///
    /// This method iterates through the properties defined within the `Typefaces` block,
    /// ensuring that each property is well-formed, separated by commas, and adheres to the
    /// required syntax rules.
    ///
    /// # Errors
    ///
    /// Returns an error if:
    /// - Duplicated commas are found in the properties block.
    /// - Properties are not separated by commas.
    fn process_typefaces_children(&mut self) -> NenyrResult<NenyrTypefaces> {
        let mut typefaces = NenyrTypefaces::new();

        loop_while_not!(
            self,
            Some("Remove any duplicated commas from the properties block in the `Typefaces` declaration. Ensure proper syntax by following valid delimiters. Example: `Declare Typefaces({ key: 'value', anotherKey: 'anotherValue', ... })`.".to_string()),
            "A duplicated comma was found in the properties block of the `Typefaces` declarations. The parser expected to find a new property statement but none was found.",
            Some("Ensure that a comma is placed after each property definition inside the `Typefaces` declaration to separate elements correctly. Proper syntax is required for the parser to process the context. Example: `Declare Typefaces({ key: 'value', anotherKey: 'anotherValue', ... })`.".to_string()),
            "The properties in the `Typefaces` declaration must be separated by commas. A comma is missing between the properties in the `Typefaces` declaration. The parser expected a comma to separate elements but did not find one.",
            || self.processing_state.is_block_active(),
            |is_active| self.processing_state.set_block_active(is_active),
            {
                self.process_typeface_identifier(&mut typefaces)?;
            }
        );

        self.processing_state.set_block_active(false);

        Ok(typefaces)
    }

    /// Processes an individual typeface identifier within the `Typefaces` declaration.
    ///
    /// This method validates the identifier, ensuring it conforms to the naming conventions, and
    /// subsequently processes the associated value for that typeface.
    ///
    /// # Parameters
    ///
    /// - `typefaces`: A mutable reference to the `NenyrTypefaces` object where the validated
    ///   typeface will be added.
    ///
    /// # Errors
    ///
    /// Returns an error if:
    /// - The identifier does not match the expected format.
    /// - An invalid identifier is specified for the typeface.
    fn process_typeface_identifier(&mut self, typefaces: &mut NenyrTypefaces) -> NenyrResult<()> {
        self.processing_state.set_block_active(true);

        if let NenyrTokens::Identifier(identifier) = self.current_token.clone() {
            if !self.is_valid_identifier(&identifier) {
                return Err(NenyrError::new(
                    Some("A valid typeface name should contain only alphanumeric characters, with the first character being an alphabetic letter. Examples: 'myTypeface1', 'typefaceName123', etc.".to_string()),
                    self.context_name.clone(),
                    self.context_path.to_string(),
                    self.add_nenyr_token_to_error("The validation of the typeface name failed. The provided name does not meet the required format."),
                    NenyrErrorKind::SyntaxError,
                    self.get_tracing(),
                ));
            }

            return self.process_typeface_value(identifier, typefaces);
        }

        Err(NenyrError::new(
            Some("Specify a valid identifier for the typeface that consists only of alphanumeric characters, with the first character being a letter. For example: 'myTypeface1', 'exampleTypeface', etc.".to_string()),
            self.context_name.clone(),
            self.context_path.to_string(),
            self.add_nenyr_token_to_error("The `Typefaces` declaration contains an invalid identifier for the typeface name. Please ensure the identifier follows the required format."),
            NenyrErrorKind::SyntaxError,
            self.get_tracing(),
        ))
    }

    /// Processes the value associated with a typeface identifier.
    ///
    /// This method checks for the presence of a colon after the identifier and expects a valid
    /// non-empty string value to follow. It then validates the value before adding it to the
    /// `Typefaces` object.
    ///
    /// # Parameters
    ///
    /// - `identifier`: The name of the typeface as a `String`.
    /// - `typefaces`: A mutable reference to the `NenyrTypefaces` object where the validated
    ///   typeface will be added.
    ///
    /// # Errors
    ///
    /// Returns an error if:
    /// - The value associated with the typeface is empty or invalid.
    fn process_typeface_value(
        &mut self,
        identifier: String,
        typefaces: &mut NenyrTypefaces,
    ) -> NenyrResult<()> {
        self.process_next_token()?;
        self.parse_colon_delimiter(
            Some(format!("Ensure that each typeface is defined with a colon after it. The correct syntax is: `Typefaces({{ {}: 'typeface value', ... }})`.", identifier)),
            &format!("The `{}` typeface in the `Typefaces` declaration is missing a colon after the typeface name definition.", identifier),
            true
        )?;

        let value = self.parse_string_literal(
            Some(format!("Ensure that all typefaces are assigned non-empty string values. You can either remove the typeface or specify a non-empty string value for it: `Typefaces({{ {}: 'typeface value', ... }})`.", identifier)), 
            &format!("The `{}` typeface in the `Typefaces` declaration should receive a non-empty string as a value, but none was found.", identifier),
            false
        )?;

        if self.is_valid_typeface(&value, &self.context_path) {
            typefaces.add_typeface(identifier, value);

            return Ok(());
        }

        Err(NenyrError::new(
            Some("Ensure that all typeface values are semantically correct to be validated. Please refer to the documentation to verify the correct way to define typeface values.".to_string()),
            self.context_name.clone(),
            self.context_path.to_string(),
            self.add_nenyr_token_to_error(&format!("The `{}` typeface in the `Typefaces` declaration contains an invalid value and could not be validated.", identifier)),
            NenyrErrorKind::SyntaxError,
            self.get_tracing(),
        ))
    }
}

#[cfg(test)]
mod tests {
    use crate::NenyrParser;

    #[test]
    fn typefaces_are_valid() {
        let raw_nenyr = "Typefaces({
        roseMartin: '../../../mocks/typefaces/rosemartin.regular.otf',
        regularEot: '../../../mocks/typefaces/showa-source-curry.regular-webfont.eot',
        regularSvg: '../../../mocks/typefaces/showa-source-curry.regular-webfont.svg',
        regularTtf: '../../../mocks/typefaces/showa-source-curry.regular-webfont.ttf',
        regularWoff: '../../../mocks/typefaces/showa-source-curry.regular-webfont.woff',
        regularWoff2: '../../../mocks/typefaces/showa-source-curry.regular-webfont.woff2'
    })";
        let mut parser = NenyrParser::new(raw_nenyr, "src/interfaces/typefaces/central.nyr");

        let _ = parser.process_next_token();
        assert_eq!(
            format!("{:?}", parser.process_typefaces_method()),
            "Ok(NenyrTypefaces { values: {\"roseMartin\": \"../../../mocks/typefaces/rosemartin.regular.otf\", \"regularEot\": \"../../../mocks/typefaces/showa-source-curry.regular-webfont.eot\", \"regularSvg\": \"../../../mocks/typefaces/showa-source-curry.regular-webfont.svg\", \"regularTtf\": \"../../../mocks/typefaces/showa-source-curry.regular-webfont.ttf\", \"regularWoff\": \"../../../mocks/typefaces/showa-source-curry.regular-webfont.woff\", \"regularWoff2\": \"../../../mocks/typefaces/showa-source-curry.regular-webfont.woff2\"} })".to_string()
        );
    }

    #[test]
    fn typefaces_are_not_valid() {
        let raw_nenyr = "Typefaces({
        roseMartin: '../../../mocks/typefaces/rosemartin.regular.otf',
        regularEot: '../../../mocks/typefaces/showa-source-curry.regular-webfont.eot',
        regularSvg: '../../../mocks/typefaces/showa-source-curry.regular-webfont.svg',
        regularTtf: '../../../mocks/typefaces/showa-source-curry.regular-webfont.ttf',
        regularWoff: '../../mocks/typefaces/showa-source-curry.regular-webfont.woff',
        regularWoff2: '../../../mocks/typefaces/showa-source-curry.regular-webfont.woff2'
    })";
        let mut parser = NenyrParser::new(raw_nenyr, "src/interfaces/typefaces/central.nyr");

        let _ = parser.process_next_token();
        assert_eq!(
            format!("{:?}", parser.process_typefaces_method()),
            "Err(NenyrError { suggestion: Some(\"Ensure that all typeface values are semantically correct to be validated. Please refer to the documentation to verify the correct way to define typeface values.\"), context_name: None, context_path: \"src/interfaces/typefaces/central.nyr\", error_message: \"The `regularWoff` typeface in the `Typefaces` declaration contains an invalid value and could not be validated. However, found `StringLiteral(\\\"../../mocks/typefaces/showa-source-curry.regular-webfont.woff\\\")` instead.\", error_kind: SyntaxError, error_tracing: NenyrErrorTracing { line_before: Some(\"        regularTtf: '../../../mocks/typefaces/showa-source-curry.regular-webfont.ttf',\"), line_after: Some(\"        regularWoff2: '../../../mocks/typefaces/showa-source-curry.regular-webfont.woff2'\"), error_line: Some(\"        regularWoff: '../../mocks/typefaces/showa-source-curry.regular-webfont.woff',\"), error_on_line: 6, error_on_col: 85, error_on_pos: 428 } })".to_string()
        );
    }

    #[test]
    fn empty_typefaces_are_valid() {
        let raw_nenyr = "Typefaces({ })";
        let mut parser = NenyrParser::new(raw_nenyr, "src/interfaces/typefaces/central.nyr");

        let _ = parser.process_next_token();
        assert_eq!(
            format!("{:?}", parser.process_typefaces_method()),
            "Ok(NenyrTypefaces { values: {} })".to_string()
        );
    }
}

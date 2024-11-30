use crate::{
    error::{NenyrError, NenyrErrorKind},
    loop_while_not,
    tokens::NenyrTokens,
    types::imports::NenyrImports,
    validators::import::NenyrImportValidator,
    NenyrParser, NenyrResult,
};

impl NenyrParser {
    /// Processes the `Imports` declaration method.
    ///
    /// This method initiates the parsing of the `Imports` block by expecting the next token to be a valid
    /// import declaration. It checks for the proper syntax, ensuring that the declaration block is
    /// enclosed with parentheses and square brackets. If the syntax is valid, it proceeds to parse the
    /// child elements (individual imports).
    ///
    /// # Errors
    ///
    /// Returns an `NenyrError` if:
    /// - The opening parenthesis `(` is missing after the `Imports` keyword.
    /// - The closing parenthesis `)` is missing at the end of the `Imports` block.
    /// - The opening square bracket `[` is not found to initiate the vector of imports.
    /// - The closing square bracket `]` is missing to terminate the vector.
    /// - There are syntax errors within the import declarations, such as duplicated commas.
    ///
    /// # Returns
    ///
    /// Returns a `NenyrImports` object containing the parsed import statements if successful.
    pub(crate) fn process_imports_method(&mut self) -> NenyrResult<NenyrImports> {
        self.process_next_token()?;

        self.parse_parenthesized_delimiter(
            Some("Ensure that the `Imports` declaration block is enclosed with both an opening and a closing parenthesis. Correct syntax example: `Declare Imports([ ... ])`.".to_string()),
            "The `Imports` block is missing an opening parenthesis `(` after the `Imports` keyword. The parser expected an opening parenthesis to begin the import declarations.",
            Some("Ensure that the `Imports` block includes both an opening and a closing parenthesis. The syntax should follow the correct format: `Declare Imports([ ... ])`.".to_string()),
            "A closing parenthesis `)` is missing for the `Imports` declaration block. The parser expected a closing parenthesis to properly end the import declarations.",
            |parser| {
                let imports = parser.parse_square_bracketed_delimiter(
                    Some("After the opening parenthesis, an opening square bracket `[` is required to properly define the `Imports` declaration. Ensure the syntax follows correct Nenyr conventions, like `Declare Imports([ ... ])`.".to_string()),
                    "The `Imports` declaration block was expected to receive a vector as a value, but an opening square bracket `[` was not found after the opening parenthesis.",
                    Some("Ensure that the vector in the `Imports` declaration is properly closed with a closing square bracket `]`. The correct syntax should look like: `Declare Imports([ ... ])`.".to_string()),
                    "The `Imports` declaration block is missing a closing square bracket `]` to properly close the vector.",
                    Self::process_imports_children,
                )?;

                parser.process_next_token()?;

                Ok(imports)
            },
        )
    }

    /// Processes the child elements of the `Imports` declaration.
    ///
    /// This method iterates through the import statements defined in the `Imports` block and ensures
    /// that each import follows the correct syntax. It checks for proper delimiters, handles duplicate
    /// commas, and aggregates the import patterns into a `NenyrImports` object.
    ///
    /// # Errors
    ///
    /// Returns an `NenyrError` if:
    /// - A duplicated comma is found in the properties block of the `Imports` declaration.
    /// - A new property statement is expected but not found.
    /// - A comma is missing between properties in the `Imports` declaration.
    ///
    /// # Returns
    ///
    /// Returns a `NenyrImports` object containing all parsed import statements if successful.
    fn process_imports_children(&mut self) -> NenyrResult<NenyrImports> {
        let mut imports = NenyrImports::new();

        loop_while_not!(
            self,
            Some("Remove any duplicated commas from the properties block in the `Imports` declaration. Ensure proper syntax by following valid delimiters. Example: `Declare Imports([ Import(' ... '), Import(' ... '), ... ])`.".to_string()),
            "A duplicated comma was found in the properties block of the `Imports` declarations. The parser expected to find a new property statement but none was found.",
            Some("Ensure that a comma is placed after each property definition inside the `Imports` declaration to separate elements correctly. Proper syntax is required for the parser to process the context. Example: `Declare Imports([ Import(' ... '), Import(' ... '), ... ])`.".to_string()),
            "The properties in the `Imports` declaration must be separated by commas. A comma is missing between the properties in the `Imports` declaration. The parser expected a comma to separate elements but did not find one.",
            || self.processing_state.is_block_active(),
            |is_active| self.processing_state.set_block_active(is_active),
            {
                self.process_import_pattern(&mut imports)?;
            }
        );

        self.processing_state.set_block_active(false);

        Ok(imports)
    }

    /// Processes a single import pattern within the `Imports` declaration.
    ///
    /// This method checks if the current token is an `Import` statement, then parses the associated
    /// import path. It ensures that the syntax is valid, including the presence of the necessary
    /// parentheses around the import path. If the import path is valid, it adds the import to the
    /// `NenyrImports` object.
    ///
    /// # Errors
    ///
    /// Returns an `NenyrError` if:
    /// - The `Import` keyword is not followed by an opening parenthesis `(`.
    /// - The import path is not enclosed by parentheses `)`.
    /// - The provided import path is invalid or empty.
    ///
    /// # Returns
    ///
    /// Returns `Ok(())` if the import pattern is processed successfully, otherwise returns an error.
    fn process_import_pattern(&mut self, imports: &mut NenyrImports) -> NenyrResult<()> {
        self.processing_state.set_block_active(true);

        if let NenyrTokens::Import = self.current_token {
            self.process_next_token()?;

            let value = self.parse_parenthesized_delimiter(
                Some("Ensure that an opening parenthesis `(` is placed after the keyword `Import` to properly define the import path. The correct syntax is: `Import('path')`.".to_string()),
                "The statement of `Import` was expecting an open parenthesis `(` after the keyword `Import`, but none was found.",
                Some("Ensure that the import path in the `Import` statement is properly closed with a parenthesis `)`. The correct syntax is: `Import('path')`.".to_string()),
                "The `Import` statement is missing a closing parenthesis `)` after the import path.",
                |parser| parser.parse_string_literal(
                    Some("All `Import` statements require a non-empty string as a path. This path must be either a valid URL or a relative path to an existing archive in the source directory, based on the context path. The correct syntax is: `Import('path')`.".to_string()),
                    "The `Import` declaration expects a path in the form of a non-empty string, but no valid path value was provided.",
                    true
                ),
            )?;

            if !self.is_valid_import(&value, &self.context_path) {
                return Err(NenyrError::new(
                    Some("Ensure that all import values are semantically correct to be validated. Please refer to the documentation to verify the correct way to define import values.".to_string()),
                    self.context_name.clone(),
                    self.context_path.to_string(),
                    self.add_nenyr_token_to_error(&format!("The `{}` import in the `Imports` declaration is an invalid value and could not be validated.", value)),
                    NenyrErrorKind::SyntaxError,
                    self.get_tracing(),
                ));
            }

            imports.add_import(value);

            return Ok(());
        }

        Err(NenyrError::new(
            Some("To properly define the `Imports` vector, ensure that each element is structured as an `Import` pattern containing a valid string argument. The correct syntax should follow this pattern: `Declare Imports([ Import('path'), Import('path'), ... ])`. This means every element within the `Imports` vector must be an `Import` statement with a string value inside the parentheses. Ensure there are no additional properties or unsupported structures within the vector.".to_string()),
            self.context_name.clone(),
            self.context_path.to_string(),
            self.add_nenyr_token_to_error("An invalid property was detected within the `Imports` vector. Each element inside `Imports` should be an `Import` statement."),
            NenyrErrorKind::SyntaxError,
            self.get_tracing(),
        ))
    }
}

#[cfg(test)]
mod tests {
    use crate::NenyrParser;

    #[test]
    fn imports_are_valid() {
        let raw_nenyr = "Imports([
        Import('https://fonts.googleapis.com/css2?family=Matemasie&display=swap'),
        Import('https://fonts.googleapis.com/css2?family=Roboto:ital,wght@0,100;0,300;0,400;0,500;0,700;0,900;1,100;1,300;1,400;1,500;1,700;1,900&display=swap'),
        Import('https://fonts.googleapis.com/css2?family=Bungee+Tint&display=swap'),
        Import('../../../mocks/imports/another_external.css'),
        Import('../../../mocks/imports/external_styles.css'),
        Import('../../../mocks/imports/styles.css'),
    ])";
        let mut parser = NenyrParser::new();
        parser.setup_dependencies(
            raw_nenyr.to_string(),
            "src/interfaces/imports/central.nyr".to_string(),
        );

        let _ = parser.process_next_token();
        let res = parser.process_imports_method().unwrap();

        assert_eq!(
            res.values
                .contains_key("https://fonts.googleapis.com/css2?family=Matemasie&display=swap"),
            true
        );

        assert_eq!(
            res.values.contains_key("https://fonts.googleapis.com/css2?family=Roboto:ital,wght@0,100;0,300;0,400;0,500;0,700;0,900;1,100;1,300;1,400;1,500;1,700;1,900&display=swap"),
            true
        );

        assert_eq!(
            res.values
                .contains_key("https://fonts.googleapis.com/css2?family=Bungee+Tint&display=swap"),
            true
        );

        assert_eq!(
            res.values
                .contains_key("../../../mocks/imports/another_external.css"),
            true
        );

        assert_eq!(
            res.values
                .contains_key("../../../mocks/imports/external_styles.css"),
            true
        );

        assert_eq!(
            res.values.contains_key("../../../mocks/imports/styles.css"),
            true
        );
    }

    #[test]
    fn imports_are_not_valid() {
        let raw_nenyr = "Imports([
        Import('https://fonts.googleapis.com/css2?family=Matemasie&display=swap'),
        Import('https://fonts.googleapis.com/css2?family=Roboto:ital,wght@0,100;0,300;0,400;0,500;0,700;0,900;1,100;1,300;1,400;1,500;1,700;1,900&display=swap'),
        Import('https://fonts.googleapis.com/css2?family=Bungee+Tint&display=swap'),
        Import('../../../mocks/imports/another_external.css'),,
        Import('../../../mocks/imports/external_styles.css'),
        Import('../../../mocks/imports/styles.css'),
    ])";
        let mut parser = NenyrParser::new();
        parser.setup_dependencies(
            raw_nenyr.to_string(),
            "src/interfaces/imports/central.nyr".to_string(),
        );

        let _ = parser.process_next_token();
        assert_eq!(
            format!("{:?}", parser.process_imports_method()),
            "Err(NenyrError { suggestion: Some(\"Remove any duplicated commas from the properties block in the `Imports` declaration. Ensure proper syntax by following valid delimiters. Example: `Declare Imports([ Import(' ... '), Import(' ... '), ... ])`.\"), context_name: None, context_path: \"src/interfaces/imports/central.nyr\", error_message: \"A duplicated comma was found in the properties block of the `Imports` declarations. The parser expected to find a new property statement but none was found. However, found `,` instead.\", error_kind: SyntaxError, error_tracing: NenyrErrorTracing { line_before: Some(\"        Import('https://fonts.googleapis.com/css2?family=Bungee+Tint&display=swap'),\"), line_after: Some(\"        Import('../../../mocks/imports/external_styles.css'),\"), error_line: Some(\"        Import('../../../mocks/imports/another_external.css'),,\"), error_on_line: 5, error_on_col: 64, error_on_pos: 403 } })".to_string()
        );
    }

    #[test]
    fn empty_imports_are_valid() {
        let raw_nenyr = "Imports([ ])";
        let mut parser = NenyrParser::new();
        parser.setup_dependencies(
            raw_nenyr.to_string(),
            "src/interfaces/imports/central.nyr".to_string(),
        );

        let _ = parser.process_next_token();
        assert_eq!(
            format!("{:?}", parser.process_imports_method()),
            "Ok(NenyrImports { values: {} })".to_string()
        );
    }
}

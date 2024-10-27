use crate::{
    error::{NenyrError, NenyrErrorKind},
    loop_while_not,
    tokens::NenyrTokens,
    types::class::NenyrStyleClass,
    validators::identifier::NenyrIdentifierValidator,
    NenyrParser, NenyrResult,
};

impl<'a> NenyrParser<'a> {
    /// Processes the `PanoramicViewer` pattern within a given class. This method handles the
    /// initial parsing of the pattern by ensuring the syntax conforms to the expected format.
    ///
    /// # Arguments
    /// - `class_name`: A reference to the class where the `PanoramicViewer` pattern is declared.
    /// - `style_class`: A mutable reference to a `NenyrStyleClass` instance, which represents
    ///   the class's style configuration being modified.
    ///
    /// # Returns
    /// - `NenyrResult<()>`: A result indicating whether the pattern processing was successful
    ///   or failed with an error.
    ///
    /// # Syntax Example
    /// The method expects a syntax like:
    /// ```nenyr
    /// Class('class_name') {
    ///     PanoramicViewer({
    ///         breakpoint({ ... }),
    ///         ...
    ///     });
    /// }
    /// ```
    ///
    /// # Errors
    /// - Throws a `SyntaxError` if the parentheses or curly braces are missing or malformed.
    pub(crate) fn process_panoramic_pattern(
        &mut self,
        class_name: &str,
        style_class: &mut NenyrStyleClass,
    ) -> NenyrResult<()> {
        self.process_next_token()?;

        // First, parse the expression within the parentheses.
        self.parse_parenthesized_delimiter(
            Some(format!("Ensure that the `PanoramicViewer` pattern in `{}` class is followed by an open parenthesis `(` right after the `PanoramicViewer` keyword. Follow the correct Nenyr syntax: `Class('{}') {{ PanoramicViewer({{ ... }}) }}`.", class_name, class_name)),
            &format!("The `{}` class contains a `PanoramicViewer` pattern declaration that was expected to have an open parenthesis `(` right after the keyword `PanoramicViewer`, but none was found.", class_name),
            Some(format!("Ensure that the `PanoramicViewer` pattern in `{}` class has a closing parenthesis `)` after the argument to properly complete the declaration. Follow the correct Nenyr syntax: `Class('{}') {{ PanoramicViewer({{ ... }}) }}`.", class_name, class_name)),
            &format!("The `{}` class contains a `PanoramicViewer` pattern declaration that is missing a closing parenthesis `)` after the argument.", class_name),
            |parser| {
                // Once inside the parentheses, parse the expression within the curly brackets.
                parser.parse_curly_bracketed_delimiter(
                    Some(format!("After the open parenthesis, an opening curly bracket `{{` is required to properly define the panoramic block in `{}` class. Ensure the panoramic pattern follows the correct Nenyr syntax, such as `Class('{}') {{ PanoramicViewer({{ ... }}) }}`.", class_name, class_name)),
                    &format!("The panoramic pattern in the `{}` class was expected to receive an object as a value, but an opening curly bracket `{{` was not found after the open parenthesis.", class_name),
                    Some(format!("Ensure that the panoramic block in `{}` class is properly closed with a closing curly bracket `}}`. The correct syntax should look like: `Class('{}') {{ PanoramicViewer({{ ... }}) }}`.", class_name, class_name)),
                    &format!("The panoramic pattern in the `{}` class is missing a closing curly bracket `}}` to properly close the panoramic block.", class_name),
                    |parser| parser.process_panoramic_block(class_name, style_class),
                )?;

                // Processes the next token
                parser.process_next_token()
            },
        )
    }

    /// Handles the parsing of individual panoramic blocks, where each breakpoint definition
    /// and their associated rules are validated.
    ///
    /// This method iterates through the panoramic block's content and ensures that every
    /// block follows the proper delimiter syntax (i.e., each breakpoint is followed by a
    /// comma).
    ///
    /// # Arguments
    /// - `class_name`: The class in which the panoramic block is defined.
    /// - `style_class`: A mutable reference to the class's style configuration.
    ///
    /// # Returns
    /// - `NenyrResult<()>`: A result indicating success or providing an error with detailed context.
    ///
    /// # Errors
    /// - Throws an error if delimiters (commas) are missing or misplaced, or if breakpoints are not properly defined.
    fn process_panoramic_block(
        &mut self,
        class_name: &str,
        style_class: &mut NenyrStyleClass,
    ) -> NenyrResult<()> {
        loop_while_not!(
            self,
            Some(format!("Remove any duplicated commas from the panoramic block statement in the `{}` class. Ensure proper syntax by following valid delimiters. Example: `PanoramicViewer({{ breakpoint({{ ... }}), breakpoint({{ ... }}), ... }})`.", class_name)),
            &format!("A duplicated comma was found in the panoramic block in the `{}` class. The parser expected to find a new breakpoint statement but none was found.", class_name),
            Some(format!("Ensure that a comma is placed after each breakpoint definition inside the panoramic block statement in the `{}` class to separate elements correctly. Proper syntax is required for the parser to process the context. Example: `PanoramicViewer({{ breakpoint({{ ... }}), breakpoint({{ ... }}), ... }})`.", class_name)),
            &format!("All breakpoints blocks of the panoramic pattern inside the `{}` class block must be separated by commas. A comma is missing in the breakpoint block of the panoramic pattern definition. The parser expected a comma to separate elements but did not find one.", class_name),
            || self.processing_state.is_complementary_block_active(),
            |is_active| self.processing_state.set_complementary_block_active(is_active),
            {
                self.retrieve_panoramic_identifier(class_name, style_class)?;
            }
        );

        self.processing_state.set_complementary_block_active(false);

        Ok(())
    }

    /// Retrieves and validates the breakpoint identifier within the panoramic block.
    ///
    /// This method ensures that each breakpoint is a valid identifier as defined in the
    /// `Breakpoints` context and applies the proper syntax for parenthesis and curly
    /// braces around the breakpoint's rules.
    ///
    /// # Arguments
    /// - `class_name`: The class in which the breakpoint is declared.
    /// - `style_class`: A mutable reference to the style class being modified.
    ///
    /// # Returns
    /// - `NenyrResult<()>`: A result indicating success or providing an error with details on the invalid identifier.
    ///
    /// # Errors
    /// - Throws an error if the breakpoint identifier is unrecognized or if delimiters are missing.
    fn retrieve_panoramic_identifier(
        &mut self,
        class_name: &str,
        style_class: &mut NenyrStyleClass,
    ) -> NenyrResult<()> {
        let breakpoint_name = self.parse_identifier_literal(
            Some(format!("Ensure that the breakpoint is a valid identifier as specified within the `Breakpoints` declaration block in the central context. The identifier must match one of the names used to define a breakpoint. Example: `Class('{}') {{ PanoramicViewer({{ validBreakpointIdentifier({{ ... }}), ... }}) }}`.", class_name)),
            &format!("The `PanoramicViewer` pattern in the `{}` class contains an unrecognized breakpoint identifier.", class_name),
            true
        )?;

        if !self.is_valid_identifier(&breakpoint_name) {
            return Err(NenyrError::new(
                Some("Ensure that the breakpoint identifier is the same name specified in the `Breakpoints` declaration within the central context. A valid breakpoint name should consist of an alphanumeric identifier with the first character being a letter.".to_string()),
                self.context_name.clone(),
                self.context_path.to_string(),
                self.add_nenyr_token_to_error(&format!("The current `{}` breakpoint of the panoramic pattern in the `{}` class failed to be validated.", breakpoint_name, class_name)),
                NenyrErrorKind::SyntaxError,
                self.get_tracing(),
            ));
        }

        self.processing_state.set_complementary_block_active(true);
        style_class.reset_panoramic_node(&breakpoint_name);

        // First, parse the expression within the parentheses.
        self.parse_parenthesized_delimiter(
            Some(format!("Ensure that each breakpoint statement must be followed by an opening parenthesis. Correct syntax is required for proper declaration. Example: `Class('{}') {{ PanoramicViewer({{ {}({{ ... }}), ... }}) }}`.", class_name, breakpoint_name)),
            &format!("The `{}` breakpoint of the panoramic pattern in the `{}` class is missing an open parenthesis after the breakpoint keyword declaration.", breakpoint_name, class_name),
            Some(format!("Ensure that each breakpoint statement is properly closed with a closing parenthesis. Example: `Class('{}') {{ PanoramicViewer({{ {}({{ ... }}), ... }}) }}`.", class_name, breakpoint_name)),
            &format!("The `{}` breakpoint of the panoramic pattern in the `{}` class is missing a closing parenthesis after the declaration.", breakpoint_name, class_name),
            |parser| {
                // Once inside the parentheses, parse the expression within the curly brackets.
                parser.parse_curly_bracketed_delimiter(
                    Some(format!("After the open parenthesis, ensure that an opening curly bracket `{{` is present to define the `{}` breakpoint block correctly. Example: `Class('{}') {{ PanoramicViewer({{ {}({{ ... }}), ... }}) }}`.", breakpoint_name, class_name, breakpoint_name)),
                    &format!("The `{}` breakpoint of the panoramic pattern in the `{}` class was expected to receive an object as its value, but an opening curly bracket `{{` was not found after the open parenthesis.", breakpoint_name, class_name),
                    Some(format!("Ensure that the `{}` breakpoint's patterns statement is properly closed with a curly bracket `}}`. Correct syntax should be: `Class('{}') {{ PanoramicViewer({{ {}({{ ... }}), ... }}) }}`.", breakpoint_name, class_name, breakpoint_name)),
                    &format!("A missing closing curly bracket `}}` was expected to properly close the `{}` breakpoint's patterns statement in the `{}` class.", breakpoint_name, class_name),
                    |parser| {
                        parser.process_panoramic_children(class_name, &breakpoint_name, style_class)
                    },
                )?;

                parser.process_next_token()
            },
        )
    }

    /// Processes children within a panoramic block for each specified breakpoint.
    ///
    /// This function processes style rules or patterns for a given class within each
    /// breakpoint block, ensuring that all delimiters are correctly handled.
    /// It performs validation to detect duplicated or missing delimiters and
    /// verifies the structure of the child elements within each breakpoint, following
    /// proper syntax requirements.
    ///
    /// # Arguments
    ///
    /// - `class_name`: The name of the class being processed, representing the
    ///   current style context.
    /// - `breakpoint_name`: The name of the breakpoint being handled, which specifies
    ///   the responsive context (e.g., mobile, tablet, desktop).
    /// - `style_class`: A mutable reference to the `NenyrStyleClass` being modified
    ///   for the given class and breakpoint.
    ///
    /// # Returns
    ///
    /// - `NenyrResult<()>`: Returns `Ok(())` if the panoramic block and its
    ///   children are processed successfully. If there is a syntax issue within
    ///   the breakpoint block (e.g., missing or duplicate delimiters), this
    ///   function returns an appropriate `NenyrError`.
    ///
    /// # Errors
    ///
    /// - Returns an error if:
    ///   - Duplicated delimiters are detected inside the breakpoint block. For instance,
    ///     multiple consecutive commas within a block, which the parser flags as incorrect.
    ///   - Missing commas are found between child elements, where each block is expected
    ///     to end with a comma to maintain proper separation.
    ///   - Child elements are incorrectly structured, violating the expected syntax rules
    ///     for nested patterns within a panoramic block.
    fn process_panoramic_children(
        &mut self,
        class_name: &str,
        breakpoint_name: &str,
        style_class: &mut NenyrStyleClass,
    ) -> NenyrResult<()> {
        loop_while_not!(
            self,
            Some(format!("Remove any duplicated commas from the `{}` breakpoint in the `{}` class block to ensure proper syntax. The parser expects every breakpoint block to follow valid delimiters. Example: `Declare Class('{}') {{ PanoramicViewer({{ {}({{ ... }}) }}) }}`.", breakpoint_name, class_name, class_name, breakpoint_name)),
            &format!("A duplicated comma was found inside the `{}` breakpoint in the `{}` class block. The parser expected to find a new breakpoint block, but it was not found.", breakpoint_name, class_name),
            Some(format!("Ensure that a comma is placed after each breakpoint block inside the `{}` class to separate elements correctly. Proper syntax is required for the parser to process the context. Example: `Declare Class('{}') {{ PanoramicViewer({{ {}({{ ... }}) }}) }}`.", class_name, class_name, breakpoint_name)),
            &format!("All breakpoint inside the `{}` class block must be separated by commas. A comma is missing after the breakpoint block definition. The parser expected a comma to separate elements but did not find one.", class_name),
            || self.processing_state.is_internal_block_active(),
            |is_active| self.processing_state.set_internal_block_active(is_active),
            {
                self.process_patterns_methods(class_name, style_class, true, &Some(breakpoint_name.to_string()))?;
            }
        );

        self.processing_state.set_internal_block_active(false);

        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use crate::{types::class::NenyrStyleClass, NenyrParser};

    #[test]
    fn panoramic_viewer_stylesheet_is_valid() {
        let raw_nenyr = "({ myBreakpoint({ Stylesheet({ backgroundColor: 'blue', border: '10px solid red' }) }) })";

        let mut parser = NenyrParser::new(raw_nenyr, "");
        let mut styles = NenyrStyleClass::new("myClassName".to_string(), None);
        let mut style_class = NenyrStyleClass::new("myClassName".to_string(), None);

        styles.add_responsive_style_rule(
            "myBreakpoint".to_string(),
            "_stylesheet".to_string(),
            "background-color".to_string(),
            "blue".to_string(),
        );
        styles.add_responsive_style_rule(
            "myBreakpoint".to_string(),
            "_stylesheet".to_string(),
            "border".to_string(),
            "10px solid red".to_string(),
        );

        let _ = parser.process_panoramic_pattern("myClassName", &mut style_class);

        assert_eq!(style_class, styles);
    }

    #[test]
    fn panoramic_viewer_stylesheet_is_not_valid() {
        let raw_nenyr = "({ myBreakpoint({ Stylesheet( backgroundColor: 'blue', border: '10px solid red' }) }) })";

        let mut parser = NenyrParser::new(raw_nenyr, "");
        let mut style_class = NenyrStyleClass::new("myClassName".to_string(), None);

        assert_eq!(
            format!(
                "{:?}",
                parser.process_panoramic_pattern("myClassName", &mut style_class)
            ),
            "Err(NenyrError { suggestion: Some(\"After the open parenthesis, an opening curly bracket `{` is required to properly define the properties block in `myClassName` class. Ensure the pattern follows the correct Nenyr syntax, such as `Class('myClassName') { Stylesheet({ ... }), Hover({ ... }), ... }`.\"), context_name: None, context_path: \"\", error_message: \"One of the patterns in the `myClassName` class was expected to receive an object as a value, but an opening curly bracket `{` was not found after the open parenthesis. However, found `BackgroundColor` instead.\", error_kind: SyntaxError, error_tracing: NenyrErrorTracing { line_before: None, line_after: None, error_line: Some(\"({ myBreakpoint({ Stylesheet( backgroundColor: 'blue', border: '10px solid red' }) }) })\"), error_on_line: 1, error_on_col: 46, error_on_pos: 45 } })".to_string()
        );
    }

    #[test]
    fn panoramic_viewer_after_is_valid() {
        let raw_nenyr =
            "({ myBreakpoint({ After({ backgroundColor: 'blue', border: '10px solid red' }) }) })";

        let mut parser = NenyrParser::new(raw_nenyr, "");
        let mut styles = NenyrStyleClass::new("myClassName".to_string(), None);
        let mut style_class = NenyrStyleClass::new("myClassName".to_string(), None);

        styles.add_responsive_style_rule(
            "myBreakpoint".to_string(),
            "::after".to_string(),
            "background-color".to_string(),
            "blue".to_string(),
        );
        styles.add_responsive_style_rule(
            "myBreakpoint".to_string(),
            "::after".to_string(),
            "border".to_string(),
            "10px solid red".to_string(),
        );

        let _ = parser.process_panoramic_pattern("myClassName", &mut style_class);

        assert_eq!(style_class, styles);
    }

    #[test]
    fn panoramic_viewer_after_is_not_valid() {
        let raw_nenyr =
            "({ myBreakpoint({ After({ backgroundColor: 'blue', border: '10px solid red' }) ) })";

        let mut parser = NenyrParser::new(raw_nenyr, "");
        let mut style_class = NenyrStyleClass::new("myClassName".to_string(), None);

        assert_eq!(
            format!(
                "{:?}",
                parser.process_panoramic_pattern("myClassName", &mut style_class)
            ),
            "Err(NenyrError { suggestion: Some(\"Ensure that a comma is placed after each breakpoint block inside the `myClassName` class to separate elements correctly. Proper syntax is required for the parser to process the context. Example: `Declare Class('myClassName') { PanoramicViewer({ myBreakpoint({ ... }) }) }`.\"), context_name: None, context_path: \"\", error_message: \"All breakpoint inside the `myClassName` class block must be separated by commas. A comma is missing after the breakpoint block definition. The parser expected a comma to separate elements but did not find one. However, found `)` instead.\", error_kind: SyntaxError, error_tracing: NenyrErrorTracing { line_before: None, line_after: None, error_line: Some(\"({ myBreakpoint({ After({ backgroundColor: 'blue', border: '10px solid red' }) ) })\"), error_on_line: 1, error_on_col: 81, error_on_pos: 80 } })".to_string()
        );
    }
}

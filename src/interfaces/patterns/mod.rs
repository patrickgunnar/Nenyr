use crate::{
    converters::{property::NenyrPropertyConverter, style_pattern::NenyrStylePatternConverter},
    error::{NenyrError, NenyrErrorKind},
    loop_while_not,
    tokens::NenyrTokens,
    types::class::NenyrStyleClass,
    validators::style_syntax::NenyrStyleSyntaxValidator,
    NenyrParser, NenyrResult,
};

impl<'a> NenyrParser<'a> {
    /// Processes the pattern methods declared within a Nenyr style class.
    ///
    /// This method parses tokens that represent patterns such as `Important`, `PanoramicViewer`,
    /// and others, and handles the state transitions required for applying styles. It differentiates
    /// between normal and panoramic contexts and ensures that nested patterns like `PanoramicViewer`
    /// are correctly handled or flagged as errors.
    ///
    /// # Arguments
    /// - `class_name`: A string representing the name of the class being parsed.
    /// - `style_class`: A mutable reference to the `NenyrStyleClass` that is being modified
    ///   based on the parsed patterns.
    /// - `is_panoramic`: A boolean indicating whether the current context is panoramic.
    /// - `breakpoint_name`: An optional string representing a breakpoint, used for handling
    ///   responsive design.
    ///
    /// # Errors
    /// Returns a `NenyrError` if an invalid or nested `PanoramicViewer` is detected, or if
    /// any pattern is declared incorrectly. The error message provides guidance on fixing
    /// the invalid pattern declaration.
    pub(crate) fn process_patterns_methods(
        &mut self,
        class_name: &str,
        style_class: &mut NenyrStyleClass,
        is_panoramic: bool,
        breakpoint_name: &Option<String>,
    ) -> NenyrResult<()> {
        if is_panoramic {
            self.processing_state.set_internal_block_active(true);
        } else {
            self.processing_state.set_block_active(true);
        }

        if let NenyrTokens::Important = self.current_token {
            let is_important = self.retrieve_important_value(class_name)?;

            style_class.set_importance(is_important);

            return Ok(());
        } else if let NenyrTokens::PanoramicViewer = self.current_token {
            if is_panoramic {
                return Err(NenyrError::new(
                    Some(format!("Remove the nested `PanoramicViewer` pattern. The `PanoramicViewer` method must be used as a direct child of the class and cannot be nested within other `PanoramicViewer` declarations. Ensure the method is properly placed as a direct child of the class. Example: `Declare Class('{}') {{ PanoramicViewer({{ ... }}) }}`.", class_name)),
                    self.context_name.clone(),
                    self.context_path.to_string(),
                    self.add_nenyr_token_to_error(&format!("The `{}` class contains a nested `PanoramicViewer` declaration, which is forbidden. Nested `PanoramicViewer` patterns are not allowed in Nenyr syntax.", class_name)),
                    NenyrErrorKind::SyntaxError,
                    self.get_tracing(),
                ));
            }

            return self.process_panoramic_pattern(class_name, style_class);
        } else {
            if let Some(pattern_name) =
                self.convert_nenyr_style_pattern_to_pseudo_selector(&self.current_token)
            {
                return self.handle_parenthesized_curly_bracketed_section(
                    &pattern_name,
                    class_name,
                    is_panoramic,
                    style_class,
                    breakpoint_name,
                );
            }
        }

        Err(NenyrError::new(
            Some(format!("Fix or remove the invalid pattern declaration. Only valid and permitted patterns are allowed within the `{}` class declaration. Please refer to the documentation to verify which patterns are permitted inside classes. Example: `Declare Class('{}') {{ Stylesheet({{ ... }}) }}`.", class_name, class_name)),
            self.context_name.clone(),
            self.context_path.to_string(),
            self.add_nenyr_token_to_error(&format!("The `{}` class contains an invalid pattern statement. Please ensure that all methods within the class are correctly defined and formatted.", class_name)),
            NenyrErrorKind::SyntaxError,
            self.get_tracing(),
        ))
    }

    /// Retrieves the `Important` value associated with a pattern.
    ///
    /// This method parses the `Important` pattern declaration within a class and ensures
    /// that it follows the correct Nenyr syntax, which requires a boolean value (`true` or `false`)
    /// enclosed in parentheses. It sets the importance value for the `NenyrStyleClass` accordingly.
    ///
    /// # Arguments
    /// - `class_name`: A string representing the class name where the `Important` pattern is defined.
    ///
    /// # Returns
    /// Returns a boolean value indicating whether the `Important` flag is set to `true` or `false`.
    ///
    /// # Errors
    /// Returns a `NenyrError` if the `Important` pattern is declared without the necessary parentheses
    /// or if a non-boolean value is provided.
    fn retrieve_important_value(&mut self, class_name: &str) -> NenyrResult<bool> {
        self.process_next_token()?;

        self.parse_parenthesized_delimiter(
            Some(format!("Ensure that the `Important` pattern in `{}` class is followed by an open parenthesis `(` right after the `Important` keyword. Follow the correct Nenyr syntax: `Important(true)` or `Important(false)`.", class_name)),
            &format!("The `{}` class contains an `Important` pattern declaration that was expected to have an open parenthesis `(` right after the keyword `Important`, but none was found.", class_name),
            Some(format!("Ensure that the `Important` pattern in `{}` class has a closing parenthesis `)` after the argument to properly complete the declaration. Follow the correct Nenyr syntax: `Important(true)` or `Important(false)`.", class_name)),
            &format!("The `{}` class contains an `Important` pattern declaration that is missing a closing parenthesis `)` after the argument.", class_name),
            |parser| parser.parse_boolean_literal(
                Some(format!("Ensure that the `Important` pattern in `{}` class is provided with a boolean value (`true` or `false`). If the importance is not required, consider removing the `Important` pattern entirely. Correct syntax: `Important(true)` or `Important(false)`.", class_name)),
                &format!("The `Important` pattern statement in the `{}` class is missing a boolean value. A boolean (`true` or `false`) was expected, but none was found.", class_name),
                true
            ),
        )
    }

    /// Handles sections enclosed in both parentheses and curly brackets for a given pattern.
    ///
    /// This method processes tokens for patterns that require both parentheses and curly brackets,
    /// such as `Stylesheet`, `Hover`, or other similar patterns. It first parses the content within
    /// the parentheses, then delegates the handling of properties enclosed in curly brackets.
    ///
    /// # Arguments
    /// - `pattern_name`: A string representing the name of the pattern being parsed.
    /// - `class_name`: The name of the class in which the pattern is being declared.
    /// - `is_panoramic`: A boolean indicating whether the current context is panoramic.
    /// - `style_class`: A mutable reference to the `NenyrStyleClass` that is being modified
    ///   based on the parsed patterns.
    /// - `breakpoint_name`: An optional string representing a breakpoint for responsive design.
    ///
    /// # Errors
    /// Returns a `NenyrError` if the parentheses or curly brackets are missing or malformed.
    /// It provides detailed error messages guiding the user to fix issues related to
    /// unbalanced or missing delimiters.
    fn handle_parenthesized_curly_bracketed_section(
        &mut self,
        pattern_name: &str,
        class_name: &str,
        is_panoramic: bool,
        style_class: &mut NenyrStyleClass,
        breakpoint_name: &Option<String>,
    ) -> NenyrResult<()> {
        self.process_next_token()?;

        // First, parse the expression within the parentheses.
        self.parse_parenthesized_delimiter(
            Some(format!("Ensure that all patterns inside the `{}` class block declaration are enclosed with both an opening and closing parenthesis. Correct syntax example: `Class('{}') {{ Stylesheet({{ ... }}), Hover({{ ... }}), ... }}`.", class_name, class_name)),
            &format!("One of the patterns in the `{}` class is missing an open parenthesis `(` after the pattern keyword declaration. The parser expected a parenthesis to begin the pattern definition.", class_name),
            Some(format!("Ensure that all patterns within the `{}` class block have both an opening and a closing parenthesis. The syntax should follow the correct format, such as `Class('{}') {{ Stylesheet({{ ... }}), Hover({{ ... }}), ... }}`.", class_name, class_name)),
            &format!("A closing parenthesis `)` is missing for one of the patterns in the `{}` class. The parser expected a closing parenthesis to properly end the pattern declaration.", class_name),
            |parser| {
                // Once inside the parentheses, parse the expression within the curly brackets.
                parser.parse_curly_bracketed_delimiter(
                    Some(format!("After the open parenthesis, an opening curly bracket `{{` is required to properly define the properties block in `{}` class. Ensure the pattern follows the correct Nenyr syntax, such as `Class('{}') {{ Stylesheet({{ ... }}), Hover({{ ... }}), ... }}`.", class_name, class_name)),
                    &format!("One of the patterns in the `{}` class was expected to receive an object as a value, but an opening curly bracket `{{` was not found after the open parenthesis.", class_name),
                    Some(format!("Ensure that the properties block within the pattern in `{}` class is properly closed with a closing curly bracket `}}`. The correct syntax should look like: `Class('{}') {{ Stylesheet({{ ... }}), Hover({{ ... }}), ... }}`.", class_name, class_name)),
                    &format!("One of the patterns in the `{}` class is missing a closing curly bracket `}}` to properly close the properties block.", class_name),
                    |parser| {
                        parser.handle_method_block(
                            pattern_name,
                            class_name,
                            is_panoramic,
                            style_class,
                            breakpoint_name,
                        )
                    },
                )?;

                // Processes the next token
                parser.process_next_token()
            },
        )
    }

    /// Handles the processing of a method block for the given pattern and class name.
    ///
    /// This method determines whether the method block is panoramic based on the
    /// `is_panoramic` flag. If it is, it delegates the processing to the
    /// `process_method_block_on_panoramic` method; otherwise, it calls
    /// `process_method_block`.
    ///
    /// # Parameters
    ///
    /// - `pattern_name`: A string slice that holds the name of the pattern to be processed.
    /// - `class_name`: A string slice that specifies the class name associated with the
    ///   method block.
    /// - `is_panoramic`: A boolean flag indicating if the method block is panoramic.
    /// - `style_class`: A mutable reference to a `NenyrStyleClass` that will be modified
    ///   based on the parsed properties.
    /// - `breakpoint_name`: An optional string that specifies the breakpoint name for
    ///   panoramic blocks.
    ///
    /// # Returns
    ///
    /// This method returns a `NenyrResult<()>`, indicating success or failure of the
    /// parsing process.
    ///
    /// # Panics
    ///
    /// This method will panic if `is_panoramic` is `true` and `breakpoint_name` is
    /// `None` - (unreachable condition).
    fn handle_method_block(
        &mut self,
        pattern_name: &str,
        class_name: &str,
        is_panoramic: bool,
        style_class: &mut NenyrStyleClass,
        breakpoint_name: &Option<String>,
    ) -> NenyrResult<()> {
        if is_panoramic {
            match breakpoint_name {
                Some(name) => self.process_method_block_on_panoramic(
                    pattern_name,
                    class_name,
                    style_class,
                    name,
                ),
                None => unreachable!(),
            }
        } else {
            self.process_method_block(pattern_name, class_name, style_class)
        }
    }

    /// Processes a standard method block for the specified pattern and class.
    ///
    /// This method handles syntax validation by checking for proper delimiter usage
    /// and ensuring that properties are correctly defined within the method block.
    /// It utilizes a loop to continuously validate the state until the block is properly
    /// processed. If any syntax errors are detected, relevant messages will be generated
    /// to guide the user in correcting them.
    ///
    /// # Parameters
    ///
    /// - `pattern_name`: A string slice that identifies the pattern being processed.
    /// - `class_name`: A string slice that represents the name of the class.
    /// - `style_class`: A mutable reference to a `NenyrStyleClass` instance that will
    ///   be updated with the parsed properties.
    ///
    /// # Returns
    ///
    /// Returns a `NenyrResult<()>`, which indicates the success or failure of the
    /// processing operation.
    fn process_method_block(
        &mut self,
        pattern_name: &str,
        class_name: &str,
        style_class: &mut NenyrStyleClass,
    ) -> NenyrResult<()> {
        // Set/Reset the pattern node before inserting into it.
        style_class.reset_pattern_node(pattern_name);

        loop_while_not!(
            self,
            Some(format!("Remove any duplicated commas from the properties block of the patterns statement in the `{}` class. Ensure proper syntax by following valid delimiters. Example: `Stylesheet({{ backgroundColor: 'blue', border: '1px solid red', ... }})`.", class_name)),
            &format!("A duplicated comma was found in the properties block of one of the patterns in the `{}` class. The parser expected to find a new property statement but none was found.", class_name),
            Some(format!("Ensure that a comma is placed after each property definition inside the patterns statement in the `{}` class to separate elements correctly. Proper syntax is required for the parser to process the context. Example: `Stylesheet({{ backgroundColor: 'blue', border: '1px solid red', ... }})`.", class_name)),
            &format!("All properties blocks in patterns inside the `{}` class block must be separated by commas. A comma is missing in the properties block of the patterns definition. The parser expected a comma to separate elements but did not find one.", class_name),
            || self.processing_state.is_nested_block_active(),
            |is_active| self.processing_state.set_nested_block_active(is_active),
            {
                self.retrieve_nenyr_property(pattern_name, class_name, false, "", style_class)?;
            }
        );

        self.processing_state.set_nested_block_active(false);

        Ok(())
    }

    /// Processes a method block specifically for panoramic contexts.
    ///
    /// This method is responsible for managing syntax checks and state resets for
    /// panoramic method blocks. It ensures that properties are correctly defined and
    /// separated by commas within the context of the panoramic viewer. Similar to
    /// `process_method_block`, it validates syntax and provides user guidance in
    /// case of errors.
    ///
    /// # Parameters
    ///
    /// - `pattern_name`: A string slice representing the name of the pattern.
    /// - `class_name`: A string slice that identifies the class name.
    /// - `style_class`: A mutable reference to a `NenyrStyleClass` instance that will
    ///   be modified with the parsed properties.
    /// - `breakpoint_name`: A string slice that specifies the name of the breakpoint
    ///   for the panoramic context.
    ///
    /// # Returns
    ///
    /// Returns a `NenyrResult<()>`, indicating whether the processing was successful or not.
    fn process_method_block_on_panoramic(
        &mut self,
        pattern_name: &str,
        class_name: &str,
        style_class: &mut NenyrStyleClass,
        breakpoint_name: &str,
    ) -> NenyrResult<()> {
        // Set/Reset the pattern node on panoramic node before inserting into it.
        style_class.reset_pattern_node_on_panoramic_node(breakpoint_name, pattern_name);

        loop_while_not!(
            self,
            Some(format!("Remove any duplicated commas from the properties block of the panoramic pattern statement in the `{}` class. Ensure proper syntax by following valid delimiters. Example: `PanoramicViewer({{ {}({{ backgroundColor: 'blue', border: '1px solid red', ... }}) }})`.", class_name, breakpoint_name)),
            &format!("A duplicated comma was found in the properties block of the panoramic pattern in the `{}` class. The parser expected to find a new property statement but none was found.", class_name),
            Some(format!("Ensure that a comma is placed after each property definition inside the panoramic pattern statement in the `{}` class to separate elements correctly. Proper syntax is required for the parser to process the context. Example: `PanoramicViewer({{ {}({{ backgroundColor: 'blue', border: '1px solid red', ... }}) }})`.", class_name, breakpoint_name)),
            &format!("All properties blocks in panoramic pattern inside the `{}` class block must be separated by commas. A comma is missing in the properties block of the panoramic pattern definition. The parser expected a comma to separate elements but did not find one.", class_name),
            || self.processing_state.is_extra_block_active(),
            |is_active| self.processing_state.set_extra_block_active(is_active),
            {
                self.retrieve_nenyr_property(
                    pattern_name,
                    class_name,
                    true,
                    breakpoint_name,
                    style_class,
                )?;
            }
        );

        self.processing_state.set_extra_block_active(false);

        Ok(())
    }

    /// Retrieves a property from a Nenyr pattern and processes its value.
    ///
    /// This method handles the extraction and validation of properties from
    /// Nenyr class patterns, differentiating between panoramic and non-panoramic
    /// contexts. It checks whether the specified property is a valid Nenyr property
    /// or an alias, and retrieves the corresponding value. If the property is
    /// invalid or improperly defined, it generates an appropriate error message
    /// and returns a `NenyrError`.
    ///
    /// # Parameters
    ///
    /// - `pattern_name`: A string slice representing the name of the pattern
    ///   from which to retrieve the property.
    /// - `class_name`: A string slice representing the name of the class
    ///   associated with the pattern.
    /// - `is_panoramic`: A boolean indicating whether the current context is
    ///   panoramic. This affects how properties are processed and validated.
    /// - `breakpoint_name`: A string slice representing the name of the
    ///   breakpoint for which the styles are being defined.
    /// - `style_class`: A mutable reference to a `NenyrStyleClass` instance
    ///   where the retrieved property and its value will be stored.
    ///
    /// # Returns
    ///
    /// Returns a `NenyrResult<()>`, which is an `Ok(())` variant upon successful
    /// retrieval and storage of the property, or a `NenyrError` if any issues
    /// occur during processing.
    ///
    /// # Error Handling
    ///
    /// This method can return a `NenyrError` under the following circumstances:
    ///
    /// - If the property is not recognized as a valid Nenyr property or alias.
    /// - If the expected property format is not adhered to (missing colon, etc.).
    /// - If the assigned value for the property is empty or invalid.
    ///
    /// Each error includes a suggestion for corrective action, helping users
    /// to rectify issues in their Nenyr definitions.
    fn retrieve_nenyr_property(
        &mut self,
        pattern_name: &str,
        class_name: &str,
        is_panoramic: bool,
        breakpoint_name: &str,
        style_class: &mut NenyrStyleClass,
    ) -> NenyrResult<()> {
        if is_panoramic {
            self.processing_state.set_extra_block_active(true);
        } else {
            self.processing_state.set_nested_block_active(true);
        }

        if let Some(property) = self.convert_nenyr_property_to_css_property(&self.current_token) {
            return self.retrieve_nenyr_value(
                pattern_name,
                class_name,
                property,
                is_panoramic,
                breakpoint_name,
                style_class,
            );
        } else if let NenyrTokens::Identifier(nickname) = self.current_token.clone() {
            return self.retrieve_nenyr_value(
                pattern_name,
                class_name,
                nickname,
                is_panoramic,
                breakpoint_name,
                style_class,
            );
        }

        let suggestion = if is_panoramic {
            format!("Ensure that all properties inside the `{}` panoramic pattern in the `{}` class are either an alias or a valid property. Please verify the documentation to know which properties are valid inside the class patterns.", breakpoint_name, class_name)
        } else {
            format!("Ensure that all properties inside the patterns in the `{}` class are either an alias or a valid property. Please verify the documentation to know which properties are valid inside the class patterns.", class_name)
        };

        let error_message = if is_panoramic {
            format!("One of the properties inside the `{}` panoramic pattern in the `{}` class is not either an alias or a valid property.", breakpoint_name, class_name)
        } else {
            format!("One of the properties inside one of the patterns in the `{}` class is not either an alias or a valid property.", class_name)
        };

        Err(NenyrError::new(
            Some(suggestion),
            self.context_name.clone(),
            self.context_path.to_string(),
            self.add_nenyr_token_to_error(&error_message),
            NenyrErrorKind::SyntaxError,
            self.get_tracing(),
        ))
    }

    /// Retrieves the value associated with a Nenyr property and validates it.
    ///
    /// This method is responsible for extracting the value assigned to a
    /// property within a Nenyr pattern, ensuring that the syntax is correct
    /// and the value is valid. It distinguishes between panoramic and
    /// non-panoramic contexts, applying appropriate validation checks.
    ///
    /// # Parameters
    ///
    /// - `pattern_name`: A string slice representing the name of the pattern
    ///   where the property value is defined.
    /// - `class_name`: A string slice representing the name of the class
    ///   that contains the property.
    /// - `property`: A string containing the name of the property whose
    ///   value is being retrieved.
    /// - `is_panoramic`: A boolean indicating whether the context is
    ///   panoramic, which alters the expected value format.
    /// - `breakpoint_name`: A string slice indicating the breakpoint
    ///   context for the styles being defined.
    /// - `style_class`: A mutable reference to a `NenyrStyleClass` instance
    ///   for storing the successfully retrieved property value.
    ///
    /// # Returns
    ///
    /// Returns a `NenyrResult<()>`, which is `Ok(())` if the property value is
    /// successfully retrieved and validated. If there are errors, it returns
    /// a `NenyrError` detailing the issue.
    ///
    /// # Error Handling
    ///
    /// This method can return a `NenyrError` in the following scenarios:
    ///
    /// - If the property is not followed by a colon.
    /// - If the assigned value is empty or not a valid string.
    /// - If the value of the property cannot be validated against Nenyr standards.
    ///
    /// Each error message includes suggestions to help users fix their
    /// definitions.
    fn retrieve_nenyr_value(
        &mut self,
        pattern_name: &str,
        class_name: &str,
        property: String,
        is_panoramic: bool,
        breakpoint_name: &str,
        style_class: &mut NenyrStyleClass,
    ) -> NenyrResult<()> {
        self.process_next_token()?;

        let suggestion = if is_panoramic {
            format!("Ensure that each property is defined with a colon after it. The correct syntax is: `{}({{ pattern({{ {}: 'property value', ... }}), ... }})`.", breakpoint_name, &property)
        } else {
            format!("Ensure that each property is defined with a colon after it. The correct syntax is: `pattern({{ {}: 'property value', ... }})`.", &property)
        };

        let error_message = if is_panoramic {
            format!("The `{}` property inside the `{}` panoramic pattern in the `{}` class is missing a colon after the property keyword definition.", &property, breakpoint_name, class_name)
        } else {
            format!("The `{}` property inside one of the patterns in the `{}` class is missing a colon after the property keyword definition.", &property, class_name)
        };

        self.parse_colon_delimiter(Some(suggestion), &error_message, true)?;

        let suggestion = if is_panoramic {
            format!("Ensure that all properties are assigned non-empty string values. You can either remove the property or specify a non-empty string value for it: `{}({{ pattern({{ {}: 'property value', ... }}), ... }})`.", breakpoint_name, &property)
        } else {
            format!("Ensure that all properties are assigned non-empty string values. You can either remove the property or specify a non-empty string value for it: `pattern({{ {}: 'property value', ... }})`.", &property)
        };

        let error_message = if is_panoramic {
            format!("The `{}` property inside the `{}` panoramic pattern in the `{}` class should receive a non-empty string as a value, but none was found.", &property, breakpoint_name, class_name)
        } else {
            format!("The `{}` property inside one of the patterns in the `{}` class should receive a non-empty string as a value, but none was found.", &property, class_name)
        };

        let value = self.parse_string_literal(Some(suggestion), &error_message, false)?;

        if self.is_valid_style_syntax(&value) {
            if is_panoramic {
                style_class.add_responsive_style_rule(
                    breakpoint_name.to_string(),
                    pattern_name.to_string(),
                    property,
                    value,
                );
            } else {
                style_class.add_style_rule(pattern_name.to_string(), property, value);
            }

            return Ok(());
        }

        let error_message = if is_panoramic {
            format!("The `{}` property inside the `{}` panoramic pattern in the `{}` class contains an invalid value, and it could not be validated.", &property, breakpoint_name, class_name)
        } else {
            format!("The `{}` property inside one of the patterns in the `{}` class contains an invalid value, and it could not be validated.", &property, class_name)
        };

        Err(NenyrError::new(
            Some("Ensure that all values are semantically correct to be validated. Please refer to the documentation to verify the correct way to define values.".to_string()),
            self.context_name.clone(),
            self.context_path.to_string(),
            self.add_nenyr_token_to_error(&error_message),
            NenyrErrorKind::SyntaxError,
            self.get_tracing(),
        ))
    }
}

#[cfg(test)]
mod tests {
    use crate::{types::class::NenyrStyleClass, NenyrParser};

    #[test]
    fn stylesheet_is_valid() {
        let raw_nenyr = "Stylesheet({ backgroundColor: 'blue', border: '10px solid red' })";

        let mut parser = NenyrParser::new(raw_nenyr, "");
        let mut styles = NenyrStyleClass::new("myClassName".to_string(), None);
        let mut style_class = NenyrStyleClass::new("myClassName".to_string(), None);

        styles.add_style_rule(
            "_stylesheet".to_string(),
            "background-color".to_string(),
            "blue".to_string(),
        );
        styles.add_style_rule(
            "_stylesheet".to_string(),
            "border".to_string(),
            "10px solid red".to_string(),
        );

        let _ = parser.process_next_token();
        let _ = parser.process_patterns_methods("myClassName", &mut style_class, false, &None);

        assert_eq!(style_class, styles);
    }

    #[test]
    fn stylesheet_is_not_valid() {
        let raw_nenyr = "Stylesheet{ backgroundColor: 'blue', border: '10px solid red' })";

        let mut parser = NenyrParser::new(raw_nenyr, "");
        let mut style_class = NenyrStyleClass::new("myClassName".to_string(), None);

        assert_eq!(
            format!(
                "{:?}",
                parser.process_patterns_methods("myClassName", &mut style_class, false, &None)
            ),
            "Err(NenyrError { suggestion: Some(\"Fix or remove the invalid pattern declaration. Only valid and permitted patterns are allowed within the `myClassName` class declaration. Please refer to the documentation to verify which patterns are permitted inside classes. Example: `Declare Class('myClassName') { Stylesheet({ ... }) }`.\"), context_name: None, context_path: \"\", error_message: \"The `myClassName` class contains an invalid pattern statement. Please ensure that all methods within the class are correctly defined and formatted. However, found `StartOfFile` instead.\", error_kind: SyntaxError, error_tracing: NenyrErrorTracing { line_before: None, line_after: None, error_line: Some(\"Stylesheet{ backgroundColor: 'blue', border: '10px solid red' })\"), error_on_line: 1, error_on_col: 1, error_on_pos: 0 } })".to_string()
        );
    }

    #[test]
    fn hover_is_valid() {
        let raw_nenyr = "Hover({ backgroundColor: 'blue', border: '10px solid red' })";

        let mut parser = NenyrParser::new(raw_nenyr, "");
        let mut styles = NenyrStyleClass::new("myClassName".to_string(), None);
        let mut style_class = NenyrStyleClass::new("myClassName".to_string(), None);

        styles.add_style_rule(
            ":hover".to_string(),
            "background-color".to_string(),
            "blue".to_string(),
        );
        styles.add_style_rule(
            ":hover".to_string(),
            "border".to_string(),
            "10px solid red".to_string(),
        );

        let _ = parser.process_next_token();
        let _ = parser.process_patterns_methods("myClassName", &mut style_class, false, &None);

        assert_eq!(style_class, styles);
    }

    #[test]
    fn hover_is_not_valid() {
        let raw_nenyr = "Hover({ backgroundColor: 'blue', border: '10px solid red' )";

        let mut parser = NenyrParser::new(raw_nenyr, "");
        let mut style_class = NenyrStyleClass::new("myClassName".to_string(), None);

        assert_eq!(
            format!(
                "{:?}",
                parser.process_patterns_methods("myClassName", &mut style_class, false, &None)
            ),
            "Err(NenyrError { suggestion: Some(\"Fix or remove the invalid pattern declaration. Only valid and permitted patterns are allowed within the `myClassName` class declaration. Please refer to the documentation to verify which patterns are permitted inside classes. Example: `Declare Class('myClassName') { Stylesheet({ ... }) }`.\"), context_name: None, context_path: \"\", error_message: \"The `myClassName` class contains an invalid pattern statement. Please ensure that all methods within the class are correctly defined and formatted. However, found `StartOfFile` instead.\", error_kind: SyntaxError, error_tracing: NenyrErrorTracing { line_before: None, line_after: None, error_line: Some(\"Hover({ backgroundColor: 'blue', border: '10px solid red' )\"), error_on_line: 1, error_on_col: 1, error_on_pos: 0 } })".to_string()
        )
    }

    #[test]
    fn panoramic_is_valid() {
        let raw_nenyr = "PanoramicViewer({ myBreakpoint({ Stylesheet({ backgroundColor: 'blue', border: '10px solid red' }) }) })";

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

        let _ = parser.process_next_token();
        let _ = parser.process_patterns_methods(
            "myClassName",
            &mut style_class,
            false,
            &Some("myBreakpoint".to_string()),
        );

        assert_eq!(style_class, styles);
    }

    #[test]
    fn panoramic_is_not_valid() {
        let raw_nenyr = "PanoramicViewer({ myBreakpoint( Stylesheet({ backgroundColor: 'blue', border: '10px solid red' }) }) })";

        let mut parser = NenyrParser::new(raw_nenyr, "");
        let mut style_class = NenyrStyleClass::new("myClassName".to_string(), None);

        assert_eq!(
            format!(
                "{:?}",
                parser.process_patterns_methods(
                    "myClassName",
                    &mut style_class,
                    true,
                    &Some("myBreakpoint".to_string())
                )
            ),
            "Err(NenyrError { suggestion: Some(\"Fix or remove the invalid pattern declaration. Only valid and permitted patterns are allowed within the `myClassName` class declaration. Please refer to the documentation to verify which patterns are permitted inside classes. Example: `Declare Class('myClassName') { Stylesheet({ ... }) }`.\"), context_name: None, context_path: \"\", error_message: \"The `myClassName` class contains an invalid pattern statement. Please ensure that all methods within the class are correctly defined and formatted. However, found `StartOfFile` instead.\", error_kind: SyntaxError, error_tracing: NenyrErrorTracing { line_before: None, line_after: None, error_line: Some(\"PanoramicViewer({ myBreakpoint( Stylesheet({ backgroundColor: 'blue', border: '10px solid red' }) }) })\"), error_on_line: 1, error_on_col: 1, error_on_pos: 0 } })".to_string()
        );
    }
}

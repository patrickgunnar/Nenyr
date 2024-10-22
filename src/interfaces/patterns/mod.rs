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
    pub(crate) fn process_patterns_methods(
        &mut self,
        class_name: &str,
        style_class: &mut NenyrStyleClass,
        is_panoramic: bool,
        panoramic_name: &Option<String>,
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
                    panoramic_name,
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

    fn handle_parenthesized_curly_bracketed_section(
        &mut self,
        pattern_name: &str,
        class_name: &str,
        is_panoramic: bool,
        style_class: &mut NenyrStyleClass,
        panoramic_name: &Option<String>,
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
                            panoramic_name,
                        )
                    },
                )?;

                // Processes the next token
                parser.process_next_token()
            },
        )
    }

    fn handle_method_block(
        &mut self,
        pattern_name: &str,
        class_name: &str,
        is_panoramic: bool,
        style_class: &mut NenyrStyleClass,
        panoramic_name: &Option<String>,
    ) -> NenyrResult<()> {
        if is_panoramic {
            match panoramic_name {
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

    fn process_method_block_on_panoramic(
        &mut self,
        pattern_name: &str,
        class_name: &str,
        style_class: &mut NenyrStyleClass,
        panoramic_name: &str,
    ) -> NenyrResult<()> {
        // Set/Reset the pattern node on panoramic node before inserting into it.
        style_class.reset_pattern_node_on_panoramic_node(panoramic_name, pattern_name);

        loop_while_not!(
            self,
            Some(format!("Remove any duplicated commas from the properties block of the panoramic pattern statement in the `{}` class. Ensure proper syntax by following valid delimiters. Example: `PanoramicViewer({{ {}({{ backgroundColor: 'blue', border: '1px solid red', ... }}) }})`.", class_name, panoramic_name)),
            &format!("A duplicated comma was found in the properties block of the panoramic pattern in the `{}` class. The parser expected to find a new property statement but none was found.", class_name),
            Some(format!("Ensure that a comma is placed after each property definition inside the panoramic pattern statement in the `{}` class to separate elements correctly. Proper syntax is required for the parser to process the context. Example: `PanoramicViewer({{ {}({{ backgroundColor: 'blue', border: '1px solid red', ... }}) }})`.", class_name, panoramic_name)),
            &format!("All properties blocks in panoramic pattern inside the `{}` class block must be separated by commas. A comma is missing in the properties block of the panoramic pattern definition. The parser expected a comma to separate elements but did not find one.", class_name),
            || self.processing_state.is_extra_block_active(),
            |is_active| self.processing_state.set_extra_block_active(is_active),
            {
                self.retrieve_nenyr_property(
                    pattern_name,
                    class_name,
                    true,
                    panoramic_name,
                    style_class,
                )?;
            }
        );

        self.processing_state.set_extra_block_active(false);

        Ok(())
    }

    fn retrieve_nenyr_property(
        &mut self,
        pattern_name: &str,
        class_name: &str,
        is_panoramic: bool,
        panoramic_name: &str,
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
                panoramic_name,
                style_class,
            );
        } else if let NenyrTokens::Identifier(nickname) = self.current_token.clone() {
            return self.retrieve_nenyr_value(
                pattern_name,
                class_name,
                nickname,
                is_panoramic,
                panoramic_name,
                style_class,
            );
        }

        let suggestion = if is_panoramic {
            format!("Ensure that all properties inside the `{}` panoramic pattern in the `{}` class are either an alias or a valid property. Please verify the documentation to know which properties are valid inside the class patterns.", panoramic_name, class_name)
        } else {
            format!("Ensure that all properties inside the patterns in the `{}` class are either an alias or a valid property. Please verify the documentation to know which properties are valid inside the class patterns.", class_name)
        };

        let error_message = if is_panoramic {
            format!("One of the properties inside the `{}` panoramic pattern in the `{}` class is not either an alias or a valid property.", panoramic_name, class_name)
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

    fn retrieve_nenyr_value(
        &mut self,
        pattern_name: &str,
        class_name: &str,
        property: String,
        is_panoramic: bool,
        panoramic_name: &str,
        style_class: &mut NenyrStyleClass,
    ) -> NenyrResult<()> {
        self.process_next_token()?;

        let suggestion = if is_panoramic {
            format!("Ensure that each property is defined with a colon after it. The correct syntax is: `{}({{ pattern({{ {}: 'property value', ... }}), ... }})`.", panoramic_name, &property)
        } else {
            format!("Ensure that each property is defined with a colon after it. The correct syntax is: `pattern({{ {}: 'property value', ... }})`.", &property)
        };

        let error_message = if is_panoramic {
            format!("The `{}` property inside the `{}` panoramic pattern in the `{}` class is missing a colon after the property keyword definition.", &property, panoramic_name, class_name)
        } else {
            format!("The `{}` property inside one of the patterns in the `{}` class is missing a colon after the property keyword definition.", &property, class_name)
        };

        self.parse_colon_delimiter(Some(suggestion), &error_message, true)?;

        let suggestion = if is_panoramic {
            format!("Ensure that all properties are assigned non-empty string values. You can either remove the property or specify a non-empty string value for it: `{}({{ pattern({{ {}: 'property value', ... }}), ... }})`.", panoramic_name, &property)
        } else {
            format!("Ensure that all properties are assigned non-empty string values. You can either remove the property or specify a non-empty string value for it: `pattern({{ {}: 'property value', ... }})`.", &property)
        };

        let error_message = if is_panoramic {
            format!("The `{}` property inside the `{}` panoramic pattern in the `{}` class should receive a non-empty string as a value, but none was found.", &property, panoramic_name, class_name)
        } else {
            format!("The `{}` property inside one of the patterns in the `{}` class should receive a non-empty string as a value, but none was found.", &property, class_name)
        };

        let value = self.parse_string_literal(Some(suggestion), &error_message, false)?;

        if self.is_valid_style_syntax(&value) {
            if is_panoramic {
                style_class.add_responsive_style_rule(
                    panoramic_name.to_string(),
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
            format!("The `{}` property inside the `{}` panoramic pattern in the `{}` class contains an invalid value, and it could not be validated.", &property, panoramic_name, class_name)
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
